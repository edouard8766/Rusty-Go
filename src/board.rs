use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Stone {
    Empty,
    Black,
    White,
}

#[derive(Resource)]
pub struct Board {
    pub size: usize,
    pub grid: Vec<Vec<(Stone, Option<Entity>)>>,
    pub turn: Stone,
    pub black_captures: usize,
    pub white_captures: usize,
    pub consecutive_passes: usize,
    pub ko_forbidden: Option<(usize, usize)>,
    pub game_over: bool,
    pub black_territory: usize,
    pub white_territory: usize,
    pub komi: f32,
    pub overlay_spawned: bool,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            size: 19,
            grid: vec![vec![(Stone::Empty, None); 19]; 19],
            turn: Stone::Black,
            black_captures: 0,
            white_captures: 0,
            consecutive_passes: 0,
            ko_forbidden: None,
            game_over: false,
            black_territory: 0,
            white_territory: 0,
            komi: 6.5,
            overlay_spawned: false,
        }
    }
}
