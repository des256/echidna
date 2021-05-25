// Echidna - Codec - Macros

extern crate proc_macro;
use {
    proc_macro::{
        TokenStream,
        TokenTree,
        Delimiter,
        token_stream::IntoIter,
        Spacing,
        Group,
    },
    proc_macro_error::abort,
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

mod tuple;
use tuple::*;

mod r#enum;
use r#enum::*;

mod r#union;
use r#union::*;

// TODO: Expression
pub(crate) type Expression = String;

impl Lexer {

    // Ah...
    pub fn parse_expression(&mut self) -> Option<Expression> {
        panic!("TODO: expressions");
        None
    }
}

/*

fn make_empty_codec(name: &str) -> TokenStream {
    let mut result = "impl Codec for ".to_string();
    result += name;
    result += " {";
    result += " fn decode(b: &[u8]) -> (";
    result += name;
    result += ",usize) { (0,0) }";
    result += " fn encode(&self,b: &mut Vec<u8>) { }";
    result += " }";
    result.parse().unwrap()
}

fn make_tuple_codec(name: &str,field_types: Vec<&str>) -> TokenStream {
    let mut result = "impl Codec for ".to_string();
    result += name;
    result += " {";
    result += " fn decode(b: &[u8]) -> ";
    result += name;
    result += " {";
    result += " let mut ofs = 0usize;";
    for i in 0..field_types.len() {
        result += " let (v";
        result += &i.to_string();
        result += ",s) = ";
        result += field_types[i];
        result += ".decode(&b[ofs]);";
        result += " ofs += s;";
    }
    result += " ((";
    for i in 0..field_types.len() {
        result += "v";
        result += &i.to_string();
        result += ",";
    }
    result += "),ofs)";
    result += " }";
    result += " fn encode(&self,b: &mut Vec<u8>) {";
    for i in 0..field_types.len() {
        result += " self.";
        result += &i.to_string();
        result += ".encode(b);";
    }
    result += " }";
    result.parse().unwrap()
}
*/

// make_struct_codec

fn render_type(t: Type) -> String {
    match t {
        Type::ImplTrait(param_bounds) => { "TODO".to_string() },
        Type::TraitObject(type_param_bounds) => { "TODO".to_string() },
        Type::Path(path) => { "TODO".to_string() },
        Type::Tuple(types) => { "TODO".to_string() },
        Type::Never => {
            "TODO".to_string(),
        },
        Type::RawPtr { mutable,ty } => {
            var mut result = "*".to_string();
            if mutable {
                result += "mut ";
            }
            result += render_type(ty);
            result
        },
        Ref { lifetime,mutable,ty } => {
            var mut result = "&".to_string();
            if let Some(lifetime) = lifetime {
                result += render_lifetime(lifetime);
                result += " ";
            }
            if mutable {
                result += "mut ";
            }
            result += render_type(ty);
            result
        },
        Array { ty,expression } => {
            var mut result = "[".to_string();
            result += render_type(ty);
            result += "; ";
            result += render_expression(expression);
            result += "]";
            result
        },
        Slice(ty) => {
            var mut result = "&[".to_string();
            result += render_type(ty);
            result += "]";
        },
        Inferred => { "TODO".to_string() },
        QPath => { "TODO".to_string() },
        Function { for_lifetimes,prelude,unsafety,abi,parameters,variadic_attributes,result } => { "TODO".to_string() },
    }
}

fn render_struct(s: Struct) -> String {
    let mut result = "impl Codec for ".to_string();
    result += s.identifier;
    result += " { fn decode(b: &[u8]) -> (";
    result += s.identifier;
    result += ",usize) { let mut o = 0usize;";
    for i in 0..s.fields.len() {
        result += " let (v";
        result += &i.to_string();
        result += ",s) = ";
        result += render_type(s.fields[i].ty);
        result += ".decode(&b[o]); o += s;";
    }
    result += " ";
    result += s.identifier;
    result += " { ";
    for i in 0..s.fields.len() {
        result += s.fields[i].identifier;
        result += ": v";
        result += &i.to_string();
        result += ", ";
    }
    result += "} } }";
    result
}

fn render_tuple(t: Tuple) -> String {

}

fn render_enum(e: Enum) -> String {

}

fn render_union(u: Union) -> String {

}

#[proc_macro_derive(codec)]
pub fn derive_codec(stream: TokenStream) -> TokenStream {
    let mut lexer = Lexer::new(stream);
    let item = lexer.parse_item();
    match item {
        Item::Struct(s) => render_struct(s),
        Item::Tuple(t) => render_tuple(t),
        Item::Enum(e) => render_enum(e),
        Item::Union(u) => render_union(u),
    }.parse().unwrap()
}
