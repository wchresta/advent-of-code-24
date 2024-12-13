pub type Pos = (isize, isize);

pub fn straight_neighbours<'a, T>(
    square: &'a Vec<Vec<T>>,
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
    square: &'a Vec<Vec<T>>,
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
