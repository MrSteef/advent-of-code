use miette::{miette, IntoDiagnostic};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let password = parse_sequence("input2.txt")?;
    Ok(password.to_string())
}

const DIAL_SIZE: i32 = 100;
const DIAL_STARTING_POSITION: i32 = 50;

fn rotate_right(dial_position: i32, distance: u32) -> (i32, u32) {
    let new_position = dial_position + (distance as i32);
    let normalized_position = new_position % DIAL_SIZE;
    let clicks = ((new_position - normalized_position) / DIAL_SIZE) as u32;
    (normalized_position, clicks)
}

fn rotate_left(dial_position: i32, distance: u32) -> (i32, u32) {
    let flipped = (DIAL_SIZE - dial_position) % DIAL_SIZE;
    let (rotated_dial, clicks) = rotate_right(flipped, distance);
    let new_position = (DIAL_SIZE - rotated_dial) % DIAL_SIZE;
    (new_position, clicks)
}

fn rotate(dial_position: i32, instruction: Instruction) -> (i32, u32) {
    match instruction.direction {
        Direction::Left => rotate_left(dial_position, instruction.distance),
        Direction::Right => rotate_right(dial_position, instruction.distance),
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug)]
struct Instruction {
    direction: Direction,
    distance: u32
}

fn parse_instruction(line: &str) -> miette::Result<Instruction> {
    let (direction, distance) = line.split_at(1);
    let dist: u32 = distance.parse().into_diagnostic()?;
    match direction {
        "L" => Ok(Instruction{direction: Direction::Left, distance: dist}),
        "R" => Ok(Instruction{direction: Direction::Right, distance: dist}),
        _ => Err(miette!("Unable to parse direction: `{direction}`")),
    }
}

fn parse_sequence(file_path: &str) -> miette::Result<u32> {
    let contents = std::fs::read_to_string(file_path)
        .into_diagnostic()?;
    
    let mut dial_position = DIAL_STARTING_POSITION;
    let mut click_counter = 0;

    for line in contents.lines().into_iter() {
        let instruction = parse_instruction(line)?;
        let (new_position, clicks) = rotate(dial_position, instruction);
        click_counter += clicks;
        dial_position = new_position;
    }

    Ok(click_counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_right() {
        assert_eq!(rotate_right(10, 20), (30, 0));
        assert_eq!(rotate_right(90, 20), (10, 1));
        assert_eq!(rotate_right(90, 120), (10, 2));
        assert_eq!(rotate_right(0, 5), (5, 0));
    }

    #[test]
    fn test_rotate_left() {
        assert_eq!(rotate_left(10, 20), (90, 1));
        assert_eq!(rotate_left(10, 120), (90, 2));
        assert_eq!(rotate_left(90, 20), (70, 0));
        assert_eq!(rotate_left(0, 5), (95, 0));
    }

    #[test]
    fn test_rotate() {
        assert_eq!(
            rotate(10, Instruction { direction: Direction::Right, distance: 20}), 
            (30, 0)
        );
        assert_eq!(
            rotate(90, Instruction { direction: Direction::Right, distance: 20}), 
            (10, 1)
        );
        assert_eq!(
            rotate(90, Instruction { direction: Direction::Right, distance: 120}), 
            (10, 2)
        );
        assert_eq!(
            rotate(10, Instruction { direction: Direction::Left, distance: 20}), 
            (90, 1)
        );
        assert_eq!(
            rotate(10, Instruction { direction: Direction::Left, distance: 120}), 
            (90, 2)
        );
        assert_eq!(
            rotate(90, Instruction { direction: Direction::Left, distance: 20}), 
            (70, 0)
        );

    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("L1").unwrap(), 
            Instruction { direction: Direction::Left, distance: 1}
        );
        assert_eq!(
            parse_instruction("L100").unwrap(), 
            Instruction { direction: Direction::Left, distance: 100}
        );
        assert_eq!(
            parse_instruction("L1000").unwrap(), 
            Instruction { direction: Direction::Left, distance: 1000}
        );
        assert_eq!(
            parse_instruction("R1").unwrap(), 
            Instruction { direction: Direction::Right, distance: 1}
        );
        assert_eq!(
            parse_instruction("R100").unwrap(), 
            Instruction { direction: Direction::Right, distance: 100}
        );
        assert_eq!(
            parse_instruction("R1000").unwrap(), 
            Instruction { direction: Direction::Right, distance: 1000}
        );
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!(
            parse_sequence("example.txt").unwrap(),
            6
        );
        Ok(())
    }
}
