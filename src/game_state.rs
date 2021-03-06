use crate::game::{GameHandle, MoveValidity};
use crate::load_map::tile_1a;
use crate::utils::get_current_time_secs;

use anyhow::{anyhow, Result};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::From;

use crate::types::{
    get_wall_color, proto_types, GameStatus, Heister, HeisterColor, Internal, MapPosition,
    MoveDirection, Player, PossibleTeleportEntry, Square, SquareType, StartingTile, Tile, WallType,
    TIMER_DURATION_SECS,
};

const MAX_PLAYERS: usize = 8;
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GameState {
    pub game_name: GameHandle,
    pub game_started: u64,
    pub timer_runs_out: u64,
    pub tiles: Vec<Tile>,
    pub heisters: Vec<Heister>,
    pub all_items_taken: bool,
    pub remaining_tiles: u32,
    pub game_status: GameStatus,
    pub players: Vec<Player>,
    pub possible_placements: Vec<MapPosition>,
    pub possible_escalators: HashMap<HeisterColor, MapPosition>,
    pub possible_teleports: HashMap<HeisterColor, Vec<MapPosition>>,
    pub players_may_speak: bool,
}

impl Internal for GameState {
    type P = proto_types::GameState;

    fn from_proto(proto: proto_types::GameState) -> Self {
        let game_name = GameHandle(proto.game_name);
        let tiles = proto
            .tiles
            .iter()
            .map(|t| Tile::from_proto(t.clone()))
            .collect();
        let heisters = proto
            .heisters
            .iter()
            .map(|h| Heister::from_proto(h.clone()))
            .collect();
        let players = proto
            .players
            .iter()
            .map(|p| Player::from_proto(p.clone()))
            .collect();
        let game_status = GameStatus::from_i32(proto.game_status).unwrap(); // TODO Handle this gracefully?
        let possible_placements = proto
            .possible_placements
            .iter()
            .map(|pp| MapPosition::from_proto(pp.clone()))
            .collect();
        let possible_escalators = proto
            .possible_escalators
            .iter()
            .map(|(c, pe)| {
                (
                    HeisterColor::from_i32(*c as i32).unwrap(),
                    MapPosition::from_proto(pe.clone()),
                )
            })
            .collect();

        // Have to actually process this list into a proper map
        let mut possible_teleports: HashMap<HeisterColor, Vec<MapPosition>> = HashMap::new();
        for entry in proto.possible_teleports {
            let color = HeisterColor::from_i32(entry.color).unwrap();
            let pos = MapPosition::from_proto(entry.position.unwrap());
            match possible_teleports.get_mut(&color) {
                Some(list) => {
                    list.push(pos);
                }
                None => {
                    let list: Vec<MapPosition> = vec![pos];
                    possible_teleports.insert(color, list);
                }
            }
        }
        GameState {
            game_name,
            game_started: proto.game_started,
            timer_runs_out: proto.timer_runs_out,
            tiles,
            heisters,
            all_items_taken: proto.all_items_taken,
            remaining_tiles: proto.remaining_tiles,
            game_status,
            players,
            possible_placements,
            possible_escalators,
            possible_teleports,
            players_may_speak: proto.players_may_speak,
        }
    }

    fn to_proto(&self) -> proto_types::GameState {
        let tiles = self.tiles.iter().map(|t| t.to_proto()).collect();
        let heisters = self.heisters.iter().map(|h| h.to_proto()).collect();
        let players = self.players.iter().map(|p| p.to_proto()).collect();
        let possible_placements = self
            .possible_placements
            .iter()
            .map(|pp| pp.to_proto())
            .collect();
        let possible_escalators = self
            .possible_escalators
            .iter()
            .map(|(c, pe)| (i32::from(*c), pe.to_proto()))
            .collect();
        let game_status = i32::from(self.game_status);

        let mut possible_teleports: Vec<PossibleTeleportEntry> = Vec::new();
        for (color, list) in self.possible_teleports.clone() {
            for pos in list {
                let entry = PossibleTeleportEntry {
                    color: i32::from(color),
                    position: Some(pos.to_proto()),
                };
                possible_teleports.push(entry);
            }
        }
        proto_types::GameState {
            game_name: self.game_name.0.to_string(),
            game_started: self.game_started,
            timer_runs_out: self.timer_runs_out,
            tiles,
            heisters,
            all_items_taken: self.all_items_taken,
            remaining_tiles: self.remaining_tiles,
            game_status,
            players,
            possible_placements,
            possible_escalators,
            possible_teleports,
            players_may_speak: self.players_may_speak,
        }
    }
}

