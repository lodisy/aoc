#[allow(dead_code)]
pub fn calculate() -> [u32; 2] {
    let mut result: Vec<Vec<u32>> = Vec::new();

    include_str!("./input.txt")
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|&s| {
            let mut data: Vec<u32> = Vec::new();

            s.split(",").collect::<Vec<&str>>().iter().for_each(|&x| {
                let y = x.split("-").collect::<Vec<&str>>();

                data.push(y[0].parse::<u32>().unwrap());

                data.push(y[1].parse::<u32>().unwrap());
            });

            result.push(data);
        });

    [
        result
            .iter()
            .map(
                |r| match r[0] <= r[2] && r[1] >= r[3] || r[0] >= r[2] && r[1] <= r[3] {
                    true => 1,
                    _ => 0,
                },
            )
            .sum::<u32>(),
        result
            .iter()
            .map(|r| match r[1] < r[2] || r[3] < r[0] {
                true => 0,
                _ => 1,
            })
            .sum::<u32>(),
    ]
}
