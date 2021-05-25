// Echidna - Codec - Macros

use crate::*;

pub(crate) enum Type {
    Path(Path),
    Tuple(Vec<Type>),
    Array {
        ty: Box<Type>,
        expr: Expr,
    },
}

impl fmt::Display for Type {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Path(p) => write!(f,"{}",p),
            Type::Tuple(t) => {
                let mut a = String::new();
                a += "(";
                let mut first = true;
                for ty in t {
                    if first {
                        first = false;
                    }
                    else {
                        a += ",";
                    }
                    a += &format!("{}",ty);
                }
                a += ")";
                write!(f,"{}",a)
            },
            Type::Array { ty,expr } => {
                write!(f,"[{}; {}]",ty,expr)
            },
        }
    }
}

impl Lexer {

    // TupleType = `(` [ Type { `,` Type } [ `,` ] `)` .
    // ArrayType = `[` Type `;` Expr `]` .
    // Type = Path | TupleType | ArrayType .
    pub(crate) fn parse_type(&mut self) -> Option<Type> {
        if let Some(group) = self.parse_paren_group() {
            let mut lexer = Lexer::new(group.stream());
            let mut types = Vec::<Type>::new();
            while let Some(_) = &lexer.token {
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
            let mut lexer = Lexer::new(group.stream());
            if let Some(ty) = lexer.parse_type() {
                if lexer.parse_punct(';') {
                    if let Some(expr) = lexer.parse_expr() {
                        Some(Type::Array {
                            ty: Box::new(ty),
                            expr: expr,
                        })
                    }
                    else {
                        panic!("expression expected after `;`");
                    }
                }
                else {
                    panic!("`;` expected");
                }
            }
            else {
                panic!("type expected in `[` `]`");
            }
        }
        else {
            if let Some(path) = self.parse_path() {
                Some(Type::Path(path))
            }
            else {
                None
            }
        }
    }
}
