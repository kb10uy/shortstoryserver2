//! Functionalities for WebAssembly.

use crate::{
    document::{Block, BlockNode, CharacterSet, CharacterType, Element, ElementNode, LineType},
    parser::Parser,
};
use serde_json::{json, Map, Value};
use std::panic;
use wasm_bindgen::prelude::*;

fn convert_element_type(et: &Element) -> &'static str {
    match et {
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
        ElementNode::Text(text) => Value::String((*text).to_string()),
        ElementNode::Surrounded {
            kind,
            children,
            parameters,
        } => {
            let mut result = Map::with_capacity(4);
            let children: Vec<_> = children.iter().map(convert_element_node).collect();
            let parameters: Vec<_> = parameters.iter().map(convert_element_node).collect();
            result.insert("children".to_string(), Value::Array(children));
            result.insert("parameters".to_string(), Value::Array(parameters));
            result.insert(
                "node_type".to_string(),
                Value::String(convert_element_type(kind).to_string()),
            );

            if let Element::Line(id, inline) = kind {
                result.insert("character_id".to_string(), Value::String(id.to_owned()));
                result.insert(
                    "inline".to_string(),
                    Value::Bool(*inline == LineType::Inline),
                );
            }

            Value::Object(result)
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

fn convert_character_set(characters: &CharacterSet) -> Value {
    let converted = characters
        .characters()
        .map(|(n, t)| match t {
            CharacterType::Male(i, dn) => (
                n.to_owned(),
                json!({
                    "color": format!("male-{}", i),
                    "display_name": dn,
                }),
            ),
            CharacterType::Female(i, dn) => (
                n.to_owned(),
                json!({
                    "color": format!("female-{}", i),
                    "display_name": dn,
                }),
            ),
            CharacterType::Mob(i, dn) => (
                n.to_owned(),
                json!({
                    "color": format!("mob-{}", i),
                    "display_name": dn,
                }),
            ),
            CharacterType::Custom(c, dn) => (
                n.to_owned(),
                json!({
                    "color": format!("#{}", c),
                    "display_name": dn,
                }),
            ),
        })
        .collect();

    Value::Object(converted)
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
                "document": {
                    "blocks": Value::Array(blocks),
                    "characters": convert_character_set(&document.characters),
                },
                "errors": Value::Null,
            })
        }
        Err(errors) => {
            let errors = errors
                .iter()
                .map(|err| {
                    json!({
                        "line_number": err.line_number as u32,
                        "description": format!("{}", err),
                    })
                })
                .collect();
            json!({
                "document": Value::Null,
                "errors": Value::Array(errors),
            })
        }
    };

    JsValue::from_serde(&result).unwrap()
}
