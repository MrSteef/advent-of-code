#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let password = parse_sequence("input1.txt");
    Ok(password.to_string())
}

const DIAL_SIZE: u32 = 100;
const DIAL_STARTING_POSITION: u32 = 50;

fn rotate_right(dial: u32, distance: u32) -> u32 {
    let dist = distance % DIAL_SIZE;
    (dial + dist) % DIAL_SIZE
}

fn rotate_left(dial: u32, distance: u32) -> u32 {
    let dist = distance % DIAL_SIZE;
    rotate_right(dial, DIAL_SIZE - dist)
}

fn parse_instruction(dial: u32, instruction: &str) -> u32 {
    let (direction, distance) = instruction.split_at(1);
    let dist: u32 = distance.parse().unwrap_or_default();
    match direction {
        "L" => rotate_left(dial, dist),
        "R" => rotate_right(dial, dist),
        _ => dial,
    }
}

fn parse_sequence(file_path: &str) -> u32 {
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut dial = DIAL_STARTING_POSITION;
    let mut zero_counter = 0;

    for instruction in contents.lines().into_iter() {
        dial = parse_instruction(dial, instruction);
        if dial == 0 {zero_counter += 1}
    }

    zero_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_right() {
        assert_eq!(rotate_right(10, 20), 30);
        assert_eq!(rotate_right(90, 20), 10);
    }

    #[test]
    fn test_rotate_left() {
        assert_eq!(rotate_left(10, 20), 90);
        assert_eq!(rotate_left(90, 20), 70);
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(parse_instruction(50, "R10"), 60);
        assert_eq!(parse_instruction(50, "L10"), 40);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!(
            parse_sequence("example.txt"),
            3
        );
        Ok(())
    }
}
