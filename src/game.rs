use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::board::{Board, Stone};

const CELL_SIZE: f32 = 40.0;
const LINE_THICKNESS: f32 = 2.0;

#[derive(Component)] struct BlackCapText;
#[derive(Component)] struct WhiteCapText;
#[derive(Component)] struct BlackTurnText;
#[derive(Component)] struct WhiteTurnText;
#[derive(Component)] struct GameOverScreen;
#[derive(Component)] struct ShadowStone;
#[derive(Component)] struct LibertyDot;

#[derive(Resource)]
struct HoverState {
    shadow_entity: Entity,
    shadow_material: Handle<ColorMaterial>,
    liberty_entities: Vec<Entity>,
}

pub struct GoPlugin;

impl Plugin for GoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, spawn_board_visuals, setup_ui, setup_hints_ui, setup_hover_visuals))
           .add_systems(Update, (handle_mouse_input, handle_keyboard_input, update_scoreboard, on_game_over, update_hover));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 1.3,
            ..OrthographicProjection::default_2d()
        }),
    ));
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
    let text_font = TextFont { font_size: 16.0, ..default() };
    let letters = ["A","B","C","D","E","F","G","H","J","K","L","M","N","O","P","Q","R","S","T"];

    for i in 0..board.size {
        let pos = i as f32 * CELL_SIZE - offset;

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(LINE_THICKNESS, board_length))),
            MeshMaterial2d(black_material.clone()),
            Transform::from_xyz(pos, 0.0, 0.0),
        ));
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(board_length, LINE_THICKNESS))),
            MeshMaterial2d(black_material.clone()),
            Transform::from_xyz(0.0, pos, 0.0),
        ));

        if i < letters.len() {
            commands.spawn((
                Text2d::new(letters[i]),
                text_font.clone(),
                text_color,
                Transform::from_xyz(pos, -offset - 25.0, 0.0),
                Anchor::CENTER,
            ));
        }
        commands.spawn((
            Text2d::new((i + 1).to_string()),
            text_font.clone(),
            text_color,
            Transform::from_xyz(-offset - 25.0, pos, 0.0),
            Anchor::CENTER,
        ));
    }

    let star_indices = [3, 9, 15];
    let star_mesh = meshes.add(Circle::new(4.0));
    for &y_idx in &star_indices {
        for &x_idx in &star_indices {
            commands.spawn((
                Mesh2d(star_mesh.clone()),
                MeshMaterial2d(black_material.clone()),
                Transform::from_xyz(x_idx as f32 * CELL_SIZE - offset, y_idx as f32 * CELL_SIZE - offset, 0.1),
            ));
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
        p.spawn((Text::new("● BLACK"), TextFont { font_size: 18.0, ..default() }, TextColor::from(Color::WHITE)));
        p.spawn((Text::new("Captures: 0"), TextFont { font_size: 14.0, ..default() }, TextColor::from(Color::srgb(0.75, 0.75, 0.75)), BlackCapText));
        p.spawn((Text::new("▶ YOUR TURN"), TextFont { font_size: 13.0, ..default() }, TextColor::from(Color::srgb(0.2, 1.0, 0.2)), BlackTurnText));
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
        p.spawn((Text::new("○ WHITE"), TextFont { font_size: 18.0, ..default() }, TextColor::from(Color::srgb(0.1, 0.1, 0.1))));
        p.spawn((Text::new("Captures: 0"), TextFont { font_size: 14.0, ..default() }, TextColor::from(Color::srgb(0.35, 0.35, 0.35)), WhiteCapText));
        p.spawn((Text::new("  waiting"), TextFont { font_size: 13.0, ..default() }, TextColor::from(Color::srgb(0.6, 0.6, 0.6)), WhiteTurnText));
    });
}

fn setup_hints_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    )).with_children(|p| {
        p.spawn((
            Text::new("P: Pass  |  R: New Game"),
            TextFont { font_size: 15.0, ..default() },
            TextColor::from(Color::srgba(0.08, 0.08, 0.08, 0.75)),
        ));
    });
}

