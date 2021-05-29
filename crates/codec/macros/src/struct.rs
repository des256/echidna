// Echidna - Codec - Macros

use crate::*;

pub(crate) struct StructField {
    pub(crate) attrs: Vec<Group>,
    pub(crate) visibility: Visibility,
    pub(crate) ident: String,
    pub(crate) ty: Box<Type>,
}

impl fmt::Display for StructField {
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
        a += &format!("{}: {}",self.ident,self.ty);
        write!(f,"{}",a)
    }
}

pub(crate) struct TupleField {
    pub(crate) attrs: Vec<Group>,
    pub(crate) visibility: Visibility,
    pub(crate) ty: Box<Type>,
}

impl fmt::Display for TupleField {
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
        a += &format!("{}",self.ty);
        write!(f,"{}",a)
    }
}

pub(crate) struct Struct {
    pub(crate) attrs: Vec<Group>,
    pub(crate) visibility: Visibility,
    pub(crate) ident: String,
    pub(crate) generics: Vec<Generic>,
    pub(crate) wheres: Vec<Where>,
    pub(crate) fields: Vec<StructField>,
}

impl fmt::Display for Struct {
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
        a += &format!("struct {}",self.ident);
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
        else {
            a += " ";
        }
        a += "{ ";
        for field in &self.fields {
            a += &format!("{}, ",field);
        }
        a += "}";
        write!(f,"{}",a)
    }
}

pub(crate) struct Tuple {
    pub(crate) attrs: Vec<Group>,
    pub(crate) visibility: Visibility,
    pub(crate) ident: String,
    pub(crate) generics: Vec<Generic>,
    pub(crate) wheres: Vec<Where>,
    pub(crate) fields: Vec<TupleField>,
}

impl fmt::Display for Tuple {
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
        a += &format!("struct {}",self.ident);
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
        a += "(";
        let mut first = true;
        for field in &self.fields {
            if first {
                first = false;
            }
            else {
                a += ",";
            }
            a += &format!("{}",field);
        }
        a += ")";
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
        write!(f,"{}",a)
    }
}

pub(crate) enum StructOrTuple {
    Struct(Struct),
    Tuple(Tuple),
}

impl fmt::Display for StructOrTuple {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StructOrTuple::Struct(s) => write!(f,"{}",s),
            StructOrTuple::Tuple(t) => write!(f,"{}",t),
        }
    }
}

impl Lexer {

    // StructField = { Attr } [ Visibility ] IDENTIFIER `:` Type .
    pub(crate) fn parse_struct_field(&mut self) -> Option<StructField> {
        let mut attrs = Vec::<Group>::new();
        while let Some(attr) = self.parse_attr() {
            attrs.push(attr);
        }
        let visibility = self.parse_visibility();
        if let Some(ident) = self.parse_some_ident() {
            if self.parse_punct(':') {
                if let Some(ty) = self.parse_type() {
                    Some(StructField {
                        attrs: attrs,
                        visibility: visibility,
                        ident: ident,
                        ty: Box::new(ty),
                    })
                }
                else {
                    panic!("type expected after `:`");
                }
            }
            else {
                panic!("`:` expected in struct field");
            }
        }
        else {
            None
        }
    }


    // TupleField = { Attr } [ Visibility ] Type .
    pub(crate) fn parse_tuple_field(&mut self) -> Option<TupleField> {
        let mut attrs = Vec::<Group>::new();
        while let Some(attr) = self.parse_attr() {
            attrs.push(attr);
        }
        let visibility = self.parse_visibility();
        if let Some(ty) = self.parse_type() {
            Some(TupleField {
                attrs: attrs,
                visibility: visibility,
                ty: Box::new(ty),
            })
        }
        else {
            None
        }
    }

    // Struct = `struct` IDENTIFIER [ Generics ] [ WhereClause ] ( `{` [ StructField { `,` StructField } [ `,` ] ] `}` ) | `;` .
    // Tuple = `struct` IDENTIFIER [ Generics ] `(` [ TupleField { `,` TupleField } [ `,` ] ] `)` [ WhereClause ] `;` .
    pub(crate) fn parse_struct_or_tuple(&mut self,attrs: Vec<Group>,visibility: Visibility) -> Option<StructOrTuple> {
        if self.parse_ident("struct") {
            if let Some(ident) = self.parse_some_ident() {
                let generics = self.parse_generics();
                if let Some(group) = self.parse_paren_group() {
                    let mut lexer = Lexer::new(group.stream());
                    let mut fields = Vec::<TupleField>::new();
                    while let Some(field) = lexer.parse_tuple_field() {
                        fields.push(field);
                        lexer.parse_punct(',');
                    }
                    let wheres = self.parse_wheres();
                    Some(StructOrTuple::Tuple(Tuple {
                        attrs: attrs,
                        visibility: visibility,
                        ident: ident,
                        generics: generics,
                        wheres: wheres,
                        fields: fields,
                    }))
                }
                else {
                    let wheres = self.parse_wheres();
                    if let Some(group) = self.parse_brace_group() {
                        let mut lexer = Lexer::new(group.stream());
                        let mut fields = Vec::<StructField>::new();
                        while let Some(field) = lexer.parse_struct_field() {
                            fields.push(field);
                            lexer.parse_punct(',');
                        }
                        Some(StructOrTuple::Struct(Struct {
                            attrs: attrs,
                            visibility: visibility,
                            ident: ident,
                            generics: generics,
                            wheres: wheres,
                            fields: fields,
                        }))
                    }
                    else {
                        panic!("{}","`{` expected to describe struct");
                    }
                }
            }
            else {
                panic!("identifier expected after `struct`");
            }
        }
        else {
            None
        }
    }
}
