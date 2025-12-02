use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    Ok(list_product_ids(input)
        .iter()
        .filter(|id| has_repeating_digits(id))
        .map(|id| id.parse::<u64>().unwrap())
        .sum::<u64>()
        .to_string())
}

fn list_product_ids(input: &str) -> Vec<String> {
    input
        .split(',')
        .filter_map(|part| {
            let (from_str, to_str) =
                part.trim().split_once('-')?;
            let from =
                from_str.trim().parse::<u64>().ok()?;
            let to = to_str.trim().parse::<u64>().ok()?;
            Some(from..=to)
        })
        .flat_map(|range| range.map(|n| n.to_string()))
        .collect()
}

fn get_divisors(n: usize) -> Vec<usize> {
    (1..=n).filter(|d| n % d == 0).collect()
}

fn has_repeating_digits(input: &str) -> bool {
    get_divisors(input.len()).into_iter().any(
        |split_size| {
            has_repeating_digits_of_size(input, split_size)
        },
    )
}

fn has_repeating_digits_of_size(
    input: &str,
    split_size: usize,
) -> bool {
    split_size < input.len()
        && input.as_bytes().chunks(split_size).all_equal()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("4174379265", process(input)?);
        Ok(())
    }

    #[test]
    fn test_get_divisors() {
        assert_eq!(get_divisors(1), vec![1]);
        assert_eq!(get_divisors(2), vec![1, 2]);
        assert_eq!(get_divisors(3), vec![1, 3]);
        assert_eq!(get_divisors(4), vec![1, 2, 4]);
        assert_eq!(get_divisors(5), vec![1, 5]);
        assert_eq!(get_divisors(6), vec![1, 2, 3, 6]);
        assert_eq!(get_divisors(7), vec![1, 7]);
        assert_eq!(get_divisors(8), vec![1, 2, 4, 8]);
        assert_eq!(get_divisors(9), vec![1, 3, 9]);
        assert_eq!(get_divisors(10), vec![1, 2, 5, 10]);
    }

    #[test]
    fn test_has_repeating_digits_of_size() {
        assert!(!has_repeating_digits_of_size("1", 1));
        assert!(has_repeating_digits_of_size("11", 1));
        assert!(!has_repeating_digits_of_size("1212", 1));
        assert!(has_repeating_digits_of_size("1212", 2));
    }
}
