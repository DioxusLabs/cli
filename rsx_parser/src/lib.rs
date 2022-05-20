use std::collections::HashMap;

use crate::{interperter::build, lexer::Lexer};
use dioxus::prelude::*;
use lalrpop_util::ParseError;
use lazy_static::lazy_static;
use lexer::LexicalError;
use parser::Token;
use qp_trie::{wrapper::BString, Trie};

mod ast;
mod attributes;
mod build_element;
mod elements;
mod interperter;
mod lexer;
mod parser;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub rsx);

#[derive(PartialEq)]
pub(crate) enum AttributeScope {
    Global,
    Specific(&'static str),
}

#[derive(PartialEq)]
pub(crate) struct AttributeEntry {
    scope: AttributeScope,
    name: &'static str,
    namespace: Option<&'static str>,
    mapped_to: Option<&'static str>,
}

lazy_static! {
    static ref ATTRIBUTES_MAP: HashMap<&'static str, Vec<AttributeEntry>> = {
        let mut trie: HashMap<&'static str, Vec<AttributeEntry>> = HashMap::new();
        for dioxus_name in attributes::NO_NAMESPACE_ATTRIBUTES {
            let new = AttributeEntry {
                scope: AttributeScope::Global,
                name: dioxus_name,
                namespace: None,
                mapped_to: None,
            };
            if let Some(v) = trie.get_mut(*dioxus_name) {
                v.push(new);
            } else {
                trie.insert(*dioxus_name, vec![new]);
            }
        }
        for (dioxus_name, html_name) in attributes::MAPPED_ATTRIBUTES {
            let new = AttributeEntry {
                scope: AttributeScope::Global,
                name: dioxus_name,
                namespace: None,
                mapped_to: Some(html_name),
            };
            if let Some(v) = trie.get_mut(*dioxus_name) {
                v.push(new);
            } else {
                trie.insert(*dioxus_name, vec![new]);
            }
        }
        for (dioxus_name, html_name) in attributes::STYLE_ATTRIBUTES {
            let new = AttributeEntry {
                scope: AttributeScope::Global,
                name: dioxus_name,
                namespace: Some("style"),
                mapped_to: Some(html_name),
            };
            if let Some(v) = trie.get_mut(*dioxus_name) {
                v.push(new);
            } else {
                trie.insert(*dioxus_name, vec![new]);
            }
        }
        for (dioxus_name, html_name) in attributes::svg::MAPPED_ATTRIBUTES {
            let new = AttributeEntry {
                scope: AttributeScope::Global,
                name: dioxus_name,
                namespace: None,
                mapped_to: Some(html_name),
            };
            if let Some(v) = trie.get_mut(*dioxus_name) {
                v.push(new);
            } else {
                trie.insert(*dioxus_name, vec![new]);
            }
        }
        for (el, attrs) in elements::ELEMENTS_WITHOUT_NAMESPACE {
            for dioxus_name in *attrs {
                let new = AttributeEntry {
                    scope: AttributeScope::Specific(*el),
                    name: dioxus_name,
                    namespace: None,
                    mapped_to: None,
                };
                if let Some(v) = trie.get_mut(*dioxus_name) {
                    v.push(new);
                } else {
                    trie.insert(*dioxus_name, vec![new]);
                }
            }
        }
        for (el, _, attrs) in elements::ELEMENTS_WITH_NAMESPACE {
            for dioxus_name in *attrs {
                let new = AttributeEntry {
                    scope: AttributeScope::Specific(*el),
                    name: dioxus_name,
                    namespace: None,
                    mapped_to: None,
                };
                if let Some(v) = trie.get_mut(*dioxus_name) {
                    v.push(new);
                } else {
                    trie.insert(*dioxus_name, vec![new]);
                }
            }
        }
        for (el, attrs) in elements::ELEMENTS_WITH_ATTRIBUTE_MAPPING {
            for (dioxus_name, html_name) in *attrs {
                let new = AttributeEntry {
                    scope: AttributeScope::Specific(*el),
                    name: dioxus_name,
                    namespace: None,
                    mapped_to: Some(html_name),
                };
                if let Some(v) = trie.get_mut(*dioxus_name) {
                    v.push(new);
                } else {
                    trie.insert(*dioxus_name, vec![new]);
                }
            }
        }
        trie
    };
    static ref ELEMENT_MAP: Trie<BString, Option<&'static str>> = {
        let mut hs = Trie::new();
        for (el, _) in elements::ELEMENTS_WITHOUT_NAMESPACE {
            hs.insert_str(*el, None);
        }
        for (el, _) in elements::ELEMENTS_WITH_ATTRIBUTE_MAPPING {
            hs.insert_str(*el, None);
        }
        for (el, ns, _) in elements::ELEMENTS_WITH_NAMESPACE {
            hs.insert_str(*el, ns.clone());
        }
        hs
    };
}

macro_rules! text_parse {
    ($s:expr, $f:expr, $n:ident) => {
        #[test]
        fn $n() {
            let parser = rsx::RsxParser::new();
            let input = $s;
            let lexer = Lexer::new(input);
            let result = parser.parse(input, lexer);
            assert_eq!(format!("{:?}", result.unwrap()).as_str(), $f);
        }
    };
}

text_parse!(
    r#"rsx!{div{width: "100px",height: "*{x}px",span{color: "red","hello world"}}}"#,
    "rsx! {\n\tdiv {\n\t\twidth: \"100px\",\n\t\theight: \"*{x}px\",\n\t\tspan {\n\t\t\tcolor: \"red\",\n\t\t\t\"hello world\"\n\t\t}\n\t}\n}",
    parse_rsx_1
);

text_parse!(
    r#"rsx!{input{r#type: "text",value: "{{x}}px",}}"#,
    "rsx! {\n\tinput {\n\t\tr#type: \"text\",\n\t\tvalue: \"{{x}}px\",\n\t}\n}",
    parse_rsx_2
);

text_parse!(
    r####"rsx!{r###"hello "## world "#"###}"####,
    "rsx! {\n\tr###\"hello \"## world \"#\"###\n}",
    parse_rsx_3
);

pub fn rsx_to_html(text: &str) -> Result<String, ParseError<usize, Token, LexicalError>> {
    let parser = rsx::RsxParser::new();
    let lexer = Lexer::new(text);
    let result = parser.parse(text, lexer)?;
    Ok(dioxus::ssr::render_lazy(LazyNodes::new(|factory| {
        build(result, &factory)
    })))
}
