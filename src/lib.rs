mod day01 {
    pub mod calorie_counting;
}

mod day02 {
    pub mod rock_paper_scissors;
}

#[cfg(test)]
mod tests {

    #[test]
    fn calorie_counting() {
        let [result_one, result_two] = crate::day01::calorie_counting::calculate();

        assert_eq!(result_one, 70_116);

        assert_eq!(result_two, 206_582);
    }

    #[test]
    fn rock_paper_scissors() {
        let result_one = crate::day02::rock_paper_scissors::calculate(true);

        assert_eq!(result_one, 11449);

        let result_two = crate::day02::rock_paper_scissors::calculate(false);

        assert_eq!(result_two, 13187);
    }
}
