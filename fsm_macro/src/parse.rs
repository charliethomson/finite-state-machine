use proc_macro2::{Ident, TokenStream};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    token::Paren,
    Result, Token,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionItem {
    Default,
    Named(String),
}
impl Default for TransitionItem {
    fn default() -> Self {
        Self::Default
    }
}
impl TransitionItem {
    pub fn get_name(&self) -> String {
        match self {
            Self::Default => "Default".into(),
            Self::Named(name) => name.clone(),
        }
    }
}
impl<S: ToString> std::cmp::PartialEq<S> for TransitionItem {
    fn eq(&self, other: &S) -> bool {
        other.to_string() == self.get_name()
    }
}

impl Parse for TransitionItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            Ok(Self::Default)
        } else {
            Ok(Self::Named(input.parse::<Ident>()?.to_string()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub prev_state: TransitionItem,
    pub event: TransitionItem,
    pub next_state: TransitionItem,
}

impl Transition {
    pub fn is_loopback(&self) -> bool {
        self.prev_state == self.next_state && matches!(self.event, TransitionItem::Default)
    }
}

impl Parse for Transition {
    fn parse(input: ParseStream) -> Result<Self> {
        let prev_state = input.parse::<TransitionItem>()?;

        let mut event = None;

        let paren_lookahead = input.lookahead1();
        if paren_lookahead.peek(Paren) {
            let event_buffer;
            parenthesized!(event_buffer in input);
            event = if event_buffer.is_empty() {
                None
            } else {
                Some(event_buffer.parse::<TransitionItem>()?)
            };
        }

        let _ = input.parse::<Token![=>]>()?;

        let next_state = input.parse::<TransitionItem>()?;

        Ok(Self {
            event: event.unwrap_or_default(),
            next_state,
            prev_state,
        })
    }
}

pub struct TransitionParser;
impl Parser for TransitionParser {
    type Output = Vec<Transition>;

    fn parse2(self, tokens: TokenStream) -> Result<Self::Output> {
        let transitions =
            Punctuated::<Transition, Token![,]>::parse_separated_nonempty.parse2(tokens)?;

        Ok(transitions.into_iter().collect())
    }
}
