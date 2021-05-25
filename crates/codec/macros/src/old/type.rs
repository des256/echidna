// Echidna - Codec - Macros

use crate::*;

#[derive(Debug)]
pub enum Type {
    ImplTrait(Vec<TypeParamBound>),
    TraitObject(Vec<TypeParamBound>),
    Path(TypePath),
    Tuple(Vec<Type>),
    Never,
    RawPtr {
        mutable: bool,
        ty: Box<Type>,
    },
    Ref {
        lifetime: Option<Lifetime>,
        mutable: bool,
        ty: Box<Type>,
    },
    Array {
        ty: Box<Type>,
        expression: Expression,
    },
    Slice(Box<Type>),
    Inferred,
    QPath,
    Function {
        for_lifetimes: Vec<LifetimeParam>,
        prelude: Prelude,
        unsafety: bool,
        abi: Option<String>,
        parameters: Vec<Parameter>,
        variadic_attributes: Option<Vec<Group>>,
        result: Option<Box<Type>>,
    },
    //Macro {
    //    path: SimplePath,
    //    tokens: Group,
    //},
}

#[derive(Debug)]
pub struct Parameter {
    attributes: Vec<Group>,
    identifier: Option<String>,
    ty: Box<Type>,
}

#[derive(Debug)]
pub enum Prelude {
    None,
    Async,
    Const,
}

impl Lexer {

    pub fn parse_type(&mut self) -> Option<Type> {
        if self.parse_ident("_") {
            Some(Type::Inferred)
        }
        else if self.parse_punct('!') {
            Some(Type::Never)
        }
        else if self.parse_punct('&') {
            let lifetime = self.parse_lifetime();
            let mutable = self.parse_ident("mut");
            if let Some(ty) = self.parse_type() {
                Some(Type::Ref {
                    lifetime: lifetime,
                    mutable: mutable,
                    ty: Box::new(ty),
                })
            }
            else {
                panic!("type expected after `&`");
            }
        }
        else if self.parse_punct('*') {
            let mutable = self.parse_ident("mut");
            self.parse_ident("const");
            if let Some(ty) = self.parse_type() {
                Some(Type::RawPtr {
                    mutable: mutable,
                    ty: Box::new(ty),
                })
            }
            else {
                panic!("type expected after `*`");
            }
        }
        else if self.parse_ident("dyn") {
            let bounds = Vec::<TypeParamBound>::new();
            while self.is_punct('`') || self.is_punct('?') || self.is_punct(':') || self.is_paren_group() || self.is_some_ident() || self.is_punct('$') {
                bounds.push(self.parse_type_param_bound());
                self.parse_punct('+');
            }
            Some(Type::TraitObject(bounds))
        }
        else if self.parse_ident("impl") {
            let bounds = Vec::<TypeParamBound>::new();
            while self.is_punct('`') || self.is_punct('?') || self.is_punct(':') || self.is_paren_group() || self.is_some_ident() || self.is_punct('$') {
                bounds.push(self.parse_type_param_bound());
                self.parse_punct('+');
            }
            Some(Type::ImplTrait(bounds))
        }
        else if let Some(group) = self.parse_paren_group() {
            let lexer = Lexer::new(group.stream());
            let types = Vec::<Type>::new();
            while let Some(token) = &lexer.token {
                if let Some(ty) = lexer.parse_type() {
                    types.push(ty);
                }
                else {
                    panic!("type expected in `(` `)`");
                }
                lexer.parse_punct(',');   
            }
            Some(Type::Tuple(types))
        }
        else if let Some(group) = self.parse_bracket_group() {
            let lexer = Lexer::new(group.stream());
            if let Some(ty) = lexer.parse_type() {
                if lexer.parse_punct(';') {
                    if let Some(expression) = lexer.parse_expression() {
                        Some(Type::Array {
                            ty: Box::new(ty),
                            expression: expression,
                        })
                    }
                    else {
                        panic!("expression expected after `;`");
                    }
                }
                else {
                    Some(Type::Slice(Box::new(ty)))
                }
            }
            else {
                panic!("type expected in `[` `]`");
            }
        }
        else if self.is_ident("for") || self.is_ident("async") || self.is_ident("const") || self.is_ident("unsafe") || self.is_ident("extern") || self.is_ident("fn") {
            let for_lifetimes = self.parse_for_lifetimes();
            let prelude = if self.parse_ident("async") {
                Prelude::Async
            }
            else if self.parse_ident("const") {
                Prelude::Const
            }
            else {
                Prelude::None
            };
            let unsafety = self.parse_ident("unsafe");
            let abi = if self.parse_ident("extern") {
                if let Some(TokenTree::Literal(literal)) = &self.token {
                    let literal = literal.to_string();
                    self.step();
                    Some(literal)
                }
                else {
                    None
                }
            }
            else {
                None
            };
            let mut parameters = Vec::<Parameter>::new();
            let mut variadic_attributes = Option::<Vec<Group>>::None;
            if let Some(group) = self.parse_paren_group() {
                let mut lexer = Lexer::new(group.stream());
                while let Some(token) = &lexer.token {
                    let mut attributes = Vec::<Group>::new();
                    while let Some(attribute) = lexer.parse_attribute() {
                        attributes.push(attribute);
                    }
                    let identifier = if let Some(TokenTree::Ident(ident)) = &lexer.token {
                        let ident = ident.to_string();
                        lexer.step();
                        let identifier = if ident == "_" {
                            None
                        }
                        else {
                            Some(ident)
                        };
                        lexer.parse_punct(':');
                        identifier
                    }
                    else if lexer.parse_punct3('.','.','.') {
                        lexer.step();
                        variadic_attributes = Some(attributes);
                        None
                    }
                    else {
                        panic!("function parameter or `...` expected");
                    };
                    if let Some(ty) = lexer.parse_type() {
                        lexer.parse_punct(',');
                        if let None = variadic_attributes {
                            parameters.push(Parameter {
                                attributes: attributes,
                                identifier: identifier,
                                ty: Box::new(ty),
                            });
                        }    
                    }
                    else {
                        panic!("type expected in function parameter list")
                    }
                }
            }
            let result = if self.parse_punct2('-','>') {
                if let Some(ty) = self.parse_type() {
                    Some(Box::new(ty))
                }
                else {
                    None
                }
            }
            else {
                panic!("`->` followed by type expected");
            };
            Some(Type::Function {
                for_lifetimes: for_lifetimes,
                prelude: prelude,
                unsafety: unsafety,
                abi: abi,
                parameters: parameters,
                variadic_attributes: variadic_attributes,
                result: result,
            })
        }
        else {
            Some(Type::Path(self.parse_type_path()))
        }
    }
}
