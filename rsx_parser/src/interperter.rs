use dioxus::core::{Attribute, NodeFactory, VNode};

use crate::{
    ast::{Node, RsxCall},
    build_element::build_element,
    AttributeScope, ATTRIBUTES_MAP,
};

pub fn build<'a>(rsx: RsxCall<'a>, factory: &NodeFactory<'a>) -> VNode<'a> {
    let children_built = factory.bump().alloc(Vec::new());
    for child in rsx.0 {
        children_built.push(build_node(child, factory));
    }
    factory.fragment_from_iter(children_built.iter())
}

fn build_node<'a>(node: Node<'a>, factory: &NodeFactory<'a>) -> VNode<'a> {
    let bump = factory.bump();
    match node {
        Node::Element(element) => {
            let tag = element.tag;
            let attributes = bump.alloc(Vec::new());
            for attr in element.attributes {
                if let Some(entries) = ATTRIBUTES_MAP.get(attr.name) {
                    if let Some(entry) = entries.iter().find(|entry| match entry.scope {
                        AttributeScope::Global => true,
                        AttributeScope::Specific(scope_tag) => scope_tag == tag,
                    }) {
                        let name = entry.mapped_to.unwrap_or(entry.name);
                        let value = bump.alloc(attr.value.to_string());

                        attributes.push(Attribute {
                            name,
                            value: value.as_str(),
                            is_static: true,
                            is_volatile: false,
                            namespace: entry.namespace,
                        })
                    }
                }
            }
            let key = None;
            let children = bump.alloc(Vec::new());
            for child in element.children {
                children.push(build_node(child, factory));
            }
            build_element(
                factory,
                tag,
                &[],
                attributes.as_slice(),
                children.as_slice(),
                key,
            )
        }
        Node::Text(text) => {
            let text: String = text.1.iter().map(|v| v.to_string()).collect();
            factory.text(format_args!("{}", text))
        }
    }
}
