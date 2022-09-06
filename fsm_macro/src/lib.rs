use expand::expand;
use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::parse::TransitionParser;
mod expand;
mod parse;

#[proc_macro]
pub fn fsm(item: TokenStream) -> TokenStream {
    let transitions = parse_macro_input!(item with TransitionParser);

    expand(transitions).into()
}
