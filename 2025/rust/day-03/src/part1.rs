#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    Ok(
        input
        .lines()
        .map(|bank| find_maximum_bank_joltage(bank))
        .sum::<u32>()
        .to_string()
    )
}

fn find_maximum_bank_joltage(input: &str) -> u32 {
    let input_length = input.len();
    let (first_digit_index, first_digit) = find_largest_digit(&input[..input_length-1]);
    let (_second_digit_index, second_digit) = find_largest_digit(&input[first_digit_index+1..input_length]);
    let first_digit_u32 = first_digit.to_digit(10).unwrap();
    let second_digit_u32 = second_digit.to_digit(10).unwrap();
    let number = first_digit_u32 * 10 + second_digit_u32;
    number
}

pub fn find_largest_digit(input: &str) -> (usize, char) {
    input
        .char_indices()
        .fold((0, '0'), |(highest_index, highest_value), (current_index, current_value)| {
            match current_value.cmp(&highest_value) {
                std::cmp::Ordering::Less => (highest_index, highest_value),
                std::cmp::Ordering::Equal => (highest_index, highest_value),
                std::cmp::Ordering::Greater => (current_index, current_value),
            }
        })
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
        assert_eq!("357", process(input)?);
        Ok(())
    }

    #[test]
    fn test_find_maximum_bank_joltage() {
        assert_eq!(find_maximum_bank_joltage("01234"), 34);
        assert_eq!(find_maximum_bank_joltage("50123"), 53);
        assert_eq!(find_maximum_bank_joltage("12321"), 32);
    }

    #[test]
    fn test_find_largest_digit() {
        assert_eq!(find_largest_digit("01234"), (4, '4'));
        assert_eq!(find_largest_digit("50123"), (0, '5'));
        assert_eq!(find_largest_digit("12321"), (2, '3'));
    }
}
