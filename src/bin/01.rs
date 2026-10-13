use advent_of_code_2026::shared::{PartSolution, Parts};

advent_of_code_2026::solution!(1, 1);

impl Parts for Solution {
    fn part_1(&self, _input: &str) -> PartSolution {
        PartSolution::U32(1)
    }

    fn part_2(&self, _input: &str) -> PartSolution {
        PartSolution::U32(1)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {

        use advent_of_code_2026::{test_example_part_1, test_part_1};

        #[test]
        fn outcome() {
            test_part_1!(1);
        }

        #[test]
        fn example() {
            test_example_part_1!(1);
        }
    }

    mod part_2 {
        use advent_of_code_2026::{test_example_part_2, test_part_2};

        #[test]
        fn outcome() {
            test_part_2!(1);
        }

        #[test]
        fn example() {
            test_example_part_2!(1);
        }
    }
}
