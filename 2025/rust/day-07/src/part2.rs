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
        .map(|char| if char == 'S' { 1 } else { 0 })
        .collect::<Vec<u64>>();
    let result = diagram.fold(
        initial_beam,
        |beam, diagram_line| {
            let resulting_beam = beam
                .into_iter()
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
                            (
                                (_, _, _),
                                (_, '^', _)
                            ) => 0,
                            (
                                (lb, cb, rb),
                                (ls, '.', rs)
                            ) => (if ls == '^' {lb} else {0}) + cb + (if rs == '^' {rb} else {0}),
                            (
                                (_, _, _),
                                (_, _, _)
                            ) => panic!("Situation not covered. Beam: [{}{}{}], Diagram: [{}{}{}]", beam_l, beam_c, beam_r, diag_l, diag_c, diag_r),
                        }
                    }
                )
                .collect::<Vec<u64>>();
            std::iter::once(0)
                .chain(resulting_beam)
                .chain(std::iter::once(0))
                .collect::<Vec<u64>>()
        },
    )
    .iter().sum::<u64>();

    Ok(result.to_string())
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
        assert_eq!("40", process(input)?);
        Ok(())
    }
}
