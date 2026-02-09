use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::board::{Board, Stone};

// pixels entre chaque intersection
const CELL_SIZE: f32 = 40.0;
const LINE_THICKNESS: f32 = 2.0;

pub struct GoPlugin;

impl Plugin for GoPlugin {
    fn build(&self, app: &mut App) {
        //runs when we launch
        app.add_systems(Startup, (setup_camera, spawn_board_visuals))
        //runs every frame
           .add_systems(Update, handle_input);
    }
}

fn setup_camera(mut commands: Commands) {
    // Spawn the camera
    // Bevy automatically adds Transform and Projection
    commands.spawn(Camera2d);
}

fn spawn_board_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<Board>, //pour avoir la board size
) {
    let size = board.size as f32;
    //pour avoir la length des gridlines
    let board_length = (size - 1.0) * CELL_SIZE;
    let offset = board_length / 2.0; // center the board
    //faut mettre un material noir pour les lignes
    let black_material = materials.add(ColorMaterial::from(Color::BLACK));
    let text_color = TextColor::from(Color::BLACK);
    let text_font = TextFont {
        font_size: 16.0,
        ..default()
    };

    let letters = vec!["A", "B", "C", "D", "E", "F", "G", "H", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T"]; // we skip I because it looks like 1

    for i in 0..board.size {
        let pos = i as f32 * CELL_SIZE - offset;
        let v_line_mesh = meshes.add(Rectangle::new(LINE_THICKNESS, board_length));
    
        commands.spawn((
            // The Shape (Geometry)
            Mesh2d(v_line_mesh),
            // The Color
            MeshMaterial2d(black_material.clone()),
            // The Position:
            // X = pos (moving left to right)
            // Y = 0.0 (centered vertically)
            // Z = 0.0 (The floor layer)
            Transform::from_xyz(pos, 0.0, 0.0),
        ));
        let h_line_mesh = meshes.add(Rectangle::new(board_length, LINE_THICKNESS));
        
        commands.spawn((
            Mesh2d(h_line_mesh),
            MeshMaterial2d(black_material.clone()),
            // Position:
            // X = 0.0 (centered horizontally)
            // Y = pos (moving bottom to top)
            // Z = 0.0 (Same layer as vertical lines)
            Transform::from_xyz(0.0, pos, 0.0),
        ));
        // grid labels
        if i < letters.len() {
            commands.spawn((
                Text2d::new(letters[i]),
                text_font.clone(),
                text_color,
                Transform::from_xyz(pos, -offset -25.0, 0.0),
                Anchor::CENTER,
            ));
        }
        let number_label = (i + 1).to_string();
        commands.spawn((
            Text2d::new(number_label),
            text_font.clone(),
            text_color,
            Transform::from_xyz(-offset - 25.0, pos, 0.0),
            Anchor::CENTER,
        ));
    }
        //star points
    let star_indices = [3, 9, 15];
    let star_mesh = meshes.add(Circle::new(4.0)); //Small dot

    for &y_idx in &star_indices {
        for &x_idx in &star_indices {
            let x_pos = x_idx as f32 * CELL_SIZE - offset;
            let y_pos = y_idx as f32 * CELL_SIZE - offset;

            commands.spawn((
                Mesh2d(star_mesh.clone()),
                MeshMaterial2d(black_material.clone()),
                // IMPORTANT: Z = 0.1
                // 0.0 = Lines
                // 0.1 = Star Points (so they sit over lines)
                // 1.0 = Stones (so they cover the star points)
                Transform::from_xyz(x_pos, y_pos, 0.1), 
            ));
        }
    }
}
fn handle_input(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>, //to convert cursor pixels to coords
    mut board: ResMut<Board>,
    mut commands: Commands,// to spawn stones
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(screen_pos) = window.cursor_position() {
            let (cam, cam_transform) = *camera;

            // Convert Screen -> World
            if let Ok(world_pos) = cam.viewport_to_world_2d(cam_transform, screen_pos) {
                // we need the offset to center it again
                let offset = (board.size as f32 - 1.0) * CELL_SIZE / 2.0;

                //Snap to nearest intersection
                let x_index = ((world_pos.x + offset) / CELL_SIZE).round();
                let y_index = ((world_pos.y + offset) / CELL_SIZE).round();

                //calculate snapped world position
                let snap_pos = Vec2::new(
                    x_index * CELL_SIZE - offset,
                    y_index * CELL_SIZE - offset
                );

                //Distance check, must click near intersection or else we ignore
                if world_pos.distance(snap_pos) > CELL_SIZE * 0.4 {
                    return; 
                }

                //float to integers for array index 
                let size = board.size as i32;
                let x = x_index as i32;
                let y = y_index as i32;
                //inside bounds
                if x >= 0 && x < size && y >= 0 && y < size {
                    let r_x = x as usize;
                    let r_y = y as usize;

                    if board.grid[r_y][r_x] == Stone::Empty {
                        // Update Data
                        board.grid[r_y][r_x] = board.turn;
                        //color depending on turn, black starts
                        let color = match board.turn {
                            Stone::Black => Color::BLACK,
                            Stone::White => Color::WHITE,
                            _ => Color::NONE,
                        };

                        //spawn stone visual                    
                        commands.spawn((
                            //circle mesh with a radius of 45% of cell size
                            Mesh2d(meshes.add(Circle::new(CELL_SIZE * 0.45))),
                            MeshMaterial2d(materials.add(ColorMaterial::from(color))),
                            //places at snapped position, z=1.0 to be above the grid lines
                            Transform::from_translation(snap_pos.extend(1.0)), 
                        ));

                        //White stone outline
                        if board.turn == Stone::White {
                            commands.spawn((
                                Mesh2d(meshes.add(Circle::new(CELL_SIZE * 0.48))),
                                MeshMaterial2d(materials.add(ColorMaterial::from(Color::BLACK))),
                                Transform::from_translation(snap_pos.extend(0.5)),
                            ));
                        }

                        // Switch Turn
                        board.turn = match board.turn {
                            Stone::Black => Stone::White,
                            _ => Stone::Black,
                        };
                    }
                }
            }
        }
    }
}