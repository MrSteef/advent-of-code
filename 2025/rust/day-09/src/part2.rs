use std::str::FromStr;

use itertools::Itertools;
use miette::{IntoDiagnostic, miette};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let corners = input
        .lines()
        .map(Tile::from_str)
        .collect::<miette::Result<Vec<_>>>()?;
    let all_xs = corners
        .iter()
        .map(|corner| corner.x)
        .unique()
        .sorted()
        .collect::<Vec<_>>();
    let width = all_xs.len();
    let all_ys = corners
        .iter()
        .map(|corner| corner.y)
        .unique()
        .sorted()
        .collect::<Vec<_>>();
    let height = all_ys.len();
    let reduced_corners = corners
        .iter()
        .map(|tile| tile.reduce(&all_xs, &all_ys))
        .collect::<miette::Result<Vec<_>>>()?;
    let mut outline =
        vec![vec![false; height]; width];

    // set the outline to true
    reduced_corners.iter().circular_tuple_windows().for_each(
        |(
            &Tile { x: x1, y: y1 },
            &Tile { x: x2, y: y2 },
        )| {
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    outline[x1][y] = true;
                }
            } else if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    outline[x][y1] = true;
                }
            }
        },
    );

    let mut grid =
        vec![vec![false; height]; width];
    
    // set the inside to true
    // this logic does not generalize to any shape, but it works here because the shape is simple enough
    for x in 0..width {
        for y in 0..height {
            if outline[x][y] == true {
                grid[x][y] = true;
                continue
            }
            
            if (0..x)
                .map(|raycast_x| outline[raycast_x][y])
                .dedup()
                .filter(|&t| t)
                .count()
                == 1 {
                    grid[x][y] = true;
                    continue;
                }

            if (0..y)
                .map(|raycast_y| outline[x][raycast_y])
                .dedup()
                .filter(|&t| t)
                .count()
                == 1 {
                    grid[x][y] = true;
                    continue;
                }

            if (x..width)
                .map(|raycast_x| outline[raycast_x][y])
                .dedup()
                .filter(|&t| t)
                .count()
                == 1 {
                    grid[x][y] = true;
                    continue;
                }

            if (y..height)
                .map(|raycast_y| outline[x][raycast_y])
                .dedup()
                .filter(|&t| t)
                .count()
                == 1 {
                    grid[x][y] = true;
                    continue;
                }
            
            // grid[x][y] = false;
        }
    }

    // for y in 0..height {
    //     for x in 0..width {
    //         print!("{}", if grid[x][y] {"x"} else {"."});
    //     }
    //     println!("");
    // }

    let rect = corners
        .iter()
        .tuple_combinations()
        .map(|(&tile_a, &tile_b)| Rect::new(tile_a, tile_b))
        .sorted_by(|a, b| b.size().cmp(&a.size()))
        .map(|rect| rect.reduce(&all_xs, &all_ys).unwrap())
        .find(|rect| rect.inner_tiles().all(|tile| grid[tile.x][tile.y]))
        .unwrap();

    let left = all_xs[rect.left];
    let right = all_xs[rect.right];
    let top = all_ys[rect.top];
    let bottom = all_ys[rect.bottom];

    let size = (right + 1 - left) * (bottom + 1 - top);

    Ok(size.to_string())
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Tile {
    x: usize,
    y: usize,
}

impl Tile {
    fn new(x: usize, y: usize) -> Self {
        Tile { x, y }
    }

    fn reduce(
        self,
        xs: &[usize],
        ys: &[usize],
    ) -> miette::Result<Tile> {
        let (x, _) = xs
            .iter()
            .find_position(|&&n| n == self.x)
            .ok_or(miette!("could not find x coordinate in list of all xs"))?;
        let (y, _) = ys
            .iter()
            .find_position(|&&n| n == self.y)
            .ok_or(miette!("could not find y coordinate in list of all ys"))?;
        Ok(Tile { x, y })
    }
}

impl FromStr for Tile {
    type Err = miette::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or(miette!("missing comma in string"))?;
        let x = x.parse().into_diagnostic()?;
        let y = y.parse().into_diagnostic()?;
        Ok(Tile::new(x, y))
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Rect {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl Rect {
    fn new(a: Tile, b: Tile) -> Self {
        let top = a.y.min(b.y);
        let bottom = a.y.max(b.y);
        let left = a.x.min(b.x);
        let right = a.x.max(b.x);
        Rect {
            top,
            bottom,
            left,
            right,
        }
    }

    fn size(&self) -> usize {
        (self.right + 1 - self.left)
            * (self.bottom + 1 - self.top)
    }

    fn inner_tiles(&self) -> impl Iterator<Item = Tile> {
        (self.top..=self.bottom)
            .flat_map(|y| 
                (self.left..=self.right).map(move |x| Tile::new(x, y))
            )
    }

    fn reduce(
        self,
        xs: &[usize],
        ys: &[usize],
    ) -> miette::Result<Rect> {
        let (top, _) = ys
            .iter()
            .find_position(|&&n| n == self.top)
            .ok_or(miette!("could not find top coordinate in list of all ys"))?;
        let (bottom, _) = ys
            .iter()
            .find_position(|&&n| n == self.bottom)
            .ok_or(miette!("could not find bottom coordinate in list of all ys"))?;
        let (left, _) = xs
            .iter()
            .find_position(|&&n| n == self.left)
            .ok_or(miette!("could not find left coordinate in list of all xs"))?;
        let (right, _) = xs
            .iter()
            .find_position(|&&n| n == self.right)
            .ok_or(miette!("could not find right coordinate in list of all xs"))?;
        Ok(Rect {
            top,
            bottom,
            left,
            right
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!("24", process(input)?);
        Ok(())
    }
}
