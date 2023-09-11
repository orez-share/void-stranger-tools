use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry::Vacant;
use std::fmt;
use std::hash::Hash;
use std::iter::{once, zip};

const LEVEL_COVETOUS: &str = "
    # # ###
### # # # #
#7###8###8##
#@...+HH+..#
#...+H.+H.>#
#d..H+..+H.#
############
";

const LEVEL_B005H: &str = "
#####
#...#
#@..#
#+++#
#x+>#
#o̟++#
#.  #
#####
";

const LEVEL_B033H: &str = "
#############
#     .     #
# >  .///.@ #
#     .     #
#####.//#####
    #/#/#
    #//.#
    #####
";

type Error = String;  // XXX: :)
type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
enum GameState {
    Win,
    Lose,
    Noop,
    Play(Level),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Floor,
    Wall,
    Trap,
    PrimedTrap,
    Hole,
    Exit,
    Button,
    Glass,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Entity {
    Player,
    GolemY,
    Hand,
    CovetousStatue,
    DeviousStatue,
    Egg,
}

impl Entity {
    fn is_squishable(&self) -> bool {
        matches!(self, Entity::Hand)
    }

    fn is_stabbable(&self) -> bool {
        matches!(self, Entity::Hand)
    }

    fn is_pushable(&self) -> bool {
        matches!(self, Entity::Egg | Entity::DeviousStatue | Entity::CovetousStatue)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Level {
    width: usize,
    height: usize,
    player_facing: usize,
    void_wings: bool,
    void_blade: bool,
    angry_statues: usize,
    held_tile: Option<Tile>,
    tiles: Vec<Tile>,
    entities: Vec<Option<Entity>>,
}

impl fmt::Debug for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut it = zip(&self.tiles, &self.entities);
        write!(f, "\n")?;
        for _ in 0..self.height {
            for (tile, entity) in it.by_ref().take(self.width) {
                if let Some(e) = entity {
                    write!(f, "{}", match e {
                        Entity::Player => '@',
                        Entity::CovetousStatue => '8',
                        Entity::DeviousStatue => '7',
                        Entity::GolemY => 'd',
                        Entity::Hand => 'H',
                        Entity::Egg => 'o',
                    })?;
                    match tile {
                        Tile::PrimedTrap => write!(f, "̟")?,
                        _ => (),
                    }
                } else {
                    write!(f, "{}", match tile {
                        Tile::Floor => '.',
                        Tile::Wall => '#',
                        Tile::Hole => ' ',
                        Tile::Trap => '+',
                        Tile::PrimedTrap => '!',
                        Tile::Exit => '>',
                        Tile::Button => 'x',
                        Tile::Glass => '/',
                    })?;
                };
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

// Assumes s1 is uniform width, because that's our use case
// and actually checking this with our unicode shenanigans is kind of a pain.
fn inline_multiline_strs(s1: &str, s2: &str) -> String {
    let mut out = String::new();
    for (a, b) in zip(s1.lines(), s2.lines()) {
        out.push_str(a);
        out.push_str("   ");
        out.push_str(b);
        out.push('\n');
    }
    out
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
                    GameState::Lose | GameState::Noop => (),
                    GameState::Play(level) => {
                        if let Vacant(slot) = backpath.entry(level.clone()) {
                            // println!("{}", inline_multiline_strs(&format!("{state:?}"), &format!("{level:?}")));
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
        level.player_facing = step;
        if level.entities[next_player_idx].is_some() {
            if level.push(next_player_idx, step) {
                return GameState::Lose;
            }
        } else {
            match level.tiles[next_player_idx] {
                Tile::Exit => {
                    if !matches!(level.held_tile, Some(Tile::Button)) && zip(&level.entities, &level.tiles)
                            .all(|(e, t)| !matches!(t, Tile::Button)
                            || matches!(e, Some(en) if en != &Entity::Player)) {
                        return GameState::Win;
                    }
                    level.entities[player_idx] = None;
                    level.entities[next_player_idx] = Some(Entity::Player);
                }
                Tile::Floor | Tile::Button => {
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
                    if !self.void_wings || matches!(level.tiles[player_idx], Tile::Hole) {
                        return GameState::Lose;
                    }
                    // XXX: also the BFS effect.. but can't happen in this puzzle.
                    level.tiles[next_player_idx] = Tile::Hole;
                    level.entities[player_idx] = None;
                    level.entities[next_player_idx] = Some(Entity::Player);
                }
                Tile::Hole => {
                    if !self.void_wings || matches!(level.tiles[player_idx], Tile::Hole) {
                        return GameState::Lose;
                    }
                    level.entities[player_idx] = None;
                    level.entities[next_player_idx] = Some(Entity::Player);
                }
                Tile::Glass => {
                    level.entities[player_idx] = None;
                    level.entities[next_player_idx] = Some(Entity::Player);
                    level.tiles[next_player_idx] = Tile::Hole; // XXX: not exactly right, but close enough.
                }
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
                    Tile::Exit | Tile::Floor | Tile::Button => {
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
                    Tile::Glass => {
                        level.tiles[next_eidx] = Tile::Hole;
                        level.entities[eidx] = None;
                        level.entities[next_eidx] = Some(Entity::GolemY);
                    }
                }
                Some(Entity::Player) => { return GameState::Lose }
                Some(_) => { level.push(next_eidx, golem_step); }
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

        if next_player_idx >= self.tiles.len() {
            return GameState::Noop;
        }
        if let Some(ent) = &self.entities[next_player_idx] { // blade kill
            if !level.void_blade { return GameState::Noop }
            match ent {
                Entity::Hand => { level.entities[next_player_idx].take(); }
                Entity::GolemY |
                Entity::DeviousStatue |
                Entity::Egg => { return GameState::Noop; } // doesn't advance the game
                _ => panic!("{ent:?}"),
            }
            if level.is_zap(player_idx) { return GameState::Lose }
            GameState::Play(level)
        } else { // staff lift
            match level.tiles[next_player_idx] {
                Tile::Hole => {
                    let Some(tile) = level.held_tile.take() else { return GameState::Noop };
                    level.tiles[next_player_idx] = tile;
                    if level.alert_devious_statue() {
                        return GameState::Lose;
                    }
                    GameState::Play(level)
                }
                Tile::Wall => GameState::Noop,
                tile => {
                    if level.held_tile.is_some() { return GameState::Noop }
                    if level.alert_devious_statue() {
                        return GameState::Lose;
                    }
                    level.held_tile = Some(tile);
                    level.tiles[next_player_idx] = Tile::Hole;
                    GameState::Play(level)
                }
            }
        }
    }

    // returns if this kills you or not.
    // XXX: this should return an enum, but
    // i dont want to.
    fn push(&mut self, idx: usize, direction: usize) -> bool {
        match self.entities[idx].unwrap() {
            Entity::Player => panic!("???"),
            Entity::Hand | Entity::GolemY => true,
            Entity::CovetousStatue | Entity::DeviousStatue | Entity::Egg => {
                let dest = idx.wrapping_add(direction);
                match self.tiles[dest] {
                    Tile::Wall => (),
                    Tile::Hole => {
                        self.entities[idx].take();
                        if self.angry_statues > 0 { todo!() }
                    }
                    Tile::PrimedTrap if self.entities[dest].is_none() => {
                        todo!();
                    }
                    _ => if self.entities[dest].map_or(true, |e| e.is_squishable()) {
                        let pushed = self.entities[idx].take();
                        self.entities[dest] = pushed;
                    }
                }
                false
            }
        }
    }

    fn alert_devious_statue(&mut self) -> bool {
        let devious_statues = self.entities.iter()
            .filter(|e| matches!(e, Some(Entity::DeviousStatue)))
            .count();
        if devious_statues > 0 {
            self.angry_statues += 1;
            if devious_statues == self.angry_statues {
                return true;
            }
        }
        false
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
        // incorrect if we're doing combining char stuff, but.. maybe it's ok.
        let width = level.lines().map(|line| line.len()).max().unwrap();
        let height = level.lines().count();
        let mut tiles = Vec::with_capacity(width * height);
        let mut entities = vec![None; width * height];

        for line in level.lines() {
            let mut line_it = line.chars().peekable();
            while let Some(chr) = line_it.next() {
                let tile = match chr {
                    '̟' => continue,
                    '#' => Tile::Wall,
                    ' ' => Tile::Hole,
                    '.' => Tile::Floor,
                    '+' => Tile::Trap,
                    'x' => Tile::Button,
                    '>' => Tile::Exit,
                    '/' => Tile::Glass,
                    _ => {
                        let entity = match chr {
                            '@' => Entity::Player,
                            '7' => Entity::DeviousStatue,
                            '8' => Entity::CovetousStatue,
                            'H' => Entity::Hand,
                            'd' => Entity::GolemY,
                            'o' => Entity::Egg,
                            _ => return Err(format!("unexpected chr '{}'", chr as char)),
                        };
                        entities[tiles.len()] = Some(entity);
                        match line_it.peek() {
                            Some('̟') => Tile::PrimedTrap,
                            _ => Tile::Floor,
                        }
                    }
                };
                tiles.push(tile);
            }
            // XXX: this will fail if an entire line is empty.
            let extra = (width - (tiles.len() % width)) % width;
            for _ in 0..extra {
                tiles.push(Tile::Hole);
            }
        }
        // assert_eq!(width * height, tiles.len());
        Ok(Level {
            width, height, tiles, entities, player_facing: width,
            void_blade: false, void_wings: false, angry_statues: 0,
            held_tile: None,
        })
    }

    fn set_wings(&mut self) -> &mut Self {
        self.void_wings = true;
        self
    }

    fn set_blade(&mut self) -> &mut Self {
        self.void_blade = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_covetous() {
        let mut level = Level::load(LEVEL_COVETOUS).unwrap();
        level.set_wings().set_blade();

        let path = level.solve().unwrap();
        assert_eq!(path.len(), 40);
    }

    #[test]
    fn test_b005h() {
        let level = Level::load(LEVEL_B005H).unwrap();
        let path = level.solve().unwrap();
        assert_eq!(path.len(), 10);
    }

    #[test]
    fn test_b033h() {
        let level = Level::load(LEVEL_B033H).unwrap();
        let path = level.solve().unwrap();
        assert_eq!(path.len(), 40);
    }
}

fn main() -> Result<()> {
    let level = Level::load(LEVEL_B033H)?;
    // println!("{level:?}");
    let path = level.solve();
    println!("{path:?}");
    Ok(())
}
