
use yew::prelude::*;
use crate::{MessageContext};
use crate::reducer::Msg;
use super::game_box_container;

use rand::seq::SliceRandom;

fn get_random_time() -> u32 {
    let colors = [2_000, 3_000, 3_500, 4_000, 4_500, 5_000];
    *colors.choose(&mut rand::thread_rng()).unwrap() as u32
}

#[function_component]
pub(crate) fn GameBoard() -> Html {
    {
        let game_state = use_context::<MessageContext>().expect("no ctx found");
        use_effect_with_deps(
            {
                let game_state = game_state.clone();

                move |_| {
                    let timeout = gloo_timers::callback::Timeout::new(get_random_time(), move || {
                        game_state.dispatch(Msg::NewBlock);
                    });
                    move || drop(timeout)
                }
            },
            (game_state.new_blocks, game_state.paused), // Only create the interval once per your component existence
        );
    }


    let game_state = use_context::<MessageContext>().expect("no ctx found");
    let mut rows = vec![];

    for x in 0..game_state.blocks.len() {
        rows.push(html! {
            <game_box_container::GameBoxContainer column={x} />
        });
    }

    html! {
        <div id="game_board">
            {rows.into_iter().collect::<Html>()}
        </div>
    }
}
