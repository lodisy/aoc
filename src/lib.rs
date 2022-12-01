mod calorie_counting_1;

#[cfg(test)]
mod tests {

    #[test]
    fn calorie_counting() {
        let path = "calorie.txt";

        let [result_one, result_two] = crate::calorie_counting_1::calculate(path);

        assert_eq!(result_one, 70_116);

        assert_eq!(result_two, 206_582);
    }
}