impl GameState {
    pub fn new(game_name: GameHandle) -> Self {
        let game_started = 0;
        let timer_runs_out = 0;
        let starting_tile = tile_1a();
        let starting_tile_enum = StartingTile::A(starting_tile.clone());
        let tiles = vec![starting_tile.clone()];
        let possible_escalators = HashMap::new();
        let possible_teleports = HashMap::new();
        let mut heisters = Vec::new();
        heisters.push(Heister::get_initial(
            HeisterColor::Yellow,
            &starting_tile_enum,
        ));
        heisters.push(Heister::get_initial(
            HeisterColor::Purple,
            &starting_tile_enum,
        ));
        heisters.push(Heister::get_initial(
            HeisterColor::Green,
            &starting_tile_enum,
        ));
        heisters.push(Heister::get_initial(
            HeisterColor::Orange,
            &starting_tile_enum,
        ));
        GameState {
            game_name,
            game_started,
            timer_runs_out,
            tiles,
            heisters,
            all_items_taken: false,
            remaining_tiles: 8,
            game_status: GameStatus::Staging,
            players: vec![],
            possible_placements: vec![],
            possible_escalators,
            possible_teleports,
            players_may_speak: true,
        }
    }

    pub fn add_player(&mut self, name: String) -> Result<()> {
        if self.game_status != GameStatus::Staging || self.players.len() >= MAX_PLAYERS {
            // If the game is already in progress or there are already MAX_PLAYERS,
            // don't actually register the player.
            return Ok(());
        }
        self.players.push(Player {
            name,
            abilities: vec![],
        });
        Ok(())
    }

    pub fn has_player(&self, name: &str) -> bool {
        for p in self.players.iter() {
            if p.name == name {
                return true;
            }
        }
        false
    }

    pub fn start_timer(&mut self) -> () {
        let now = get_current_time_secs();
        self.game_started = now;
        self.timer_runs_out = now + TIMER_DURATION_SECS;
    }

    pub fn get_absolute_grid(&self) -> HashMap<MapPosition, Square> {
        let mut grid: HashMap<MapPosition, Square> = HashMap::new();
        for tile in self.tiles.iter() {
            // this is the top position for the tile - we can assign positions for this
            let tile_pos = &tile.position;
            for (i, square) in tile.squares.iter().enumerate() {
                let sq_x = (i % 4) as i32;
                let sq_y = (i / 4) as i32;
                let grid_x = tile_pos.x + sq_x;
                let grid_y = tile_pos.y + sq_y;
                let mp = MapPosition {
                    x: grid_x,
                    y: grid_y,
                };
                grid.insert(mp, square.clone());
            }
        }
        grid
    }

    // NOTE: Would be nice if self.game_state.heisters was a map<color, heister>
    // or even <color, pos>
    pub fn get_mut_heister_from_vec(&mut self, hc: HeisterColor) -> Option<&mut Heister> {
        for h in self.heisters.iter_mut() {
            if h.heister_color == hc {
                return Some(h);
            }
        }
        return None;
    }

    pub fn get_heister_from_vec(&self, hc: HeisterColor) -> Option<&Heister> {
        for h in self.heisters.iter() {
            if h.heister_color == hc {
                return Some(h);
            }
        }
        return None;
    }

    pub fn get_index_and_tile(&self, position: &MapPosition) -> Option<(usize, Tile)> {
        for (i, t) in self.tiles.iter().enumerate() {
            let x_distance = position.x - t.position.x;
            let x_distance_within_tile = x_distance >= 0 && x_distance < 4;
            match x_distance_within_tile {
                true => {
                    let y_distance = position.y - t.position.y;
                    let y_distance_within_tile = y_distance >= 0 && y_distance < 4;
                    match y_distance_within_tile {
                        true => return Some((i, t.clone())),
                        false => continue,
                    }
                }
                false => continue,
            }
        }
        None
    }

