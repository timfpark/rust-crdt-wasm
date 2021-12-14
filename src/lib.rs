use crdts::{CmRDT, Dot, GCounter};
use num::ToPrimitive;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GCounterString {
    inner: GCounter<String>,
}

#[wasm_bindgen]
impl GCounterString {
    pub fn new() -> Self {
        Self {
            inner: GCounter::new(),
        }
    }

    pub fn inc(&mut self, actor: String) -> DotString {
        DotString {
            inner: self.inner.inc(actor),
        }
    }

    pub fn apply(&mut self, op: DotString) {
        self.inner.apply(op.inner)
    }

    pub fn read(&self) -> u32 {
        self.inner.read().to_u32().unwrap()
    }
}

#[wasm_bindgen]
pub struct DotString {
    inner: Dot<String>,
}

#[wasm_bindgen]
impl DotString {
    pub fn from_json(json_string: String) -> Self {
        Self {
            inner: serde_json::from_str(&json_string).unwrap(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.inner).unwrap()
    }
}
