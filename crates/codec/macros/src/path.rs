// Echidna - Codec - Macros

use crate::*;

pub(crate) enum GenericArg {
    Lifetime(String),
    Type(Type),
    Binding {
        ident: String,
        ty: Box<Type>,
    },
    Qualifier {
        ident: String,
        path: Box<Path>,
    },
}

impl fmt::Display for GenericArg {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenericArg::Lifetime(ident) => write!(f,"'{}",ident),
            GenericArg::Type(ty) => write!(f,"{}",ty),
            GenericArg::Binding { ident,ty } => write!(f,"{}: {}",ident,ty),
            GenericArg::Qualifier { ident,path } => write!(f,"{} as {}",ident,path),
        }
    }
}

pub(crate) enum PathSeg {
    Ident(String),
    Generic(Vec<GenericArg>),
}

impl fmt::Display for PathSeg {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathSeg::Ident(ident) => write!(f,"{}",ident),
            PathSeg::Generic(args) => {
                let mut a = String::new();
                a += "<";
                let mut first = true;
                for arg in args {
                    if first {
                        first = false;
                    }
                    else {
                        a += ",";
                    }
                    a += &format!("{}",arg);
                }
                a += ">";
                write!(f,"{}",a)
            },
        }
    }
}

pub(crate) struct Path {
    pub(crate) abs: bool,
    pub(crate) segs: Vec<PathSeg>,
}

impl fmt::Display for Path {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut a = String::new();
        if self.abs {
            a += "::";
        }
        let mut first = true;
        for seg in &self.segs {
            if first {
                first = false;
            }
            else {
                a += "::";
            }
            a += &format!("{}",seg);
        }
        write!(f,"{}",a)
    }
}

impl Lexer {
    pub(crate) fn parse_path_seg(&mut self) -> Option<PathSeg> {

        if self.parse_punct('<') {
            let mut args = Vec::<GenericArg>::new();
            while !self.is_punct('>') {
                if self.parse_punct('\'') {
                    if let Some(ident) = self.parse_some_ident() {
                        args.push(GenericArg::Lifetime(ident));
                    }
                    else {
                        panic!("identifier expected after `'`");
                    }
                }
                else if let Some(ident) = self.parse_some_ident() {
                    if self.parse_punct('=') {
                        if let Some(ty) = self.parse_type() {
                            args.push(GenericArg::Binding { ident: ident.clone(), ty: Box::new(ty), });
                        }
                        else {
                            panic!("type expected after `=`");
                        }
                    }
                    if self.parse_ident("as") {
                        if let Some(path) = self.parse_path() {
                            args.push(GenericArg::Qualifier { ident: ident.clone(), path: Box::new(path), });
                        }
                        else {
                            panic!("path expected after `as`")
                        }
                    }
                    else {
                        let mut segs = Vec::<PathSeg>::new();
                        segs.push(PathSeg::Ident(ident));
                        args.push(GenericArg::Type(Type::Path(Path { abs: false, segs: segs, })));
                    }
                }
                else {
                    if let Some(ty) = self.parse_type() {
                        args.push(GenericArg::Type(ty));
                    }
                    else {
                        panic!("type expected");
                    }
                }
                self.parse_punct(',');
            }
            self.parse_punct('>');
            Some(PathSeg::Generic(args))
        }
        else if let Some(ident) = self.parse_some_ident() {
            Some(PathSeg::Ident(ident))
        }       
        else {
            None
        }
    }

    pub(crate) fn parse_path(&mut self) -> Option<Path> {
        let abs = if self.is_punct(':') {
            self.parse_punct2(':',':');
            true
        }
        else {
            false
        };
        let mut segs = Vec::<PathSeg>::new();
        if let Some(seg) = self.parse_path_seg() {
            segs.push(seg);
            while self.is_punct(':') || self.is_punct('<') {
                self.parse_punct2(':',':');
                if let Some(seg) = self.parse_path_seg() {
                    segs.push(seg);
                }
                else {
                    panic!("path segment expected after `::`");
                }
            }
            Some(Path {
                abs: abs,
                segs: segs,
            })
        }
        else {
            None
        }
    }
}
