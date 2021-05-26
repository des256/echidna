// Echidna - Codec - Macros

use {
    proc_macro::{
        TokenStream,
        TokenTree,
        Delimiter,
        token_stream::IntoIter,
        Spacing,
        Group,
    },
    std::fmt,
};

mod lexer;
use lexer::*;

mod core;
use crate::core::*;

mod r#type;
use r#type::*;

mod path;
use path::*;

mod generics;
use generics::*;

mod r#where;
use r#where::*;

mod r#struct;
use r#struct::*;

mod r#enum;
use r#enum::*;

pub(crate) enum Item {
    Struct(Struct),
    Tuple(Tuple),
    Enum(Enum),
}

impl fmt::Display for Item {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Struct(s) => {
                write!(f,"{}",s)
            },
            Item::Tuple(t) => {
                write!(f,"{}",t)
            },
            Item::Enum(e) => {
                write!(f,"{}",e)
            },
        }
    }
}

fn _dump(stream: &mut proc_macro::token_stream::IntoIter) -> String {
    let mut result = String::new();
    loop {
        let token = match stream.next() {
            None => { return result; },
            Some(token) => token,
        };
        match token {
            TokenTree::Group(group) => {
                match group.delimiter() {
                    Delimiter::Parenthesis => { result += "PAREN("; },
                    Delimiter::Brace => { result += "BRACE("; },
                    Delimiter::Bracket => { result += "BRACKET("; },
                    Delimiter::None => { result += "UNKNOWN("; },
                }
                let mut stream = group.stream().into_iter();
                result += &_dump(&mut stream);
                result += ")";
            },
            TokenTree::Ident(ident) => {
                result += "'";
                result += &ident.to_string();
                result += "'";
            },
            TokenTree::Punct(punct) => {
                result += &punct.to_string();
            },
            TokenTree::Literal(literal) => {
                result += &literal.to_string();
            },
        }
        result += " ";
    }
}

impl Lexer {

    // Item = { Attr } [ Visibility ] Struct | Tuple | Enum | Union .
    pub(crate) fn parse_item(&mut self) -> Option<Item> {
        if let Some(s_t) = self.parse_struct_or_tuple() {
            match s_t {
                StructOrTuple::Struct(s) => {
                    Some(Item::Struct(s))
                },
                StructOrTuple::Tuple(t) => {
                    Some(Item::Tuple(t))
                },
            }
        }
        else if let Some(e) = self.parse_enum() {
            Some(Item::Enum(e))
        }
        else {
            None
        }
    }
}

fn render_expr(_: &Expr) -> String {
    String::new()
}

fn render_path(path: &Path) -> String {
    let mut r = String::new();
    let mut dsc = path.abs;
    for seg in &path.segs {
        if dsc {
            r += "::";
        }
        else {
            dsc = true;
        }
        match seg {
            PathSeg::Ident(ident) => {
                r += &ident;
            },
            PathSeg::Generic(args) => {
                r += "<";
                let mut first = true;
                for arg in args {
                    if first {
                        first = false;
                    }
                    else {
                        r += ",";
                    }
                    match arg {
                        GenericArg::Lifetime(ident) => {
                            r += "\'";
                            r += &ident;
                        },
                        GenericArg::Type(ty) => {
                            r += &render_type(&ty);
                        },
                        GenericArg::Binding { ident,ty } => {
                            r += &ident;
                            r += "=";
                            r += &render_type(ty.as_ref());
                        },
                        GenericArg::Qualifier { ident,path } => {
                            r += &ident;
                            r += " as ";
                            r += &render_path(path.as_ref());
                        },
                    }
                }
                r += ">";
            },
        }
    }
    r
}

fn render_type(ty: &Type) -> String {
    match ty {
        Type::Path(path) => {
            render_path(path)
        },
        Type::Tuple(types) => {
            let mut r = "(".to_string();
            for ty in types {
                r += &render_type(ty);
                r += ",";
            }
            r += ")";
            r
        },
        Type::Array { ty,expr } => {
            let mut r = "[".to_string();
            r += &render_type(ty);
            r += "; ";
            r += &render_expr(expr);
            r += "]";
            r
        },
    }
}

