//! Functionalities for WebAssembly.

use std::panic;
use wasm_bindgen::prelude::*;
use crate::document::{Block, BlockNode, Element, ElementNode, Document};

#[wasm_bindgen]
pub struct Node {
    node_type: String,
    children: Box<[JsValue]>,
    parameters: Box<[JsValue]>,
}

fn convert_element_node<'a>(element: &ElementNode<'a>) -> JsValue {
    match element {
        ElementNode::Text(text) => JsValue::from_str(text),
        ElementNode::Surrounded { kind, children, parameters } => {
            JsValue::from(Some(Node {
                node_type: "no".to_string(),
                children: vec![].into_boxed_slice(),
                parameters: vec![].into_boxed_slice(),
            }))
        }
    }
}

/// Initializes the library WASM.
#[wasm_bindgen]
pub fn initialize() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn a() -> JsValue {
}
