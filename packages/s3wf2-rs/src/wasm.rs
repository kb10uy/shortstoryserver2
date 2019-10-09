//! Functionalities for WebAssembly.

use crate::{
    document::{Block, BlockNode, Element, ElementNode},
    parser::Parser,
};
use std::panic;
use wasm_bindgen::prelude::*;
use serde_json::{Value, json};

fn convert_element_type(element: &Element) -> &'static str {
    match element {
        Element::Bold => "bold",
        Element::Italic => "italic",
        Element::Underlined => "underline",
        Element::Deleted => "strikethrough",
        Element::Dotted => "dots",
        Element::Monospaced => "monospace",
        Element::Link => "link",
        Element::Ruby => "ruby",
        Element::Item => "item",
        Element::Newline => "newline",
        Element::Line(_, _) => "line",
        Element::Parameter => "parameter",
    }
}

fn convert_block_type(element: Block) -> &'static str {
    match element {
        Block::Paragraph => "paragraph",
        Block::Section => "section",
        Block::Subsection => "subsection",
        Block::Quotation => "quotation",
        Block::UnorderedList => "list",
        Block::Horizontal => "horizontal",
    }
}

fn convert_element_node<'a>(element: &ElementNode<'a>) -> Value {
    match element {
        ElementNode::Text(text) => Value::String(text.to_string()),
        ElementNode::Surrounded {
            kind,
            children,
            parameters,
        } => {
            let children: Vec<_> = children.iter().map(convert_element_node).collect();
            let parameters: Vec<_> = parameters.iter().map(convert_element_node).collect();
            json!({
                "node_type": convert_element_type(kind).to_string(),
                "children": children,
                "parameters": parameters,
            })
        }
    }
}

fn convert_block_node<'a>(block: &BlockNode<'a>) -> Value {
    let children: Vec<_> = block.children.iter().map(convert_element_node).collect();
    json!({
        "node_type": convert_block_type(block.kind).to_string(),
        "children": children,
    })
}

/// Initializes the library WASM.
#[wasm_bindgen]
pub fn initialize() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

/// Parses the input source.
#[wasm_bindgen]
pub fn parse(source: &str) -> JsValue {
    let parser = Parser::new();
    let parse_result = parser.parse(source);
    let result = match parse_result {
        Ok(document) => {
            let blocks = document.blocks.iter().map(convert_block_node).collect();
            json!({
                "document": Value::Array(blocks),
                "errors": Value::Null,
            })
        }
        Err(errors) => {
            let errors = errors.iter().map(|err| json!({
                "line_number": err.line_number as u32,
                "description": format!("{}", err),
            })).collect();
            json!({
                "document": Value::Null,
                "errors": Value::Array(errors),
            })
        }
    };

    JsValue::from_serde(&result).unwrap()
}
