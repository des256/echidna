// Echidna - Codec - Macros

use crate::*;

pub(crate) struct BareItem {
    pub(crate) attrs: Vec<Group>,
    pub(crate) visibility: Visibility,
    pub(crate) ident: String,
}

impl fmt::Display for BareItem {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut a = String::new();
        for attr in &self.attrs {
            a += &format!("#[{}] ",attr);
        }
        if let Visibility::Private = self.visibility {
        }
        else {
            a += &format!("{} ",self.visibility);
        }
        a += &format!("{} ",self.ident);
        write!(f,"{}",a)
    }
}

pub(crate) struct StructItem {
    pub(crate) attrs: Vec<Group>,
    pub(crate) visibility: Visibility,
    pub(crate) ident: String,
    pub(crate) fields: Vec<StructField>,
}

impl fmt::Display for StructItem {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut a = String::new();
        for attr in &self.attrs {
            a += &format!("#[{}] ",attr);
        }
        if let Visibility::Private = self.visibility {
        }
        else {
            a += &format!("{} ",self.visibility);
        }
        a += &format!("{} {{ ",self.ident);
        for field in &self.fields {
            a += &format!("{}, ",field);
        }
        a += "}}";
        write!(f,"{}",a)
    }
}

pub(crate) struct TupleItem {
    pub(crate) attrs: Vec<Group>,
    pub(crate) visibility: Visibility,
    pub(crate) ident: String,
    pub(crate) fields: Vec<TupleField>,
}

impl fmt::Display for TupleItem {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut a = String::new();
        for attr in &self.attrs {
            a += &format!("#[{}] ",attr);
        }
        if let Visibility::Private = self.visibility {
        }
        else {
            a += &format!("{} ",self.visibility);
        }
        a += &format!("{} (",self.ident);
        for field in &self.fields {
            a += &format!("{}, ",field);
        }
        a += ")";
        write!(f,"{}",a)
    }
}

pub(crate) struct DiscrItem {
    pub(crate) attrs: Vec<Group>,
    pub(crate) visibility: Visibility,
    pub(crate) ident: String,
    pub(crate) expr: Expr,
}

impl fmt::Display for DiscrItem {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut a = String::new();
        for attr in &self.attrs {
            a += &format!("#[{}] ",attr);
        }
        if let Visibility::Private = self.visibility {
        }
        else {
            a += &format!("{} ",self.visibility);
        }
        a += &format!("{} = {}",self.ident,self.expr);
        write!(f,"{}",a)
    }
}

pub(crate) enum EnumItem {
    Bare(BareItem),
    Struct(StructItem),
    Tuple(TupleItem),
    Discr(DiscrItem),
}

impl fmt::Display for EnumItem {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnumItem::Bare(b) => write!(f,"{}",b),
            EnumItem::Struct(s) => write!(f,"{}",s),
            EnumItem::Tuple(t) => write!(f,"{}",t),
            EnumItem::Discr(d) => write!(f,"{}",d),
        }
    }
}

pub(crate) struct Enum {
    pub(crate) attrs: Vec<Group>,
    pub(crate) visibility: Visibility,
    pub(crate) ident: String,
    pub(crate) generics: Vec<Generic>,
    pub(crate) wheres: Vec<Where>,
    pub(crate) items: Vec<EnumItem>,
}

impl fmt::Display for Enum {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut a = String::new();
        for attr in &self.attrs {
            a += &format!("#[{}] ",attr);
        }
        if let Visibility::Private = self.visibility {
        }
        else {
            a += &format!("{} ",self.visibility);
        }
        a += &format!("enum {}",self.ident);
        if self.generics.len() > 0 {
            a += "<";
            let mut first = true;
            for generic in &self.generics {
                if first {
                    first = false;
                }
                else {
                    a += ", ";
                }
                a += &format!("{}",generic);
            }
            a += ">";
        }
        if self.wheres.len() > 0 {
            a += " where ";
            let mut first = true;
            for w in &self.wheres {
                if first {
                    first = false;
                }
                else {
                    a += ", ";
                }
                a += &format!("{}",w);
            }
        }
        a += "{ ";
        for item in &self.items {
            a += &format!("{}, ",item);
        }
        a += "}";
        write!(f,"{}",a)
    }
}

impl Lexer {

    // EnumItemStruct = `{` [ StructField { `,` StructField } [ `,` ] ] `}` .
    // EnumItemTuple = `(` [ TupleField { `,` TupleField } [ `,` ] ] `)` .
    // EnumItemDiscr = `=` Expr .
    // EnumItem = { Attr } [ Visibility ] IDENTIFIER [ EnumItemStruct | EnumItemTuple | EnumItemDiscr ] .
    pub(crate) fn parse_enum_item(&mut self) -> Option<EnumItem> {
        let mut attrs = Vec::<Group>::new();
        while let Some(attr) = self.parse_attr() {
            attrs.push(attr);
        }
        let visibility = self.parse_visibility();
        if let Some(ident) = self.parse_some_ident() {
            if let Some(group) = self.parse_paren_group() {
                let mut lexer = Lexer::new(group.stream());
                let mut fields = Vec::<TupleField>::new();
                while let Some(field) = lexer.parse_tuple_field() {
                    fields.push(field);
                    lexer.parse_punct(',');
                }
                Some(EnumItem::Tuple(TupleItem {
                    attrs: attrs,
                    visibility: visibility,
                    ident: ident,
                    fields: fields,
                }))
            }
            else if let Some(group) = self.parse_brace_group() {
                let mut lexer = Lexer::new(group.stream());
                let mut fields = Vec::<StructField>::new();
                while let Some(field) = lexer.parse_struct_field() {
                    fields.push(field);
                    lexer.parse_punct(',');
                }
                Some(EnumItem::Struct(StructItem {
                    attrs: attrs,
                    visibility: visibility,
                    ident: ident,
                    fields: fields,
                }))
            }
            else if self.parse_punct('=') {
                if let Some(expr) = self.parse_expr() {
                    Some(EnumItem::Discr(DiscrItem {
                        attrs: attrs,
                        visibility: visibility,
                        ident: ident,
                        expr: expr,
                    }))
                }
                else {
                    panic!("expression expected after `=`");
                }
            }
            else {
                Some(EnumItem::Bare(BareItem {
                    attrs: attrs,
                    visibility: visibility,
                    ident: ident,
                }))
            }
        }
        else {
            None
        }
    }

    // Enum = `enum` IDENTIFIER [ Generics ] [ WhereClause ] `{` [ EnumItem { `,` EnumItem } [ `,` ] ] `}` .    
    pub(crate) fn parse_enum(&mut self,attrs: Vec<Group>,visibility: Visibility) -> Option<Enum> {
        if self.parse_ident("enum") {
            if let Some(ident) = self.parse_some_ident() {
                let generics = self.parse_generics();
                let wheres = self.parse_wheres();
                if let Some(group) = self.parse_brace_group() {
                    let mut lexer = Lexer::new(group.stream());
                    let mut items = Vec::<EnumItem>::new();
                    while let Some(item) = lexer.parse_enum_item() {
                        items.push(item);
                        lexer.parse_punct(',');
                    }
                    Some(Enum {
                        attrs: attrs,
                        visibility: visibility,
                        ident: ident,
                        generics: generics,
                        wheres: wheres,
                        items: items,
                    })
                }
                else {
                    panic!("{}","`{` expected to describe enum");
                }
            }
            else {
                panic!("identifier expected after `enum`");
            }
        }
        else {
            None
        }
    }
}