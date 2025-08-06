use super::item::InferItem;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::result::Result;

pub trait Skill<'s>: Serialize + Deserialize<'s> {
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
