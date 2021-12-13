use crdts::{CmRDT, Dot, GCounter};
use num::ToPrimitive;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GCounterU32 {
    inner: GCounter<u32>,
}

#[wasm_bindgen]
impl GCounterU32 {
    pub fn new() -> Self {
        Self {
            inner: GCounter::new(),
        }
    }

    pub fn inc(&mut self, actor: u32) -> DotU32 {
        DotU32 {
            inner: self.inner.inc(actor),
        }
    }

    pub fn apply(&mut self, op: DotU32) {
        self.inner.apply(op.inner)
    }

    pub fn read(&self) -> u32 {
        self.inner.read().to_u32().unwrap()
    }
}

#[wasm_bindgen]
pub struct DotU32 {
    inner: Dot<u32>,
}

#[wasm_bindgen]
impl DotU32 {
    pub fn from_json(json_string: String) -> Self {
        Self {
            inner: serde_json::from_str(&json_string).unwrap(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.inner).unwrap()
    }
}
