use crate::interperter::build;
use dioxus::{prelude::*, rsx::CallBody};
use syn::{parse_str, Result};

mod attributes;
mod build_element;
mod interperter;

pub fn rsx_to_html(text: &str) -> Result<String> {
    let result: CallBody = parse_str(text)?;
    Ok(dioxus::ssr::render_lazy(LazyNodes::new(|factory| {
        build(result, &factory)
    })))
}
