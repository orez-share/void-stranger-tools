use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry::Vacant;
use std::fmt;
use std::hash::Hash;
use std::iter::{once, zip};

const LEVEL: &str = "
    # # ###
    # # # #
#####8###8##
#@...+HH+..#
#...+H.+H.>#
#d..H+..+H.#
############
";

type Error = String;  // XXX: :)
type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
enum GameState {
    Win,
    Lose,
    Play(Level),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Floor,
    Wall,
    Trap,
    PrimedTrap,
    Hole,
    Exit,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Entity {
    Player,
    GolemY,
    Hand,
    CovetousStatue,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Level {
    width: usize,
    height: usize,
    player_facing: usize,
    tiles: Vec<Tile>,
    entities: Vec<Option<Entity>>,
}

impl fmt::Debug for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut it = zip(&self.tiles, &self.entities);
        for _ in 0..self.height {
            for (tile, entity) in it.by_ref().take(self.width) {
                let chr = if let Some(e) = entity {
                    match e {
                        Entity::Player => '@',
                        Entity::CovetousStatue => '8',
                        Entity::GolemY => 'd',
                        Entity::Hand => 'H',
                    }
                } else {
                    match tile {
                        Tile::Floor => '.',
                        Tile::Wall => '#',
                        Tile::Hole => ' ',
                        Tile::Trap => '+',
                        Tile::PrimedTrap => '!',
                        Tile::Exit => '>',
                    }
                };
                write!(f, "{}", chr)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Level {
    pub fn solve(&self) -> Result<Vec<Level>> {
        let mut state_count = 0;
        let mut backpath = HashMap::new();
        backpath.insert(self.clone(), None);
        let mut frontier: VecDeque<_> = once(self.clone()).collect();
        while let Some(state) = frontier.pop_front() {
            state_count += 1;
            for next in state.next_states() {
                match next {
                    GameState::Win => return Ok(Level::reconstruct_path(backpath, state)),
                    GameState::Lose => (),
                    GameState::Play(level) => {
                        if let Vacant(slot) = backpath.entry(level.clone()) {
                            slot.insert(Some(state.clone()));  // XXX: worth Rc-ing? probably.. not.
                            frontier.push_back(level);
                        }
                    }
                }
            }
        }
        Err(format!("no path (checked {state_count} states)"))
    }

    fn reconstruct_path(mut backpath: HashMap<Level, Option<Level>>, mut level: Level) -> Vec<Level> {
        let mut out = Vec::new();
        out.push(level.clone());
        while let Some(lvl) = backpath.get_mut(&level).unwrap().take() {
            out.push(lvl.clone());
            level = lvl;
        }
        out.reverse();
        out
    }

    fn next_states(&self) -> Vec<GameState> {
        vec![
            self.walk(0_usize.wrapping_sub(self.width)),
            self.walk(usize::MAX), // -1
            self.walk(1),
            self.walk(self.width),
            self.act(),
        ]
    }

    fn walk(&self, step: usize) -> GameState {
        let mut level = self.clone();
        let player_idx = self.entities
            .iter()
            .position(|e| matches!(e, Some(Entity::Player)))
            .unwrap();
        let next_player_idx = player_idx.wrapping_add(step);
        if next_player_idx >= self.tiles.len() { unimplemented!("OOB"); }
        // XXX: ideally we deal with this case-by-case,
        // but right now everything kills us anyway!
        if level.entities[next_player_idx].is_some() {
            return GameState::Lose;
        }
        level.player_facing = step;
        match level.tiles[next_player_idx] {
            Tile::Exit => return GameState::Win,
            Tile::Floor => {
                level.entities[player_idx] = None;
                level.entities[next_player_idx] = Some(Entity::Player);
            }
            Tile::Wall => if matches!(level.tiles[player_idx], Tile::Hole) {
                return GameState::Lose;
            },
            Tile::Trap => {
                level.tiles[next_player_idx] = Tile::PrimedTrap;
                level.entities[player_idx] = None;
                level.entities[next_player_idx] = Some(Entity::Player);
            }
            Tile::PrimedTrap => {
                // XXX: i'm not actually sure this is right..
                // idk if it can even happen on this level though.
                if matches!(level.tiles[player_idx], Tile::Hole) {
                    return GameState::Lose;
                }
                // XXX: also the BFS effect.. but can't happen in this puzzle.
                level.tiles[next_player_idx] = Tile::Hole;
                level.entities[player_idx] = None;
                level.entities[next_player_idx] = Some(Entity::Player);
            }
            Tile::Hole => {
                // XXX: flight!
                if matches!(level.tiles[player_idx], Tile::Hole) {
                    return GameState::Lose;
                }
                level.entities[player_idx] = None;
                level.entities[next_player_idx] = Some(Entity::Player);
            }
        }

        if level.is_zap(next_player_idx) { return GameState::Lose }
        // XXX: enemy logic could be very complicated,
        // but for this level it's very simple.
        for (eidx, entity) in self.entities.iter().enumerate() {
            let Some(Entity::GolemY) = entity else { continue };
            let golem_step = match step {
                1 | usize::MAX => step,
                _ => 0_usize.wrapping_sub(step),
            };
            let next_eidx = eidx.wrapping_add(golem_step);
            // XXX: boundscheck
            match level.entities[next_eidx] {
                None => match level.tiles[next_eidx] {
                    Tile::Exit | Tile::Floor => {
                        level.entities[eidx] = None;
                        level.entities[next_eidx] = Some(Entity::GolemY);
                    }
                    Tile::Wall => (),
                    Tile::Hole | Tile::PrimedTrap => { return GameState::Lose; }  // XXX: not strictly true..
                    Tile::Trap => {
                        level.tiles[next_eidx] = Tile::PrimedTrap;
                        level.entities[eidx] = None;
                        level.entities[next_eidx] = Some(Entity::GolemY);
                    }
                }
                Some(Entity::Hand) => continue,
                Some(Entity::CovetousStatue) => {
                    // XXX: push is hard
                    // in this puzzle it just means kill.
                    level.entities[next_eidx].take();
                }
                Some(Entity::Player) => { return GameState::Lose }
                Some(Entity::GolemY) => todo!(),
            }
        }
        if level.is_zap(next_player_idx) { return GameState::Lose }
        GameState::Play(level)
    }

    fn act(&self) -> GameState {
        let mut level = self.clone();
        let player_idx = self.entities
            .iter()
            .position(|e| matches!(e, Some(Entity::Player)))
            .unwrap();
        let next_player_idx = player_idx.wrapping_add(self.player_facing);
        // XXX: not actually game over, but won't be req'd for solving.
        if next_player_idx >= self.tiles.len() {
            return GameState::Lose;
        }
        if let Some(ent) = &self.entities[next_player_idx] {
            match ent {
                Entity::Hand => { level.entities[next_player_idx].take(); }
                Entity::GolemY => { return GameState::Lose; } // doesn't advance the game
                _ => panic!("{ent:?}"),
            }
            if level.is_zap(player_idx) { return GameState::Lose }
            GameState::Play(level)
        } else {
            // XXX: else staff lift, but not in this puzzle.
            GameState::Lose
        }
    }

    fn is_zap(&self, player_idx: usize) -> bool {
        for step in self.directions() {
            let mut idx = player_idx.wrapping_add(step);
            while idx < self.tiles.len() {
                if matches!(self.tiles[idx], Tile::Wall) { break }
                match self.entities[idx] {
                    Some(Entity::CovetousStatue) => return true,
                    Some(_) => break,
                    None => (),
                }
                idx = idx.wrapping_add(step);
            }
        }
        false
    }

    fn directions(&self) -> Vec<usize> {
        vec![
            0_usize.wrapping_sub(self.width),
            usize::MAX, // -1
            1,
            self.width,
        ]
    }

    fn load(level: &str) -> Result<Level> {
        let width = level.lines().map(|line| line.len()).max().unwrap();
        let height = level.lines().count();
        let mut tiles = Vec::with_capacity(width * height);
        let mut entities = vec![None; width * height];
        for line in level.lines() {
            for chr in line.bytes() {
                let tile = match chr {
                    b'#' => Tile::Wall,
                    b' ' => Tile::Hole,
                    b'.' => Tile::Floor,
                    b'+' => Tile::Trap,
                    b'>' => Tile::Exit,
                    _ => {
                        let entity = match chr {
                            b'@' => Entity::Player,
                            b'8' => Entity::CovetousStatue,
                            b'H' => Entity::Hand,
                            b'd' => Entity::GolemY,
                            _ => return Err(format!("unexpected chr '{}'", chr as char)),
                        };
                        entities[tiles.len()] = Some(entity);
                        Tile::Floor
                    }
                };
                tiles.push(tile);
            }
            for _ in 0..width - line.len() {
                tiles.push(Tile::Hole);
            }
        }
        assert_eq!(width * height, tiles.len());
        Ok(Level { width, height, tiles, entities, player_facing: width })
    }
}

fn main() -> Result<()> {
    let level = Level::load(LEVEL)?;
    println!("{level:?}");
    let path = level.solve();
    println!("{path:?}");
    Ok(())
}
