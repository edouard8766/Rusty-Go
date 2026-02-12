use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
//on derive clone/copy pour mettre les stones faciles
//partialeq pour les comparer
pub enum Stone {
    Empty,
    Black,
    White,
}
//on derive resource pour que bevy le store
#[derive(Resource)] 
pub struct Board {
    pub size: usize,
    pub grid: Vec<Vec<(Stone, Option<Entity>)>>, //faut entity pour pouvoir delete la pierre quand captured
    pub turn: Stone,
    pub black_captures: usize,
    pub white_captures: usize,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            size: 19,
            grid: vec![vec![(Stone::Empty, None); 19]; 19],
            turn: Stone::Black,
            black_captures: 0,
            white_captures: 0,
        }
    }
}