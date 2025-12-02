#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    Ok(
        list_product_ids(input.trim())
            .iter()
            .filter_map(|id| find_repeating_digits(id))
            .map(|digits| digits.parse::<u64>().unwrap())
            .sum::<u64>()
            .to_string()
    )
}

fn list_product_ids(input: &str) -> Vec<String> {
    input
        .split(',')
        .filter_map(|part| {
            let (from_str, to_str) = part.trim().split_once('-')?;
            let from = from_str.trim().parse::<u64>().ok()?;
            let to = to_str.trim().parse::<u64>().ok()?;
            Some(from..=to)
        })
        .flat_map(|range| range.map(|n| n.to_string()))
        .collect()
}

fn find_repeating_digits(input: &str) -> Option<&str> {
    let full_length = input.len();
    if full_length % 2 == 1 {return None}
    let window_size = full_length / 2;
    match find_repeating_digits_of_size(input, window_size) {
        Some(_digits) => {
            Some(input)
        },
        None => None,
    }

    // nvm this was not needed, learn how to read Stefan ;)
    // for window_size in 1..=max_window_size {
    //     if let Some(_digits) = find_repeating_digits_of_size(&input, window_size) {
    //         return Some(input)
    //     }
    // }

    // None
}

fn find_repeating_digits_of_size(input: &str, window_size: usize) -> Option<&str> {
    let full_length = input.len();
    let first_index = 0;
    let final_index = full_length - 2 * window_size;

    for left_start_index in first_index..=final_index {
        let left_end_index = left_start_index + window_size - 1;
        let right_start_index = left_end_index + 1;
        let right_end_index = right_start_index + window_size - 1;

        let lhs = &input[left_start_index..=left_end_index];
        let rhs = &input[right_start_index..=right_end_index];

        if lhs == rhs {return Some(lhs)}
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }

    #[test]
    fn test_find_repeating_digits_of_size() {
        assert_eq!(find_repeating_digits_of_size("11", 1), Some("1"));
        assert_eq!(find_repeating_digits_of_size("12", 1), None);
        assert_eq!(find_repeating_digits_of_size("1212", 1), None);
        assert_eq!(find_repeating_digits_of_size("1212", 2), Some("12"));
        assert_eq!(find_repeating_digits_of_size("12323", 2), Some("23"));
        assert_eq!(find_repeating_digits_of_size("123423", 2), None);
    }
}
