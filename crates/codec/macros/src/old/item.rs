// Echidna - Codec - Macros

use crate::*;

#[derive(Debug)]
pub enum WhereClauseItem {
    Lifetime {
        lifetime: Lifetime,
        bounds: Vec<Lifetime>,
    },
    TypeBound {
        for_lifetimes: Vec<LifetimeParam>,
        ty: Type,
        bounds: Vec<TypeParamBound>,
    },
}

#[derive(Debug)]
pub struct StructField {
    pub attributes: Vec<Group>,
    pub visibility: Visibility,
    pub identifier: String,
    pub ty: Type,
}

#[derive(Debug)]
pub struct Struct {
    pub attributes: Vec<Group>,
    pub visibility: Visibility,
    pub identifier: String,
    pub generics: Option<Generics>,
    pub where_clause: Vec<WhereClauseItem>,
    pub fields: Vec<StructField>,
}

#[derive(Debug)]
pub struct TupleField {
    pub attributes: Vec<Group>,
    pub visibility: Visibility,
    pub ty: Type,
}

#[derive(Debug)]
pub struct Tuple {
    pub attributes: Vec<Group>,
    pub visibility: Visibility,
    pub identifier: String,
    pub generics: Option<Generics>,
    pub where_clause: Vec<WhereClauseItem>,
    pub fields: Vec<TupleField>,
}

#[derive(Debug)]
pub struct SimpleItem {
    pub attributes: Vec<Group>,
    pub visibility: Visibility,
    pub identifier: String,
}

#[derive(Debug)]
pub struct TupleItem {
    pub attributes: Vec<Group>,
    pub visibility: Visibility,
    pub identifier: String,
    pub fields: Vec<TupleField>,
}

#[derive(Debug)]
pub struct StructItem {
    pub attributes: Vec<Group>,
    pub visibility: Visibility,
    pub identifier: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug)]
pub struct DiscriminantItem {
    pub attributes: Vec<Group>,
    pub visibility: Visibility,
    pub identifier: String,
    pub expression: Expression,
}

#[derive(Debug)]
pub enum EnumItem {
    Simple(SimpleItem),
    Tuple(TupleItem),
    Struct(StructItem),
    Discriminant(DiscriminantItem),
}

#[derive(Debug)]
pub struct Enum {
    pub attributes: Vec<Group>,
    pub visibility: Visibility,
    pub identifier: String,
    pub generics: Option<Generics>,
    pub where_clause: Vec<WhereClauseItem>,
    pub items: Vec<EnumItem>,
}

#[derive(Debug)]
pub struct Union {
    pub attributes: Vec<Group>,
    pub visibility: Visibility,
    pub identifier: String,
    pub generics: Option<Generics>,
    pub where_clause: Vec<WhereClauseItem>,
    pub fields: Vec<StructField>,
}    

#[derive(Debug)]
pub enum Item {
    Struct(Struct),
    Tuple(Tuple),
    Enum(Enum),
    Union(Union),
}

impl Lexer {

    // `where` WhereClauseItem { `,` WhereClauseItem } [ `,` ]
    pub fn parse_where_clause(&mut self) -> Option<Vec<WhereClauseItem>> {
        if self.parse_ident("where") {
            let items = Vec::<WhereClauseItem>::new();
            while !self.is_empty() && !self.is_brace_group() && !self.is_punct(';') {
                if let Some(lifetime) = self.parse_lifetime() {
                    self.parse_punct(':');
                    let bounds = Vec::<Lifetime>::new();
                    while let Some(lifetime) = self.parse_lifetime() {
                        bounds.push(lifetime);
                        self.parse_punct('+');
                    }
                    items.push(WhereClauseItem::Lifetime {
                        lifetime: lifetime,
                        bounds: bounds,
                    });
                }
                else {
                    let for_lifetimes = self.parse_for_lifetimes();
                    if let Some(ty) = self.parse_type() {
                        self.parse_punct(':');
                        let bounds = Vec::<TypeParamBound>::new();
                        while self.is_punct('`') || self.is_punct('?') || self.is_punct(':') || self.is_paren_group() || self.is_some_ident() || self.is_punct('$') {
                            bounds.push(self.parse_type_param_bound());
                            self.parse_punct('+');
                        }
                        items.push(WhereClauseItem::TypeBound {
                            for_lifetimes: for_lifetimes,
                            ty: ty,
                            bounds: bounds,
                        });
                    }
                    else {
                        panic!("type expected in where clause");
                    }
                }
            }
            Some(items)
        }
        else {
            None
        }
    }

