// Echidna - Codec - Macros

use crate::*;

pub(crate) struct TraitBound {
    pub(crate) question: bool,
    pub(crate) for_lifetimes: Vec<String>,
    pub(crate) path: Path,
}

impl fmt::Display for TraitBound {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut a = String::new();
        if self.question {
            a += "? ";
        }
        if self.for_lifetimes.len() > 0 {
            a += "for <";
            let mut first = true;
            for fl in &self.for_lifetimes {
                if first {
                    first = false;
                }
                else {
                    a += ", ";
                }
                a += &format!("'{}",fl);
            }
            a += ">";
        }
        a += &format!("{}",self.path);
        write!(f,"{}",a)
    }
}
        
pub(crate) enum TypeParamBound {
    Lifetime(String),
    Trait(TraitBound),
}

impl fmt::Display for TypeParamBound {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeParamBound::Lifetime(ident) => write!(f,"'{}",ident),
            TypeParamBound::Trait(traitbound) => write!(f,"{}",traitbound),
        }
    }
}

pub(crate) enum Generic {
    Lifetime {
        ident: String,
        bounds: Vec<String>,
    },
    Type {
        ident: String,
        bounds: Vec<TypeParamBound>,
        ty: Option<Box<Type>>,
    },
}

impl fmt::Display for Generic {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Generic::Lifetime { ident,bounds } => {
                let mut a = String::new();
                a += &format!("'{}: ",ident);
                let mut first = true;
                for bound in bounds {
                    if first {
                        first = false;
                    }
                    else {
                        a += " + ";
                    }
                    a += &format!("'{}",bound);
                }
                write!(f,"{}",a)
            },
            Generic::Type { ident,bounds,ty } => {
                let mut a = String::new();
                a += &format!("{}",ident);
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
                if let Some(ty) = ty {
                    a += &format!(" = {}",ty);
                }
                write!(f,"{}",a)
            },
        }
    }
}

impl Lexer {

    // TraitBound = [ `?` ] [ `for` `<` `'` IDENTIFIER { `,` `'` IDENTIFIER } `>` ] TypePath .
    pub(crate) fn parse_trait_bound(&mut self) -> Option<TraitBound> {
        let question = self.parse_punct('?');
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
        if let Some(path) = self.parse_path() {
            Some(TraitBound {
                question: question,
                for_lifetimes: for_lifetimes,
                path: path,
            })
        }
        else {
            None
        }
    }

    // TypeParamBound = ( `'` IDENTIFIER ) | ( `(` TraitBound `)` ) | TraitBound .
    pub(crate) fn parse_type_param_bound(&mut self) -> Option<TypeParamBound> {
        if self.parse_punct('\'') {
            if let Some(ident) = self.parse_some_ident() {
                Some(TypeParamBound::Lifetime(ident.to_string()))
            }
            else {
                panic!("identifier expected after `'`");
            }
        }
        else {
            if let Some(group) = self.parse_paren_group() {
                let mut lexer = Lexer::new(group.stream());
                if let Some(trait_bound) = lexer.parse_trait_bound() {
                    Some(TypeParamBound::Trait(trait_bound))
                }
                else {
                    panic!("trait bound expected");
                }
            }
            else {
                if let Some(trait_bound) = self.parse_trait_bound() {
                    Some(TypeParamBound::Trait(trait_bound))
                }
                else {
                    None
                }
            }
        }
    }

    // TypeParam = IDENTIFIER [ `:` [ TypeParamBound { `+` TypeParamBound } [ `+` ] ] ] [ `=` Type ] .
    // LifetimeParam = `'` IDENTIFIER [ `:` `'` IDENTIFIER { `+` `'` IDENTIFIER } [ `+` ] ] .
    // Generic = LifetimeParam | TypeParam .
    // Generics = `<` Generic { `,` Generic } [ `,` ] `>` .
    pub(crate) fn parse_generics(&mut self) -> Vec<Generic> {
        if self.parse_punct('<') {
            let mut params = Vec::<Generic>::new();
            while !self.is_punct('>') {

                // LifetimeParam
                if self.parse_punct('\'') {
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
                        params.push(Generic::Lifetime {
                            ident: ident,
                            bounds: bounds,
                        });
                    }
                    else {
                        panic!("identifier expected after `'`");
                    }
                }

                // TypeParam
                else if let Some(ident) = self.parse_some_ident() {
                    let mut bounds = Vec::<TypeParamBound>::new();
                    let mut ty = None;
                    if self.parse_punct(':') {
                        while !self.is_punct('>') && !self.is_punct(',') && !self.is_punct('=') {
                            if let Some(bound) = self.parse_type_param_bound() {
                                bounds.push(bound);
                            }
                            self.parse_punct('+');
                        }
                        if self.parse_punct('=') {
                            if let Some(t) = self.parse_type() {
                                ty = Some(Box::new(t));
                            }
                            else {
                                panic!("type expected after `=`");
                            }
                        }
                    }
                    params.push(Generic::Type {
                        ident: ident,
                        bounds: bounds,
                        ty: ty,
                    });
                }
            }
            self.parse_punct('>');
            params
        }
        else {
            Vec::<Generic>::new()
        }
    }
}