    pub fn adjacent_move_blocked_by_wall(
        &self,
        grid: &HashMap<MapPosition, Square>,
        heister_pos: &MapPosition,
        dest_pos: &MapPosition,
    ) -> MoveValidity {
        // Assumes that heister_pos & dest_pos are adjacent
        let heister_square = match grid.get(&heister_pos) {
            Some(s) => s,
            None => {
                return MoveValidity::Invalid(format!(
                    "Heister square {:?} doesn't exist",
                    heister_pos
                ))
            }
        };
        let blocking_wall = match heister_pos.get_move_direction(dest_pos) {
            Some(MoveDirection::North) => heister_square.north_wall,
            Some(MoveDirection::East) => heister_square.east_wall,
            Some(MoveDirection::South) => heister_square.south_wall,
            Some(MoveDirection::West) => heister_square.west_wall,
            None => {
                return MoveValidity::Invalid(format!(
                    "Cannot compute blocking walls for non-cardinal move"
                ))
            }
        };

        let invalid_msg = format!("Wall {:?} cannot be passed through", blocking_wall);
        match blocking_wall {
            WallType::Clear => MoveValidity::Valid,
            WallType::Impassable => {
                MoveValidity::Invalid("Can't pass through impassable wall".to_string())
            }
            // Wildcard matches each tile-discovery type (one per color)
            _wildcard => MoveValidity::Invalid(invalid_msg),
        }
    }

    pub fn position_is_occupied(&self, position: &MapPosition) -> MoveValidity {
        for h in &self.heisters {
            match &h.map_position == position {
                true => {
                    let msg = format!("Heister {:?} is on {:?}", h.heister_color, position);
                    return MoveValidity::Invalid(msg);
                }
                false => {}
            }
        }
        return MoveValidity::Valid;
    }

    pub fn validate_adjacent_move(
        &self,
        grid: &HashMap<MapPosition, Square>,
        heister_pos: &MapPosition,
        dest_pos: &MapPosition,
    ) -> MoveValidity {
        let validity = self.adjacent_move_blocked_by_wall(&grid, &heister_pos, &dest_pos);
        match validity {
            MoveValidity::Invalid(_) => return validity,
            _ => (),
        }
        self.position_is_occupied(&dest_pos)
    }

    /// Get a Vec<MapPosition> from heister_pos to dest_pos.
    /// Does not include heister_pos - the reason is that when checking for
    /// collisions, we don't want to disallow moves if a heister would "collide"
    /// with itself.
    /// Also, you can't create a range from a larger number to a smaller number.
    /// That's why for negative-oriented directions (North, West) we go from dest to
    /// heister rather than the more intuitive heister-to-dest.
    fn get_positions_on_path(
        &self,
        heister_pos: &MapPosition,
        dest_pos: &MapPosition,
        direction: &MoveDirection,
    ) -> Vec<MapPosition> {
        let mut positions: Vec<MapPosition> = Vec::new();
        match direction {
            MoveDirection::North => {
                for y in dest_pos.y..heister_pos.y {
                    positions.push(MapPosition {
                        x: heister_pos.x,
                        y,
                    });
                }
            }
            MoveDirection::South => {
                for y in (heister_pos.y + 1)..=dest_pos.y {
                    positions.push(MapPosition {
                        x: heister_pos.x,
                        y,
                    });
                }
            }
            MoveDirection::East => {
                for x in (heister_pos.x + 1)..=dest_pos.x {
                    positions.push(MapPosition {
                        x,
                        y: heister_pos.y,
                    });
                }
            }
            MoveDirection::West => {
                for x in dest_pos.x..heister_pos.x {
                    positions.push(MapPosition {
                        x,
                        y: heister_pos.y,
                    });
                }
            }
        }
        positions
    }

