use std::io::{self, BufRead, Write};

use fsm_macro::fsm;

fsm!(
    $(A) => A,
    A(R) => AR,
    A(A) => A,
    A => $,
    AR(A) => A,
    AR(C) => ARC,
    AR(T) => ART,
    AR => $,
    ARC => ARC,
    ART => ART
);

// mod machine {
//     use crate::MachineItem;

//     #[derive(Debug, Clone, Copy)]
//     pub enum State {
//         Locked,
//         Unlocked,
//     }
//     impl MachineItem for State {
//         const COUNT: usize = 2;
//         fn to_id(self) -> usize {
//             match self {
//                 Self::Locked => 0,
//                 Self::Unlocked => 1,
//             }
//         }
//     }

//     #[derive(Debug, Clone, Copy)]
//     pub enum Event {
//         Coin,
//         Push,
//     }
//     impl MachineItem for Event {
//         const COUNT: usize = 2;
//         fn to_id(self) -> usize {
//             match self {
//                 Self::Coin => 0,
//                 Self::Push => 1,
//             }
//         }
//     }
//
//      [Locked(Coin)]
//     const LOOKUP: [[State; Event::COUNT]; State::COUNT] = [
//         // Coin           Push
//         [State::Unlocked, State::Locked], // Locked
//         [State::Unlocked, State::Locked], // Unlocked
//     ];

//     pub fn next(state: State, event: Event) -> State {
//         LOOKUP[state.to_id()][event.to_id()]
//     }
// }

/**
 * fsm!(
 *  Locked(Coin) => Unlocked,
 *  Locked(Push) => Locked,
 *  Unlocked(Coin) => Unlocked,
 *  Locked(Push) => Locked
 * )
 *
 * // Art/Arc anywhere
 * fsm!(
 *   $(A) => A,
 *   A(R) => AR,
 *   A(A) => A,
 *   A => $,
 *   AR(A) => A,
 *   AR(C) => ARC,
 *   AR(T) => ART,
 *   AR => $,
 *   ARC => ARC,
 *   ART => ART,
 * )
 *
 * States: None, A, AR, ARC, ART
 * Events: Default, A, R, T, C
 *
 * Syntax
 * <StateName:Ident|DefaultStateName:$><(<EventName:Ident>)?> => <StateName:Ident|DefaultStateName:$>
 *
 *  */

fn main() {
    let initial_state = Machine::State::Default;
    let mut state = initial_state;
    let mut line = String::new();
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    println!("{:?}", state);
    loop {
        print!("> ");
        stdout.flush().unwrap();
        line.clear();
        stdin.read_line(&mut line).unwrap();

        match line.as_str().to_lowercase().trim() {
            "r" => state = Machine::next(state, Machine::Event::R),
            "c" => state = Machine::next(state, Machine::Event::C),
            "t" => state = Machine::next(state, Machine::Event::T),
            "a" => state = Machine::next(state, Machine::Event::A),
            "quit" => break,
            "reset" => state = initial_state,
            _ => state = Machine::next(state, Machine::Event::Default),
            // ev => eprintln!("ERROR: Unknown event {}", ev),
        }

        println!("State: {:?}", state)
    }
}
