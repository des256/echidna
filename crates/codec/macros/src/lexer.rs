// Echidna - Codec - Macros

use crate::*;

pub struct Lexer {
    pub token: Option<TokenTree>,
    pub stream: IntoIter,
}

impl Lexer {

    pub fn new(stream: TokenStream) -> Lexer {
        let mut stream = stream.into_iter();
        let token = stream.next();
        Lexer {
            token: token,
            stream: stream,
        }
    }

    pub fn step(&mut self) {
        self.token = self.stream.next();
    }

    pub fn is_punct(&self,c: char) -> bool {
        if let Some(TokenTree::Punct(punct)) = &self.token {
            if punct.as_char() == c {
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

    pub fn parse_punct(&mut self,c: char) -> bool {
        if self.is_punct(c) {
            self.step();
            true
        }
        else {
            false
        }
    }

    pub fn parse_punct2(&mut self,c0: char,c1: char) -> bool {
        if let Some(TokenTree::Punct(punct)) = &self.token {
            if (punct.as_char() == c0) && (punct.spacing() == Spacing::Joint) {
                self.step();
                if let Some(TokenTree::Punct(punct)) = &self.token {
                    if (punct.as_char() == c1) && (punct.spacing() == Spacing::Alone) {
                        self.step();
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
            else {
                false
            }
        }
        else {
            false
        }
    }

    pub fn is_some_ident(&self) -> bool {
        if let Some(TokenTree::Ident(_)) = &self.token {
            true
        }
        else {
            false
        }
    }

    pub fn is_ident(&self,s: &str) -> bool {
        if let Some(TokenTree::Ident(ident)) = &self.token {
            if ident.to_string() == s {
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

    pub fn parse_some_ident(&mut self) -> Option<String> {
        if let Some(TokenTree::Ident(ident)) = &self.token {
            let ident = ident.to_string();
            self.step();
            Some(ident)
        }
        else {
            None
        }
    }

    pub fn parse_ident(&mut self,s: &str) -> bool {
        if self.is_ident(s) {
            self.step();
            true
        }
        else {
            false
        }
    }

    pub fn is_paren_group(&self) -> bool {
        if let Some(TokenTree::Group(group)) = &self.token {
            if group.delimiter() == Delimiter::Parenthesis {
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

    pub fn is_bracket_group(&self) -> bool {
        if let Some(TokenTree::Group(group)) = &self.token {
            if group.delimiter() == Delimiter::Bracket {
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

    pub fn parse_paren_group(&mut self) -> Option<Group> {
        if let Some(TokenTree::Group(group)) = &self.token {
            if group.delimiter() == Delimiter::Parenthesis {
                let group = group.clone();
                self.step();
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

    pub fn parse_bracket_group(&mut self) -> Option<Group> {
        if let Some(TokenTree::Group(group)) = &self.token {
            if group.delimiter() == Delimiter::Bracket {
                let group = group.clone();
                self.step();
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

    pub fn parse_brace_group(&mut self) -> Option<Group> {
        if let Some(TokenTree::Group(group)) = &self.token {
            if group.delimiter() == Delimiter::Brace {
                let group = group.clone();
                self.step();
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
}
