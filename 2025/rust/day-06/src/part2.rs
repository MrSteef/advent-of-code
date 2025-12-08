#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut rows: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let op_row = rows.pop().unwrap();
    let height = rows.len();

    let mut grand_total: u64 = 0;
    let mut current_total: u64 = 0;
    let mut current_op: fn(u64, u64) -> u64 = u64::saturating_add;

    for (x, &op) in op_row.iter().enumerate() {
        match op {
            '+' => {
                grand_total += current_total;
                current_total = 0;
                current_op = u64::saturating_add;
            }
            '*' => {
                grand_total += current_total;
                current_total = 1;
                current_op = u64::saturating_mul;
            }
            _ => {}
        }

        let column: String = rows
            .iter()
            .take(height)
            .map(|row| row[x])
            .collect();

        if let Ok(number) = column.trim().parse::<u64>() {
            current_total = current_op(current_total, number);
        }
    }

    grand_total += current_total;
    Ok(grand_total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!("3263827", process(input)?);
        Ok(())
    }
}
