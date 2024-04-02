use std::fmt::Write;

use anyhow::Error;
use lightningcss::values::string::CowArcStr;
use smallvec::smallvec;

use crate::writer::Writer;

use super::{Decl, NodeList, ToCss};

#[derive(Debug, Clone, PartialEq)]
pub struct Rule<'a> {
    pub selector: String,
    pub nodes: Vec<AstNode<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode<'a> {
    Rule(Rule<'a>),
    Decl(Decl<'a>),
}

impl<'a> AstNode<'a> {
    pub fn decl<S: Into<CowArcStr<'a>>, SS: Into<CowArcStr<'a>>>(
        name: S,
        value: SS,
    ) -> Self {
        Self::Decl(Decl::new(name, value))
    }

    pub fn rule(selector: &str, nodes: Vec<AstNode<'a>>) -> Self {
        Self::Rule(Rule {
            selector: selector.to_string(),
            nodes,
        })
    }
}

impl<'a> Into<NodeList<'a>> for AstNode<'a> {
    fn into(self) -> NodeList<'a> {
        vec![self]
    }
}

impl<'a> ToCss for Rule<'a> {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        writer.write_str(&self.selector)?;
        writer.whitespace()?;
        writer.write_char('{')?;
        writer.indent();
        writer.newline()?;
        for node in self.nodes.iter() {
            node.to_css(writer)?;
        }
        writer.dedent();
        writer.write_char('}')?;
        writer.newline()?;
        Ok(())
    }
}

impl<'a> ToCss for AstNode<'a> {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            Self::Rule(rule) => rule.to_css(writer),
            Self::Decl(decl) => decl.to_css(writer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrowcss_css_macro::css;

    #[test]
    fn test_rule_to_css() {
        let nodes = css!(
            "@media (min-width: 768px)" {
                "color": "red";
                "background-color": "blue";
            }
        );
        let mut w = String::new();
        let mut writer = Writer::default(&mut w);
        nodes.to_css(&mut writer).unwrap();
        assert_eq!(writer.dest, "@media (min-width: 768px) {\n  color: red;\n  background-color: blue;\n}\n");
    }
}