    /// Validate multispace move - for mouse-based click and drag across
    /// multiple spaces in a single move.
    /// Very similar to validate_adjacent_move, except we check whether any two
    /// adjacent squares on the path are blocked by a wall.
    /// We also check for map existence and collisions.
    pub fn validate_multispace_move(
        &self,
        grid: &HashMap<MapPosition, Square>,
        heister_pos: &MapPosition,
        dest_pos: &MapPosition,
    ) -> MoveValidity {
        // This is very similar to validate_adjacent_move, except!
        // We must check if ANY wall is blocked from heister_pos to dest_pos

        // Assume direction is valid, because we had to check it higher in the stack
        let direction = heister_pos.get_move_direction(dest_pos).unwrap();
        let path: Vec<MapPosition> = self.get_positions_on_path(heister_pos, dest_pos, &direction);
        // validate that all positions on path exist in grid & check for collisions
        for pos in &path {
            match grid.get(&pos) {
                Some(_) => (),
                None => {
                    return MoveValidity::Invalid(format!(
                        "Cannot move through position {:?} because it does not exist.",
                        pos
                    ));
                }
            }
            let collision = self.position_is_occupied(&pos);
            if collision != MoveValidity::Valid {
                return collision;
            }
        }
        // Do a pairwise-sliding check for if any pair of squares are blocked
        for pair in path.windows(2) {
            let validity = self.adjacent_move_blocked_by_wall(&grid, &pair[0], &pair[1]);
            match validity {
                MoveValidity::Invalid(_) => return validity,
                _ => (),
            }
        }
        self.position_is_occupied(&dest_pos)
    }

    /// Return the places from which you could draw a tile
    /// AKA - squares where a matching heister is on a square with a HeisterColor door
    fn heister_tile_placement_positions(
        &self,
        grid: &HashMap<MapPosition, Square>,
    ) -> Vec<MapPosition> {
        let mut placement_locations: Vec<MapPosition> = Vec::new();
        for heister in &self.heisters {
            if heister.has_escaped {
                continue;
            }
            let color = heister.heister_color;
            let square = grid
                .get(&heister.map_position)
                .expect("Heister on invalid square");
            let maybe_door = square.get_door_wall();
            let door = match maybe_door {
                Some(d) => d,
                None => continue,
            };
            match get_wall_color(door) {
                Some(door_color) => {
                    if door_color == color {
                        placement_locations.push(heister.map_position.clone());
                    }
                }
                None => continue,
            }
        }
        placement_locations
    }

    /// Returns a map from current heister positions to their prospective tile_entrance
    /// positions, one tile away (if there are any such locations)
    pub fn heister_to_tile_entrance_positions(
        &self,
        grid: &HashMap<MapPosition, Square>,
    ) -> HashMap<MapPosition, MapPosition> {
        let heister_door_positions = self.heister_tile_placement_positions(&grid);
        let mut tile_entrance_positions: HashMap<MapPosition, MapPosition> = HashMap::new();
        for heister_pos in heister_door_positions {
            let square = grid
                .get(&heister_pos)
                .expect("Heister must be on a valid square");
            let dir = square
                .get_door_direction()
                .expect("Square must have a door on it to be entered through");
            tile_entrance_positions
                .insert(heister_pos.clone(), heister_pos.move_in_direction(&dir));
        }
        tile_entrance_positions
    }

    /// Possible placements for new tiles that Heisters can discover
    pub fn update_possible_placements(&mut self, grid: &HashMap<MapPosition, Square>) -> () {
        let heister_to_tile_entrance_locs = self.heister_to_tile_entrance_positions(&grid);

        let mut v = Vec::new();
        for val in heister_to_tile_entrance_locs.values() {
            v.push(val.clone());
        }
        self.possible_placements = v;
    }

    /// Possible escalator destinations that a Heister can reach with an Escalator move
    pub fn update_possible_escalators(&mut self, grid: &HashMap<MapPosition, Square>) -> () {
        let mut m: HashMap<HeisterColor, MapPosition> = HashMap::new();
        for heister in &self.heisters {
            if heister.has_escaped {
                continue;
            }
            let color = heister.heister_color;
            let pos = &heister.map_position;

            let square = grid.get(&pos).unwrap();
            if square.square_type == SquareType::Escalator {
                let (_idx, tile) = self.get_index_and_tile(&pos).unwrap();
                let dest_pos = tile.get_escalator_dest(&pos).unwrap();
                m.insert(color, dest_pos);
            }
        }
        self.possible_escalators = m;
    }

