use crate::part1::find_largest_digit;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    Ok(
        input
        .lines()
        .map(|bank| find_joltage_for_n_batteries(bank, 12))
        .sum::<u64>()
        .to_string()
    )
}

fn find_joltage_for_n_batteries(input: &str, number_of_batteries: u32) -> u64 {
    (0..number_of_batteries)
    .fold((0u64, 0usize), |(joltage, start_index), battery_index| {
        let remaining_batteries = number_of_batteries - battery_index - 1;
        let max_allowed_index = input.len() - (remaining_batteries as usize);
        let search_string = &input[start_index..max_allowed_index];
        let (index, digit) = find_largest_digit(search_string);
        let value = (digit.to_digit(10).unwrap() as u64) * 10_u64.pow(remaining_batteries);

        (joltage + value, start_index + index + 1)
    }).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("3121910778619", process(input)?);
        Ok(())
    }
}
