use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, Span};
use quote::quote;

use crate::parse::{Transition, TransitionItem};

fn make_trait() -> proc_macro2::TokenStream {
    quote! {
        pub fn next(state: State, event: Event) -> State {
            LOOKUP[state.to_id()][event.to_id()]
        }
    }
}

fn make_next_impl() -> proc_macro2::TokenStream {
    quote! {
        trait MachineItem: Copy {
            const COUNT: usize;
            fn to_id(self) -> usize;
        }
    }
}

pub fn expand(transitions: Vec<Transition>) -> proc_macro2::TokenStream {
    let (states, events) = get_sets(&transitions);

    let events_typename = Ident::new("Event", Span::call_site());
    let states_typename = Ident::new("State", Span::call_site());

    let events_enum = make_enum(&events_typename, &events);
    let states_enum = make_enum(&states_typename, &states);
    let (events_impl, events_ordered_names) = make_trait_impl(&events_typename, &events);
    let (states_impl, states_ordered_names) = make_trait_impl(&states_typename, &states);
    let lookup = make_lookup(
        &transitions,
        events_ordered_names.clone(),
        states_ordered_names.clone(),
    );

    let trait_def = make_trait();
    let next_impl = make_next_impl();

    quote!(
        mod Machine {
            #trait_def
            #events_enum
            #states_enum
            #events_impl
            #states_impl
            #lookup
            #next_impl
        }
    )
}

fn get_sets(transitions: &[Transition]) -> (HashMap<String, Ident>, HashMap<String, Ident>) {
    let mut events = HashMap::<String, Ident>::new();
    let mut states = HashMap::<String, Ident>::new();
    for transition in transitions {
        let prev_name = transition.prev_state.get_name();
        let next_name = transition.next_state.get_name();
        let event_name = transition.event.get_name();

        let prev_ident = Ident::new(prev_name.as_str(), Span::call_site());
        let next_ident = Ident::new(next_name.as_str(), Span::call_site());
        let event_ident = Ident::new(event_name.as_str(), Span::call_site());

        states.insert(prev_name, prev_ident);
        states.insert(next_name, next_ident);
        events.insert(event_name, event_ident);
    }

    (states, events)
}

fn make_enum(typename: &Ident, fields: &HashMap<String, Ident>) -> proc_macro2::TokenStream {
    let idents = fields.values().cloned().collect::<Vec<_>>();
    quote!(
        #[derive(Debug, Clone, Copy)]
        pub enum #typename {
            #(#idents,)*
        }
    )
}

fn make_trait_impl(
    typename: &Ident,
    fields: &HashMap<String, Ident>,
) -> (proc_macro2::TokenStream, Vec<String>) {
    let (names, mapping) = fields
        .iter()
        .enumerate()
        .map(|(idx, (label, ident))| {
            (
                (idx, label),
                quote!(
                    Self::#ident => #idx,
                ),
            )
        })
        .fold((vec![], vec![]), |mut acc, (name, mapping)| {
            acc.0.push(name);
            acc.1.push(mapping);
            (acc.0, acc.1)
        });

    let count = names.len();

    let mut ordered_names = vec![String::new(); count];
    for (idx, name) in names {
        ordered_names.insert(idx, name.clone());
    }
    ordered_names = ordered_names
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let implementation = quote!(
        impl MachineItem for #typename {
            const COUNT: usize = #count;
            fn to_id(self) -> usize {
                match self {
                    #(#mapping)*
                }
            }
        }
    );
    (implementation, ordered_names)
}

fn make_lookup(
    transitions: &[Transition],
    event_mapping: Vec<String>,
    state_mapping: Vec<String>,
) -> proc_macro2::TokenStream {
    let transitions = transitions.to_vec();

    let has_default_state = state_mapping
        .iter()
        .find(|state| *state == "Default")
        .is_some();

    let rows = state_mapping.into_iter().map(|state| {
        let cells = event_mapping.iter().map(|event| {
            let name = transitions
                .iter()
                .find(|transition| transition.event == event && transition.prev_state == state)
                .or(transitions.iter().find(|transition| transition.prev_state == state && transition.is_loopback()))
                .cloned();

                println!("{:?}", name.clone().map(|name| name.is_loopback()));
            let name = match name {
                None if has_default_state => TransitionItem::Default,
                Some(name) => name.next_state,
                _ => panic!("No default state configured, is this case expected? {{ event: {event}, prev_state: {state} }}"),
            }.get_name();
            println!("event: {event}, prev_state: {state}, next_state: {name}");
            let ident = Ident::new(&name, Span::call_site());
            quote!(State::#ident)
        });
        quote!(
            [ #(#cells,)* ], // #state
        )
    });

    quote!(
        const LOOKUP: [[State; Event::COUNT]; State::COUNT] = [
            #(#rows)*
        ];
    )
}
