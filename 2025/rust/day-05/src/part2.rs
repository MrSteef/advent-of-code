use std::cmp::Ordering;

use itertools::Itertools;

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    Ok(input
        .split_once("\n\n")
        .ok_or_else(|| miette::miette!("invalid input"))?
        .0
        .lines()
        .map(|range| range.split_once('-').unwrap())
        .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
        .sorted_by(|(a_start, _a_end), (b_start, _b_end)| Ord::cmp(&a_start, &b_start))
        .fold((0, None::<u64>), |(fresh_id_count, current_max_id), (start, end)| {
            let Some(highest_seen_id) = current_max_id else {
                return (end-start+1,Some(end))
            };
            match ( start.cmp(&highest_seen_id), end.cmp(&highest_seen_id) ) {
                (Ordering::Greater, _) => (fresh_id_count + (end-       start       +1), Some(end)),
                (_, Ordering::Greater) => (fresh_id_count + (end-(highest_seen_id+1)+1), Some(end)),
                (_, _                ) => (fresh_id_count, Some(highest_seen_id))
            }
        })
        .0
        .to_string()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
