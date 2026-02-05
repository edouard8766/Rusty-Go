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
    pub grid: Vec<Vec<Stone>>,
    pub turn: Stone, 
}

impl Default for Board {
    fn default() -> Self {
        Self {
            size: 19,
            grid: vec![vec![Stone::Empty; 19]; 19],
            turn: Stone::Black,
        }
    }
}