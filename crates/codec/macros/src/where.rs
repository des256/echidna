// Echidna - Codec - Macros

use crate::*;

pub(crate) enum Where {
    Lifetime {
        ident: String,
        bounds: Vec<String>,
    },
    Type {
        for_lifetimes: Vec<String>,
        ty: Box<Type>,
        bounds: Vec<TypeParamBound>,
    },
}

impl fmt::Display for Where {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Where::Lifetime { ident,bounds }=> {
                let mut a = String::new();
                a += &format!("{}",ident);
                if bounds.len() > 0 {
                    a += &format!(": ");
                    let mut first = true;
                    for bound in bounds {
                        if first {
                            first = false;
                        }
                        else {
                            a += &format!(" + ");
                        }
                        a += &format!("'{}",bound);
                    }
                }
                write!(f,"{}",a)
            },
            Where::Type { for_lifetimes,ty,bounds } => {
                let mut a = String::new();
                if for_lifetimes.len() > 0 {
                    a += "for <";
                    let mut first = true;
                    for lt in for_lifetimes {
                        if first {
                            first = false;
                        }
                        else {
                            a += ",";
                        }
                        a += &format!("'{}",lt);
                    }
                    a += ">";
                }
                a += &format!("{}",ty);
                if bounds.len() > 0 {
                    a += ": ";
                    let mut first = true;
                    for bound in bounds {
                        if first {
                            first = false;
                        }
                        else {
                            a += " + ";
                        }
                        a += &format!("{}",bound);
                    }
                }
                write!(f,"{}",a)
            },
        }
    }
}

impl Lexer {

    fn is_where(&self) -> bool {
        if self.is_punct(':') {
            true
        }
        else if self.is_punct('<') {
            true
        }
        else if self.is_some_ident() {
            true
        }
        else if self.is_paren_group() {
            true
        }
        else if self.is_bracket_group() {
            true
        }
        else {
            false
        }
    }

    // LifetimeWhere = `'` IDENTIFIER `:` `'` IDENTIFIER { `+` `'` IDENTIFIER } [ `+` ] .
    // TypeBoundWhere = [ `for` `<` `'` IDENTIFIER { `,` `'` IDENTIFIER } `>` ] Type `:` TypeParamBound { `+` TypeParamBound } [ `+` ] .
    // Where = LifetimeWhere | TypeBoundWhere .
    // WhereClause = `where` { Where `,` } [ Where ] .
    pub(crate) fn parse_wheres(&mut self) -> Vec<Where> {
        if self.parse_ident("where") {
            let mut wheres = Vec::<Where>::new();
            while self.is_where() {
                if self.is_punct('\'') {
                    if let Some(ident) = self.parse_some_ident() {
                        let mut bounds = Vec::<String>::new();
                        if self.parse_punct(':') {
                            while self.parse_punct('\'') {
                                if self.is_some_ident() {
                                    bounds.push(ident.to_string());
                                }
                                else {
                                    panic!("identifier expected after `'`");
                                }
                                self.parse_punct('+');
                            }
                        }
                        else {
                            panic!("`:` expected after lifetime");
                        }
                        wheres.push(Where::Lifetime {
                            ident: ident,
                            bounds: bounds,
                        });
                    }
                    else {
                        panic!("identifier expected after `'`");
                    }
                }
                else {
                    let mut for_lifetimes = Vec::<String>::new();
                    if self.parse_ident("for") {
                        if self.parse_punct('<') {
                            while !self.is_punct('>') {
                                if self.parse_punct('\'') {
                                    if let Some(ident) = self.parse_some_ident() {
                                        for_lifetimes.push(ident);
                                    }
                                    else {
                                        panic!("identifier expected after `'`");
                                    }
                                    self.parse_punct(',');
                                }
                                else {
                                    panic!("lifetime expected in `for <` `>`");
                                }
                            }
                        }
                        else {
                            panic!("`<` expected after `for`");
                        }
                    }
                    if let Some(ty) = self.parse_type() {
                        // Type
                        let mut bounds = Vec::<TypeParamBound>::new();
                        if self.parse_punct(':') {
                            while let Some(bound) = self.parse_type_param_bound() {
                                bounds.push(bound);
                                self.parse_punct('+');
                            }
                        }
                        else {
                            panic!("`:` expected after type");
                        }
                        wheres.push(Where::Type {
                            for_lifetimes: for_lifetimes,
                            ty: Box::new(ty),
                            bounds: bounds,
                        });
                    }
                    else {
                        panic!("type expected in where clause");
                    }
                }
                self.parse_punct(',');
            }
            wheres
        }
        else {
            Vec::<Where>::new()
        }
    }
}