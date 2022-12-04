use std::collections::HashSet;

const ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[allow(dead_code)]
pub fn calculate() -> [usize; 2] {
    // Part one
    let mut duplicates: Vec<HashSet<char>> = Vec::new();

    include_str!("./input.txt")
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|&s| {
            let str = s.chars().collect::<Vec<char>>();

            let (left, right) = str.split_at(s.len() / 2);

            let mut h: HashSet<char> = HashSet::new();

            for j in 0..left.len() {
                let p = right.iter().position(|r| *r == left[j]);

                if let Some(position) = p {
                    h.insert(right[position]);
                }
            }

            duplicates.push(h);
        });

    let mut sum_one: usize = 0;

    duplicates.iter().for_each(|array| {
        sum_one = sum_one
            + array
                .iter()
                .map(|key| match ALPHABET.iter().position(|c| c == key) {
                    Some(position) => position + 1,
                    _ => 0,
                })
                .sum::<usize>();
    });

    // Part two
    let strings_two: Vec<&str> = include_str!("./input.txt")
        .split_inclusive("\n")
        .collect::<Vec<&str>>();

    let mut index: usize = 0;

    let mut result: Vec<String> = Vec::new();

    let mut duplicates_two: Vec<char> = Vec::new();

    while index < strings_two.len() - 2 {
        result.push(
            [
                strings_two[index],
                strings_two[index + 1],
                strings_two[index + 2],
            ]
            .concat(),
        );

        index += 3;
    }

    result.iter().for_each(|s| {
        let a = s.split("\n").collect::<Vec<&str>>();

        let b = a[0].chars().collect::<Vec<char>>();

        for i in 0..b.len() {
            if let Some(_) = a[1].find(b[i]) {
                if let Some(_) = a[2].find(b[i]) {
                    duplicates_two.push(b[i]);
                    break; // cause we know there is only one badage
                }
            }
        }
    });

    let sum_two = duplicates_two
        .iter()
        .map(|key| match ALPHABET.iter().position(|c| c == key) {
            Some(position) => position + 1,
            _ => 0,
        })
        .sum::<usize>();

    [sum_one, sum_two]
}
