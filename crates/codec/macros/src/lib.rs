// Echidna - Codec - Macros

extern crate proc_macro;
use {
    proc_macro::{
        TokenStream,
        TokenTree,
        Delimiter,
        token_stream::IntoIter,
        Spacing,
    },
};

mod rust {

    use super::TokenTree;

    // StructField = { OuterAttribute } [ Visibility ] IDENTIFIER `:` Type .
    pub struct StructField {
        pub attributes: Vec<TokenTree>,
        pub visibility: Visibility,
        pub identifier: String,
        pub ty: Type,
    }

    // TupleField = { OuterAttribute } [ Visibility ] Type .
    pub struct TupleField {
        pub attributes: Vec<TokenTree>,
        pub visibility: Visibility,
        pub ty: Type,
    }

    pub struct TupleItem {
        pub attributes: Vec<TokenTree>,
        pub visibility: Visibility,
        pub identifier: String,
        pub fields: Vec<TupleField>,
    }

    pub struct StructItem {
        pub attributes: Vec<TokenTree>,
        pub visibility: Visibility,
        pub identifier: String,
        pub fields: Vec<StructField>,
    }

    pub struct DiscriminantItem {
        pub attributes: Vec<TokenTree>,
        pub visibility: Visibility,
        pub identifier: String,
        pub expression: Expression,
    }

    // EnumItem = { OuterAttribute } [ Visibility ] IDENTIFIER [ EnumItemTuple | EnumItemStruct | EnumItemDiscriminant ] .
    pub enum EnumItem {
        Tuple(TupleItem),
        Struct(StructItem),
        Discriminant(DiscriminantItem),
    }

    pub struct Struct {
        pub attributes: Vec<TokenTree>,
        pub visibility: Visibility,
        pub identifier: String,
        pub generics: Option<Generics>,
        pub where_clause: Vec<WhereClauseItem>,
        pub fields: Vec<StructField>,
    }

    pub struct Tuple {
        pub attributes: Vec<TokenTree>,
        pub visibility: Visibility,
        pub identifier: String,
        pub generics: Option<Generics>,
        pub where_clause: Vec<WhereClauseItem>,
        pub fields: Vec<TupleField>,
    }

    pub struct Enum {
        pub attributes: Vec<TokenTree>,
        pub visibility: Visibility,
        pub identifier: String,
        pub generics: Option<Generics>,
        pub where_clause: Vec<WhereClauseItem>,
        pub items: Vec<EnumItem>,
    }

    pub struct Union {
        pub attributes: Vec<TokenTree>,
        pub visibility: Visibility,
        pub identifier: String,
        pub generics: Option<Generics>,
        pub where_clause: Vec<WhereClauseItem>,
        pub fields: Vec<StructField>,
    }
    
    // Item = { OuterAttribute } [ Visibility ] Struct | Enumeration | Union | ... .
    pub enum Item {
        Struct(Struct),
        Tuple(Tuple),
        Enum(Enum),
        Union(Union),
    }


    // WhereClauseItem = LifetimeWhereClauseItem | TypeBoundWhereClauseItem .
    pub enum WhereClauseItem {

        // LifetimeWhereClauseItem = Lifetime `:` LifetimeBounds .
        Lifetime {
            lifetime: Lifetime,
            bounds: Vec<Lifetime>,  // LifetimeBounds = Lifetime { `+` Lifetime } [ `+` ] .
        },

        // TypeBoundWhereClauseItem = [ ForLifetimes ] Type `:` [ TypeParamBounds ] .
        TypeBound {
            for_lifetimes: Vec<LifetimeParam>,  // ForLifetimes = `for` `<` LifetimeParam { `,` LifetimeParam } `>` .
            ty: Type,
            bounds: Option<Vec<TypeParamBound>>,  // TypeParamBounds = TypeParamBound { `+` TypeParamBound } [ `+` ] .
        },
    }

    // Visibility = `pub` [ `(` `crate` | `self` | `super` | ( `in` SimplePath ) `)` ] .
    pub enum Visibility {
        Private,
        Pub,
        PubCrate,
        PubSelf,
        PubSuper,
        PubIn(SimplePath),
    }

    // OuterAttribute = `#` `[` ... `]` .

    // SimplePath = [ `::` ] SimplePathSegment { `::` SimplePathSegment } .
    pub struct SimplePath {
        pub absolute: bool,
        pub segments: Vec<SimplePathSegment>,
    }

    // SimplePathSegment = IDENTIFIER | `super` | `self` | `crate` | `$crate` .
    pub enum SimplePathSegment {
        Identifier(String),
        Super,
        Self_,
        Crate,
        MacroCrate,
    }