fn render_struct(s: &Struct) -> String {
    let mut r = "impl Codec for ".to_string();
    r += &s.ident;
    r += " { fn decode(b: &[u8]) -> Option<(usize,Self)> { let mut ofs = 0usize; ";
    for field in &s.fields {
        r += "let ";
        r += &field.ident;
        r += "= if let Some((l,";
        r += &field.ident;
        r += ")) = ";
        r += &render_type(field.ty.as_ref());
        r += "::decode(&b[ofs..]) { ofs += l; ";
        r += &field.ident;
        r += " } else { return None; }; ";
    }
    r += " Some((ofs,";
    r += &s.ident;
    r += " { ";
    for field in &s.fields {
        r += &field.ident;
        r += ": ";
        r += &field.ident;
        r += ", ";
    }
    r += "})) } fn encode(&self,b: &mut Vec<u8>) -> usize { let mut ofs = 0usize; ";
    for field in &s.fields {
        r += "ofs += self.";
        r += &field.ident;
        r += ".encode(b); "
    }
    r += "ofs } fn size(&self) -> usize { let mut ofs = 0usize; ";
    for field in &s.fields {
        r += "ofs += self.";
        r += &field.ident;
        r += ".size(); ";
    }
    r += "ofs } }";
    //eprintln!("{}",r);
    r
}

fn render_tuple(t: &Tuple) -> String {
    let mut r = "impl Codec for ".to_string();
    r += &t.ident;
    r += " { fn decode(b: &[u8]) -> Option<(usize,Self)> { let mut ofs = 0usize; ";
    for i in 0..t.fields.len() {
        r += "let f";
        r += &i.to_string();
        r += " = if let Some((l,f)) = ";
        r += &render_type(t.fields[i].ty.as_ref());
        r += "::decode(&b[ofs..]) { ofs += l; f } else { return None; }; ";
    }
    r += " Some((ofs,";
    r += &t.ident;
    r += "(";
    for i in 0..t.fields.len() {
        r += "f";
        r += &i.to_string();
        r += ", ";
    }
    r += "))) } fn encode(&self,b: &mut Vec<u8>) -> usize { let mut ofs = 0usize; ";
    for i in 0..t.fields.len() {
        r += "ofs += self.";
        r += &i.to_string();
        r += ".encode(b); ";
    }
    r += "ofs } fn size(&self) -> usize { let mut ofs = 0usize; ";
    for i in 0..t.fields.len() {
        r += "ofs += self.";
        r += &i.to_string();
        r += ".size(); ";
    }
    r += " ofs } }";
    r
}

