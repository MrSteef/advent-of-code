#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid = make_grid(input)?;
    let dims = get_grid_dims(&grid);
    let accessible_rolls = get_grid_coords(&grid)
        .into_iter()
        .filter(|&coord| has_paper_roll(&grid, coord).unwrap_or(false))
        .filter(|&roll| number_of_neighbouring_rolls(&grid, dims, roll) < 4)
        .count();
    Ok(accessible_rolls.to_string())
}

pub fn make_grid(input: &str) -> miette::Result<Vec<Vec<bool>>> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|cell| match cell {
                    '.' => Ok(false),
                    '@' => Ok(true),
                    other => Err(miette::miette!("unexpected character: {}", other)),
                })
                .collect::<miette::Result<Vec<bool>>>()
        })
        .collect()
}

pub fn has_paper_roll(grid: &Vec<Vec<bool>>, loc: (usize, usize)) -> Option<bool> {
    let (y, x) = loc;
    grid.get(y)
        .and_then(|row| row.get(x))
        .copied()
}

pub fn get_grid_dims(grid: &Vec<Vec<bool>>) -> (usize, usize) {
    let height = grid.len();
    let width = grid.first().map(|row| row.len()).unwrap_or(0);
    (height, width)
}

pub fn get_grid_coords(grid: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let (height, width) = get_grid_dims(grid);
    (0..height).flat_map(|y| (0..width).map(move |x| (y, x))).collect()
}

pub fn get_neighbour_coords(dims: (usize, usize), loc: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    let (y, x) = loc;
    let (height, width) = dims;

    let (dx, dy): (isize, isize) = match direction {
        Direction::North     => ( 0, -1),
        Direction::NorthEast => ( 1, -1),
        Direction::East      => ( 1,  0),
        Direction::SouthEast => ( 1,  1),
        Direction::South     => ( 0,  1),
        Direction::SouthWest => (-1,  1),
        Direction::West      => (-1,  0),
        Direction::NorthWest => (-1, -1),
    };

    let nx = x as isize + dx;
    let ny = y as isize + dy;

    if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
        None
    } else {
        Some((ny as usize, nx as usize))
    }
}

pub fn get_all_neighbours(dims: (usize, usize), loc: (usize, usize)) -> Vec<(usize, usize)> {
    let (y, x) = loc;
    
    DIRECTIONS
        .iter()
        .filter_map(|&direction| get_neighbour_coords(dims, (y, x), direction))
        .collect()
}