    // Generics = `<` [ GenericParam { `,` GenericParam } [ `,` ] `>` .
    // GenericParam = [ OuterAttribute ] LIFETIME_OR_LABEL | IDENTIFIER [ `:` LifetimeBounds | [ TypeParamBounds ] ] [ `=` Type ] .
    pub struct Generics {
        pub lifetime_params: Vec<LifetimeParam>,
        pub type_params: Vec<TypeParam>,
    }

    // LifetimeParam = [ OuterAttribute ] LIFETIME_OR_LABEL [ `:` LifetimeBounds ] .
    pub struct LifetimeParam {
        pub attributes: Vec<TokenTree>,
        pub identifier: String,
        pub bounds: Vec<Lifetime>,  // LifetimeBounds = Lifetime { `+` Lifetime } [ `+` ] .
    }

    // TypeParam = [ OuterAttribute ] IDENTIFIER [ `:` [ TypeParamBounds ] ] [ `=` Type ] .
    pub struct TypeParam {
        pub attributes: Vec<TokenTree>,
        pub identifier: String,
        pub bounds: Vec<TypeParamBound>,  // TypeParamBounds = TypeParamBound { `+` TypeParamBound } [ `+` ] .
        pub ty: Option<Type>,
    }

    // TypeParamBound = Lifetime | TraitBound .  
    pub enum TypeParamBound {
        Lifetime(Lifetime),
        TraitBound(TraitBound),
    }

    // Lifetime = LIFETIME_OR_LABEL | `'static` | `'_` .
    pub enum Lifetime {
        Static,
        Anonymous,
        Named(String),
    }

    // TraitBound = ( [ `?` ] [ ForLifetimes ] TypePath ) | ( `(` [ `?` ] [ ForLifetimes ] TypePath `)` ) .
    pub struct TraitBound {
        pub for_lifetimes: Vec<LifetimeParam>,  // ForLifetimes = `for` `<` LifetimeParam { `,` LifetimeParam } `>` .
        pub path: TypePath,
    }

    // TypePath = [ `::` ] TypePathSegment { `::` TypePathSegment } .
    pub struct TypePath {
        absolute: bool,
        segments: Vec<TypePathSegment>,
    }

    // TypePathSegment = PathIdentSegment [ `::` ] [ GenericArgs | TypePathFn ] .
    pub enum TypePathSegment {
        Generic {
            segment: PathIdentSegment,
            args: GenericArgs,
        },

        Function {
            segment: PathIdentSegment,
            // TypePathFn = `(` [ TypePathFnInputs ] `)` [ `->` Type ] .
            parameters: Vec<Type>,  // TypePathFnInputs = Type { `,` Type } [ `,` ] .
            result: Option<Type>,
        },
    }

    // PathIdentSegment = IDENTIFIER | `super` | `self` | `Self` | `crate` | `$crate` .
    pub enum PathIdentSegment {
        Identifier(String),
        Super,
        Self_,
        CapitalSelf,
        Crate,
        MacroCrate,
    }

    // GenericArgs = `<` [ GenericArgsLifetimes ] [ GenericArgsTypes ] [ GenericArgsBindings ] `>` .
    pub struct GenericArgs {
        lifetimes: Vec<Lifetime>,  // GenericArgsLifetimes = Lifetime { `,` Lifetime } [ `,` ] .
        types: Vec<Type>,  // GenericArgsTypes = Type { `,` Type } [ `,` ] .
        bindings: Vec<GenericArgsBinding>,  // GenericArgsBindings = GenericArgsBinding { `,` GenericArgsBinding } .
    }

    // GenericArgsBinding = IDENTIFIER `=` Type .
    pub struct GenericArgsBinding {
        identifier: String,
        ty: Type,
    }

    // Type = TypeNoBounds | ImplTraitType | TraitObjectType .
    // TypeNoBounds = ParenthesizedType | ImplTraitTypeOneBound | TraitObjectTypeOneBound | TypePath | TupleType | NeverType | RawPointerType | ReferenceType | ArrayType | SliceType | InferredType | QualifiedPathInType | BareFunctionType | MacroInvocation .
    pub enum Type {

        // ImplTraitType = `impl` TypeParamBounds .
        ImplTrait(Vec<TypeParamBound>),  // TypeParamBounds = TypeParamBound { `+` TypeParamBound } [ `+` ] .
        
        // TraitObjectType = [ `dyn` ] TypeParamBounds .
        TraitObject(Vec<TypeParamBound>),  // TypeParamBounds = TypeParamBound { `+` TypeParamBound } [ `+` ] .

