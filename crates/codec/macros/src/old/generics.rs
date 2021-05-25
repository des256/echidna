// Echidna - Codec - Macros

use crate::*;

#[derive(Debug)]
pub struct Generics {
    pub lifetime_params: Vec<LifetimeParam>,
    pub type_params: Vec<TypeParam>,
}

impl Generics {
    pub fn render(&self) -> String {
        let mut a = "<".to_string();
        if self.lifetime_params.len() > 0 {
            let mut cnt = self.lifetime_params.len();
            for param in self.lifetime_params {
                a += &param.render();
                cnt -= 1;
                if cnt > 0 {
                    a += ",";
                }
            }
        }
        if self.type_params.len() > 0 {
            let mut cnt = self.type_params.len();
            for param in self.type_params {
                a += &param.render();
                cnt -= 1;
                if cnt > 0 {
                    a += ",";
                }
            }
        }
        a += ">";
        a
    }
}

#[derive(Debug)]
pub struct LifetimeParam {
    pub attributes: Vec<Group>,
    pub lifetime: Lifetime,
    pub bounds: Vec<Lifetime>,
}

impl LifetimeParam {
    pub fn render(&self) -> String {
        let mut a = self.lifetime.render();
        if self.bounds.len() > 0 {
            a += ": ";
            let mut cnt = self.bounds.len();
            for bound in self.bounds {
                a += &bound.render();
                cnt -= 1;
                if cnt > 0 {
                    a += ",";
                }
            }
        }
        a
    }
}

#[derive(Debug)]
pub struct TypeParam {
    pub attributes: Vec<Group>,
    pub identifier: String,
    pub bounds: Vec<TypeParamBound>,
    pub ty: Option<Type>,
}

impl TypeParam {
    pub fn render(&self) -> String {
        let mut a = self.identifier.clone();
        if self.bounds.len() > 0 {
            a += ": ";
            let mut cnt = self.bounds.len();
            for bound in self.bounds {
                a += &bound.render();
                cnt -= 1;
                if cnt > 0 {
                    a += ",";
                }
            }
        }
        a += " = ";
        a += self.ty.render();
        a
    }
}

#[derive(Debug)]
pub enum TypeParamBound {
    Lifetime(Lifetime),
    TraitBound(TraitBound),
}

impl TypeParamBound {
    pub fn render(&self) -> String {
        match self {
            TypeParamBound::Lifetime(lifetime) => lifetime.render(),
            TypeParamBound::TraitBound(traitbound) => traitbound.render(),
        }
    }
}

#[derive(Debug)]
pub enum Lifetime {
    Static,
    Anonymous,
    Named(String),
}

impl Lifetime {
    pub fn render(&self) -> String {
        match self {
            Lifetime::Static => "'static".to_string(),
            Lifetime::Anonymous => "'_".to_string(),
            Lifetime::Named(identifier) => {
                let mut a = "'".to_string();
                a += identifier;
                a
            },
        }
    }
}

#[derive(Debug)]
pub struct TraitBound {
    pub question_mark: bool,
    pub for_lifetimes: Vec<LifetimeParam>,
    pub path: TypePath,
}

impl TraitBound {
    pub fn render(&self) -> String {
        let mut a = "".to_string();
        if self.question_mark {
            a += "?";
        }
        if self.for_lifetimes.len() > 0 {
            a += "for ";
            let mut cnt = self.for_lifetimes.len();
            for param in self.for_lifetimes {
                a += &param.render();
                cnt -= 1;
                if cnt > 0 {
                    a += ",";
                }
                else {
                    a += " ";
                }
            }
        }
        a += &self.path.render();
        a
    }
}

#[derive(Debug)]
pub struct GenericArgs {
    pub lifetimes: Vec<Lifetime>,
    pub types: Vec<Type>,
    pub bindings: Vec<GenericArgsBinding>,
}

impl GenericArgs {
    pub fn render(&self) -> String {
        let mut a = "<".to_string();
        let mut cnt = self.lifetimes.len() + self.types.len() + self.bindings.len();
        for lifetime in self.lifetimes {
            a += lifetime.render();
            cnt -= 1;
            if cnt > 0 {
                a += ",";
            }
        }
        for ty in self.types {
            a += ty.render();
            cnt -= 1;
            if cnt > 0 {
                a += ",";
            }
        }
        for binding in self.bindings {
            a += binding.render();
            cnt -= 1;
            if cnt > 0 {
                a += ",";
            }
        }
        a += ">";
        a
    }
}
#[derive(Debug)]
pub struct GenericArgsBinding {
    identifier: String,
    ty: Type,
}

impl Lexer {

    // Lifetime = ``static` | ``_` | `IDENTIFIER .
    pub fn parse_lifetime(&mut self) -> Option<Lifetime> {
        if self.parse_punct('`') {
            if self.parse_ident("static") {
                Some(Lifetime::Static)
            }
            else if self.parse_ident("_") {
                Some(Lifetime::Anonymous)
            }
            else if self.is_some_ident() {
                Some(Lifetime::Named(ident.to_string()))
            }
            else {
                panic!("invalid lifetime specifier")
            }
        }
        else {
            None
        }
    }