fn render_enum(e: &Enum) -> String {
    let mut r = "impl Codec for ".to_string();
    r += &e.ident;
    r += " { fn decode(b: &[u8]) -> Option<(usize,Self)> { if let Some((_,a)) = u32::decode(b) { match a { ";
    for i in 0..e.items.len() {
        r += &i.to_string();
        r += " => ";
        match &e.items[i] {
            EnumItem::Bare(b) => {
                r += "Some((4,";
                r += &e.ident;
                r += "::";
                r += &b.ident;
                r += "))";
            },
            EnumItem::Struct(s) => {
                r += "{ let mut ofs = 4; ";
                for k in 0..s.fields.len() {
                    r += "let ";
                    r += &s.fields[k].ident;
                    r += " = if let Some((l,";
                    r += &s.fields[k].ident;
                    r += ")) = ";
                    r += &render_type(s.fields[k].ty.as_ref());
                    r += "::decode(&b[ofs..]) { ofs += l; ";
                    r += &s.fields[k].ident;
                    r += " } else { return None; };"
                }
                r += "Some((ofs,";
                r += &e.ident;
                r += "::";
                r += &s.ident;
                r += " { ";
                for k in 0..s.fields.len() {
                    r += &s.fields[k].ident;
                    r += ": ";
                    r += &s.fields[k].ident;
                    r += ", ";
                }
                r += "})) }";
            },
            EnumItem::Tuple(t) => {
                r += "{ let mut ofs = 4; ";
                for k in 0..t.fields.len() {
                    r += "let f";
                    r += &k.to_string();
                    r += " = if let Some((l,f)) = ";
                    r += &render_type(t.fields[k].ty.as_ref());
                    r += "::decode(&b[ofs..]) { ofs += l; f } else { return None; }; ";
                }
                r += "Some((ofs,";
                r += &e.ident;
                r += "::";
                r += &t.ident;
                r += "(";
                let mut first = true;
                for k in 0..t.fields.len() {
                    if first {
                        first = false;
                    }
                    else {
                        r += ",";
                    }
                    r += "f";
                    r += &k.to_string();
                }
                r += "))) }";
            },
            EnumItem::Discr(_) => { },
        }
        r += ", ";
    }
    r += "_ => None } } else { None } } fn encode(&self,b: &mut Vec<u8>) -> usize { match self { ";
    for i in 0..e.items.len() {
        match &e.items[i] {
            EnumItem::Bare(b) => {
                r += &e.ident;
                r += "::";
                r += &b.ident;
                r += " => { u32::encode(&";
                r += &i.to_string();
                r += ",b); 4 }";
            },
            EnumItem::Struct(s) => {
                r += &e.ident;
                r += "::";
                r += &s.ident;
                r += " { ";
                for k in 0..s.fields.len() {
                    r += &s.fields[k].ident;
                    r += ", ";
                }
                r += "} => { u32::encode(&";
                r += &i.to_string();
                r += ",b); let mut ofs = 4; ";
                for k in 0..s.fields.len() {
                    r += "ofs += ";
                    r += &s.fields[k].ident;
                    r += ".encode(b); ";
                }
                r += "ofs }";
            },
            EnumItem::Tuple(t) => {
                r += &e.ident;
                r += "::";
                r += &t.ident;
                r += "(";
                let mut first = true;
                for k in 0..t.fields.len() {
                    if first {
                        first = false;
                    }
                    else {
                        r += ",";
                    }
                    r += "f";
                    r += &k.to_string();
                }
                r += ") => { u32::encode(&";
                r += &i.to_string();
                r += ",b); let mut ofs = 4; ";
                for k in 0..t.fields.len() {
                    r += "ofs += f";
                    r += &k.to_string();
                    r += ".encode(b); ";
                }
                r += "ofs }";
            },
            EnumItem::Discr(_) => { },
        }
        r += ", ";
    }
    r += "} } fn size(&self) -> usize { match self { ";
    for i in 0..e.items.len() {
        match &e.items[i] {
            EnumItem::Bare(b) => {
                r += &e.ident;
                r += "::";
                r += &b.ident;
                r += " => 4";
            },
            EnumItem::Struct(s) => {
                r += &e.ident;
                r += "::";
                r += &s.ident;
                r += " { ";
                for k in 0..s.fields.len() {
                    r += &s.fields[k].ident;
                    r += ", ";
                }
                r += "} => { let mut ofs = 4; ";
                for k in 0..s.fields.len() {
                    r += "ofs += ";
                    r += &s.fields[k].ident;
                    r += ".size(); ";
                }
                r += " ofs }";
            },
            EnumItem::Tuple(t) => {
                r += &e.ident;
                r += "::";
                r += &t.ident;
                r += "(";
                let mut first = true;
                for k in 0..t.fields.len() {
                    if first {
                        first = false;
                    }
                    else {
                        r += ",";
                    }
                    r += "f";
                    r += &k.to_string();
                }
                r += ") => { let mut ofs = 4; ";
                for k in 0..t.fields.len() {
                    r += "ofs += f";
                    r += &k.to_string();
                    r += ".size(); ";
                }
                r += " ofs }";
            },
            EnumItem::Discr(_) => { },
        }
        r += ", ";
    }
    r += "} } }";
    //eprintln!("{}",r);
    r
}

#[proc_macro_derive(codec)]
pub fn derive_codec(stream: TokenStream) -> TokenStream {
    let mut lexer = Lexer::new(stream);
    if let Some(item) = lexer.parse_item() {
        //eprintln!("{}",item);
        match item {
            Item::Struct(s) => render_struct(&s),
            Item::Tuple(t) => render_tuple(&t),
            Item::Enum(e) => render_enum(&e),
        }.parse().unwrap()
    }
    else {
        panic!("only `struct` or `enum` supported");
    }
}
