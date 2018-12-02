use helper;
use std::collections::hash_map::HashMap;

fn part_1(lines: &Vec<String>) -> i32 {
    let mut two: i32 = 0;
    let mut three: i32 = 0;

    for line in lines {
        let mut repetitions: HashMap<char, i32> = HashMap::new();
        line.chars().for_each(|c| {
            if repetitions.contains_key(&c) {
                if let Some(x) = repetitions.get_mut(&c) {
                    *x += 1;
                }
            } else {
                repetitions.insert(c, 1);
            }
        });
        if repetitions.values().any(|v| v == &2) {
            two += 1;
        }
        if repetitions.values().any(|v| v == &3) {
            three += 1;
        }
    }
    two * three
}

fn string_number_of_differences(str1: &str, str2: &str) -> (i32, String) {
    let vec1: Vec<char> = str1.chars().collect();
    let vec2: Vec<char> = str2.chars().collect();
    let mut diffs = 0;
    let mut common = String::new();
    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
            diffs += 1
        } else {
            common.push(vec1[i])
        }
    }
    (diffs, common)
}

fn part_2(lines: &Vec<String>) -> String {
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            let (diffs, common) = string_number_of_differences(&lines[i], &lines[j]);
            if diffs == 1 {
                return common;
            }
        }
    }
    "".to_string()
}

pub fn handle_day(filename: &str) {
    let lines = helper::lines_from_file(filename);
    let solution_1 = part_1(&lines);
    println!("Solution 1 = {}", solution_1);
    let solution_2 = part_2(&lines);
    println!("Solution 2 = {}", solution_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input: Vec<String> = vec![
            "abcdef".to_string(),
            "bababc".to_string(),
            "abbcde".to_string(),
            "abcccd".to_string(),
            "aabcdd".to_string(),
            "abcdee".to_string(),
            "ababab".to_string(),
        ];
        assert_eq!(12, part_1(&input));
    }

    #[test]
    fn test_part_2() {
        let input: Vec<String> = vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "fguij".to_string(),
            "axcye".to_string(),
            "wvxyz".to_string(),
        ];
        assert_eq!("fgij", part_2(&input));
    }

}
