use super::item::InferItem;
use serde::{Deserialize, Serialize};
use std::result::Result;

pub trait Skill<'s>: Serialize + Deserialize<'s> {
    fn ready(&self) -> bool;
    fn _prepare(&self, item: &InferItem) -> Result<(), &'static str>;

    fn prepare(&self, item: &InferItem) -> Result<(), &'static str> {
        self._prepare(item)
    }
    fn _process(&self, item: &mut InferItem) -> Result<(), &'static str>;

    fn process(&self, item: &mut InferItem) -> Result<(), &'static str> {
        if !self.ready() {
            return Err("Skill is not ready");
        }
        self._process(item)
    }
}
