#[cfg(test)]
mod test_solutions {
    use crate::{solution_part_1, solution_part_2};

    #[test]
    fn test_solution_part_1() {
        assert_eq!(5, solution_part_1("testData.txt"));
    }

    #[test]
    fn test_solution_part_2() {
        assert_eq!("mxmxvkd,sqjhc,fvjkl", solution_part_2("testData.txt"));
    }
}