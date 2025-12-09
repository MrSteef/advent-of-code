use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = input
        .lines()
        .map(|line| {
            let (x,y) = line.split_once(',').unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .tuple_combinations()
        .map(|((x1,y1),(x2,y2))| area((x1, y1), (x2, y2)))
        // .sorted_by(|a, b| b.cmp(&a))
        .max()
        .unwrap();

    Ok(result.to_string())
}

pub fn area((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
    ((x1.max(x2) + 1) - x1.min(x2)) * ((y1.max(y2) + 1) - y1.min(y2))
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
        assert_eq!("50", process(input)?);
        Ok(())
    }
}