fn handle_mouse_input(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if board.game_over { return; }

    if mouse.just_pressed(MouseButton::Left) {
        if let Some(screen_pos) = window.cursor_position() {
            let (cam, cam_transform) = *camera;
            if let Ok(world_pos) = cam.viewport_to_world_2d(cam_transform, screen_pos) {
                let offset = (board.size as f32 - 1.0) * CELL_SIZE / 2.0;
                let x_index = ((world_pos.x + offset) / CELL_SIZE).round();
                let y_index = ((world_pos.y + offset) / CELL_SIZE).round();

                if x_index < 0.0 || y_index < 0.0 { return; }

                let snap_pos = Vec2::new(x_index * CELL_SIZE - offset, y_index * CELL_SIZE - offset);
                if world_pos.distance(snap_pos) > CELL_SIZE * 0.4 { return; }

                let x = x_index as i32;
                let y = y_index as i32;
                let size = board.size as i32;

                if x >= 0 && x < size && y >= 0 && y < size {
                    let r_x = x as usize;
                    let r_y = y as usize;

                    if board.grid[r_y][r_x].0 == Stone::Empty {
                        // Ko rule: forbid playing at the ko point
                        if board.ko_forbidden == Some((r_x, r_y)) { return; }

                        let move_valid = attempt_move(&mut board, &mut commands, &mut meshes, &mut materials, r_x, r_y, snap_pos);
                        if move_valid {
                            board.consecutive_passes = 0;
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

fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    game_over_query: Query<Entity, With<GameOverScreen>>,
) {
    // Pass
    if keyboard.just_pressed(KeyCode::KeyP) && !board.game_over {
        board.ko_forbidden = None;
        board.consecutive_passes += 1;
        board.turn = match board.turn {
            Stone::Black => Stone::White,
            _ => Stone::Black,
        };
        if board.consecutive_passes >= 2 {
            board.game_over = true;
        }
        return;
    }

    // New game
    if keyboard.just_pressed(KeyCode::KeyR) && board.game_over {
        for row in &board.grid {
            for (_, entity) in row {
                if let Some(e) = entity {
                    commands.entity(*e).despawn();
                }
            }
        }
        for entity in &game_over_query {
            commands.entity(entity).despawn();
        }
        *board = Board::default();
    }
}

fn on_game_over(
    mut board: ResMut<Board>,
    mut commands: Commands,
) {
    if !board.game_over || board.overlay_spawned { return; }
    board.overlay_spawned = true;

    {
        let (bt, wt) = calculate_territory(&*board);
        board.black_territory = bt;
        board.white_territory = wt;

        let black_total = bt + board.black_captures;
        let white_total = wt as f32 + board.white_captures as f32 + board.komi;

        let result_str = if black_total as f32 > white_total {
            format!("Black wins by {:.1} points", black_total as f32 - white_total)
        } else {
            format!("White wins by {:.1} points", white_total - black_total as f32)
        };

        let bc = board.black_captures;
        let wc = board.white_captures;
        let komi = board.komi;

        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.80)),
            GameOverScreen,
        )).with_children(|p| {
            p.spawn((
                Text::new("— GAME OVER —"),
                TextFont { font_size: 50.0, ..default() },
                TextColor::from(Color::WHITE),
            ));
            p.spawn((
                Text::new(format!("● Black:  Territory {}  +  Captures {}  =  {}", bt, bc, black_total)),
                TextFont { font_size: 20.0, ..default() },
                TextColor::from(Color::srgb(0.88, 0.88, 0.88)),
            ));
            p.spawn((
                Text::new(format!("○ White:  Territory {}  +  Captures {}  +  Komi {:.1}  =  {:.1}", wt, wc, komi, white_total)),
                TextFont { font_size: 20.0, ..default() },
                TextColor::from(Color::srgb(0.88, 0.88, 0.88)),
            ));
            p.spawn((
                Text::new(result_str),
                TextFont { font_size: 34.0, ..default() },
                TextColor::from(Color::srgb(1.0, 0.85, 0.2)),
            ));
            p.spawn((
                Text::new("Press R to play again"),
                TextFont { font_size: 16.0, ..default() },
                TextColor::from(Color::srgb(0.55, 0.55, 0.55)),
            ));
        });
    }
}

/// Flood-fill territory counting (Japanese rules).
/// Empty regions surrounded only by Black = Black territory, only by White = White territory.
fn calculate_territory(board: &Board) -> (usize, usize) {
    let size = board.size;
    let mut visited = vec![vec![false; size]; size];
    let mut black_territory = 0usize;
    let mut white_territory = 0usize;

    for sy in 0..size {
        for sx in 0..size {
            if visited[sy][sx] || board.grid[sy][sx].0 != Stone::Empty {
                continue;
            }

            let mut stack = vec![(sx, sy)];
            let mut region_count = 0usize;
            let mut bordering_black = false;
            let mut bordering_white = false;

            while let Some((x, y)) = stack.pop() {
                if visited[y][x] { continue; }
                visited[y][x] = true;
                region_count += 1;

                for (dx, dy) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < size as i32 && ny >= 0 && ny < size as i32 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        match board.grid[ny][nx].0 {
                            Stone::Empty  => if !visited[ny][nx] { stack.push((nx, ny)); }
                            Stone::Black  => bordering_black = true,
                            Stone::White  => bordering_white = true,
                        }
                    }
                }
            }

            match (bordering_black, bordering_white) {
                (true, false) => black_territory += region_count,
                (false, true) => white_territory += region_count,
                _ => {} // contested or edge-of-board — no points
            }
        }
    }

    (black_territory, white_territory)
}