        // ImplTraitTypeOneBound = `impl` TraitBound .
        ImplTraitOne(TraitBound),

        // TraitObjectTypeOneBound = [ `dyn` ] TraitBound .
        TraitObjectOne(TraitBound),

        // ParenthesizedType = `(` Type `)` .
        Paren(Box<Type>),

        Path,

        // TupleType = `(` [ Type { `,` Type } [ `,` ] `)` .
        Tuple(Vec<Type>),

        // NeverType = `!` .
        Never,

        // RawPointerType = `*` `mut` | `const` TypeNoBounds .
        RawPtr {
            mutable: bool,
            ty: Box<Type>,
        },

        // ReferenceType = `&` [ Lifetime ] [ `mut` ] TypeNoBounds .
        Ref {
            lifetime: Option<Lifetime>,
            mutable: bool,
            ty: Box<Type>,
        },

        // ArrayType = [ Type `;` Expression ] .
        Array {
            ty: Box<Type>,
            expression: Expression,
        },

        // SliceType = `[` Type `]` .
        Slice(Box<Type>),

        // InferredType = `_` .
        Inferred,

        QPath,

        // BareFunctionType = [ ForLifetimes ] FunctionQualifiers `fn` `(` [ FunctionParametersMaybeNamedVariadic ] `)` [ BareFunctionReturnType ] .
        Function {
            for_lifetimes: Vec<Lifetime>,
            qualifiers: FunctionQualifiers,
            parameters: FunctionParametersMaybeNamedVariadic,
            result: Box<Type>,  // BareFunctionReturnType = `->` TypeNoBounds .
        },

        // MacroInvocation = SimplePath `!` DelimTokenTree .
        Macro {
            path: SimplePath,
            tokens: TokenTree,
        }
    }

    // FunctionParametersMaybeNamedVariadic = MaybeNamedFunctionParameters | MaybeNamedFunctionParametersVariadic .
    pub enum FunctionParametersMaybeNamedVariadic {
        // MaybeNamedFunctionParameters = MaybeNamedParam { `,` MaybeNamedParam } [ `,` ] .
        MaybeNamed {
            params: Vec<MaybeNamedParam>,
        },

        // MaybeNamedFunctionParametersVariadic = { MaybeNamedParam `,` } MaybeNamedParam `,` { OuterAttribute } `...` .
        MaybeNamedVariadic {
            params: Vec<MaybeNamedParam>,
            attributes: Vec<TokenTree>,
        },
    }

    // MaybeNamedParam = { OuterAttribute } [ IDENTIFIER | `_` `:` ] Type .
    pub struct MaybeNamedParam {
        attributes: Vec<TokenTree>,
        identifier: Option<String>,
        ty: Box<Type>,
    }

    // FunctionQualifiers = [ AsyncConstQualifiers ] [ `unsafe` ] [ `extern` [ Abi ] ] .
    pub struct FunctionQualifiers {
        prelude: FunctionQualifiersPrelude,
        unsafety: bool,
        abi: Option<String>,
    }

    pub enum FunctionQualifiersPrelude {

        None,

        // AsyncConstQualifiers = `async` | `const` .
        Async,
        Const,
    }

    // TODO: Expression
    pub type Expression = String;
}

