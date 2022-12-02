mod calorie_counting_1;

mod rock_paper_scissors_2;

#[cfg(test)]
mod tests {

    #[test]
    fn calorie_counting() {
        let path = "calorie.txt";

        let [result_one, result_two] = crate::calorie_counting_1::calculate(path);

        assert_eq!(result_one, 70_116);

        assert_eq!(result_two, 206_582);
    }

    #[test]
    fn rock_paper_scissors() {
        let path = "input.txt";

        let result_one = crate::rock_paper_scissors_2::calculate(path, true);

        assert_eq!(result_one, 11449);

        let result_two = crate::rock_paper_scissors_2::calculate(path, false);

        assert_eq!(result_two, 13187);
    }
}
