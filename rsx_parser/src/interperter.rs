use dioxus::{
    core::{Attribute, NodeFactory, VNode},
    rsx::{BodyNode, CallBody, ElementAttr, IfmtInput},
};
use syn::{__private::ToTokens, parse2};

use crate::{attributes::attrbute_to_static_str, build_element::build_element};

pub fn build<'a>(rsx: CallBody, factory: &NodeFactory<'a>) -> VNode<'a> {
    let children_built = factory.bump().alloc(Vec::new());
    for (i, child) in rsx.roots.into_iter().enumerate() {
        children_built.push(build_node(child, factory, i.to_string().as_str()));
    }
    factory.fragment_from_iter(children_built.iter())
}

fn build_node<'a>(node: BodyNode, factory: &NodeFactory<'a>, key: &str) -> VNode<'a> {
    let bump = factory.bump();
    match node {
        BodyNode::Text(text) => factory.text(format_args!("{}", text.value())),
        BodyNode::Element(el) => {
            let attributes: &mut Vec<Attribute> = bump.alloc(Vec::new());
            for attr in el.attributes {
                let result: Option<(String, IfmtInput)> = match attr.attr {
                    ElementAttr::AttrText { name, value } => {
                        Some((name.to_string(), parse2(value.into_token_stream()).unwrap()))
                    }

                    ElementAttr::AttrExpression { name, value } => {
                        Some((name.to_string(), parse2(value.into_token_stream()).unwrap()))
                    }

                    ElementAttr::CustomAttrText { name, value } => {
                        Some((name.value(), parse2(value.into_token_stream()).unwrap()))
                    }

                    ElementAttr::CustomAttrExpression { name, value } => {
                        Some((name.value(), parse2(value.into_token_stream()).unwrap()))
                    }

                    ElementAttr::EventTokens { .. } => None,

                    ElementAttr::Meta(_) => None,
                };
                if let Some((name, value)) = result {
                    let name = attrbute_to_static_str(&name).unwrap();
                    let value = bump.alloc(value.format_literal.value());
                    attributes.push(Attribute {
                        name,
                        value,
                        is_static: true,
                        is_volatile: false,
                        namespace: None,
                    })
                }
            }
            let children = bump.alloc(Vec::new());
            for (i, child) in el.children.into_iter().enumerate() {
                let node = build_node(child, factory, i.to_string().as_str());
                children.push(node);
            }
            let tag = bump.alloc(el.name.to_string());
            build_element(
                factory,
                tag,
                &[],
                attributes.as_slice(),
                children.as_slice(),
                Some(format_args!("{}", key)),
            )
        }
        BodyNode::Component(_) => todo!(),
        BodyNode::RawExpr(_) => todo!(),
        BodyNode::Meta(_) => todo!(),
    }
}
