use std::rc::Rc;
use log::info;
use yew::{Reducible, UseReducerHandle};
use crate::game_box::BoxState;
use crate::get_random_color;
use rand::seq::SliceRandom;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameState {
    pub blocks: Vec<Vec<BoxState>>,
    pub paused: bool,
    pub score: usize,
    pub new_blocks: usize
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Msg {
    BoxClicked {
        row: usize,
        column: usize,
    },
    BlockDied {
        row: usize,
        column: usize,
    },
    NewBlock
}


impl Reducible for GameState {
    type Action = Msg;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        info!("reducing: {:?}", action);
        match action {
            Msg::BoxClicked { row, column } => {
                let mut new_blocks = self.blocks.clone();
                new_blocks[column][row].dying = true;

                Rc::new(GameState {
                    score: self.score + 450,
                    blocks: new_blocks,
                    paused: true,
                    new_blocks: self.new_blocks
                })
            },
            Msg::BlockDied { row, column } => {
                let mut new_blocks = self.blocks.clone();
                new_blocks[column].remove(row);
                if new_blocks[column].len() == 0 {
                    new_blocks.remove(column);
                }

                Rc::new(GameState {
                    score: self.score,
                    blocks: new_blocks,
                    paused: false,
                    new_blocks: self.new_blocks
                })
            }
            Msg::NewBlock => {
                let mut new_blocks = self.blocks.clone();
                let mut cols = vec![];
                for column in 0..new_blocks.len(){
                    if new_blocks[column].len() < 10 {
                        cols.push(column);
                    }
                }
                new_blocks[*cols.choose(&mut rand::thread_rng()).unwrap() as usize].push(BoxState {
                    color: get_random_color(),
                    dying: false,
                });

                Rc::new(GameState {
                    score: self.score,
                    blocks: new_blocks,
                    paused: self.paused,
                    new_blocks: self.new_blocks + 1
                })
            }
        }
    }
}

pub type MessageContext = UseReducerHandle<GameState>;