fn update_scoreboard(
    board: Res<Board>,
    mut q_bc: Query<&mut Text, (With<BlackCapText>, Without<WhiteCapText>, Without<BlackTurnText>, Without<WhiteTurnText>)>,
    mut q_wc: Query<&mut Text, (With<WhiteCapText>, Without<BlackCapText>, Without<BlackTurnText>, Without<WhiteTurnText>)>,
    mut q_bt: Query<(&mut Text, &mut TextColor), (With<BlackTurnText>, Without<BlackCapText>, Without<WhiteCapText>, Without<WhiteTurnText>)>,
    mut q_wt: Query<(&mut Text, &mut TextColor), (With<WhiteTurnText>, Without<BlackCapText>, Without<WhiteCapText>, Without<BlackTurnText>)>,
) {
    if !board.is_changed() { return; }

    if let Ok(mut t) = q_bc.single_mut() { t.0 = format!("Captures: {}", board.black_captures); }
    if let Ok(mut t) = q_wc.single_mut() { t.0 = format!("Captures: {}", board.white_captures); }

    if board.game_over {
        if let Ok((mut t, mut c)) = q_bt.single_mut() { t.0 = "  —".into(); *c = TextColor::from(Color::srgb(0.5, 0.5, 0.5)); }
        if let Ok((mut t, mut c)) = q_wt.single_mut() { t.0 = "  —".into(); *c = TextColor::from(Color::srgb(0.5, 0.5, 0.5)); }
        return;
    }

    let black_turn = board.turn == Stone::Black;
    if let Ok((mut t, mut c)) = q_bt.single_mut() {
        if black_turn { t.0 = "▶ YOUR TURN".into(); *c = TextColor::from(Color::srgb(0.2, 1.0, 0.2)); }
        else          { t.0 = "  waiting".into();   *c = TextColor::from(Color::srgb(0.5, 0.5, 0.5)); }
    }
    if let Ok((mut t, mut c)) = q_wt.single_mut() {
        if !black_turn { t.0 = "▶ YOUR TURN".into(); *c = TextColor::from(Color::srgb(0.1, 0.8, 0.1)); }
        else           { t.0 = "  waiting".into();   *c = TextColor::from(Color::srgb(0.6, 0.6, 0.6)); }
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
    let opponent = match color { Stone::Black => Stone::White, _ => Stone::Black };
    let visual_color = if color == Stone::Black { Color::BLACK } else { Color::WHITE };

    let new_entity = commands.spawn((
        Mesh2d(meshes.add(Circle::new(CELL_SIZE * 0.45))),
        MeshMaterial2d(materials.add(ColorMaterial::from(visual_color))),
        Transform::from_translation(pos.extend(1.0)),
    )).id();

    if color == Stone::White {
        let outline = commands.spawn((
            Mesh2d(meshes.add(Circle::new(CELL_SIZE * 0.48))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::BLACK))),
            Transform::from_translation(Vec3::new(0.0, 0.0, -0.5)),
        )).id();
        commands.entity(new_entity).add_child(outline);
    }
    board.grid[y][x] = (color, Some(new_entity));

    // Capture opponent groups with no liberties
    let mut all_captured: Vec<(usize, usize)> = Vec::new();
    for (dx, dy) in [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)] {
        let ny = (y as i32 + dy) as usize;
        let nx = (x as i32 + dx) as usize;
        if ny < board.size && nx < board.size && board.grid[ny][nx].0 == opponent {
            if !has_liberties(board, nx, ny) {
                let captured = remove_group(board, commands, nx, ny);
                all_captured.extend(captured);
            }
        }
    }
    let captured_total = all_captured.len();

    if color == Stone::Black { board.black_captures += captured_total; }
    else                     { board.white_captures += captured_total; }

    // Ko rule: if exactly 1 stone was captured, that point is forbidden next turn
    board.ko_forbidden = if captured_total == 1 { Some(all_captured[0]) } else { None };

    // Suicide check
    if captured_total == 0 && !has_liberties(board, x, y) {
        commands.entity(new_entity).despawn();
        board.grid[y][x] = (Stone::Empty, None);
        board.ko_forbidden = None;
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
        for (dx, dy) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
            let ny = (y as i32 + dy) as usize;
            let nx = (x as i32 + dx) as usize;
            if ny < board.size && nx < board.size {
                let (stone, _) = board.grid[ny][nx];
                if stone == Stone::Empty { return true; }
                if stone == color && !visited.contains(&(nx, ny)) { stack.push((nx, ny)); }
            }
        }
    }
    false
}

