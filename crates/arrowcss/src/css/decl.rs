use std::{
    fmt::Write,
    ops::{Deref, DerefMut},
};

use smallvec::{smallvec, SmallVec};
use smol_str::SmolStr;

use super::ToCss;
use crate::writer::Writer;

#[derive(Clone, Debug, PartialEq)]
pub struct Decl {
    pub name: SmolStr,
    pub value: SmolStr,
}

impl Decl {
    pub fn new(name: impl Into<SmolStr>, value: impl Into<SmolStr>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

// pub fn decl<'a, S: Into<SmolStr>>(name: S, value: S) -> CssDecl {
//     CssDecl::new(name, value)
// }

impl<A: Into<SmolStr>, B: Into<SmolStr>> From<(A, B)> for Decl {
    fn from(val: (A, B)) -> Self {
        Decl::new(val.0.into(), val.1.into())
    }
}

impl<A: Into<SmolStr>, B: Into<SmolStr>> FromIterator<(A, B)> for DeclList {
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DeclList(pub SmallVec<[Decl; 1]>);

impl IntoIterator for DeclList {
    type Item = Decl;
    type IntoIter = smallvec::IntoIter<[Decl; 1]>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const N: usize> From<[Decl; N]> for DeclList {
    fn from(decls: [Decl; N]) -> Self {
        Self(decls.into_iter().collect())
    }
}

impl Deref for DeclList {
    type Target = [Decl];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DeclList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Decl> for DeclList {
    fn from(decl: Decl) -> Self {
        Self(smallvec![decl])
    }
}

impl From<Vec<Decl>> for DeclList {
    fn from(decl: Vec<Decl>) -> Self {
        Self(decl.into())
    }
}

impl FromIterator<Decl> for DeclList {
    fn from_iter<T: IntoIterator<Item = Decl>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl DeclList {
    pub fn new() -> Self {
        Self(smallvec![])
    }

    pub fn multi<D: Into<Decl>, I: IntoIterator<Item = D>>(decls: I) -> Self {
        Self(decls.into_iter().map(Into::into).collect())
    }
}

impl ToCss for &Decl {
    fn to_css<W>(self, writer: &mut Writer<W>) -> Result<(), std::fmt::Error>
    where
        W: Write,
    {
        writer.write_str(&self.name)?;
        writer.write_str(":")?;
        writer.whitespace()?;
        writer.write_str(&self.value)?;
        writer.write_str(";")?;
        writer.newline()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_css_decl_macro() {
        // let decls: NodeList = css! {
        //     "color": "red";
        //     "@media" {
        //         "display": "flex";
        //     }
        //     // "background-color": "blue";
        // };

        // assert_eq!(
        //     decls,
        //     vec![
        //         AstNode::Decl(Decl::new("color", "red")),
        //         // AstNode::Decl(Decl::new("background-color", "blue")),
        //     ]
        // );
    }
}