pub fn number_of_neighbouring_rolls(grid: &Vec<Vec<bool>>, dims: (usize, usize), loc: (usize, usize)) -> usize {
    get_all_neighbours(dims, loc)
        .into_iter()
        .filter(|&neighbour| has_paper_roll(grid, neighbour).unwrap_or(false))
        .count()
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

const DIRECTIONS: [Direction; 8] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("13", process(input)?);
        Ok(())
    }

    #[test]
    fn test_make_grid() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let grid = make_grid(input)?;
        assert_eq!(grid,
            vec![
                vec![false, false, true, true, false, true, true, true, true, false],
                vec![true, true, true, false, true, false, true, false, true, true],
                vec![true, true, true, true, true, false, true, false, true, true],
                vec![true, false, true, true, true, true, false, false, true, false],
                vec![true, true, false, true, true, true, true, false, true, true],
                vec![false, true, true, true, true, true, true, true, false, true],
                vec![false, true, false, true, false, true, false, true, true, true],
                vec![true, false, true, true, true, false, true, true, true, true],
                vec![false, true, true, true, true, true, true, true, true, false],
                vec![true, false, true, false, true, true, true, false, true, false]
            ]
        );
        Ok(())
    }

    #[test]
    fn test_has_paper_roll() -> miette::Result<()> {
        let input = "@.";
        let grid = make_grid(input)?;
        assert_eq!(has_paper_roll(&grid, (0, 0)), Some(true));
        assert_eq!(has_paper_roll(&grid, (0, 1)), Some(false));
        assert_eq!(has_paper_roll(&grid, (0, 2)), None);
        assert_eq!(has_paper_roll(&grid, (1, 0)), None);
        Ok(())
    }

    #[test]
    fn test_get_grid_dims() -> miette::Result<()> {
        assert_eq!(
            get_grid_dims(&make_grid("@")?), (1,1)
        );
        assert_eq!(
            get_grid_dims(&make_grid("@@")?), (1,2)
        );
        assert_eq!(
            get_grid_dims(&make_grid("@
@")?), (2,1)
        );
        assert_eq!(
            get_grid_dims(&make_grid("@.
.@")?), (2,2)
        );
        Ok(())
    }

    #[test]
    fn test_get_grid_coords() -> miette::Result<()> {
        assert_eq!(
            get_grid_coords(&make_grid("@")?), vec![(0,0)]
        );
        assert_eq!(
            get_grid_coords(&make_grid("@@")?), vec![(0,0), (0,1)]
        );
        assert_eq!(
            get_grid_coords(&make_grid("@
@")?), vec![(0,0), (1,0)]
        );
        assert_eq!(
            get_grid_coords(&make_grid("@.
.@")?), vec![(0,0), (0,1), (1,0), (1,1)]
        );
        Ok(())
    }

        #[test]
    fn test_get_neighbour_coords_center() -> miette::Result<()> {
        let dims = (3, 3);

        // center
        assert_eq!(
            get_neighbour_coords(dims, (1, 1), Direction::North),
            Some((0, 1))
        );
        assert_eq!(
            get_neighbour_coords(dims, (1, 1), Direction::NorthEast),
            Some((0, 2))
        );
        assert_eq!(
            get_neighbour_coords(dims, (1, 1), Direction::East),
            Some((1, 2))
        );
        assert_eq!(
            get_neighbour_coords(dims, (1, 1), Direction::SouthEast),
            Some((2, 2))
        );
        assert_eq!(
            get_neighbour_coords(dims, (1, 1), Direction::South),
            Some((2, 1))
        );
        assert_eq!(
            get_neighbour_coords(dims, (1, 1), Direction::SouthWest),
            Some((2, 0))
        );
        assert_eq!(
            get_neighbour_coords(dims, (1, 1), Direction::West),
            Some((1, 0))
        );
        assert_eq!(
            get_neighbour_coords(dims, (1, 1), Direction::NorthWest),
            Some((0, 0))
        );

        // topleft
        assert_eq!(
            get_neighbour_coords(dims, (0, 0), Direction::North),
            None
        );
        assert_eq!(
            get_neighbour_coords(dims, (0, 0), Direction::West),
            None
        );
        assert_eq!(
            get_neighbour_coords(dims, (0, 0), Direction::NorthWest),
            None
        );
        assert_eq!(
            get_neighbour_coords(dims, (0, 0), Direction::South),
            Some((1, 0))
        );
        assert_eq!(
            get_neighbour_coords(dims, (0, 0), Direction::East),
            Some((0, 1))
        );
        assert_eq!(
            get_neighbour_coords(dims, (0, 0), Direction::SouthEast),
            Some((1, 1))
        );
        // bottomright
        assert_eq!(
            get_neighbour_coords(dims, (2, 2), Direction::South),
            None
        );
        assert_eq!(
            get_neighbour_coords(dims, (2, 2), Direction::East),
            None
        );
        assert_eq!(
            get_neighbour_coords(dims, (2, 2), Direction::SouthEast),
            None
        );
        assert_eq!(
            get_neighbour_coords(dims, (2, 2), Direction::North),
            Some((1, 2))
        );
        assert_eq!(
            get_neighbour_coords(dims, (2, 2), Direction::West),
            Some((2, 1))
        );
        assert_eq!(
            get_neighbour_coords(dims, (2, 2), Direction::NorthWest),
            Some((1, 1))
        );
        Ok(())
    }

        #[test]
    fn test_get_all_neighbours() -> miette::Result<()> {
        let dims = (3, 3);

        assert_eq!(
            get_all_neighbours(dims, (1, 1)),
            vec![
                (0, 1),
                (0, 2),
                (1, 2),
                (2, 2),
                (2, 1),
                (2, 0),
                (1, 0),
                (0, 0),
            ]
        );

        assert_eq!(
            get_all_neighbours(dims, (0, 0)),
            vec![
                (0, 1),
                (1, 1),
                (1, 0),
            ]
        );

        assert_eq!(
            get_all_neighbours(dims, (0, 1)),
            vec![
                (0, 2),
                (1, 2),
                (1, 1),
                (1, 0),
                (0, 0),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_number_of_neighbouring_rolls_3x6_combined() -> miette::Result<()> {
        let input = "@@@@..
@@@.@.
@@@..@";
        let grid = make_grid(input)?;
        let dims = get_grid_dims(&grid);

        assert_eq!(
            number_of_neighbouring_rolls(&grid, dims, (1, 1)),
            8
        );

        assert_eq!(
            number_of_neighbouring_rolls(&grid, dims, (0, 0)),
            3
        );

        assert_eq!(
            number_of_neighbouring_rolls(&grid, dims, (0, 5)),
            1
        );

        assert_eq!(
            number_of_neighbouring_rolls(&grid, dims, (1, 4)),
            2
        );
        Ok(())
    }
}