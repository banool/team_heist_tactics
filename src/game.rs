use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;

use crate::game_state::GameState;
use crate::load_map;
use crate::types::main_message::Body;
use crate::types::{
    Ability, GameStatus, Heister, HeisterColor, Internal, MainMessage, MapPosition, Move,
    MoveDirection, PlaceTile, PlayerName, Square, SquareType, Tile, ESCAPED, TIMER_DURATION_SECS,
};
use crate::utils::get_current_time_secs;

use log::{debug, info};

#[derive(Debug)]
pub struct Game {
    pub game_handle: GameHandle,
    pub game_options: GameOptions,
    pub game_state: GameState,
    pub tile_deck: Vec<Tile>,
    pub game_created: u64,
    revealed_teleporters: HashMap<HeisterColor, Vec<MapPosition>>,
}
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct GameHandle(pub String);

#[derive(Debug)]
pub struct GameOptions {
    pub shuffle_tiles: bool,
    pub teleport_only_from_portal: bool,
}

impl Default for GameOptions {
    fn default() -> Self {
        GameOptions {
            shuffle_tiles: true,
            teleport_only_from_portal: false,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum MoveValidity {
    Valid,
    Invalid(String),
}

impl MoveValidity {
    pub fn is_invalid(&self) -> bool {
        match self {
            MoveValidity::Invalid(_) => true,
            MoveValidity::Valid => false,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.is_invalid()
    }
}

impl Game {
    pub fn new(game_handle: GameHandle, game_options: GameOptions) -> Game {
        let game_state = GameState::new(game_handle.clone());
        let mut tile_deck: Vec<Tile> = load_map::load_tiles_from_json();
        if game_options.shuffle_tiles {
            let mut rng = thread_rng();
            tile_deck.shuffle(&mut rng);
        }
        let game_created = get_current_time_secs();
        // NOTE: Assumption: All games start with only one tile revealed
        let mut revealed_teleporters: HashMap<HeisterColor, Vec<MapPosition>> = HashMap::new();
        Self::update_revealed_teleporters(
            &mut revealed_teleporters,
            game_state.tiles.first().unwrap(),
        );
        Game {
            game_handle,
            game_options,
            game_state,
            tile_deck,
            game_created,
            revealed_teleporters,
        }
    }

    fn draw_tile(&mut self) -> Option<Tile> {
        let tile = self.tile_deck.pop();
        self.game_state.remaining_tiles = self.tile_deck.len().try_into().unwrap();
        tile
    }

    pub fn add_player(&mut self, name: String) -> Result<()> {
        self.game_state.add_player(name)
    }

    pub fn start_game(&mut self) -> MoveValidity {
        // When we start the game, we can figure out how to break up the abilities.
        let player_abilities: Vec<Vec<Ability>> =
            get_player_abilities(self.game_state.players.len());
        for (i, player) in self.game_state.players.iter_mut().enumerate() {
            player.abilities = player_abilities[i].clone();
        }

        // Set the game status to ONGOING.
        self.game_state.game_status = GameStatus::PreFirstMove;

        // TODO Add this later, it would be too annoying for now for testing.
        /*
        if self.game_state.players.len() < 2 {
            MoveValidity::Invalid("There must be at least 2 players".to_string())
        } else {
            MoveValidity::Valid
        }
        */
        MoveValidity::Valid
    }

    fn rotate_abilities(&mut self) {
        let mut player_abilities: Vec<Vec<Ability>> = self
            .game_state
            .players
            .iter()
            .map(|p| p.abilities.clone())
            .collect();
        player_abilities.rotate_right(1);
        for (i, player) in self.game_state.players.iter_mut().enumerate() {
            player.abilities = player_abilities[i].clone();
        }
    }

    fn player_has_ability(&self, player_name: &PlayerName, ability: &Ability) -> bool {
        let player = self
            .game_state
            .players
            .iter()
            .find(|p| p.name == player_name.0)
            .expect("Tried to check for ability for player not in game");
        player.abilities.contains(ability)
    }

    pub fn has_player(&self, name: &str) -> bool {
        self.game_state.has_player(name)
    }

    pub fn get_game_state(&self) -> GameState {
        self.game_state.clone()
    }

    fn position_squaretype(
        grid: &HashMap<MapPosition, Square>,
        pos: &MapPosition,
    ) -> Result<SquareType> {
        let square = match grid.get(&pos) {
            Some(s) => s,
            None => {
                return Err(anyhow!("No square at pos {:?}", pos));
            }
        };
        Ok(square.square_type)
    }

    fn position_is_escalator(
        grid: &HashMap<MapPosition, Square>,
        pos: &MapPosition,
    ) -> MoveValidity {
        match grid.get(&pos) {
            Some(square) => match square.square_type {
                SquareType::Escalator => MoveValidity::Valid,
                _wildcard => {
                    let msg = format!("Square at {:?} is not escalator", pos);
                    MoveValidity::Invalid(msg)
                }
            },
            None => {
                let msg = format!("Position {:?} not found in grid", pos);
                MoveValidity::Invalid(msg)
            }
        }
    }

    /// To validate an escalator move, we need to do a few checks:
    /// 1. is the dest position on an escalator square?
    /// 2. is the heister on an escalator square?
    /// 3. is the heister in the same tile as the dest escalator?
    fn validate_escalator_move(
        &self,
        grid: &HashMap<MapPosition, Square>,
        heister_pos: &MapPosition,
        dest_pos: &MapPosition,
    ) -> MoveValidity {
        match Self::position_is_escalator(grid, dest_pos) {
            MoveValidity::Valid => {
                match grid.get(&heister_pos).unwrap().square_type {
                    SquareType::Escalator => {
                        // last check: is the heister & dest on the same tile?
                        let ht = self.game_state.get_index_and_tile(heister_pos).unwrap().1;
                        let dt = self.game_state.get_index_and_tile(dest_pos).unwrap().1;
                        match ht == dt {
                            true => MoveValidity::Valid,
                            false => MoveValidity::Invalid(
                                "Heister and dest escalator are on different tiles".to_string(),
                            ),
                        }
                    }
                    _wildcard => {
                        MoveValidity::Invalid("Heister is not on an escalator".to_string())
                    }
                }
            }
            _invalid => _invalid,
        }
    }

    /// To validate a teleporter move, we need to do a few checks:
    /// 1. is the dest position on a teleporter square?
    /// 2. is the heister color matching the teleporter color?
    /// 3.optional: is the source position on a teleporter square matching its color?
    fn validate_teleport(
        &self,
        grid: &HashMap<MapPosition, Square>,
        heister: &Heister,
        dest_pos: &MapPosition,
        teleport_only_from_portal_option: bool,
    ) -> MoveValidity {
        let heister_color = heister.heister_color;
        let heister_pos = &heister.map_position;
        let heister_square_type = Self::position_squaretype(grid, &heister_pos).unwrap();
        match grid.get(&dest_pos) {
            Some(dest_square) => {
                if !dest_square.teleport_matches_color(heister_color) {
                    let msg = "Heister and teleporter color do not match";
                    return MoveValidity::Invalid(msg.to_string());
                }
                if !teleport_only_from_portal_option {
                    return MoveValidity::Valid;
                }
                match heister_square_type == dest_square.square_type {
                    true => MoveValidity::Valid,
                    false => {
                        let msg = "Source and Dest teleporter colors do not match";
                        MoveValidity::Invalid(msg.to_string())
                    }
                }
            }
            None => {
                let msg = format!("Destination is not on the grid: {:?}", dest_pos);
                MoveValidity::Invalid(msg.to_string())
            }
        }
    }

    fn pre_update_auxilliary_state(&mut self) {
        self.game_state.update_game_status();
    }

    fn update_auxiliary_state(&mut self) -> () {
        let grid = self.game_state.get_absolute_grid();
        self.game_state.update_items_taken(&grid);
        self.game_state.update_possible_placements(&grid);
        self.game_state.update_possible_escalators(&grid);
        self.update_possible_teleports(&grid);
    }

    /// Possible teleport destinations that a Heister can reach with a Teleport move
    fn update_possible_teleports(&mut self, grid: &HashMap<MapPosition, Square>) -> () {
        let mut m: HashMap<HeisterColor, Vec<MapPosition>> = HashMap::new();
        for heister in &self.game_state.heisters {
            if heister.has_escaped {
                continue;
            }
            let color = heister.heister_color;
            let pos = &heister.map_position;
            let square = grid.get(&pos).unwrap();
            if !self.game_options.teleport_only_from_portal || square.is_teleport() {
                match self.revealed_teleporters.get(&color) {
                    Some(list) => {
                        m.insert(color, list.to_vec());
                    }
                    None => {}
                }
            }
        }
        self.game_state.possible_teleports = m;
    }

    /// Return all revealed teleporters, called after adding new tiles
    fn update_revealed_teleporters(
        already_revealed: &mut HashMap<HeisterColor, Vec<MapPosition>>,
        new_tile: &Tile,
    ) -> () {
        for (color, teleporter_pos) in new_tile.get_teleporters() {
            match already_revealed.get_mut(&color) {
                Some(already_revealed_list) => {
                    // append the new list onto the end
                    already_revealed_list.push(teleporter_pos);
                }
                None => {
                    // set the new list from teleporter list
                    let teleporter_list: Vec<MapPosition> = vec![teleporter_pos];
                    already_revealed.insert(color, teleporter_list);
                }
            }
        }
    }

    fn validate_player_has_move_direction_ability(
        &self,
        current_pos: &MapPosition,
        dest_pos: &MapPosition,
        player_name: &PlayerName,
    ) -> MoveValidity {
        let direction = current_pos.adjacent_move_direction(&dest_pos);
        match direction {
            MoveDirection::North => {
                if !self.player_has_ability(&player_name, &Ability::MoveNorth) {
                    return MoveValidity::Invalid("You cannot move heisters North".to_string());
                }
            }
            MoveDirection::East => {
                if !self.player_has_ability(&player_name, &Ability::MoveEast) {
                    return MoveValidity::Invalid("You cannot move heisters East".to_string());
                }
            }
            MoveDirection::South => {
                if !self.player_has_ability(&player_name, &Ability::MoveSouth) {
                    return MoveValidity::Invalid("You cannot move heisters South".to_string());
                }
            }
            MoveDirection::West => {
                if !self.player_has_ability(&player_name, &Ability::MoveWest) {
                    return MoveValidity::Invalid("You cannot move heisters West".to_string());
                }
            }
        }
        MoveValidity::Valid
    }

    /// This function handles checking validity of Moves and executing them.
    /// It also updates auxiliary game state like the timer depending on if the
    /// move activated a timer.
    /// 1. Rather than return early, it should set the return value, and continue
    ///  that way, if we fall through to other cases (ie. adjacent teleport)
    ///  the move may still work.
    /// 2. If we defer the return until the end, then we can have logic
    ///  that handles differently based on whether the final result is valid
    ///  (or invalid) - AKA timer, "items taken", "you may speak" logic
    fn process_move(&mut self, m: Move, player_name: &PlayerName) -> MoveValidity {
        let heister_color = m.heister_color;
        let heister = self.game_state.get_heister_from_vec(heister_color).unwrap();
        let heister_pos = &heister.map_position;
        let dest_pos = m.position;
        let all_items_taken = self.game_state.all_items_taken;
        let timer_runs_out = self.game_state.timer_runs_out;
        let mut validity = MoveValidity::Valid;

        let grid = self.game_state.get_absolute_grid();
        let mut dest_square = Square::default();
        match grid.get(&dest_pos) {
            Some(square) => {
                dest_square = *square;
            }
            None => {
                let msg = format!("Position {:?} not on map", dest_pos);
                validity = MoveValidity::Invalid(msg);
            }
        }
        match dest_square.square_type {
            // Handle escalator move
            SquareType::Escalator => {
                if !self.player_has_ability(&player_name, &Ability::UseEscalator) {
                    validity = MoveValidity::Invalid("You cannot use escalators".to_string());
                }
                validity = if validity.is_invalid() {
                    validity
                } else {
                    self.validate_escalator_move(&grid, heister_pos, &dest_pos)
                };
            }
            // Handle teleport move
            SquareType::OrangeTeleportPad
            | SquareType::YellowTeleportPad
            | SquareType::PurpleTeleportPad
            | SquareType::GreenTeleportPad => {
                if all_items_taken {
                    validity = MoveValidity::Invalid(
                        "All items have been taken, so teleports are disabled!".to_string(),
                    );
                }
                if !self.player_has_ability(&player_name, &Ability::Teleport) {
                    validity = MoveValidity::Invalid("You cannot use teleporters".to_string());
                }
                validity = if validity.is_invalid() {
                    validity
                } else {
                    self.validate_teleport(
                        &grid,
                        heister,
                        &dest_pos,
                        self.game_options.teleport_only_from_portal,
                    )
                }
            }
            _ => {
                validity = MoveValidity::Invalid("move wasn't teleport nor escalator".to_string());
            }
        }

        if heister_pos.is_adjacent(&dest_pos) && validity.is_invalid() {
            validity = self.validate_player_has_move_direction_ability(
                &heister_pos,
                &dest_pos,
                &player_name,
            );
            validity = if validity.is_invalid() {
                validity
            } else {
                self.game_state
                    .validate_adjacent_move(&grid, heister_pos, &dest_pos)
            };
        }

        // Regardless of the move type, if the move is valid, we execute it
        if validity == MoveValidity::Valid {
            let heister = self
                .game_state
                .get_mut_heister_from_vec(heister_color.clone())
                .unwrap();
            let mut destination = dest_pos;
            if dest_square.is_escape() && all_items_taken {
                destination = *ESCAPED;
                heister.has_escaped = true;
            }
            heister.map_position = destination;

            // If this is the first move, then let's start the game timer
            if self.game_state.game_status == GameStatus::PreFirstMove {
                self.game_state.start_timer();
                self.game_state.game_status = GameStatus::Ongoing;
            }
            // If this square was a timer, we need to mark it used, and update
            // timer_runs_out to the new time limit
            if dest_square.square_type == SquareType::TimerFlip {
                // step 1: mark used
                let (idx, tile) = self.game_state.get_index_and_tile(&dest_pos).unwrap();
                let mut flipped_tile = tile.clone();
                flipped_tile.flip_timer();
                self.game_state.tiles[idx] = flipped_tile;

                // step 2: update timer_runs_out
                let now = get_current_time_secs();
                if now > timer_runs_out {
                    self.game_state.timer_runs_out = now;
                } else {
                    let time_left: i64 = (self.game_state.timer_runs_out - get_current_time_secs())
                        .try_into()
                        .unwrap();
                    let new_time_left: u64 =
                        (TIMER_DURATION_SECS as i64 - time_left).try_into().unwrap();
                    self.game_state.timer_runs_out = get_current_time_secs() + new_time_left;
                }
            }
        }
        validity
    }

    fn place_tile(&mut self, position: &MapPosition, direction: &MoveDirection) -> MoveValidity {
        let tile = self.draw_tile();
        match tile {
            Some(t) => {
                let new_pos = position.new_tile_position(direction);
                let num_rotations = match direction {
                    MoveDirection::North => 0,
                    MoveDirection::East => 1,
                    MoveDirection::South => 2,
                    MoveDirection::West => 3,
                };
                let mut m: Vec<Vec<Square>> = t.to_matrix();
                for _ in 0..num_rotations {
                    m = Tile::rotate_matrix_clockwise(&m);
                }
                let rotated_tile = Tile::from_matrix(m, t.name.clone(), new_pos, num_rotations);
                info!(
                    "Added Tile {} at {:?} to Game map",
                    rotated_tile.name, rotated_tile.position
                );

                // Add the tile before opening doors on it, that way helpers that
                // rely on the tile's presence in game.tiles work correctly
                let new_tile_idx = self.game_state.tiles.len();
                Self::update_revealed_teleporters(&mut self.revealed_teleporters, &rotated_tile);
                self.game_state.tiles.push(rotated_tile);
                self.game_state.update_tile_doors(new_tile_idx); // Must be called _after_ push
                MoveValidity::Valid
            }
            None => MoveValidity::Invalid("No tiles left in deck to draw".to_string()),
        }
    }

    fn process_tile_placement(&mut self, pt: PlaceTile, player_name: &PlayerName) -> MoveValidity {
        if !self.player_has_ability(player_name, &Ability::RevealTiles) {
            return MoveValidity::Invalid("You cannot reveal tiles".to_string());
        }
        let grid = self.game_state.get_absolute_grid();
        let heister_to_tile_entrance_locs =
            self.game_state.heister_to_tile_entrance_positions(&grid);
        let maybe_heister_pos_tuple = heister_to_tile_entrance_locs
            .iter()
            .find(|&(_, te)| te == &pt.tile_entrance);
        let heister_pos = match maybe_heister_pos_tuple {
            Some(pos_tuple) => pos_tuple.0,
            None => {
                return MoveValidity::Invalid(
                    "When placing a tile, the heister must be at a tile-reveal door".to_string(),
                );
            }
        };

        let heister_square = grid
            .get(heister_pos)
            .expect("Heister must be on valid square");
        let dir = heister_square
            .get_door_direction()
            .expect("Heister must be on a square with a door");

        match self
            .game_state
            .open_door(heister_pos.clone(), *heister_square, &dir)
        {
            Ok(_) => self.place_tile(&pt.tile_entrance, &dir),
            Err(e) => {
                let msg = format!("Couldn't open door for newly placed tile: {}", e);
                MoveValidity::Invalid(msg.to_string())
            }
        }
    }

    fn game_is_ongoing(&self) -> MoveValidity {
        match &self.game_state.game_status {
            GameStatus::Ongoing | GameStatus::PreFirstMove => MoveValidity::Valid,
            wildcard => {
                let msg = format!(
                    "Game {:?} is in state {:?} and is not accepting moves",
                    self.game_handle.0, wildcard
                );
                MoveValidity::Invalid(msg)
            }
        }
    }

    pub fn handle_message(
        &mut self,
        message: MainMessage,
        player_name: &PlayerName,
    ) -> MoveValidity {
        // If we receive GameState or InvalidRequest at this endpoint, panic, it should never happen.
        debug!("Received message: {:?}", message);
        self.pre_update_auxilliary_state();
        let body = message.body.unwrap();
        let validity = match body {
            Body::StartGame(_) => self.start_game(),
            Body::Move(m) => {
                let valid_game_state = self.game_is_ongoing();
                match valid_game_state {
                    MoveValidity::Invalid(_) => return valid_game_state,
                    MoveValidity::Valid => {}
                }
                self.process_move(Move::from_proto(m), &player_name)
            }
            Body::PlaceTile(pt) => {
                let valid_game_state = self.game_is_ongoing();
                match valid_game_state {
                    MoveValidity::Invalid(_) => return valid_game_state,
                    MoveValidity::Valid => {}
                }
                self.process_tile_placement(PlaceTile::from_proto(pt), &player_name)
            }
            Body::GameState(_gs) => {
                MoveValidity::Invalid("GameState Message is invalid from players".to_string())
            }
            Body::InvalidRequest(_ir) => {
                MoveValidity::Invalid("InvalidRequest Message is invalid from players".to_string())
            }
            Body::Chat(_c) => MoveValidity::Valid,
        };
        self.update_auxiliary_state();

        validity
    }
}

// TODO When we make it that games can't start until 2 - 8 players have joined,
// remove the matches on 0 and 1.
fn get_player_abilities(num_players: usize) -> Vec<Vec<Ability>> {
    let mut player_abilities = match num_players {
        0 | 1 => vec![vec![
            Ability::MoveNorth,
            Ability::MoveEast,
            Ability::MoveSouth,
            Ability::MoveWest,
            Ability::Teleport,
            Ability::RevealTiles,
            Ability::UseEscalator,
        ]],
        2 => vec![
            vec![Ability::MoveNorth, Ability::MoveEast, Ability::Teleport],
            vec![
                Ability::MoveSouth,
                Ability::MoveWest,
                Ability::RevealTiles,
                Ability::UseEscalator,
            ],
        ],
        3 => vec![
            vec![Ability::MoveNorth, Ability::MoveEast],
            vec![
                Ability::MoveSouth,
                Ability::RevealTiles,
                Ability::UseEscalator,
            ],
            vec![Ability::MoveWest, Ability::Teleport],
        ],
        4 => vec![
            vec![Ability::MoveNorth],
            vec![Ability::MoveEast, Ability::UseEscalator],
            vec![Ability::MoveSouth, Ability::RevealTiles],
            vec![Ability::MoveWest, Ability::Teleport],
        ],
        5 => vec![
            vec![Ability::MoveNorth],
            vec![Ability::MoveEast, Ability::UseEscalator],
            vec![Ability::MoveSouth, Ability::RevealTiles],
            vec![Ability::MoveWest],
            vec![Ability::MoveWest, Ability::Teleport],
        ],
        6 => vec![
            vec![Ability::MoveNorth],
            vec![Ability::MoveEast],
            vec![Ability::MoveEast, Ability::UseEscalator],
            vec![Ability::MoveSouth, Ability::RevealTiles],
            vec![Ability::MoveWest],
            vec![Ability::MoveWest, Ability::Teleport],
        ],
        7 => vec![
            vec![Ability::MoveNorth],
            vec![Ability::MoveEast],
            vec![Ability::MoveEast, Ability::UseEscalator],
            vec![Ability::MoveSouth],
            vec![Ability::MoveSouth, Ability::RevealTiles],
            vec![Ability::MoveWest],
            vec![Ability::MoveWest, Ability::Teleport],
        ],
        8 => vec![
            vec![Ability::MoveNorth],
            vec![Ability::MoveNorth],
            vec![Ability::MoveEast],
            vec![Ability::MoveEast, Ability::UseEscalator],
            vec![Ability::MoveSouth],
            vec![Ability::MoveSouth, Ability::RevealTiles],
            vec![Ability::MoveWest],
            vec![Ability::MoveWest, Ability::Teleport],
        ],
        wildcard => panic!("Invalid number of players somehow: {}", wildcard),
    };
    let mut rng = thread_rng();
    player_abilities.shuffle(&mut rng);
    player_abilities
}
