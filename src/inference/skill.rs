use super::item::InferItem;
use serde_json::{Value, json};
use std::result::Result;

pub trait Skill {
    fn ready(&self) -> bool;
    fn set_ready(&mut self, ready: bool);

    fn extra_state(&self) -> Value {
        Value::Null
    }
    fn restore_extra_state(&mut self, _state: &Value) {
        // Default implementation does nothing
    }

    fn state(&self) -> Value {
        json!({
            "ready": self.ready(),
            "extra": self.extra_state(),
        })
    }

    fn restore_from_state(&mut self, state: &Value) {
        if let Some(ready) = state.get("ready").and_then(Value::as_bool) {
            self.set_ready(ready);
        } else {
            panic!("State does not contain 'ready' field");
        }
        if let Some(extra) = state.get("extra") {
            self.restore_extra_state(extra);
        }
    }

    fn _prepare(&self, item: &InferItem);

    fn prepare(&mut self, item: &InferItem) {
        self._prepare(item);
        self.set_ready(true);
    }
    fn _process(&self, item: &mut InferItem) -> Result<(), &'static str>;

    fn process(&self, item: &mut InferItem) {
        let result = self._process(item);
        match result {
            Ok(_) => item.result = Some(Ok(())),
            Err(e) => item.result = Some(Err(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests_skill_trait {
    use super::*;
    use image::ImageBuffer;

    struct DummySkill {
        ready: bool,
    }

    impl Skill for DummySkill {
        fn ready(&self) -> bool {
            self.ready
        }
        fn set_ready(&mut self, ready: bool) {
            self.ready = ready;
        }
        fn _prepare(&self, _item: &InferItem) {
            // Dummy implementation
        }
        fn _process(&self, _item: &mut InferItem) -> Result<(), &'static str> {
            // Dummy implementation
            Ok(())
        }
    }

    #[test]
    fn test_prepare_and_process() {
        let mut skill = DummySkill { ready: false };
        let mut item = InferItem::new(None, ImageBuffer::new(4, 4), false);
        skill.prepare(&item);
        assert!(skill.ready());
        skill.process(&mut item);
        assert!(item.result.is_some());
        assert!(item.result.as_ref().unwrap().is_ok());
    }

    #[test]
    fn test_state() {
        let mut skill = DummySkill { ready: false };
        let state = skill.state();
        assert_eq!(state.get("ready").and_then(Value::as_bool), Some(false));
        assert_eq!(state.get("extra").and_then(Value::as_null), Some(()));
        skill.set_ready(true);
        let new_state = skill.state();
        assert_eq!(new_state.get("ready").and_then(Value::as_bool), Some(true));
        assert_eq!(new_state.get("extra").and_then(Value::as_null), Some(()));
    }


    #[test]
    fn test_restore_from_state() {
        let mut skill = DummySkill { ready: false };
        let state = json!({
            "ready": true,
            "extra": null,
        });
        skill.restore_from_state(&state);
        assert!(skill.ready());

        // Test with missing fields
        let incomplete_state = json!({
            "ready": false,
        });
        skill.restore_from_state(&incomplete_state);
        assert!(!skill.ready());
    }
}