    // { Attribute } [ Visibility ] IDENTIFIER `:` Type
    pub fn parse_struct_field(&mut self) -> StructField {
        let mut attributes = Vec::<Group>::new();
        while let Some(attribute) = self.parse_attribute() {
            attributes.push(attribute);
        }
        let visibility = self.parse_visibility();
        if self.is_some_ident() {
            let identifier = ident.to_string();
            self.step();
            self.parse_punct(':');
            if let Some(ty) = self.parse_type() {
                StructField {
                    attributes: attributes,
                    visibility: visibility,
                    identifier: identifier,
                    ty: ty,
                }
            }
            else {
                panic!("type expected after `:`");
            }
        }
        else {
            panic!("identifier expected in struct field");
        }
    }

    // { Attribute } [ Visibility ] Type
    pub fn parse_tuple_field(&mut self) -> TupleField {
        let mut attributes = Vec::<Group>::new();
        while let Some(attribute) = self.parse_attribute() {
            attributes.push(attribute);
        }
        let visibility = self.parse_visibility();
        if let Some(ty) = self.parse_type() {
            TupleField {
                attributes: attributes,
                visibility: visibility,
                ty: ty,
            }
        }
        else {
            panic!("type expected in tuple field");
        }
    }

    // { Attribute } [ Visibility ] IDENTIFIER ( `(` TupleFields `)` ) | ( `{` StructFields `}` ) | ( `=` Expression ) .
    pub fn parse_enum_item(&mut self) -> EnumItem {
        let mut attributes = Vec::<Group>::new();
        while let Some(attribute) = self.parse_attribute() {
            attributes.push(attribute);
        }
        let visibility = self.parse_visibility();
        if self.is_some_ident() {
            let identifier = ident.to_string();
            self.step();
            if let Some(group) = self.parse_paren_group() {
                let lexer = Lexer::new(group.stream());
                let fields = Vec::<TupleField>::new();
                while let Some(token) = &lexer.token {
                    fields.push(self.parse_tuple_field());
                    self.parse_punct(',');
                }
                EnumItem::Tuple(TupleItem {
                    attributes: attributes,
                    visibility: visibility,
                    identifier: identifier,
                    fields: fields,
                })
            }
            else if let Some(group) = self.parse_brace_group() {
                let lexer = Lexer::new(group.stream());
                let fields = Vec::<StructField>::new();
                while let Some(token) = &lexer.token {
                    fields.push(self.parse_struct_field());
                    self.parse_punct(',');
                }
                EnumItem::Struct(StructItem {
                    attributes: attributes,
                    visibility: visibility,
                    identifier: identifier,
                    fields: fields,
                })
            }
            else if self.parse_punct('=') {
                if let Some(expression) = self.parse_expression() {
                    EnumItem::Discriminant(DiscriminantItem {
                        attributes: attributes,
                        visibility: visibility,
                        identifier: identifier,
                        expression: expression,
                    })
                }
                else {
                    panic!("expression expected after `=`'");
                }
            }
            else {
                EnumItem::Simple(SimpleItem {
                    attributes: attributes,
                    visibility: visibility,
                    identifier: identifier,
                })
            }
        }
        else {
            panic!("identifier expected in enum item");
        }
    }

