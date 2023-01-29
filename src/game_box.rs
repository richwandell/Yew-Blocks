use yew::prelude::*;
use crate::{MessageContext};
use crate::reducer::Msg;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub color: String,
    pub row: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoxState {
    pub color: String,
    pub dying: bool,
}

#[function_component]
pub(crate) fn GameBox(props: &Props) -> Html {
    let mut class_names = "game_box";

    let game_state = use_context::<MessageContext>().expect("no ctx found");
    if game_state.blocks[props.column][props.row].dying {
        class_names = "game_box magictime holeOut";
        let other_row = props.row.clone();
        let column = props.column.clone();
        let timeout = gloo_timers::callback::Timeout::new(1_000, move || {
            game_state.dispatch(Msg::BlockDied {
                row: other_row,
                column,
            });
        });
        timeout.forget();
    }


    let onclick = {
        let game_state = use_context::<MessageContext>().expect("no ctx found");
        match game_state.paused {
            true => Callback::noop(),
            false => {
                let other_row = props.row.clone();
                let column = props.column.clone();
                Callback::from(move |_| {
                    game_state.dispatch(Msg::BoxClicked {
                        row: other_row,
                        column,
                    });
                })
            }
        }
    };

    html! {
        <div class={class_names} {onclick} style={format!("background-color: {}", props.color)}></div>
    }
}