    // LifetimeParam = [ Attribute ] LIFETIME_OR_LABEL [ `:` Lifetime { `+` Lifetime } [ `+` ] ] .
    pub fn parse_lifetime_param(&mut self) -> LifetimeParam {
        let mut attributes = Vec::<Group>::new();
        while let Some(group) = self.parse_attribute() {
            attributes.push(group);
        }
        if let Some(lifetime) = self.parse_lifetime() {
            let mut bounds = Vec::<Lifetime>::new();
            if self.parse_punct(':') {
                while let Some(lifetime) = self.parse_lifetime() {
                    bounds.push(lifetime);
                    self.parse_punct('+');
                }
            }
            LifetimeParam {
                attributes: attributes,
                lifetime: lifetime,
                bounds: bounds,
            }
        }
        else {
            panic!("lifetime expected");
        }
    }

    // ForLifetimes = `for` `<` LifetimeParam { `,` LifetimeParam } `>` .
    pub fn parse_for_lifetimes(&mut self) -> Vec<LifetimeParam> {
        if self.parse_ident("for") {
            if self.parse_punct('<') {
                let lifetime_params = Vec::<LifetimeParam>::new();
                lifetime_params.push(self.parse_lifetime_param());
                while self.parse_punct(',') {
                    lifetime_params.push(self.parse_lifetime_param());
                }
                self.parse_punct(',');
                if !self.parse_punct('>') {
                    panic!("`>` expected at end of for-clause");
                }
                lifetime_params
            }
            else {
                panic!("`<` expected after `for`");
            }
        }
        else {
            Vec::<LifetimeParam>::new()
        }
    }

    // GenericArgsBinding = IDENTIFIER `=` Type .
    pub fn parse_generic_args_binding(&mut self) -> Option<GenericArgsBinding> {
        if self.is_some_ident() {
            let identifier = ident.to_string();
            self.step();
            if self.is_punct('=') {
                self.step();
                if let Some(ty) = self.parse_type() {
                    Some(GenericArgsBinding {
                        identifier: identifier,
                        ty: ty,
                    })
                }
                else {
                    panic!("type expected in generic args binding");
                }
            }
            else {
                panic!("`=` expected in generic args binding");
            }
        }
        else {
            None
        }
    }

    // GenericArgs = `<` [ GenericArgsLifetimes ] [ GenericArgsTypes ] [ GenericArgsBindings ] `>` .
    pub fn parse_generic_args(&mut self) -> Option<GenericArgs> {
        if self.parse_punct('<') {
            let lifetimes = Vec::<Lifetime>::new();
            while let Some(lifetime) = self.parse_lifetime() {
                lifetimes.push(lifetime);
                self.parse_punct(',');
            }
            let types = Vec::<Type>::new();
            while let Some(ty) = self.parse_type() {
                types.push(ty);
                self.parse_punct(',');
            }
            let bindings = Vec::<GenericArgsBinding>::new();
            while let Some(binding) = self.parse_generic_args_binding() {
                bindings.push(binding);
                self.parse_punct(',');
            }
            Some(GenericArgs {
                lifetimes: lifetimes,
                types: types,
                bindings: bindings,
            })
        }
        else {
            None
        }
    }

    // [ `?` ] [ ForLifetimes ] TypePath .
    pub fn parse_trait_bound(&mut self) -> TraitBound {
        let question_mark = self.parse_punct('?');
        let for_lifetimes = self.parse_for_lifetimes();
        let path = self.parse_type_path();
        TraitBound {
            question_mark: question_mark,
            for_lifetimes: for_lifetimes,
            path: path,
        }
    }

    // Lifetime | TraitBound | ( `(` TraitBound `)` ) .
    pub fn parse_type_param_bound(&mut self) -> TypeParamBound {
        if let Some(lifetime) = self.parse_lifetime() {
            TypeParamBound::Lifetime(lifetime)
        }
        else if let Some(group) = self.parse_paren_group() {
            let mut lexer = Lexer::new(group.stream());
            TypeParamBound::TraitBound(lexer.parse_trait_bound())
        }
        else {
            TypeParamBound::TraitBound(self.parse_trait_bound())
        }
    }

    // `<` [ GenericParam { `,` GenericParam } [ `,` ] `>` .
    pub fn parse_generics(&mut self) -> Option<Generics> {
        if self.parse_punct('<') {
            let mut lifetime_params = Vec::<LifetimeParam>::new();
            let mut type_params = Vec::<TypeParam>::new();
            while self.is_punct('#') || self.is_punct('`') || self.is_some_ident() {
                let mut attributes = Vec::<Group>::new();
                while let Some(attribute) = self.parse_attribute() {
                    attributes.push(attribute);
                }
                if self.is_punct('`') {
                    let lifetime = self.parse_lifetime().unwrap();
                    let bounds = Vec::<Lifetime>::new();
                    if self.parse_punct(':') {
                        while let Some(bound) = self.parse_lifetime() {
                            bounds.push(bound);
                            self.parse_punct('+');
                        }
                    }
                    lifetime_params.push(LifetimeParam {
                        attributes: attributes,
                        lifetime: lifetime,
                        bounds: bounds,
                    });
                }
                else if self.is_some_ident() {
                    let identifier = ident.to_string();
                    self.step();
                    let bounds = Vec::<TypeParamBound>::new();
                    if self.parse_punct(':') {
                        while !self.is_punct('=') && !self.is_punct(',') && !self.is_punct('>') {
                            bounds.push(self.parse_type_param_bound());
                            self.parse_punct('+');
                        }
                    }
                    let ty: Option<Type> = None;
                    if self.parse_punct('=') {
                        ty = self.parse_type();
                    }
                    type_params.push(TypeParam {
                        attributes: attributes,
                        identifier: identifier,
                        bounds: bounds,
                        ty: ty,
                    });
                }
            }
            Some(Generics {
                lifetime_params: lifetime_params,
                type_params: type_params,
            })
        }
        else {
            None
        }
    }
}