fn dump(stream: &mut proc_macro::token_stream::IntoIter) -> String {
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
                result += &dump(&mut stream);
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

struct Lexer {
    token: Option<TokenTree>,
    stream: IntoIter,
}

impl Lexer {
    fn new(stream: TokenStream) -> Lexer {
        let mut stream = stream.into_iter();
        let token = stream.next();
        Lexer {
            token: token,
            stream: stream,
        }
    }

    fn step(&mut self) {
        self.token = self.stream.next();
    }

    fn peek_alone_punct(&self,c: char) -> bool {
        if let Some(TokenTree::Punct(punct)) = &self.token {
            if (punct.as_char() == c) && (punct.spacing() == Spacing::Alone) {
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    fn peek_joint_punct(&self,c: char) -> bool {
        if let Some(TokenTree::Punct(punct)) = &self.token {
            if (punct.as_char() == c) && (punct.spacing() == Spacing::Joint) {
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }
}

fn parse_double_semi_opt(lexer: &mut Lexer) -> bool {
    // [ `::` ]
    if lexer.peek_joint_punct(':') {
        lexer.step();
        if lexer.peek_alone_punct(':') {
            lexer.step();
            true
        }
        else {
            // error: `:` should follow `:`
            false
        }
    }
    else {
        false
    }
}

fn parse_simple_path_segment(lexer: &mut Lexer) -> Option<rust::SimplePathSegment> {

    if let Some(token) = &mut lexer.token {
        match &token {
            TokenTree::Ident(ident) => {
                let ident = ident.to_string();
                if ident == "super" {
                    lexer.step();
                    Some(rust::SimplePathSegment::Super)
                }
                else if ident == "self" {
                    lexer.step();
                    Some(rust::SimplePathSegment::Self_)
                }
                else if ident == "crate" {
                    lexer.step();
                    Some(rust::SimplePathSegment::Crate)
                }
                else {
                    lexer.step();
                    Some(rust::SimplePathSegment::Identifier(ident))
                }
            },
    
            TokenTree::Punct(punct) => {
                if punct.as_char() == '$' {
                    lexer.step();
                    if let Some(TokenTree::Ident(ident)) = &lexer.token {
                        if ident.to_string() == "crate" {
                            Some(rust::SimplePathSegment::MacroCrate)
                        }
                        else {
                            // error: `$` should be followed by `crate`
                            None
                        }
                    }
                    else {
                        // error: `$` should be followed by `crate`
                        None
                    }
                }
                else {
                    // error: simple path segment expected
                    None
                }
            },
    
            _ => {
                // error: simple path segment expected
                None
            }    
        }
    }
    else {
        // error: simple path segment expected
        None
    }
}

fn parse_simple_path(lexer: &mut Lexer) -> Option<rust::SimplePath> {

    // SimplePath = [ `::` ] SimplePathSegment { `::` SimplePathSegment } .
    let absolute = parse_double_semi_opt(lexer);
    let mut segments = Vec::<rust::SimplePathSegment>::new();

    // SimplePathSegment
    if let Some(segment) = parse_simple_path_segment(lexer) {
        segments.push(segment);
    }
    else {
        // error: (handled by parse_simple_path_segment)
        return None;
    }

    // { `::` SimplePathSegment }
    while parse_double_semi_opt(lexer) {
        if let Some(segment) = parse_simple_path_segment(lexer) {
            segments.push(segment);
        }
        else {
            // error: (handled by parse_simple_path_segment)
            return None;
        }
    }

    Some(rust::SimplePath {
        absolute: absolute,
        segments: segments,
    })
}

fn parse_visibility(lexer: &mut Lexer) -> Option<rust::Visibility> {
    // [ Visibility ] = [ `pub` [ `(` `crate` | `self` | `super` | ( `in` SimplePath ) `)` ] ] .
    if let Some(TokenTree::Ident(ident)) = &lexer.token {
        if ident.to_string() == "pub" {
            lexer.step();
            // [ `(` `crate` | `self` | `super` | ( `in` SimplePath ) `)` ]
            if let Some(TokenTree::Group(group)) = &lexer.token {
                if group.delimiter() == Delimiter::Parenthesis {
                    // `crate` | `self` | `super` | ( `in` SimplePath )
                    let mut lexer = Lexer::new(group.stream());
                    if let Some(TokenTree::Ident(ident)) = &lexer.token {
                        let ident = ident.to_string();
                        if ident == "crate" {
                            lexer.step();
                            Some(rust::Visibility::PubCrate)
                        }
                        else if ident == "self" {
                            lexer.step();
                            Some(rust::Visibility::PubSelf)
                        }
                        else if ident == "super" {
                            lexer.step();
                            Some(rust::Visibility::PubSuper)
                        }
                        else if ident == "in" {
                            lexer.step();
                            if let Some(simple_path) = parse_simple_path(&mut lexer) {
                                Some(rust::Visibility::PubIn(simple_path))
                            }
                            else {
                                // error: simple path expected after `in`
                                None
                            }
                        }
                        else {
                            Some(rust::Visibility::Pub)
                        }
                    }
                    else {
                        Some(rust::Visibility::Pub)
                    }
                }
                else {
                    Some(rust::Visibility::Pub)
                }
            }
            else {
                Some(rust::Visibility::Pub)
            }
        }
        else {
            Some(rust::Visibility::Private)
        }
    }
    else {
        Some(rust::Visibility::Private)
    }
}

fn parse_outer_attribute_opt(lexer: &mut Lexer) -> Option<TokenTree> {
    // [ OuterAttribute ] = `#` `[` ... `]` .
    if let Some(TokenTree::Punct(punct)) = &lexer.token {
        if punct.as_char() == '#' {
            lexer.step();
            if let Some(TokenTree::Group(group)) = &lexer.token {
                if group.delimiter() == Delimiter::Bracket {
                    let group = group.clone();
                    lexer.step();
                    // `[` ... `]`
                    Some(TokenTree::Group(group))
                }
                else {
                    // error: `#` should be followed by `[`
                    None
                }
            }
            else {
                // error: `#` should be followed by `[`
                None
            }
        }
        else {
            // not an outer attribute
            None
        }
    }
    else {
        // not an outer attribute
        None
    }
}

fn parse_lifetime_opt(lexer: &mut Lexer) -> Option<rust::Lifetime> {
    if let Some(TokenTree::Punct(punct)) = &lexer.token {
        if punct.as_char() != '`' {
            return None;
        }
    }
    lexer.step();
    let identifier = if let Some(TokenTree::Ident(ident)) = &lexer.token {
        let ident = ident.to_string();
        lexer.step();
        ident
    }
    else {
        return None;
    };
    if identifier == "static" {
        Some(rust::Lifetime::Static)
    }
    else if identifier == "_" {
        Some(rust::Lifetime::Anonymous)
    }
    else {
        Some(rust::Lifetime::Named(identifier))
    }
}

fn parse_type_param_bound_opt(lexer: &mut Lexer) -> Option<rust::TypeParamBound> {
    // Lifetime | TraitBound .
    if let Some(TokenTree::Punct(punct)) = &lexer.token {
        if punct.as_char() != '`' {
            lexer.step();
            let identifier = if let Some(TokenTree::Ident(ident)) = &lexer.token {
                let ident = ident.to_string();
                lexer.step();
                ident
            }
            else {
                return None;
            };
            if identifier == "static" {
                Some(rust::TypeParamBound::Lifetime(rust::Lifetime::Static))
            }
            else if identifier == "_" {
                Some(rust::TypeParamBound::Lifetime(rust::Lifetime::Anonymous))
            }
            else {
                Some(rust::TypeParamBound::Lifetime(rust::Lifetime::Named(identifier)))
            }
        }
        else {
            // error: ``` expected
            None
        }
    }
    else if let Some(TokenTree::Group(group)) = &lexer.token {
        if group.delimiter() == Delimiter::Parenthesis {
            let mut lexer = Lexer::new(group.stream());
            // [ `?` ] [ ForLifetimes ] TypePath
            let for_lifetimes = Vec::<rust::LifetimeParam>::new();
            Some(rust::TypeParamBound::TraitBound(rust::TraitBound {
                for_lifetimes: for_lifetimes,
                path: path,
            }))
        }
        else {
            // error: `(` expected
            None
        }
    }
    else {
        // [ `?` ] [ ForLifetimes ] TypePath
        let for_lifetimes = Vec::<rust::LifetimeParam>::new();
        Some(rust::TypeParamBound::TraitBound(rust::TraitBound {
            for_lifetimes: for_lifetimes,
            path: path,
        }))
    }
}

fn parse_generics_opt(lexer: &mut Lexer) -> Option<rust::Generics> {

    // `<` [ GenericParam { `,` GenericParam } [ `,` ] `>` .
    if let Some(TokenTree::Punct(punct)) = &lexer.token {
        if punct.as_char() != '<' {
            return None;
        }
    }
    else {
        return None;
    }
    lexer.step();

    let mut lifetime_params = Vec::<rust::LifetimeParam>::new();
    let mut type_params = Vec::<rust::TypeParam>::new();
    while {
        if let Some(TokenTree::Punct(punct)) = &lexer.token {
            if punct.as_char() == '>' {
                false
            }
            else {
                true
            }
        }
        else {
            false
        }
    } {
        // [ OuterAttribute ] LIFETIME_OR_LABEL | IDENTIFIER [ `:` LifetimeBounds | [ TypeParamBounds ] ] [ `=` Type ] [ `,` ] .
        let mut attributes = Vec::<TokenTree>::new();
        while let Some(attribute) = parse_outer_attribute_opt(lexer) {
            attributes.push(attribute);
        }

        // LIFETIME_OR_LABEL | IDENTIFIER [ `:` LifetimeBounds | [ TypeParamBounds ] ] [ `=` Type ] [ `,` ] .
        let mut is_lifetime = false;
        if let Some(TokenTree::Punct(punct)) = &lexer.token {
            if punct.as_char() == '`' {
                is_lifetime = true;
                lexer.step();
            }
            else {
                // error: lifetime expected
                return None;
            }
        }

        // IDENTIFIER [ `:` LifetimeBounds | [ TypeParamBounds ] ] [ `=` Type ] [ `,` ] .
        let identifier = if let Some(TokenTree::Ident(ident)) = &lexer.token {
            let ident = ident.to_string();
            lexer.step();
            ident
        }
        else {
            // error: identifier expected
            return None;
        };

        // [ `:` LifetimeBounds | [ TypeParamBounds ] ] [ `=` Type ] [ `,` ] .
        let lifetime_bounds = Vec::<rust::Lifetime>::new();
        let type_param_bounds = Vec::<rust::TypeParamBound>::new();
        if let Some(TokenTree::Punct(punct)) = &lexer.token {
            if punct.as_char() == ':' {
                lexer.step();
                if is_lifetime {
                    // { Lifetime [ `+` ] }
                    while let Some(lifetime) = parse_lifetime_opt(&mut lexer) {
                        lifetime_bounds.push(lifetime);
                        if let Some(TokenTree::Punct(punct)) = &lexer.token {
                            if punct.as_char() == '+' {
                                lexer.step();
                            }
                        }
                    }    
                }
                else {
                    // { TypeParamBound [ '+' ] }
                    while let Some(type_param_bound) = parse_type_param_bound_opt(&mut lexer) {
                        type_param_bounds.push(type_param_bound);
                        if let Some(TokenTree::Punct(punct)) = &lexer.token {
                            if punct.as_char() == '+' {
                                lexer.step();
                            }
                        }
                    }
                }
            }
        }

        // [ `=` Type ] [ `,` ] .
        let mut ty: Option<rust::Type> = None;
        if let Some(TokenTree::Punct(punct)) = &lexer.token {
            if punct.as_char() == '=' {
                lexer.step();
                ty = parse_type(lexer);
            }
        }

        // [ `,` ] .
        if let Some(TokenTree::Punct(punct)) = &lexer.token {
            if punct.as_char() == ',' {
                lexer.step();
            }
        }

        if is_lifetime {
            lifetime_params.push(rust::LifetimeParam {
                attributes: attributes,
                identifier: identifier,
                bounds: lifetime_bounds,
            })
        }
        else {
            type_params.push(rust::TypeParam {
                attributes: attributes,
                identifier: identifier,
                bounds: type_param_bounds,
                ty: ty,
            })
        }
    }

    if let Some(TokenTree::Punct(punct)) = &lexer.token {
        if punct.as_char() == '>' {
            lexer.step();
            Some(rust::Generics {
                lifetime_params: lifetime_params,
                type_params: type_params,
            })
        }
        else {
            // error: `>` expected
            None
        }
    }
    else {
        // error: `>` expected
        None
    }
}

fn parse_where_clause_opt(lexer: &mut Lexer) -> Option<Vec<rust::WhereClauseItem>> {
    None
}

fn parse_type(lexer: &mut Lexer) -> Option<rust::Type> {
    None
}

fn parse_expression(lexer: &mut Lexer) -> Option<rust::Expression> {
    None
}

fn parse_tuple_field_opt(lexer: &mut Lexer) -> Option<rust::TupleField> {

    // { OuterAttribute } [ Visibility ] Type .
    let mut attributes = Vec::<TokenTree>::new();
    while let Some(attribute) = parse_outer_attribute_opt(lexer) {
        attributes.push(attribute);
    }

    // [ Visibility ] Type .
    let visibility = if let Some(visibility) = parse_visibility(lexer) {
        visibility
    }
    else {
        return None;
    };

    // Type .
    let ty = if let Some(ty) = parse_type(lexer) {
        ty
    }
    else {
        return None;
    };

    Some(rust::TupleField {
        attributes: attributes,
        visibility: visibility,
        ty: ty,
    })
}

fn parse_struct_field_opt(lexer: &mut Lexer) -> Option<rust::StructField> {
    
    // { OuterAttribute } [ Visibility ] IDENTIFIER `:` Type .
    let mut attributes = Vec::<TokenTree>::new();
    while let Some(attribute) = parse_outer_attribute_opt(lexer) {
        attributes.push(attribute);
    }

    // [ Visibility ] IDENTIFIER `:` Type .
    let visibility = if let Some(visibility) = parse_visibility(lexer) {
        visibility
    }
    else {
        return None;
    };

    // IDENTIFIER `:` Type .
    let identifier = if let Some(TokenTree::Ident(ident)) = &lexer.token {
        let ident = ident.to_string();
        lexer.step();
        ident
    }
    else {
        // error: identifier expected
        return None;
    };

    // `:` Type .
    if let Some(TokenTree::Punct(punct)) = &lexer.token {
        if punct.as_char() != ':' {
            // error: `:` expected
            return None;
        }
    }
    else {
        // error: `:` expected
        return None;
    }

    // Type .
    let ty = if let Some(ty) = parse_type(lexer) {
        ty
    }
    else {
        return None;
    };

    Some(rust::StructField {
        attributes: attributes,
        visibility: visibility,
        identifier: identifier,
        ty: ty,
    })
}

fn parse_enum_item_opt(lexer: &mut Lexer) -> Option<rust::EnumItem> {

    // { OuterAttribute } [ Visibility ] IDENTIFIER  ( `(` TupleItems `)` ) | ( `{` StructItems `}` ) | ( `=` Expression ) .
    let mut attributes = Vec::<TokenTree>::new();
    while let Some(attribute) = parse_outer_attribute_opt(lexer) {
        attributes.push(attribute);
    }

    // [ Visibility ] IDENTIFIER  ( `(` TupleItems `)` ) | ( `{` StructItems `}` ) | ( `=` Expression ) .
    let visibility = if let Some(visibility) = parse_visibility(lexer) {
        visibility
    }
    else {
        return None;
    };

    // IDENTIFIER ( `(` TupleItems `)` ) | ( `{` StructItems `}` ) | ( `=` Expression ) .
    let identifier = if let Some(TokenTree::Ident(ident)) = &lexer.token {
        let ident = ident.to_string();
        lexer.step();
        ident
    }
    else {
        // error: identifier expected
        return None;
    };

    // ( `(` TupleFields `)` ) | ( `{` StructFields `}` ) | ( `=` Expression ) .
    if let Some(TokenTree::Group(group)) = &lexer.token {
        if group.delimiter() == Delimiter::Parenthesis {
            let mut lexer = Lexer::new(group.stream());
            let mut fields = Vec::<rust::TupleField>::new();
            while let Some(field) = parse_tuple_field_opt(&mut lexer) {
                fields.push(field);
                if let Some(TokenTree::Punct(punct)) = &lexer.token {
                    if punct.as_char() == ',' {
                        lexer.step();
                    }
                }
            }
            Some(rust::EnumItem::Tuple(rust::TupleItem {
                attributes: attributes,
                visibility: visibility,
                identifier: identifier,
                fields: fields,
            }))
        }
        else if group.delimiter() == Delimiter::Brace {
            let mut lexer = Lexer::new(group.stream());
            let mut fields = Vec::<rust::StructField>::new();
            while let Some(field) = parse_struct_field_opt(&mut lexer) {
                fields.push(field);
                if let Some(TokenTree::Punct(punct)) = &lexer.token {
                    if punct.as_char() == ',' {
                        lexer.step();
                    }
                }
            }
            Some(rust::EnumItem::Struct(rust::StructItem {
                attributes: attributes,
                visibility: visibility,
                identifier: identifier,
                fields: fields,
            }))
        }
        else {
            // error: `(`, `{` or `=` expected
            None
        }
    }
    else if let Some(TokenTree::Punct(punct)) = &lexer.token {
        if punct.as_char() == '=' {
            lexer.step();
            if let Some(expression) = parse_expression(lexer) {
                Some(rust::EnumItem::Discriminant(rust::DiscriminantItem {
                    attributes: attributes,
                    visibility: visibility,
                    identifier: identifier,
                    expression: expression,    
                }))
            }
            else {
                None
            }
        }
        else {
            // error: `=` expected
            None
        }
    }
    else {
        // error: `(`, `{` or `=` expected
        None
    }
}

fn parse_item(lexer: &mut Lexer) -> Option<rust::Item> {

    // { OuterAttribute } [ Visibility ] `struct` | `enum` | `union` IDENTIFIER [ Generics ] [ WhereClause ] ( `(` [ TupleFields ] `)` ) | ( `{` [ StructFields ] | [ EnumItems ] `}` ) [ WhereClause ] [ `;` ] .
    let mut attributes = Vec::<TokenTree>::new();
    while let Some(attribute) = parse_outer_attribute_opt(lexer) {
        attributes.push(attribute);
    }

    // [ Visibility ] `struct` | `enum` | `union` IDENTIFIER [ Generics ] [ WhereClause ] ( `(` [ TupleFields ] `)` ) | ( `{` [ StructFields ] | [ EnumItems ] `}` ) [ WhereClause ] [ `;` ] .
    let visibility = if let Some(visibility) = parse_visibility(lexer) {
        visibility
    }
    else {
        return None;
    };
    
    // `struct` | `enum` | `union` IDENTIFIER [ Generics ] [ WhereClause ] ( `(` [ TupleFields ] `)` ) | ( `{` [ StructFields ] | [ EnumItems ] `}` ) [ WhereClause ] [ `;` ] .
    let group_ident = if let Some(TokenTree::Ident(ident)) = &lexer.token {
        let ident = ident.to_string();
        lexer.step();
        ident
    }
    else {
        // error: `struct`, `enum` or `union` expected
        return None;
    };

    // IDENTIFIER [ Generics ] [ WhereClause ] ( `(` [ TupleFields ] `)` ) | ( `{` [ StructFields ] | [ EnumItems ] `}` ) [ WhereClause ] [ `;` ] .
    let identifier = if let Some(TokenTree::Ident(ident)) = &lexer.token {
        let ident = ident.to_string();
        lexer.step();
        ident
    }
    else {
        // error: identifier expected
        return None;
    };

    // [ Generics ] [ WhereClause ] ( `(` [ TupleFields ] `)` ) | ( `{` [ StructFields ] | [ EnumItems ] `}` ) [ WhereClause ] [ `;` ] .
    let generics = parse_generics_opt(lexer);

    // [ WhereClause ] ( `(` [ TupleFields ] `)` ) | ( `{` [ StructFields ] | [ EnumItems ] `}` ) [ WhereClause ] [ `;` ] .
    let where_clause_before = if let Some(where_clause) = parse_where_clause_opt(lexer) {
        where_clause
    }
    else {
        return None;
    };

    // ( `(` [ TupleFields ] `)` ) | ( `{` [ StructFields ] | [ EnumItems ] `}` ) [ WhereClause ] [ `;` ] .
    let mut tuple_fields = Vec::<rust::TupleField>::new();
    let mut struct_fields = Vec::<rust::StructField>::new();
    let mut enum_items = Vec::<rust::EnumItem>::new();
    let mut should_be_tuple = false;
    if let Some(TokenTree::Group(group)) = &lexer.token {
        if group.delimiter() == Delimiter::Parenthesis {
            should_be_tuple = true;
            let mut lexer = Lexer::new(group.stream());

            // [ TupleFields ]
            while let Some(field) = parse_tuple_field_opt(&mut lexer) {
                tuple_fields.push(field);
                if let Some(TokenTree::Punct(punct)) = &lexer.token {
                    if punct.as_char() == ',' {
                        lexer.step();
                    }
                }
            }
        }
        else if group.delimiter() == Delimiter::Brace {
            let mut lexer = Lexer::new(group.stream());

            // [ StructFields ] | [ EnumItems ]
            if group_ident == "enum" {
                while let Some(item) = parse_enum_item_opt(&mut lexer) {
                    enum_items.push(item);
                    if let Some(TokenTree::Punct(punct)) = &lexer.token {
                        if punct.as_char() == ',' {
                            lexer.step();
                        }
                    }
                }
            }
            else {
                while let Some(field) = parse_struct_field_opt(&mut lexer) {
                    struct_fields.push(field);
                    if let Some(TokenTree::Punct(punct)) = &lexer.token {
                        if punct.as_char() == ',' {
                            lexer.step();
                        }
                    }
                }
            }
        }
        else {
            // error: `(` or `{` expected
            return None;
        }
    }
    else {
        // error: `(` or `{` expected
        return None;
    }

    // [ WhereClause ] [ `;` ] .
    let where_clause_after = if let Some(where_clause) = parse_where_clause_opt(lexer) {
        where_clause
    }
    else {
        return None;
    };

    // [ `;` ] .
    if let Some(TokenTree::Punct(punct)) = &lexer.token {
        if punct.as_char() == ';' {
            lexer.step();
        }
    }

    if group_ident == "struct" {
        if should_be_tuple {
            Some(rust::Item::Tuple(rust::Tuple {
                attributes: attributes,
                visibility: visibility,
                identifier: identifier,
                generics: generics,
                where_clause: where_clause_after,
                fields: tuple_fields,
            }))
        }
        else {
            Some(rust::Item::Struct(rust::Struct {
                attributes: attributes,
                visibility: visibility,
                identifier: identifier,
                generics: generics,
                where_clause: where_clause_before,
                fields: struct_fields,
            }))
        }
    }
    else if group_ident == "enum" {
        Some(rust::Item::Enum(rust::Enum {
            attributes: attributes,
            visibility: visibility,
            identifier: identifier,
            generics: generics,
            where_clause: where_clause_after,
            items: enum_items,
        }))
    }
    else if group_ident == "union" {
        Some(rust::Item::Union(rust::Union {
            attributes: attributes,
            visibility: visibility,
            identifier: identifier,
            generics: generics,
            where_clause: where_clause_after,
            fields: struct_fields,
        }))
    }
    else {
        // error: `struct`, `enum` or `union` expected
        None
    }
}

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

// make_struct_codec

#[proc_macro_derive(codec)]
pub fn derive_codec(stream: TokenStream) -> TokenStream {
    let mut lexer = Lexer::new(stream);
    let item = parse_item(&mut lexer);

    TokenStream::new()
}