use std::collections::HashMap;

pub type Pos = (isize, isize);

pub type M<T> = Vec<Vec<T>>;

pub fn straight_neighbours<'a, T>(
    square: &'a M<T>,
    (x, y): Pos,
    is_neighbour: impl Fn(Pos, &T) -> bool + 'a,
) -> impl Iterator<Item = (Pos, &'a T)>
where
    T: Sized + 'a,
{
    straight_neighbour_pos(square, (x, y)).filter_map(move |(x, y)| {
        let v = &square[y as usize][x as usize];
        if is_neighbour((x, y), v) {
            Some(((x, y), v))
        } else {
            None
        }
    })
}

pub fn straight_neighbour_pos<'a, T>(
    square: &'a M<T>,
    (x, y): Pos,
) -> impl Iterator<Item = Pos> + 'a {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .filter_map(move |(dx, dy)| {
            if x == 0 && *dx < 0 || y == 0 && *dy < 0 {
                return None;
            }
            if (x + dx) >= square.len() as isize || (y + dy) >= square[0].len() as isize {
                return None;
            }
            return Some((x + dx, y + dy));
        })
}

pub trait PosGet<T> {
    fn pos_get(&self, pos: Pos) -> T;
}

impl<T> PosGet<T> for M<T>
where
    T: Copy,
{
    fn pos_get(&self, pos: Pos) -> T {
        self[pos.1 as usize][pos.0 as usize]
    }
}

pub trait PosSafeGet<T> {
    fn pos_safe_get(&self, pos: Pos) -> Option<&T>;
}

impl<T> PosSafeGet<T> for M<T> {
    fn pos_safe_get(&self, pos: Pos) -> Option<&T> {
        if pos.1 < 0 || pos.0 < 0 {
            return None;
        }
        self.get(pos.1 as usize)?.get(pos.0 as usize)
    }
}

pub trait PeekStraight<T> {
    fn peek_rel(&self, from: Pos, towards: Pos) -> Option<&T>;

    fn peek_north(&self, pos: Pos) -> Option<&T> {
        self.peek_rel(pos, (0, -1))
    }
    fn peek_south(&self, pos: Pos) -> Option<&T> {
        self.peek_rel(pos, (0, 1))
    }
    fn peek_east(&self, pos: Pos) -> Option<&T> {
        self.peek_rel(pos, (1, 0))
    }
    fn peek_west(&self, pos: Pos) -> Option<&T> {
        self.peek_rel(pos, (-1, 0))
    }
}

impl<B: PosSafeGet<T>, T> PeekStraight<T> for B {
    fn peek_rel(&self, from: Pos, towards: Pos) -> Option<&T> {
        self.pos_safe_get(pos_add::<isize>(from, towards))
    }
}

pub trait PosSet<T> {
    fn pos_set(&mut self, pos: Pos, t: T);
}

impl<T> PosSet<T> for M<T> {
    fn pos_set(&mut self, pos: Pos, t: T) {
        self[pos.1 as usize][pos.0 as usize] = t;
    }
}

pub trait PosFind<T> {
    fn pos_find(&self, val: T) -> Option<Pos>;
}

impl<T: Eq + Copy> PosFind<T> for M<T> {
    fn pos_find(&self, val: T) -> Option<Pos> {
        self.pos_iter()
            .filter(|(_, v)| *v == val)
            .map(|(p, _)| p)
            .next()
    }
}

pub trait PosIter<T> {
    fn pos_iter(&self) -> impl Iterator<Item = (Pos, T)>;
}

impl<T> PosIter<T> for M<T>
where
    T: Copy,
{
    fn pos_iter(&self) -> impl Iterator<Item = (Pos, T)> {
        self.iter().enumerate().flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(move |(j, t)| ((j as isize, i as isize), *t))
        })
    }
}

pub fn pos_add<T: Into<isize>>(a: Pos, b: (T, T)) -> Pos {
    (a.0 + b.0.into(), a.1 + b.1.into())
}

pub fn pos_sub<T: Into<isize>>(a: Pos, b: (T, T)) -> Pos {
    (a.0 - b.0.into(), a.1 - b.1.into())
}

#[derive(Debug, Clone)]
pub struct Counter<T> {
    pub counts: HashMap<T, isize>,
}

impl<T> Counter<T>
where
    T: Eq + core::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn inc(&mut self, t: T) {
        self.add(t, 1)
    }

    pub fn add(&mut self, t: T, a: isize) {
        *self.counts.entry(t).or_default() += a
    }
}
