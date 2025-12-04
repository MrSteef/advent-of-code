use crate::part1::*;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut grid = make_grid(input)?;
    let dims = get_grid_dims(&grid);
    let mut removed_rolls = 0;

    loop {
        let accessible_rolls: Vec<(usize, usize)> = get_grid_coords(&grid)
            .into_iter()
            .filter(|&coord| has_paper_roll(&grid, coord).unwrap_or(false))
            .filter(|&roll| number_of_neighbouring_rolls(&grid, dims, roll) < 4)
            .collect();

        let number_of_rolls = accessible_rolls.len();
        if number_of_rolls == 0 {
            break;
        }
        removed_rolls += number_of_rolls;

        accessible_rolls.into_iter().for_each(|roll| {
            let (y, x) = roll;
            grid[y][x] = false;
        });
    }

    Ok(removed_rolls.to_string())
}



#[cfg(test)]
mod tests {
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
        assert_eq!("43", process(input)?);
        Ok(())
    }
}
