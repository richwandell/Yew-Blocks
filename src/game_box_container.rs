use yew::prelude::*;
use super::game_box;
use rand::seq::SliceRandom;

use crate::{MessageContext};
use crate::reducer::Msg;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub column: usize,
}



#[function_component]
pub(crate) fn GameBoxContainer(props: &Props) -> Html {
    let game_state = use_context::<MessageContext>().expect("no ctx found");

    let mut row = vec![];
    for y in 0..game_state.blocks[props.column].len() {
        row.push(html! {
                <game_box::GameBox
                    row={y}
                    column={props.column}
                    color={game_state.blocks[props.column][y].color.clone()} />
            })
    }

    html! {
        <div class="game_box_container">{row.into_iter().collect::<Html>()}</div>
    }
}
