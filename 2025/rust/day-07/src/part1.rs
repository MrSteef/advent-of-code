use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut diagram = input.lines().map(|line| {
        std::iter::once('.')
            .chain(line.chars())
            .chain(std::iter::once('.'))
    });
    let initial_beam = diagram
        .next()
        .unwrap()
        .map(|char| if char == 'S' { '|' } else { ' ' })
        .collect::<String>();
    let result = diagram.fold(
        (initial_beam, 0),
        |(beam, split_count), diagram_line| {
            let mut local_split_count = 0;
            let resulting_beam = beam
                .chars()
                .tuple_windows()
                .zip(diagram_line.tuple_windows())
                .map(
                    |(
                        (beam_l, beam_c, beam_r),
                        (diag_l, diag_c, diag_r),
                    )| 
                    {
                        match (
                            (beam_l, beam_c, beam_r),
                            (diag_l, diag_c, diag_r),
                        ) {
                            ( // beam already exists and is not split up
                                (_, '|', _),
                                (_, '.', _)
                            ) => {
                                // print!("|");
                                '|'
                            },
                            ( // beam existed and got split up
                                (_, '|', _),
                                (_, '^', _)
                            ) => {
                                // print!("^");
                                local_split_count += 1;
                                ' '
                            },
                            ( // beam got split up on the lefthand side
                                ('|', _, _),
                                ('^', _, _)
                            ) => {
                                // print!("\\");
                                '|'
                            },
                            ( // beam got split up on the righthand side
                                (_, _, '|'),
                                (_, _, '^')
                            ) => {
                                // print!("/");
                                '|'
                            },
                            ( // beam did not exist, and did not get split up on either side (one of the prior arms wouldve matched)
                                (_, ' ', _),
                                (_, _, _)
                            ) => {
                                // print!(".");
                                ' '
                            },
                            (
                                (_, _, _),
                                (_, _, _)
                            ) => panic!("Situation not covered. Beam: [{}{}{}], Diagram: [{}{}{}]", beam_l, beam_c, beam_r, diag_l, diag_c, diag_r),
                        }
                    }
                )
                .collect::<String>();
            // println!(" new splits: {}, total_splits: {}", local_split_count, split_count);
            let padded_resulting_beam = std::iter::once('.')
                .chain(resulting_beam.chars())
                .chain(std::iter::once('.'))
                .collect::<String>();
            (padded_resulting_beam, split_count + local_split_count)
        },
    );

    Ok(result.1.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
