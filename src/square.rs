pub type Pos = (usize, usize);

pub fn straight_neighbours<'a, T>(
    square: &'a Vec<Vec<T>>,
    (x, y): Pos,
    is_neighbour: impl Fn(Pos, &T) -> bool + 'a,
) -> impl Iterator<Item = (Pos, &'a T)>
where
    T: Sized + 'a,
{
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .filter_map(move |(dx, dy)| {
            let (x2, y2) = (x as isize + dx, y as isize + dy);
            if x2.is_negative() || y2.is_negative() {
                return None;
            }
            let pos = (x2 as usize, y2 as usize);
            let v = square.get(pos.1)?.get(pos.0)?;
            if is_neighbour(pos, v) {
                Some((pos, v))
            } else {
                None
            }
        })
}
