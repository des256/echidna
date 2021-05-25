// Echidna - Codec - Macros

use crate::*;

#[derive(Debug)]
pub struct SimplePath {
    pub absolute: bool,
    pub segments: Vec<SimplePathSegment>,
}

impl SimplePath {
    pub fn render(&self) -> String {
        let mut a = "".to_string();
        if self.absolute {
            a += "::";
        }
        let cnt = self.segments.len();
        for segment in self.segments {
            a += &segment.render();
            cnt -= 1;
            if cnt > 0 {
                a += "::";
            }
        }
        a
    }
}

#[derive(Debug)]
pub enum SimplePathSegment {
    Identifier(String),
    Super,
    Self_,
    Crate,
    MacroCrate,
}

impl SimplePathSegment {
    pub fn render(&self) -> String {
        match self {
            SimplePathSegment::Identifier(identifier) => identifier.clone(),
            SimplePathSegment::Super => "super".to_string(),
            SimplePathSegment::Self_ => "self".to_string(),
            SimplePathSegment::Crate => "crate".to_string(),
            SimplePathSegment::MacroCrate => "$crate".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct TypePath {
    pub absolute: bool,
    pub segments: Vec<TypePathSegment>,
}

impl TypePath {
    pub fn render(&self) -> String {
        let mut a = "".to_string();
        if self.absolute {
            a += "::";
        }
        let cnt = self.segments.len();
        for segment in self.segments {
            a += &segment.render();
            cnt -= 1;
            if cnt != 0 {
                a += "::";
            }
        }
        a
    }
}

#[derive(Debug)]
pub enum TypePathSegment {
    Naked(PathIdentSegment),
    Generic {
        segment: PathIdentSegment,
        args: GenericArgs,
    },
    Function {
        segment: PathIdentSegment,
        parameters: Vec<Type>,
        result: Option<Type>,
    },
}

impl TypePathSegment {
    pub fn render(&self) -> String {
        match self {
            TypePathSegment::Naked(segment) => segment.render(),
            TypePathSegment::Generic { segment,args } => {
                let mut a = segment.render();
                a += "::";
                a += args.render();
                a
            },
            TypePathSegment::Function { segment,parameters,result } => {
                let mut a = segment.render();
                a += "(";
                let mut count = parameters.len();
                for parameter in parameters {
                    a += parameter.render();
                    count -= 1;
                    if count > 0 {
                        a += ",";
                    }
                }
                a += ")";
                if let Some(ty) = result {
                    a += "->";
                    a += ty.render();
                }
                a
            }
        }
    }
}

#[derive(Debug)]
pub enum PathIdentSegment {
    Identifier(String),
    Super,
    Self_,
    CapitalSelf,
    Crate,
    MacroCrate,
}

impl PathIdentSegment {
    pub fn render(&self) -> String {
        match self {
            PathIdentSegment::Identifier(identifier) => identifier.clone(),
            PathIdentSegment::Super => "super".to_string(),
            PathIdentSegment::Self_ => "self".to_string(),
            PathIdentSegment::CapitalSelf => "Self".to_string(),
            PathIdentSegment::Crate => "crate".to_string(),
            PathIdentSegment::MacroCrate => "$crate".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Visibility {
    Private,
    Pub,
    PubCrate,
    PubSelf,
    PubSuper,
    PubIn(SimplePath),
}

impl Visibility {
    pub fn render(&self) -> String {
        match self {
            Visibility::Private => "".to_string(),
            Visibility::Pub => "pub ".to_string(),
            Visibility::PubCrate => "pub(crate) ".to_string(),
            Visibility::PubSelf => "pub(self) ".to_string(),
            Visibility::PubSuper => "pub(super) ".to_string(),
            Visibility::PubIn(path) => {
                let a = "pub(in ".to_string();
                a += &path.render();
                a += ")";
                a
            }
        }
    }
}

impl Lexer {

    // SimplePathSegment = `super` | `self` | `crate` | `$crate` | IDENTIFIER .
    pub fn parse_simple_path_segment(&mut self) -> SimplePathSegment {
        if self.parse_ident("super") {
            SimplePathSegment::Super
        }
        else if self.parse_ident("self") {
            SimplePathSegment::Self_
        }
        else if self.parse_ident("crate") {
            SimplePathSegment::Crate
        }
        else if self.is_some_ident() {
            let ident = ident.to_string();
            self.step();
            SimplePathSegment::Identifier(ident)
        }
        else if self.is_punct('$') {
            self.step();
            if self.is_ident("crate") {
                self.step();
                SimplePathSegment::MacroCrate
            }
            else {
                panic!("'crate' expected after '$' in simple path segment");
            }
        }
        else {
            panic!("simple path segment expected");
        }
    }    

    // SimplePath = [ `::` ] SimplePathSegment { `::` SimplePathSegment } .
    pub fn parse_simple_path(&mut self) -> SimplePath {
        let absolute = self.parse_punct2(':',':');
        let mut segments = Vec::<SimplePathSegment>::new();
        segments.push(self.parse_simple_path_segment());
        while self.parse_punct2(':',':') {
            segments.push(self.parse_simple_path_segment());
        }    
        SimplePath {
            absolute: absolute,
            segments: segments,
        }
    }

    // [ Attribute ] = [ `#` `[` ... `]` ] .
    pub fn parse_attribute(&mut self) -> Option<Group> {
        if self.parse_punct('#') {
            if let Some(group) = self.parse_bracket_group() {
                Some(group)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    // PathIdentSegment = IDENTIFIER | `super` | `self` | `Self` | `crate` | `$crate` .
    pub fn parse_path_ident_segment(&mut self) -> PathIdentSegment {
        if self.parse_ident("super") {
            PathIdentSegment::Super
        }
        else if self.parse_ident("self") {
            PathIdentSegment::Self_
        }
        else if self.parse_ident("Self") {
            PathIdentSegment::CapitalSelf
        }
        else if self.parse_ident("crate") {
            PathIdentSegment::Crate
        }
        else if self.is_some_ident() {
            let ident = ident.to_string();
            self.step();
            PathIdentSegment::Identifier(ident)
        }
        else if self.is_punct('$') {
            self.step();
            if self.is_ident("crate") {
                self.step();
                PathIdentSegment::MacroCrate
            }
            else {
                panic!("'crate' expected after '$' in path identifier segment");
            }
        }
        else {
            panic!("path identifier segment expected");
        }
    }

    // TypePathSegment = PathIdentSegment [ `::` ] [ GenericArgs | TypePathFn ] .
    pub fn parse_type_path_segment(&mut self) -> TypePathSegment {
        let segment = self.parse_path_ident_segment();
        self.parse_punct2(':',':');
        if let Some(args) = self.parse_generic_args() {
            TypePathSegment::Generic {
                segment: segment,
                args: args,
            }
        }
        else {
            if let Some(group) = self.parse_paren_group() {
                let mut lexer = Lexer::new(group.stream());
                let parameters = Vec::<Type>::new();
                while let Some(ty) = self.parse_type() {
                    parameters.push(ty);
                    self.parse_punct(',');
                }
                let result = if self.parse_punct('-') {
                    if self.parse_punct('>') {
                        self.parse_type()
                    }
                    else {
                        panic!("`>` expected after `-`")
                    }
                }
                else {
                    None
                };
                TypePathSegment::Function {
                    segment: segment,
                    parameters: parameters,
                    result: result,
                }
            }
            else {
                TypePathSegment::Naked(segment)
            }            
        }
    }

    // TypePath = [ `::` ] TypePathSegment { `::` TypePathSegment } .
    pub fn parse_type_path(&mut self) -> TypePath {
        let absolute = self.parse_punct2(':',':');
        let mut segments = Vec::<TypePathSegment>::new();
        segments.push(self.parse_type_path_segment());
        while self.parse_punct2(':',':') {
            segments.push(self.parse_type_path_segment());
        }    
        TypePath {
            absolute: absolute,
            segments: segments,
        }
    }

    // [ Visibility ] = [ `pub` [ `(` `crate` | `self` | `super` | ( `in` SimplePath ) `)` ] ] .
    pub fn parse_visibility(&mut self) -> Visibility {
        if self.parse_ident("pub") {
            if let Some(group) = self.parse_paren_group() {
                let mut lexer = Lexer::new(group.stream());
                if lexer.parse_ident("crate") {
                    Visibility::PubCrate
                }
                else if lexer.parse_ident("self") {
                    Visibility::PubSelf
                }
                else if lexer.parse_ident("super") {
                    Visibility::PubSuper
                }
                else if lexer.parse_ident("in") {
                    Visibility::PubIn(lexer.parse_simple_path())
                }
                else {
                    Visibility::Pub
                }
            }
            else {
                Visibility::Private
            }
        }
        else {
            Visibility::Private
        }
    }
}