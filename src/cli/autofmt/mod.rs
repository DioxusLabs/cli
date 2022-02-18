use std::fmt::{self, Write};

use dioxus_macro_inner::rsx::*;

fn fmt_block(block: &str) -> Option<String> {
    let parsed: CallBody = syn::parse_str(block).ok()?;

    let mut buf = String::new();

    for node in parsed.roots.iter() {
        write_ident(&mut buf, node, 0).ok()?;
    }

    Some(buf)
}

fn write_ident(buf: &mut dyn Write, node: &BodyNode, indent: usize) -> fmt::Result {
    match node {
        BodyNode::Element(el) => {
            let Element {
                name,
                key,
                attributes,
                children,
                _is_static,
            } = el;

            write_tabs(buf, indent)?;
            writeln!(buf, "{name} {{")?;
            if let Some(key) = key {
                let key = key.value();
                write_tabs(buf, indent + 1)?;
                write!(buf, "key: \"{key}\"")?;
                if !attributes.is_empty() {
                    writeln!(buf, ",")?;
                }
            }
            for attr in attributes {
                write_tabs(buf, indent + 1)?;
                match &attr.attr {
                    ElementAttr::AttrText { name, value } => {
                        writeln!(buf, "{name}: \"{value}\",", value = value.value())?;
                    }
                    ElementAttr::AttrExpression { name, value } => todo!(),
                    ElementAttr::CustomAttrText { name, value } => todo!(),
                    ElementAttr::CustomAttrExpression { name, value } => todo!(),
                    ElementAttr::EventTokens { name, tokens } => {
                        //
                        todo!()
                    }
                    ElementAttr::EventClosure { name, closure } => {
                        use quote::quote;
                        let toks = quote!(fn main() { #closure });
                        let out = toks.to_string();

                        dbg!(&out);

                        let input = syn::parse_file(&out).unwrap();
                        let pretty = prettyplease::unparse(&input);

                        writeln!(buf, "{name}: {pretty},",)?;
                    }
                }
            }

            for child in children {
                write_ident(buf, child, indent + 1)?;
            }
            write_tabs(buf, indent)?;
            writeln!(buf, "}}")?;
        }
        BodyNode::Component(_) => {
            //
            // write!(buf, "{}", " ".repeat(ident))
        }
        BodyNode::Text(_) => {
            //
            // write!(buf, "{}", " ".repeat(ident))
        }
        BodyNode::RawExpr(_) => {
            //
            // write!(buf, "{}", " ".repeat(ident))
        }
    }

    Ok(())
}

#[test]
fn formats_block() {
    let block = r#"
        div {
                                    div {
                                    class: "asd",
                                    class: "asd",class: "asd",class: "asd",class: "asd",class: "asd",
                                    key: "ddd",
                                    onclick: move |_| {},
            }
        }
    "#;

    let formatted = fmt_block(block).unwrap();

    print!("{formatted}");
}

fn write_tabs(f: &mut dyn Write, num: usize) -> std::fmt::Result {
    for _ in 0..num {
        write!(f, "    ")?
    }
    Ok(())
}
