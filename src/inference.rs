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
    let mut state = serializer.serialize_struct("ImageBuffer", 2)?;
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
    let map = HashMap::<String, Vec<u8>>::deserialize(deserializer)?;
    let width = map
        .get("width")
        .and_then(|v| v.first())
        .ok_or_else(|| serde::de::Error::missing_field("width"))?;
    let height = map
        .get("height")
        .and_then(|v| v.first())
        .ok_or_else(|| serde::de::Error::missing_field("height"))?;
    let data = map
        .get("data")
        .ok_or_else(|| serde::de::Error::missing_field("data"))?;
    let image = ImageBuffer::from_raw(*width as u32, *height as u32, data.clone())
        .ok_or_else(|| serde::de::Error::custom("Failed to create ImageBuffer from raw data"))?;
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
    use serde_test::{Token, assert_tokens};

    const TEST_ID: &str = "123e4567-e89b-12d3-a456-426614174000";
    const TEST_ID_STR : &str = "123e4567e89b12d3a456426614174000";

    #[test]
    fn test_infer_item_serialization() {
        let item = InferItem::new(Option::Some(
            uuid::Uuid::parse_str(TEST_ID).unwrap()
        ), ImageBuffer::new(4, 4), true);

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
                Token::U64(2),
                Token::Str("height"),
                Token::U64(2),
                Token::Str("data"),
                Token::Seq { len: Some(12) },
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
}
