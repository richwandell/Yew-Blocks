mod game_board;
mod game_box_container;
mod game_box;
mod reducer;

use yew::prelude::*;
use rand::seq::SliceRandom;
use crate::game_box::BoxState;
use crate::reducer::{GameState, Msg};
use crate::reducer::MessageContext;


fn get_random_color() -> String {
    let colors = ["PINK", "PURPLE", "BLUE", "YELLOW", "RED", "GREEN", "BROWN"];
    let c = colors.choose(&mut rand::thread_rng()).unwrap();
    c.to_string()
}



#[function_component]
fn App() -> Html {

    let game_state = use_reducer(|| GameState {
        new_blocks: 0,
        score: 0,
        paused: false,
        blocks: (0..8).map(|_| {
            (0..10).map(|_| {
                BoxState {
                    color: get_random_color(),
                    dying: false,
                }
            }).collect::<Vec<BoxState>>()
        }).collect::<Vec<Vec<BoxState>>>()
    });



    html! {
        <div id="game_container">
            <ContextProvider<MessageContext> context={game_state}>
                <game_board::GameBoard />
            </ContextProvider<MessageContext> >
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}