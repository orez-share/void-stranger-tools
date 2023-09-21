mod entities;

use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry::Vacant;
use std::fmt;
use std::hash::Hash;
use std::iter::{once, zip};
use entities::{Entity, Entities};

type Error = String;  // XXX: :)
type Result<T> = core::result::Result<T, Error>;

type GameState<L> = core::result::Result<L, GameOver>;
const Continue: GameState<()> = Ok(());

#[derive(Debug, Clone)]
enum GameOver {
    Win,
    Lose,
    Noop,
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
    Chest,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Level {
    width: usize,
    height: usize,
    player_facing: usize,
    void_wings: bool,
    void_blade: bool,
    must_get_chests: bool,
    // XXX: `golem_pacifist` is a weird thing to codify.
    // as we come up with more cases we want to support, we may
    // rework this. For now though, this serves our use cases.
    golem_pacifist: bool,
    angry_statues: usize,
    held_tile: Option<Tile>,
    tiles: Vec<Tile>,
    entities: Entities,
}

impl fmt::Debug for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut it = zip(&self.tiles, self.entities.iter_grid());
        write!(f, "\n")?;
        for _ in 0..self.height {
            for (tile, entity) in it.by_ref().take(self.width) {
                if let Some(e) = entity {
                    write!(f, "{}", match e {
                        Entity::Player => '@',
                        Entity::CovetousStatue => '8',
                        Entity::DeviousStatue => '7',
                        Entity::RagingStatue => '5',
                        Entity::DarkIdol => '4',
                        Entity::Golem => 'q',
                        Entity::GolemX => 'p',
                        Entity::GolemY => 'd',
                        Entity::GolemXY => 'b',
                        Entity::Hand => 'H',
                        Entity::Egg => 'o',
                        Entity::Bull(_) => 'K',
                    })?;
                    match tile {
                        Tile::PrimedTrap => write!(f, "̟")?,
                        Tile::Exit => write!(f, "͕")?,
                        Tile::Button => write!(f, "͓")?,
                        Tile::Glass => write!(f, "̭")?,
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
                        Tile::Glass => '^',
                        Tile::Chest => ')',
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
                    Err(GameOver::Win) => return Ok(Level::reconstruct_path(backpath, state)),
                    Err(GameOver::Lose | GameOver::Noop) => (),
                    Ok(level) => {
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

    fn next_states(&self) -> Vec<GameState<Level>> {
        vec![
            self.walk(self.up()),
            self.walk(usize::MAX), // -1
            self.walk(1),
            self.walk(self.down()),
            self.act(),
        ]
    }

    fn walk(&self, step: usize) -> GameState<Level> {
        let mut level = self.clone();
        let player_idx = self.entities
            .position(|e| matches!(e, Entity::Player))
            .unwrap();
        let mut next_player_idx = player_idx.wrapping_add(step);
        if next_player_idx >= self.tiles.len() { unimplemented!("OOB"); }
        level.player_facing = step;
        if level.entities.at(next_player_idx).is_some() {
            level.push_entity(next_player_idx, step, true)?;
            next_player_idx = player_idx;
        } else {
            match level.tiles[next_player_idx] {
                Tile::Exit => {
                    // move player _before_ checking the escape.
                    // this ensures the player can't escape by holding their own button
                    level.entities.step(player_idx, next_player_idx);
                    level.check_escape(next_player_idx)?;
                }
                Tile::Floor | Tile::Button => {
                    level.entities.step(player_idx, next_player_idx);
                }
                Tile::Wall | Tile::Chest => if matches!(level.tiles[player_idx], Tile::Hole) {
                    return Err(GameOver::Lose);
                },
                Tile::Trap => {
                    level.tiles[next_player_idx] = Tile::PrimedTrap;
                    level.entities.step(player_idx, next_player_idx);
                }
                Tile::PrimedTrap => {
                    if !self.void_wings || matches!(level.tiles[player_idx], Tile::Hole) {
                        return Err(GameOver::Lose);
                    }
                    level.entities.step(player_idx, next_player_idx);
                    level.drop_primed_traps(next_player_idx)?;
                }
                Tile::Hole => {
                    if !self.void_wings || matches!(level.tiles[player_idx], Tile::Hole) {
                        return Err(GameOver::Lose);
                    }
                    level.entities.step(player_idx, next_player_idx);
                }
                Tile::Glass => {
                    level.entities.step(player_idx, next_player_idx);
                    level.tiles[next_player_idx] = Tile::Hole; // XXX: not exactly right, but close enough.
                }
            }
        }

        level.is_zap(next_player_idx)?;
        // XXX: enemy logic could be very complicated.
        // in particular, I don't know how turn order necessarily works out.
        // we'll do our best for now.
        for (eidx, entity) in &self.entities {
            match entity {
                Entity::Golem | Entity::GolemX |
                Entity::GolemY | Entity::GolemXY => level.golem_step(eidx, step)?,
                _ => continue
            }
        }

        // raging check
        if !level.entities.iter().any(|(_, e)| e.is_enemy()) {
            level.entities.retain(|e| !matches!(e, Entity::RagingStatue));
        }

        level.check_escape(next_player_idx)?;

        // covetous check
        level.is_zap(next_player_idx)?;
        Ok(level)
    }

    fn act(&self) -> GameState<Level> {
        let mut level = self.clone();
        let player_idx = self.entities
            .position(|e| matches!(e, Entity::Player))
            .unwrap();
        let next_player_idx = player_idx.wrapping_add(self.player_facing);

        if next_player_idx >= self.tiles.len() {
            return Err(GameOver::Noop);
        }
        if let Some(ent) = &self.entities.at(next_player_idx) { // blade kill
            if !level.void_blade { return Err(GameOver::Noop) }
            match ent {
                Entity::Hand => { level.entities.lift(next_player_idx); }
                Entity::GolemY |
                Entity::DeviousStatue |
                Entity::Egg |
                Entity::DarkIdol |
                Entity::RagingStatue => { return Err(GameOver::Noop); } // doesn't advance the game
                _ => panic!("{ent:?}"),
            }
            level.is_zap(player_idx)?;
            Ok(level)
        } else { // staff lift
            match level.tiles[next_player_idx] {
                Tile::Hole => {
                    let Some(tile) = level.held_tile.take() else { return Err(GameOver::Noop) };
                    level.tiles[next_player_idx] = tile;
                    if level.alert_devious_statue() {
                        return Err(GameOver::Lose);
                    }
                    Ok(level)
                }
                Tile::Wall => Err(GameOver::Noop),
                Tile::Chest => if level.player_facing == level.up() {
                    // open the chest!
                    level.player_facing = level.down();
                    level.tiles[next_player_idx] = Tile::Wall;
                    Ok(level)
                } else { Err(GameOver::Noop) }
                tile => {
                    if level.held_tile.is_some() { return Err(GameOver::Noop )}
                    if level.alert_devious_statue() {
                        return Err(GameOver::Lose);
                    }
                    level.held_tile = Some(tile);
                    level.tiles[next_player_idx] = Tile::Hole;
                    Ok(level)
                }
            }
        }
    }

    // XXX: `is_player` flag is kinda gross but ᖍ(∙⟞∙)ᖌ
    fn push_entity(&mut self, idx: usize, direction: usize, is_player: bool) -> GameState<()> {
        match self.entities.at(idx).unwrap() {
            Entity::Player => panic!("???"),
            Entity::Hand | Entity::Golem | Entity::GolemX | Entity::GolemY | Entity::GolemXY | Entity::Bull(_) => {
                if is_player { Err(GameOver::Lose) }
                else { Continue }
            }
            Entity::CovetousStatue |
            Entity::DeviousStatue |
            Entity::Egg |
            Entity::DarkIdol |
            Entity::RagingStatue => {
                let dest = idx.wrapping_add(direction);
                let can_squish = self.entities.at(dest).map_or(true, |e| e.is_squishable());
                match self.tiles[dest] {
                    Tile::Wall => (),
                    Tile::Hole => {
                        self.entities.lift(idx);
                        if self.angry_statues > 0 { todo!() }
                    }
                    Tile::PrimedTrap => if can_squish {
                        self.entities.step(idx, dest);
                        self.drop_primed_traps(dest)?;
                    }
                    _ => if can_squish {
                        self.entities.step(idx, dest);
                    }
                }
                Continue
            }
        }
    }

    fn alert_devious_statue(&mut self) -> bool {
        let devious_statues = self.entities.iter()
            .filter(|(_, e)| matches!(e, Entity::DeviousStatue))
            .count();
        if devious_statues > 0 {
            self.angry_statues += 1;
            if devious_statues == self.angry_statues {
                return true;
            }
        }
        false
    }

    fn check_escape(&self, player_idx: usize) -> GameState<()> {
        if matches!(self.tiles[player_idx], Tile::Exit) &&
            !matches!(self.held_tile, Some(Tile::Button)) &&
            zip(self.entities.iter_grid(), &self.tiles)
                .all(|(e, t)| !matches!(t, Tile::Button) || e.is_some()) {
            // you win! ...unless there's a dark idol lookin at you,
            // in which case you Very Lose.
            if matches!(self.entities.at(player_idx - self.width), Some(Entity::DarkIdol)) {
                return Err(GameOver::Lose);
            }
            // ...or if you're supposed to get the chests, and you did not.
            if self.must_get_chests && self.tiles.iter().any(|t| matches!(t, Tile::Chest)) {
                return Err(GameOver::Lose);
            }
            return Err(GameOver::Win);
        }
        Ok(())
    }

    fn is_zap(&self, player_idx: usize) -> GameState<()> {
        for step in self.directions() {
            let mut idx = player_idx.wrapping_add(step);
            while idx < self.tiles.len() {
                if matches!(self.tiles[idx], Tile::Wall) { break }
                match self.entities.at(idx) {
                    Some(Entity::CovetousStatue) => return Err(GameOver::Lose),
                    Some(_) => break,
                    None => (),
                }
                idx = idx.wrapping_add(step);
            }
        }
        Continue
    }

    fn golem_step(&mut self, eidx: usize, step: usize) -> GameState<()> {
        let Some(entity) = self.entities.at(eidx) else {
            panic!("expected golem to golemstep");
        };
        let mob_step = match entity {
            Entity::Golem => step,
            Entity::GolemX => match step {
                1 | usize::MAX => 0_usize.wrapping_sub(step),
                _ => step,
            }
            Entity::GolemY => match step {
                1 | usize::MAX => step,
                _ => 0_usize.wrapping_sub(step),
            }
            Entity::GolemXY => 0_usize.wrapping_sub(step),
            _ => panic!("expected golem to golem_step"),
        };
        let next_eidx = eidx.wrapping_add(mob_step);
        // XXX: boundscheck
        match self.entities.at(next_eidx) {
            None => match self.tiles[next_eidx] {
                Tile::Exit | Tile::Floor | Tile::Button => {
                    self.entities.step(eidx, next_eidx);
                }
                Tile::Wall | Tile::Chest => (),
                Tile::Hole | Tile::PrimedTrap => {
                    // we're already struggling with performance.
                    // on many levels, it's clear that this case means Game Over.
                    // XXX: maybe also do some precompute to try to identify these.
                    if self.golem_pacifist {
                        return Err(GameOver::Lose);
                    }
                    self.entities.lift(eidx);
                }
                Tile::Trap => {
                    self.tiles[next_eidx] = Tile::PrimedTrap;
                    self.entities.step(eidx, next_eidx);
                }
                Tile::Glass => {
                    self.tiles[next_eidx] = Tile::Hole;
                    self.entities.step(eidx, next_eidx);
                }
            }
            Some(Entity::Player) => { return Err(GameOver::Lose) }
            Some(_) => { self.push_entity(next_eidx, mob_step, false)?; }
        }
        Continue
    }

    fn up(&self) -> usize {
        0_usize.wrapping_sub(self.width)
    }

    fn down(&self) -> usize {
        self.width
    }

    fn directions(&self) -> Vec<usize> {
        vec![
            self.up(),
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
        let mut entities = Entities::new(width * height);

        let mut player_position = None;
        for line in level.lines() {
            let mut line_it = line.chars().peekable();
            while let Some(chr) = line_it.next() {
                let tile = match chr {
                    '̟' | '͕' | '͓' | '̭' => continue,
                    '#' => Tile::Wall,
                    ' ' => Tile::Hole,
                    '.' => Tile::Floor,
                    '+' => Tile::Trap,
                    'x' => Tile::Button,
                    '>' => Tile::Exit,
                    '^' => Tile::Glass,
                    ')' => Tile::Chest,
                    _ => {
                        let entity = match chr {
                            '@' => {
                                if let Some(pos) = player_position {
                                    return Err(format!(
                                        "expected exactly one player (found one at {pos} and one at {}",
                                        tiles.len()
                                    ));
                                }
                                player_position = Some(tiles.len());
                                Entity::Player
                            }
                            '4' => Entity::DarkIdol,
                            '5' => Entity::RagingStatue,
                            '7' => Entity::DeviousStatue,
                            '8' => Entity::CovetousStatue,
                            'H' => Entity::Hand,
                            'q' => Entity::Golem,
                            'p' => Entity::GolemX,
                            'd' => Entity::GolemY,
                            'b' => Entity::GolemXY,
                            'o' => Entity::Egg,
                            'K' => Entity::Bull(0),
                            _ => return Err(format!("unexpected chr '{}'", chr as char)),
                        };
                        entities.add(tiles.len(), entity);
                        match line_it.peek() {
                            Some('̟') => Tile::PrimedTrap,
                            Some('͕') => Tile::Exit,
                            Some('͓') => Tile::Button,
                            Some('̭') => Tile::Glass,
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
        if player_position.is_none() {
            return Err("expected exactly one player (found none)".into());
        }
        // assert_eq!(width * height, tiles.len());
        Ok(Level {
            width, height, tiles, entities, player_facing: width,
            void_blade: false, void_wings: false, must_get_chests: false, golem_pacifist: false,
            angry_statues: 0, held_tile: None,
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

    fn must_get_chests(&mut self) -> &mut Self {
        self.must_get_chests = true;
        self
    }

    fn set_golem_pacifist(&mut self) -> &mut Self {
        self.golem_pacifist = true;
        self
    }

    fn drop_primed_traps(&mut self, start: usize) -> GameState<()> {
        // XXX: skipping bounds check for now. in particular LR wrap is insidious
        let mut frontier = vec![start];
        while let Some(spot) = frontier.pop() {
            if !matches!(self.tiles[spot], Tile::PrimedTrap) { continue }
            self.tiles[spot] = Tile::Hole;
            let Some(entity) = self.entities.at(spot) else { continue };
            match entity {
                Entity::Player => if !self.void_wings { return Err(GameOver::Lose) }
                _ => { self.entities.lift(spot); }
            }
            for step in self.directions() {
                let next = spot.wrapping_add(step);
                // XXX: for performance, maybe keep a seen set and don't add dups to the frontier.
                frontier.push(next);
            }
        }
        Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_covetous() {
        let skb = include_str!("levels/covetous.skb");
        let mut level = Level::load(skb).unwrap();
        level.set_wings().set_blade().set_golem_pacifist();
        let path = level.solve().unwrap();
        assert_eq!(path.len(), 40);
    }

    #[test]
    fn test_b005h() {
        let skb = include_str!("levels/b005h.skb");
        let level = Level::load(skb).unwrap();
        let path = level.solve().unwrap();
        assert_eq!(path.len(), 10);
    }

    #[test]
    fn test_b033h() {
        let skb = include_str!("levels/b033h.skb");
        let level = Level::load(skb).unwrap();
        let path = level.solve().unwrap();
        assert_eq!(path.len(), 40);
    }

    // #[test]
    // fn test_b062h() {
    //     let skb = include_str!("levels/b062h.skb");
    //     let level = Level::load(skb).unwrap();
    //     let path = level.solve().unwrap();
    //     assert_eq!(path.len(), 40);
    // }

    #[test]
    fn test_b072h() {
        // just the first part of the level
        let skb = include_str!("levels/b072h.skb");
        let level = Level::load(skb).unwrap();
        let path = level.solve().unwrap();
        assert_eq!(path.len(), 25);
    }

    #[test]
    fn test_b074h() {
        let skb = include_str!("levels/b074h.skb");
        let level = Level::load(skb).unwrap();
        let path = level.solve().unwrap();
        assert_eq!(path.len(), 17);
    }

    // #[test]
    // fn test_b075h() {
    //     let skb = include_str!("levels/b075h.skb");
    //     let level = Level::load(skb).unwrap();
    //     let path = level.solve().unwrap();
    //     assert_eq!(path.len(), 17);
    // }

    #[test]
    fn test_b149h() {
        let skb = include_str!("levels/b149h.skb");
        let level = Level::load(skb).unwrap();
        let path = level.solve().unwrap();
        assert_eq!(path.len(), 19);
    }

    #[test]
    fn test_b165h() {
        let skb = include_str!("levels/b165h.skb");
        let level = Level::load(skb).unwrap();
        let path = level.solve().unwrap();
        // DSSSAWWZSZS
        assert_eq!(path.len(), 11);
    }

    #[test]
    fn test_b166h() {
        let skb = include_str!("levels/b166h.skb");
        let level = Level::load(skb).unwrap();
        let path = level.solve().unwrap();
        assert_eq!(path.len(), 7);
    }

    #[test]
    fn test_b166h_chest() {
        let skb = include_str!("levels/b166h.skb");
        let mut level = Level::load(skb).unwrap();
        level.must_get_chests();
        let path = level.solve().unwrap();
        assert_eq!(path.len(), 8);
    }

    #[test]
    fn test_statue_exit_bonk() {
        let skb = "######\n#@5͕5H#\n######";
        let level = Level::load(skb).unwrap();
        let path = level.solve();
        assert!(path.is_err());
    }
}

fn play(mut level: Level, commands: &str) -> GameState<Level> {
    for c in commands.chars() {
        println!("{level:?}");
        level = match c {
            'W' => level.walk(level.up())?,
            'A' => level.walk(usize::MAX)?,
            'S' => level.walk(level.down())?,
            'D' => level.walk(1)?,
            'Z' => level.act()?,
            _ => panic!("unexpected char: '{c}'"),
        };
    }
    println!("{level:?}");
    Ok(level)
}

fn main() -> Result<()> {
    let skb = include_str!("levels/b149h.skb");
    let mut level = Level::load(skb)?;
    level.must_get_chests();
    // match play(level, "DSSSAWDAWZSZS") {
    //     Err(GameOver::Win) => println!("Win!"),
    //     e @ Err(_) => return Err(format!("{e:?}")),
    //     Ok(_) => (),
    // }
    println!("{level:?}");
    let path = level.solve();
    println!("{path:?}");
    Ok(())
}