fn remove_group(board: &mut Board, commands: &mut Commands, start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
    let color = board.grid[start_y][start_x].0;
    let mut stack = vec![(start_x, start_y)];
    let mut visited = std::collections::HashSet::new();
    let mut to_remove = Vec::new();

    while let Some((x, y)) = stack.pop() {
        if !visited.insert((x, y)) { continue; }
        to_remove.push((x, y));
        for (dx, dy) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
            let ny = (y as i32 + dy) as usize;
            let nx = (x as i32 + dx) as usize;
            if ny < board.size && nx < board.size && board.grid[ny][nx].0 == color && !visited.contains(&(nx, ny)) {
                stack.push((nx, ny));
            }
        }
    }

    for &(rx, ry) in &to_remove {
        if let Some(entity) = board.grid[ry][rx].1 {
            commands.entity(entity).despawn();
        }
        board.grid[ry][rx] = (Stone::Empty, None);
    }
    to_remove
}

fn setup_hover_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Shadow stone: starts transparent, color updated each frame based on current turn
    let shadow_material = materials.add(ColorMaterial::from(Color::srgba(0.0, 0.0, 0.0, 0.0)));
    let shadow_entity = commands.spawn((
        Mesh2d(meshes.add(Circle::new(CELL_SIZE * 0.45))),
        MeshMaterial2d(shadow_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.8),
        Visibility::Hidden,
        ShadowStone,
    )).id();

    // Liberty dots: small green circles, shared material, toggled individually
    let liberty_material = materials.add(ColorMaterial::from(Color::srgba(0.15, 0.70, 0.30, 0.85)));
    let liberty_entities = (0..4).map(|_| {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(CELL_SIZE * 0.13))),
            MeshMaterial2d(liberty_material.clone()),
            Transform::from_xyz(0.0, 0.0, 0.75),
            Visibility::Hidden,
            LibertyDot,
        )).id()
    }).collect();

    commands.insert_resource(HoverState { shadow_entity, shadow_material, liberty_entities });
}

