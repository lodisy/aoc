use std::{fs::File, io::Read, path::Path};

#[allow(dead_code)]
pub fn calculate(path: &str, part_one: bool) -> i32 {
    let path = Path::new(path);

    let mut file = File::open(&path).unwrap();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    let mut s: Vec<&str> = buffer.split("\n").collect();

    let total: Vec<i32> = s
        .iter_mut()
        .map(|x| {
            let sub: Vec<&str> = x.split(" ").collect();

            match sub[0] {
                // Rock 1
                "A" => match sub[1] {
                    "X" => {
                        if part_one {
                            1 + 3
                        } else {
                            0 + 3
                        }
                    }
                    "Y" => {
                        if part_one {
                            2 + 6
                        } else {
                            3 + 1
                        }
                    }
                    "Z" => {
                        if part_one {
                            3 + 0
                        } else {
                            6 + 2
                        }
                    }
                    _ => 0,
                },
                // Paper 2
                "B" => match sub[1] {
                    //"X" => 1 + 0,
                    "X" => 0 + 1,
                    // "Y" => 2 + 3,
                    "Y" => 3 + 2,
                    // "Z" => 3 + 6,
                    "Z" => 6 + 3,
                    _ => 0,
                },
                // Scissors 3
                "C" => match sub[1] {
                    "X" => {
                        if part_one {
                            1 + 6
                        } else {
                            0 + 2
                        }
                    }

                    "Y" => {
                        if part_one {
                            2 + 0
                        } else {
                            3 + 3
                        }
                    }

                    "Z" => {
                        if part_one {
                            3 + 3
                        } else {
                            6 + 1
                        }
                    }
                    _ => 0,
                },
                _ => 0,
            }
        })
        .collect();

    let total: i32 = total.iter().sum();

    total
}
