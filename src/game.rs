use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::board::{Board, Stone};

// pixels entre chaque intersection
const CELL_SIZE: f32 = 40.0;
const LINE_THICKNESS: f32 = 2.0;

#[derive(Component)]
struct BlackCapText;

#[derive(Component)]
struct WhiteCapText;

#[derive(Component)]
struct BlackTurnText;

#[derive(Component)]
struct WhiteTurnText;

pub struct GoPlugin;

impl Plugin for GoPlugin {
    fn build(&self, app: &mut App) {
        //runs when we launch
        app.add_systems(Startup, (setup_camera, spawn_board_visuals, setup_ui))
        //runs every frame
           .add_systems(Update, (handle_input, update_scoreboard));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 1.3,
            ..OrthographicProjection::default_2d()
        })));
}

fn spawn_board_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<Board>,
) {
    let size = board.size as f32;
    let board_length = (size - 1.0) * CELL_SIZE;
    let offset = board_length / 2.0;
    let black_material = materials.add(ColorMaterial::from(Color::BLACK));
    let text_color = TextColor::from(Color::BLACK);
    let text_font = TextFont {
        font_size: 16.0,
        ..default()
    };

    let letters = vec!["A", "B", "C", "D", "E", "F", "G", "H", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T"];

    for i in 0..board.size {
        let pos = i as f32 * CELL_SIZE - offset;
        let v_line_mesh = meshes.add(Rectangle::new(LINE_THICKNESS, board_length));

        commands.spawn((
            Mesh2d(v_line_mesh),
            MeshMaterial2d(black_material.clone()),
            Transform::from_xyz(pos, 0.0, 0.0),
        ));
        let h_line_mesh = meshes.add(Rectangle::new(board_length, LINE_THICKNESS));

        commands.spawn((
            Mesh2d(h_line_mesh),
            MeshMaterial2d(black_material.clone()),
            Transform::from_xyz(0.0, pos, 0.0),
        ));
        // grid labels
        if i < letters.len() {
            commands.spawn((
                Text2d::new(letters[i]),
                text_font.clone(),
                text_color,
                Transform::from_xyz(pos, -offset - 25.0, 0.0),
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

    // star points
    let star_indices = [3, 9, 15];
    let star_mesh = meshes.add(Circle::new(4.0));

    for &y_idx in &star_indices {
        for &x_idx in &star_indices {
            let x_pos = x_idx as f32 * CELL_SIZE - offset;
            let y_pos = y_idx as f32 * CELL_SIZE - offset;

            commands.spawn((
                Mesh2d(star_mesh.clone()),
                MeshMaterial2d(black_material.clone()),
                Transform::from_xyz(x_pos, y_pos, 0.1),
            ));
        }
    }
}

fn handle_input(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(screen_pos) = window.cursor_position() {
            let (cam, cam_transform) = *camera;

            if let Ok(world_pos) = cam.viewport_to_world_2d(cam_transform, screen_pos) {
                let offset = (board.size as f32 - 1.0) * CELL_SIZE / 2.0;

                let x_index = ((world_pos.x + offset) / CELL_SIZE).round();
                let y_index = ((world_pos.y + offset) / CELL_SIZE).round();

                if x_index < 0.0 || y_index < 0.0 {
                    return;
                }

                let snap_pos = Vec2::new(
                    x_index * CELL_SIZE - offset,
                    y_index * CELL_SIZE - offset
                );

                if world_pos.distance(snap_pos) > CELL_SIZE * 0.4 {
                    return;
                }

                let size = board.size as i32;
                let x = x_index as i32;
                let y = y_index as i32;

                if x >= 0 && x < size && y >= 0 && y < size {
                    let r_x = x as usize;
                    let r_y = y as usize;

                    if board.grid[r_y][r_x].0 == Stone::Empty {
                        let move_valid = attempt_move(
                            &mut board, &mut commands, &mut meshes, &mut materials, r_x, r_y, snap_pos
                        );
                        if move_valid {
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
}

fn setup_ui(mut commands: Commands) {
    // Black player panel (top-left)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(4.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.08, 0.08, 0.08, 0.88)),
    )).with_children(|p| {
        p.spawn((
            Text::new("● BLACK"),
            TextFont { font_size: 18.0, ..default() },
            TextColor::from(Color::WHITE),
        ));
        p.spawn((
            Text::new("Captures: 0"),
            TextFont { font_size: 14.0, ..default() },
            TextColor::from(Color::srgb(0.75, 0.75, 0.75)),
            BlackCapText,
        ));
        p.spawn((
            Text::new("▶ YOUR TURN"),
            TextFont { font_size: 13.0, ..default() },
            TextColor::from(Color::srgb(0.2, 1.0, 0.2)),
            BlackTurnText,
        ));
    });

    // White player panel (top-right)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(4.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.95, 0.95, 0.95, 0.88)),
    )).with_children(|p| {
        p.spawn((
            Text::new("○ WHITE"),
            TextFont { font_size: 18.0, ..default() },
            TextColor::from(Color::srgb(0.1, 0.1, 0.1)),
        ));
        p.spawn((
            Text::new("Captures: 0"),
            TextFont { font_size: 14.0, ..default() },
            TextColor::from(Color::srgb(0.35, 0.35, 0.35)),
            WhiteCapText,
        ));
        p.spawn((
            Text::new("  waiting"),
            TextFont { font_size: 13.0, ..default() },
            TextColor::from(Color::srgb(0.6, 0.6, 0.6)),
            WhiteTurnText,
        ));
    });
}

fn update_scoreboard(
    board: Res<Board>,
    mut q_bc: Query<&mut Text, (With<BlackCapText>, Without<WhiteCapText>, Without<BlackTurnText>, Without<WhiteTurnText>)>,
    mut q_wc: Query<&mut Text, (With<WhiteCapText>, Without<BlackCapText>, Without<BlackTurnText>, Without<WhiteTurnText>)>,
    mut q_bt: Query<(&mut Text, &mut TextColor), (With<BlackTurnText>, Without<BlackCapText>, Without<WhiteCapText>, Without<WhiteTurnText>)>,
    mut q_wt: Query<(&mut Text, &mut TextColor), (With<WhiteTurnText>, Without<BlackCapText>, Without<WhiteCapText>, Without<BlackTurnText>)>,
) {
    if !board.is_changed() { return; }

    let black_turn = board.turn == Stone::Black;

    if let Ok(mut t) = q_bc.single_mut() {
        t.0 = format!("Captures: {}", board.black_captures);
    }
    if let Ok(mut t) = q_wc.single_mut() {
        t.0 = format!("Captures: {}", board.white_captures);
    }
    if let Ok((mut t, mut c)) = q_bt.single_mut() {
        if black_turn {
            t.0 = "▶ YOUR TURN".into();
            *c = TextColor::from(Color::srgb(0.2, 1.0, 0.2));
        } else {
            t.0 = "  waiting".into();
            *c = TextColor::from(Color::srgb(0.5, 0.5, 0.5));
        }
    }
    if let Ok((mut t, mut c)) = q_wt.single_mut() {
        if !black_turn {
            t.0 = "▶ YOUR TURN".into();
            *c = TextColor::from(Color::srgb(0.1, 0.8, 0.1));
        } else {
            t.0 = "  waiting".into();
            *c = TextColor::from(Color::srgb(0.6, 0.6, 0.6));
        }
    }
}

fn attempt_move(
    board: &mut Board,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: usize,
    y: usize,
    pos: Vec2,
) -> bool {
    let color = board.turn;
    let opponent = match color {
        Stone::Black => Stone::White,
        _ => Stone::Black,
    };
    let visual_color = if color == Stone::Black { Color::BLACK } else { Color::WHITE };
    let new_entity = commands.spawn((
        Mesh2d(meshes.add(Circle::new(CELL_SIZE * 0.45))),
        MeshMaterial2d(materials.add(ColorMaterial::from(visual_color))),
        Transform::from_translation(pos.extend(1.0)),
    )).id();

    // White outline — transform is LOCAL to the parent stone, so (0,0) keeps it centered.
    // Z = -0.5 places it behind the white stone (stone Z=1.0, outline world Z=0.5).
    // The outline circle is larger (0.48 vs 0.45) so the ring peeks out around the edge.
    if color == Stone::White {
        let outline = commands.spawn((
            Mesh2d(meshes.add(Circle::new(CELL_SIZE * 0.48))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::BLACK))),
            Transform::from_translation(Vec3::new(0.0, 0.0, -0.5)),
        )).id();
        commands.entity(new_entity).add_child(outline);
    }
    board.grid[y][x] = (color, Some(new_entity));

    // capture logic
    let mut captured_total = 0;
    let directions = [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)];
    for (dx, dy) in directions {
        let ny = (y as i32 + dy) as usize;
        let nx = (x as i32 + dx) as usize;
        if ny < board.size && nx < board.size {
            if board.grid[ny][nx].0 == opponent {
                if !has_liberties(board, nx, ny) {
                    captured_total += remove_group(board, commands, nx, ny);
                }
            }
        }
    }
    if color == Stone::Black {
        board.black_captures += captured_total;
    } else {
        board.white_captures += captured_total;
    }

    // suicide check
    if captured_total == 0 && !has_liberties(board, x, y) {
        commands.entity(new_entity).despawn();
        board.grid[y][x] = (Stone::Empty, None);
        return false;
    }

    true
}

fn has_liberties(board: &Board, start_x: usize, start_y: usize) -> bool {
    let color = board.grid[start_y][start_x].0;
    let mut stack = vec![(start_x, start_y)];
    let mut visited = std::collections::HashSet::new();

    while let Some((x, y)) = stack.pop() {
        if !visited.insert((x, y)) { continue; }

        let directions = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in directions {
            let ny = (y as i32 + dy) as usize;
            let nx = (x as i32 + dx) as usize;

            if ny < board.size && nx < board.size {
                let (stone, _) = board.grid[ny][nx];
                if stone == Stone::Empty { return true; }
                if stone == color && !visited.contains(&(nx, ny)) {
                    stack.push((nx, ny));
                }
            }
        }
    }
    false
}

fn remove_group(board: &mut Board, commands: &mut Commands, start_x: usize, start_y: usize) -> usize {
    let color = board.grid[start_y][start_x].0;
    let mut stack = vec![(start_x, start_y)];
    let mut visited = std::collections::HashSet::new();
    let mut to_remove = Vec::new();

    while let Some((x, y)) = stack.pop() {
        if !visited.insert((x, y)) { continue; }
        to_remove.push((x, y));

        let directions = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in directions {
            let ny = (y as i32 + dy) as usize;
            let nx = (x as i32 + dx) as usize;
            if ny < board.size && nx < board.size {
                if board.grid[ny][nx].0 == color && !visited.contains(&(nx, ny)) {
                    stack.push((nx, ny));
                }
            }
        }
    }

    let count = to_remove.len();
    for (rx, ry) in to_remove {
        if let Some(entity) = board.grid[ry][rx].1 {
            // despawn removes the stone and its outline child (white stones)
            commands.entity(entity).despawn();
        }
        board.grid[ry][rx] = (Stone::Empty, None);
    }
    count
}