    // { OuterAttribute } [ Visibility ] IDENTIFIER StructOrTuple | Enum | Union .
    pub fn parse_item(&mut self) -> Item {
        let mut attributes = Vec::<Group>::new();
        while let Some(attribute) = self.parse_attribute() {
            attributes.push(attribute);
        }
        let visibility = self.parse_visibility();
        if self.is_ident("struct") {
            self.step();
            if self.is_some_ident() {
                let identifier = ident.to_string();
                self.step();
                let generics = self.parse_generics();
                if let Some(where_clause) = self.parse_where_clause() {
                    if let Some(group) = self.parse_brace_group() {
                        let lexer = Lexer::new(group.stream());
                        let fields = Vec::<StructField>::new();
                        while let Some(token) = &lexer.token {
                            fields.push(self.parse_struct_field());
                            self.parse_punct(',');
                        }
                        self.parse_punct(';');
                        Item::Struct(Struct {
                            attributes: attributes,
                            visibility: visibility,
                            identifier: identifier,
                            generics: generics,
                            where_clause: where_clause,
                            fields: fields,
                        })
                    }
                    else {
                        panic!("struct declaration expected after where clause");
                    }
                }
                else if let Some(group) = self.parse_brace_group() {
                    let lexer = Lexer::new(group.stream());
                    let fields = Vec::<StructField>::new();
                    while let Some(token) = &lexer.token {
                        fields.push(self.parse_struct_field());
                        self.parse_punct(',');
                    }
                    self.parse_punct(';');
                    Item::Struct(Struct {
                        attributes: attributes,
                        visibility: visibility,
                        identifier: identifier,
                        generics: generics,
                        where_clause: Vec::<WhereClauseItem>::new(),
                        fields: fields,
                    })
                }
                else {
                    if let Some(group) = self.parse_paren_group() {
                        let lexer = Lexer::new(group.stream());
                        let fields = Vec::<TupleField>::new();
                        while let Some(token) = &lexer.token {
                            fields.push(self.parse_tuple_field());
                            self.parse_punct(',');
                        }
                        let where_clause = if let Some(where_clause) = self.parse_where_clause() {
                            where_clause
                        }
                        else {
                            Vec::<WhereClauseItem>::new()
                        };
                        self.parse_punct(';');
                        Item::Tuple(Tuple {
                            attributes: attributes,
                            visibility: visibility,
                            identifier: identifier,
                            generics: generics,
                            where_clause: where_clause,
                            fields: fields,
                        })
                    }
                    else {
                        panic!("struct or tuple declaration expected")
                    }                    
                }
            }
            else {
                panic!("identifier expected after `struct`");
            }
        }
        else if self.is_ident("enum") {
            self.step();
            if self.is_some_ident() {
                let identifier = ident.to_string();
                self.step();
                let generics = self.parse_generics();
                let where_clause = if let Some(where_clause) = self.parse_where_clause() {
                    where_clause
                }
                else {
                    Vec::<WhereClauseItem>::new()
                };
                if let Some(group) = self.parse_brace_group() {
                    let lexer = Lexer::new(group.stream());
                    let items = Vec::<EnumItem>::new();
                    while let Some(token) = &lexer.token {
                        items.push(self.parse_enum_item());
                        self.parse_punct(',');
                    }
                    self.parse_punct(';');
                    Item::Enum(Enum {
                        attributes: attributes,
                        visibility: visibility,
                        identifier: identifier,
                        generics: generics,
                        where_clause: where_clause,
                        items: items,
                    })
                }
                else {
                    panic!("enum declaration expected");
                }
            }
            else {
                panic!("identifier expected after `enum`");
            }
        }
        else if self.is_ident("union") {
            self.step();
            if self.is_some_ident() {
                let identifier = ident.to_string();
                self.step();
                let generics = self.parse_generics();
                let where_clause = if let Some(where_clause) = self.parse_where_clause() {
                    where_clause
                }
                else {
                    Vec::<WhereClauseItem>::new()
                };
                if let Some(group) = self.parse_brace_group() {
                    let lexer = Lexer::new(group.stream());
                    let fields = Vec::<StructField>::new();
                    while let Some(token) = &lexer.token {
                        fields.push(self.parse_struct_field());
                        self.parse_punct(',');
                    }
                    self.parse_punct(';');
                    Item::Union(Union {
                        attributes: attributes,
                        visibility: visibility,
                        identifier: identifier,
                        generics: generics,
                        where_clause: where_clause,
                        fields: fields,
                    })
                }
                else {
                    panic!("union declaration expected");
                }
            }
            else {
                panic!("identifier expected after `union`");
            }
        }
        else {
            panic!("unrecognized item");
        }
    }
}
