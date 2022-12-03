#[allow(dead_code)]
pub fn calculate() -> [i32; 2] {
    let data = include_str!("./calorie.txt");

    let mut s: Vec<&str> = data.split("\n\n").collect();

    let mut s: Vec<String> = s.iter_mut().map(|x| x.replace("\n", ",")).collect();

    let mut result: Vec<i32> = s
        .iter_mut()
        .map(|x| {
            let mut sub: Vec<&str> = x.split(",").collect();

            let sub: Vec<i32> = sub.iter_mut().map(|x| x.parse::<i32>().unwrap()).collect();

            sub.iter().sum()
        })
        .collect();

    result.sort();

    let result_one = match result.last() {
        Some(value) => *value,
        None => -1,
    };

    let result_two: i32 = result[result.len() - 3..].iter().sum();

    [result_one, result_two]
}
