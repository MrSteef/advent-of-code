use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    str::FromStr,
    sync::Mutex,
};

use itertools::Itertools;
use miette::{IntoDiagnostic, miette};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let boxes: Vec<JunctionBox> = input
        .lines()
        .map(JunctionBox::from_str)
        .collect::<miette::Result<Vec<JunctionBox>>>()?;
    let nr_of_boxes = boxes.len();
    let mut circuit_membership: HashMap<
        &JunctionBox,
        Rc<Mutex<HashSet<&JunctionBox>>>,
    > = HashMap::with_capacity(boxes.len());
    boxes
        .iter()
        .for_each(|junction_box| {
            let set =
                Rc::new(Mutex::new(HashSet::new()));
            set.lock().unwrap().insert(junction_box);
            circuit_membership
                .insert(junction_box, set.clone());
        });
    let distances = boxes
        .iter()
        .tuple_combinations()
        .map(|(box1, box2)| {
            (box1, box2, box1.distance_from(box2))
        })
        .sorted_by(|a, b| a.2.total_cmp(&b.2));

    for (box1, box2, _distance) in distances {
        let box1_rc = circuit_membership
            .get(box1)
            .expect(
                "expected box to be a member of a circuit",
            )
            .clone();

        let box2_rc = circuit_membership
            .get(box2)
            .expect(
                "expected box to be a member of a circuit",
            )
            .clone();

        if Rc::ptr_eq(&box1_rc, &box2_rc) {
            continue;
        }

        let moved_boxes: Vec<&JunctionBox> = {
            let mut box2_set = box2_rc.lock().unwrap();
            box2_set.drain().collect()
        };

        {
            let mut box1_set = box1_rc.lock().unwrap();
            box1_set.extend(moved_boxes.iter().copied());
        }

        for b in moved_boxes {
            circuit_membership.insert(b, box1_rc.clone());
        }

        if box1_rc.lock().unwrap().len() == nr_of_boxes {
            return Ok((box1.x * box2.x).to_string());
        }
    }

    Err(miette!("something went wrong"))
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct JunctionBox {
    x: i128,
    y: i128,
    z: i128,
}

impl JunctionBox {
    fn distance_from(&self, other: &JunctionBox) -> f64 {
        let JunctionBox { x, y, z } = other;
        let dist_x = (self.x - x).pow(2) as f64;
        let dist_y = (self.y - y).pow(2) as f64;
        let dist_z = (self.z - z).pow(2) as f64;
        (dist_x + dist_y + dist_z).powf(0.5)
    }
}

impl FromStr for JunctionBox {
    type Err = miette::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        let x = split.next().ok_or(miette!(
            "expected a non-empty string"
        ))?;
        let y = split
            .next()
            .ok_or(miette!("expected 2 comma's found 0"))?;
        let z = split
            .next()
            .ok_or(miette!("expected 2 comma's found 1"))?;

        let x = x.parse::<i128>().into_diagnostic()?;
        let y = y.parse::<i128>().into_diagnostic()?;
        let z = z.parse::<i128>().into_diagnostic()?;

        Ok(JunctionBox { x, y, z })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!("25272", process(input)?);
        Ok(())
    }
}