fn update_hover(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    board: Res<Board>,
    hover_state: Res<HoverState>,
    mut transforms: Query<&mut Transform>,
    mut visibilities: Query<&mut Visibility>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let hide = |hover_state: &HoverState, visibilities: &mut Query<&mut Visibility>| {
        if let Ok(mut v) = visibilities.get_mut(hover_state.shadow_entity) { *v = Visibility::Hidden; }
        for &e in &hover_state.liberty_entities {
            if let Ok(mut v) = visibilities.get_mut(e) { *v = Visibility::Hidden; }
        }
    };

    if board.game_over {
        hide(&hover_state, &mut visibilities);
        return;
    }

    let Some(screen_pos) = window.cursor_position() else {
        hide(&hover_state, &mut visibilities);
        return;
    };

    let (cam, cam_transform) = *camera;
    let Ok(world_pos) = cam.viewport_to_world_2d(cam_transform, screen_pos) else {
        hide(&hover_state, &mut visibilities);
        return;
    };

    let offset = (board.size as f32 - 1.0) * CELL_SIZE / 2.0;
    let x_index = ((world_pos.x + offset) / CELL_SIZE).round();
    let y_index = ((world_pos.y + offset) / CELL_SIZE).round();
    let snap_pos = Vec2::new(x_index * CELL_SIZE - offset, y_index * CELL_SIZE - offset);

    let size = board.size as i32;
    let x = x_index as i32;
    let y = y_index as i32;

    let valid = x >= 0 && x < size && y >= 0 && y < size
        && world_pos.distance(snap_pos) <= CELL_SIZE * 0.4;

    if !valid {
        hide(&hover_state, &mut visibilities);
        return;
    }

    let (r_x, r_y) = (x as usize, y as usize);
    let cell_empty = board.grid[r_y][r_x].0 == Stone::Empty;
    let ko_blocked = board.ko_forbidden == Some((r_x, r_y));

    if !cell_empty || ko_blocked {
        hide(&hover_state, &mut visibilities);
        return;
    }

    // Show shadow stone in the current player's color (semi-transparent)
    let shadow_color = if board.turn == Stone::Black {
        Color::srgba(0.05, 0.05, 0.05, 0.42)
    } else {
        Color::srgba(0.94, 0.94, 0.94, 0.55)
    };
    if let Some(mat) = materials.get_mut(&hover_state.shadow_material) {
        mat.color = shadow_color;
    }
    if let Ok(mut t) = transforms.get_mut(hover_state.shadow_entity) {
        t.translation = snap_pos.extend(0.8);
    }
    if let Ok(mut v) = visibilities.get_mut(hover_state.shadow_entity) {
        *v = Visibility::Visible;
    }

    // Show liberty dots on each empty adjacent cell
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (i, (dx, dy)) in dirs.iter().enumerate() {
        let nx = r_x as i32 + dx;
        let ny = r_y as i32 + dy;
        let in_bounds = nx >= 0 && nx < size && ny >= 0 && ny < size;
        let is_liberty = in_bounds && board.grid[ny as usize][nx as usize].0 == Stone::Empty;

        if let Ok(mut v) = visibilities.get_mut(hover_state.liberty_entities[i]) {
            *v = if is_liberty { Visibility::Visible } else { Visibility::Hidden };
        }
        if is_liberty {
            let dot_pos = Vec2::new(nx as f32 * CELL_SIZE - offset, ny as f32 * CELL_SIZE - offset);
            if let Ok(mut t) = transforms.get_mut(hover_state.liberty_entities[i]) {
                t.translation = dot_pos.extend(0.75);
            }
        }
    }
}