    /// This will check for victory/defeat conditions as part of update_auxiliary_state.
    /// (intended to capture state changes due to timer)
    pub fn update_game_status(&mut self) -> () {
        if self.game_status == GameStatus::Ongoing {
            if self.all_items_taken && self.heisters.iter().all(|h| h.has_escaped) {
                self.game_status = GameStatus::Victory;
                return;
            }
            let now = get_current_time_secs();
            if self.game_started != 0 && now > self.timer_runs_out {
                info!("Time ran out for game {:?}, you lost!", self.game_name);
                self.game_status = GameStatus::Defeat;
                return;
            }
        }
    }

    /// This will check for each heister and update its "has_taken_item" field
    /// This will also update the game state field "all_items_taken"
    pub fn update_items_taken(&mut self, grid: &HashMap<MapPosition, Square>) -> () {
        for heister in &mut self.heisters {
            match grid.get(&heister.map_position) {
                Some(square) => {
                    heister.has_taken_item = self.all_items_taken
                        || square.is_item() && square.color().unwrap() == heister.heister_color;
                }
                None => {}
            }
        }
        self.all_items_taken =
            self.all_items_taken || self.heisters.iter().all(|h| h.has_taken_item);
    }

    /// in order to update the door to be a clear wall, we need a few things:
    /// 1. we need a reference to the tile in self.tiles that contains the heister_square
    /// 2. we need to be able to know which wall on which square  to update
    /// 3. we need to replace that square wth one who has a clear wall instead of a door
    pub fn open_door(
        &mut self,
        door_pos: MapPosition,
        src_square: Square,
        dir: &MoveDirection,
    ) -> Result<()> {
        let current_tile_position = door_pos.current_tile_position(&dir);
        let mut tile = &mut Tile::default();
        for t in &mut self.tiles {
            if t.position == current_tile_position {
                tile = t;
                break;
            }
        }
        if tile.squares.len() == 0 {
            return Err(anyhow!("No tile found at pos {:?}", door_pos));
        }

        // TODO: the helper i am writing will change THIS iterator in open_door
        // helper = something like "get tile door squares"
        for mut square in &mut tile.squares {
            if square == &src_square {
                // Open The Door
                match dir {
                    MoveDirection::North => {
                        square.north_wall = WallType::Clear;
                    }
                    MoveDirection::East => {
                        square.east_wall = WallType::Clear;
                    }
                    MoveDirection::South => {
                        square.south_wall = WallType::Clear;
                    }
                    MoveDirection::West => {
                        square.west_wall = WallType::Clear;
                    }
                }
                return Ok(());
            }
        }
        return Err(anyhow!(
            "When opening a door, we expect the square to have a door to open"
        ));
    }

    /// In addition to rotating a new tile, we also need to open/close any doors
    /// it has that align with existing doors.
    /// * This takes an index into self.tiles, because it can only operate on a
    /// tile that has already been added to self.tiles
    pub fn update_tile_doors(&mut self, tile_idx: usize) -> () {
        let grid = self.get_absolute_grid();
        let tile = &self.tiles[tile_idx];

        for (dir, position) in tile.adjacent_entrances() {
            match grid.get(&position) {
                Some(neighbor_square) => {
                    let my_door_pos = position.move_in_direction(&dir.opposite());
                    let my_square = grid.get(&my_door_pos).unwrap();

                    let mut_tile = &mut self.tiles[tile_idx];
                    if my_square.has_door() {
                        if neighbor_square.has_door() {
                            mut_tile.open_door_in_dir(dir);
                            let (idx, mut neighbor_tile) =
                                self.get_index_and_tile(&position).unwrap();
                            neighbor_tile.open_door_in_dir(dir.opposite());
                            self.tiles[idx] = neighbor_tile;
                        } else {
                            // If there isn't a door on the other side, close door
                            // that way, we know it won't be a possible_placement
                            mut_tile.close_door_in_dir(dir);
                        }
                    } else {
                        // If my square does NOT have a door, but neighbor does
                        if neighbor_square.has_door() {
                            let (idx, mut neighbor_tile) =
                                self.get_index_and_tile(&position).unwrap();
                            neighbor_tile.close_door_in_dir(dir.opposite());
                            self.tiles[idx] = neighbor_tile;
                        }
                    }
                }
                None => {}
            }
        }
    }
}
