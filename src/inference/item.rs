use image::ImageBuffer;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct InferItem {
    #[serde(with = "uuid::serde::simple")]
    pub id: uuid::Uuid,
    #[serde(
        serialize_with = "image_serialize",
        deserialize_with = "image_deserialize"
    )]
    pub image: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    pub result: Option<Result<(), String>>,
    pub debug: bool,
}

impl Debug for InferItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InferItem {{ id: {}, image: {:?}, result: {:?}, debug: {} }}",
            self.id,
            (self.image.width(), self.image.height()),
            self.result,
            self.debug
        )
    }
}

fn image_serialize<S>(
    image: &ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut state = serializer.serialize_struct("ImageBuffer", 3)?;
    state.serialize_field("width", &image.width())?;
    state.serialize_field("height", &image.height())?;
    state.serialize_field("data", &image.as_raw())?;
    state.end()
}

fn image_deserialize<'de, D>(
    deserializer: D,
) -> Result<ImageBuffer<image::Rgb<u8>, Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut map: HashMap<String, serde_json::Value> = HashMap::deserialize(deserializer)?;
    let width = map
        .get("width")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| serde::de::Error::missing_field("width"))? as u32;
    let height = map
        .get("height")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| serde::de::Error::missing_field("height"))? as u32;
    let data = map
        .get("data")
        .and_then(|v| v.as_array())
        .and_then(|v| {
            // Convert the array of values to a flat Vec<u8>
            let mut flat_data = Vec::with_capacity(v.len() * 3);
            for value in v {
                if let Some(byte) = value.as_u64() {
                    flat_data.push(byte as u8);
                } else {
                    return None; // Invalid data type
                }
            }
            Some(flat_data)
        })
        .ok_or_else(|| serde::de::Error::missing_field("data"))?;

    let image = ImageBuffer::from_vec(width, height, data)
        .ok_or_else(|| serde::de::Error::custom("Invalid image data"))?;

    Ok(image)
}

impl InferItem {
    pub fn new(
        id: Option<uuid::Uuid>,
        image: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
        debug: bool,
    ) -> Self {
        Self {
            id: id.unwrap_or(uuid::Uuid::new_v4()),
            image,
            result: None,
            debug,
        }
    }
}

#[cfg(test)]
mod inference_tests {
    use super::InferItem;
    use image::ImageBuffer;
    use serde_json;
    use serde_test::{Token, assert_tokens};

    const TEST_ID: &str = "123e4567-e89b-12d3-a456-426614174000";
    const TEST_ID_STR: &str = "123e4567e89b12d3a456426614174000";

    #[test]
    fn test_infer_item_serialization() {
        let item = InferItem::new(
            Some(uuid::Uuid::parse_str(TEST_ID).unwrap()),
            ImageBuffer::new(4, 4),
            true,
        );
        assert_tokens(
            &item,
            &[
                Token::Struct {
                    name: "InferItem",
                    len: 4,
                },
                Token::Str("id"),
                Token::Str(&TEST_ID_STR),
                Token::Str("image"),
                Token::Struct {
                    name: "ImageBuffer",
                    len: 3,
                },
                Token::Str("width"),
                Token::U32(4),
                Token::Str("height"),
                Token::U32(4),
                Token::Str("data"),
                Token::Seq { len: Some(48) },
                Token::Bytes(&[]),
                Token::SeqEnd,
                Token::StructEnd,
                Token::Str("result"),
                Token::None,
                Token::Str("debug"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_infer_item_serde_equal() {
        let item = InferItem::new(
            Some(uuid::Uuid::parse_str(TEST_ID).unwrap()),
            ImageBuffer::new(4, 4),
            true,
        );
        let serialized = serde_json::to_string(&item).unwrap();
        let deserialized: InferItem = serde_json::from_str(&serialized).unwrap();
        assert_eq!(item, deserialized);
    }
}
