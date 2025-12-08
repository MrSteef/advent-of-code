#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (fresh_ranges, available_ingredients) = input
        .split_once("\n\n")
        .unwrap();

    let fresh_ranges = fresh_ranges
        .lines()
        .map(|range| range.split_once('-').unwrap())
        .map(|(start_str, end_str)| {
            let start = start_str.parse::<u64>().unwrap();
            let end = end_str.parse::<u64>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();

    let available_ingredients = available_ingredients
        .lines()
        .map(|ingredient_id| ingredient_id.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let fresh_ingredient_count = available_ingredients
        .iter()
        .filter(|ingredient_id| {
            fresh_ranges
                .iter()
                .any(|fresh_range| fresh_range.contains(&ingredient_id))}
        )
        .count();
        

    // println!("{fresh_ranges:?}");

    // todo!();

    Ok(fresh_ingredient_count.to_string())
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
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
