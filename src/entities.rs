#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Entity {
    Player,
    Golem,
    GolemX,
    GolemY,
    GolemXY,
    Hand,
    CovetousStatue,
    DeviousStatue,
    RagingStatue,
    DarkIdol,
    Egg,
    Bull(usize),
}

impl Entity {
    pub fn is_enemy(&self) -> bool {
        use Entity::*;
        matches!(self, Golem | GolemX | GolemY | GolemXY | Hand | Bull(_))
    }

    pub fn is_squishable(&self) -> bool {
        matches!(self, Entity::Hand | Entity::Bull(_))
    }

    pub fn is_stabbable(&self) -> bool {
        matches!(self, Entity::Hand | Entity::Bull(_))
    }

    pub fn is_pushable(&self) -> bool {
        use Entity::*;
        matches!(self, Egg | DeviousStatue | CovetousStatue | DarkIdol | RagingStatue)
    }
}

// ===

/// Opaque type representing a
pub struct LiftedEntity<'a> {
    entities: &'a mut Entities,
    roster_idx: usize,
    entity: Entity,
}

impl LiftedEntity<'_> {
    fn place(self, grid_idx: usize) {
        let LiftedEntity { entities, roster_idx, entity } = self;
        entities.grid[grid_idx] = Some((roster_idx, entity));
        entities.roster[roster_idx] = Some(grid_idx);
    }
}

// ===

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Entities {
    grid: Vec<Option<(usize, Entity)>>,
    roster: Vec<Option<usize>>,
}

impl Entities {
    pub fn new(size: usize) -> Self {
        Entities {
            grid: vec![None; size],
            roster: Vec::new(),
        }
    }

    pub fn add(&mut self, grid_idx: usize, entity: Entity) {
        // validate_entities(&self, "add - before");
        let roster_idx = self.roster.len();
        self.roster.push(Some(grid_idx));
        assert!(self.grid[grid_idx].is_none());
        self.grid[grid_idx] = Some((roster_idx, entity));
        // validate_entities(&self, "add - after");
    }

    pub fn lift(&mut self, grid_idx: usize) -> LiftedEntity {
        // validate_entities(&self, "lift - before");
        let (roster_idx, entity) = self.grid[grid_idx].take().expect("spot at `grid_idx` should have entity");
        self.roster[roster_idx] = None;
        // validate_entities(&self, "lift - after");
        LiftedEntity { entities: self, roster_idx, entity }
    }

    pub fn position<P: Fn(Entity) -> bool>(&self, pred: P) -> Option<usize> {
        self.roster.iter().flatten().copied().find(|&grid_idx| {
            let (_, entity) = self.grid[grid_idx].expect("`roster` entries should appear in `grid`");
            pred(entity)
        })
    }

    pub fn iter<'a>(&'a self) -> EntityIter<'a> {
        (&self).into_iter()
    }

    pub fn step(&mut self, from: usize, to: usize) {
        if from == to { return }
        // validate_entities(&self, "step - before");
        if self.grid[to].is_some() {
            self.lift(to);
        }
        let (roster_idx, entity) = self.grid[from].take()
            .expect("an entity at `from` (found None)");
        self.grid[to] = Some((roster_idx, entity));
        self.roster[roster_idx] = Some(to);
        // validate_entities(&self, "step - after");
    }

    pub fn iter_grid(&self) -> impl Iterator<Item=Option<Entity>> + '_ {
        self.grid.iter().map(|pair| pair.map(|(_, e)| e))
    }

    pub fn at(&self, idx: usize) -> Option<Entity> {
        Some(self.grid[idx]?.1)
    }

    pub fn retain<F: Fn(Entity) -> bool>(&mut self, pred: F) {
        // validate_entities(&self, "retain - before");
        for roster_idx in 0..self.roster.len() {
            let Some(grid_idx) = self.roster[roster_idx] else { continue };
            let (_, entity) = self.grid[grid_idx].expect("`roster` entries should appear in `grid`");
            if !pred(entity) {
                self.lift(grid_idx);
            }
        }
        // validate_entities(&self, "retain - after");
    }
}

fn validate_entities(es: &Entities, context: &str) {
    es.roster.iter().copied().enumerate()
        .filter_map(|(r, roster_elem)| roster_elem.map(|e| (r, e)))
        .for_each(|(roster_idx, grid_idx)| {
            assert_eq!(es.grid[grid_idx].unwrap().0, roster_idx, "roster elem mismatch grid {context:?}")
        });
    es.grid.iter().enumerate()
        .filter_map(|(g, grid_elem)| grid_elem.map(|e| (g, e.0)))
        .for_each(|(grid_idx, roster_idx)|
            assert_eq!(es.roster[roster_idx].unwrap(), grid_idx, "grid elem mismatch roster {context:?}"));
}

impl<'a> IntoIterator for &'a Entities {
    type Item = (usize, Entity);
    type IntoIter = EntityIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        // self.roster.iter().flatten().copied()
        //     .map(|idx| {
        //         let entity = self.grid[idx].expect("`roster` entries should appear in `grid`");
        //         (idx, entity)
        //     })
        EntityIter { entities: self, roster_idx: 0 }
    }
}

// ===

// XXX: praying every day for stable impl Trait in type aliases.
// https://github.com/rust-lang/rust/issues/63063
pub struct EntityIter<'a> {
    entities: &'a Entities,
    roster_idx: usize,
}

impl Iterator for EntityIter<'_> {
    type Item = (usize, Entity);

    fn next(&mut self) -> Option<Self::Item> {
        let mut idx = None;
        while idx.is_none() {
            idx = *self.entities.roster.get(self.roster_idx)?;
            self.roster_idx += 1;
        }
        let idx = idx.unwrap();
        let (_, entity) = self.entities.grid[idx]
            .expect("`roster` entries should appear in `grid`");
        Some((idx, entity))
    }
}
