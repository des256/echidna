// Echidna - Codec - Macros

use crate::*;

pub(crate) struct Expr { }

impl fmt::Display for Expr {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Expr")
    }
}

pub(crate) enum Visibility {
    Private,
    Public,
    PubCrate,
    PubSelf,
    PubSuper,
    PubIn(Path),
}

impl fmt::Display for Visibility {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::Private => write!(f,"Visibility::Private"),
            Visibility::Public => write!(f,"Visibility::Public"),
            Visibility::PubCrate => write!(f,"Visibility::PubCrate"),
            Visibility::PubSelf => write!(f,"Visibility::PubSelf"),
            Visibility::PubSuper => write!(f,"Visibility::PubSuper"),
            Visibility::PubIn(path) => write!(f,"Visibility::PubIn({})",path),
        }
    }
}

impl Lexer {

    pub(crate) fn parse_expr(&mut self) -> Option<Expr> {
        None
    }

    // Visibility = `pub` [ `(` `crate` | `self` | `super` | ( `in` Path ) `)` ] .
    pub(crate) fn parse_visibility(&mut self) -> Visibility {
        if self.parse_ident("pub") {
            if let Some(group) = self.parse_paren_group() {
                let mut lexer = Lexer::new(group.stream());
                if lexer.parse_ident("in") {
                    if let Some(path) = lexer.parse_path() {
                        Visibility::PubIn(path)
                    }
                    else {
                        panic!("crate path expected after `in`");
                    }
                }
                else if lexer.parse_ident("crate") {
                    Visibility::PubCrate
                }
                else if lexer.parse_ident("self") {
                    Visibility::PubSelf
                }
                else if lexer.parse_ident("super") {
                    Visibility::PubSuper
                }
                else {
                    Visibility::Public
                }
            }
            else {
                Visibility::Public
            }
        }
        else {
            Visibility::Private
        }
    }

    // Attr = `#` `[` Path [ DelimTokenTree | ( `=` LiteralSuffixlessExpr ) ] `]` .
    pub(crate) fn parse_attr(&mut self) -> Option<Group> {
        if self.parse_punct('#') {
            self.parse_bracket_group()
        }
        else {
            None
        }
    }
}
