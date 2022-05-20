use logos::Logos;
use std::fmt::{self, Debug, Formatter};

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Value<'a> {
    #[regex(r#"((\{\{)|(\}\})|([^{}]))*"#, |t| t.slice())]
    Constant(&'a str),

    #[regex(r"\{[\w_][\w\d_]*:?((#?\?)|(:?.?[.<>^]?))?\}", |t|{
        let mut text = t.slice();
        text = &text[1..text.len() - 1];
        text.rsplit_once(':').map(|t| t.0).unwrap_or(text)
    }, priority = 2)]
    Variable(&'a str),

    #[error]
    Error,
}

impl<'a> Value<'a> {
    pub fn to_string(&self) -> String {
        match self {
            Value::Constant(s) => s.to_string(),
            Value::Variable(s) => format!("{{{}}}", s),
            _ => todo!(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Values<'a>(pub String, pub Vec<Value<'a>>, pub String);

impl Values<'_> {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for val in &self.1 {
            match &val {
                Value::Constant(s) => result += s,
                Value::Variable(s) => result += format!("{{{}}}", s).as_str(),
                Value::Error => result += "!error!",
            }
        }
        result
    }
}

impl fmt::Display for Values<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut text = self.0.clone();
        text += &self.to_string();
        text += &self.2;
        f.write_str(&text)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct RsxCall<'a>(pub Vec<Node<'a>>);

impl Debug for RsxCall<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("rsx! {\n")?;
        for node in &self.0 {
            node.fmt(1, f)?
        }
        f.write_str("}")?;
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub enum Node<'a> {
    Element(Element<'a>),
    Text(Values<'a>),
}

impl Node<'_> {
    fn fmt(&self, padding: usize, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Node::Text(s) => {
                f.write_str("\t".repeat(padding).as_str())?;
                std::fmt::Display::fmt(s, f)?;
                f.write_str("\n")
            }
            Node::Element(s) => s.fmt(padding, f),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Element<'a> {
    pub tag: &'a str,
    pub attributes: Vec<AttributeSet<'a>>,
    pub children: Vec<Node<'a>>,
}

impl Element<'_> {
    fn fmt(&self, mut padding: usize, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&"\t".repeat(padding))?;
        f.write_str(self.tag)?;
        f.write_str(" {\n")?;
        padding += 1;
        for attr in &self.attributes {
            f.write_str("\t".repeat(padding).as_str())?;
            f.write_str(attr.name)?;
            f.write_str(": ")?;
            std::fmt::Display::fmt(&attr.value, f)?;
            f.write_str(",\n")?;
        }
        for c in &self.children {
            c.fmt(padding, f)?;
        }
        padding -= 1;
        f.write_str(&"\t".repeat(padding))?;
        f.write_str("}\n")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeSet<'a> {
    pub name: &'static str,
    pub value: Values<'a>,
}
