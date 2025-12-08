#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut lines = input
        .lines()
        .into_iter()
        .map(|line| {
            line.split(' ')
                .filter(|item| !item.is_empty())
        });
    let ops = lines.next_back().unwrap();
    let mut nums = lines.collect::<Vec<_>>();

    let total = ops.fold(0, |total_acc, op| {
        let col_total = total_acc + nums
            .iter_mut()
            .fold(0, |col_acc, line| {
                let num = line.next().unwrap().parse::<u128>().unwrap();
                match op {
                    "+" => col_acc + num,
                    "*" => if col_acc == 0 {num} else {col_acc * num},
                    _ => col_acc,
                }
            });
        col_total
    });

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";
        assert_eq!("4277556", process(input)?);
        Ok(())
    }
}
