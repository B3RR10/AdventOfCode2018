use helper;

fn part_1(input: &Vec<String>) -> i32 {
    input.iter().map(|x| x.parse::<i32>().unwrap()).sum()
}

fn part_2(input: &Vec<String>) -> i32 {
    let mut freq: i32 = 0;
    let mut freqs: Vec<i32> = vec![0];
    loop {
        for it in input {
            freq += it.parse::<i32>().unwrap();
            if freqs.iter().any(|x| x == &freq) {
                return freq;
            } else {
                freqs.push(freq)
            }
        }
    }
}

pub fn handle_day(input: &str) {
    let lines = helper::lines_from_file(input);
    let solution_1 = part_1(&lines);
    println!("Solution 1 = {}", solution_1);
    let solution_2 = part_2(&lines);
    println!("Solution 2 = {}", solution_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    // --------------- Part 1 --------------- //
    #[test]
    fn test_part_1_1() {
        let input: Vec<String> = vec!["+1".to_string(), "+1".to_string(), "+1".to_string()];
        assert_eq!(3, part_1(&input));
    }

    #[test]
    fn test_part_1_2() {
        let input: Vec<String> = vec!["+1".to_string(), "+1".to_string(), "-2".to_string()];
        assert_eq!(0, part_1(&input));
    }

    #[test]
    fn test_part_1_3() {
        let input: Vec<String> = vec!["-1".to_string(), "-2".to_string(), "-3".to_string()];
        assert_eq!(-6, part_1(&input));
    }

    // --------------- Part 2 --------------- //
    #[test]
    fn test_part_2_1() {
        let input: Vec<String> = vec!["+1".to_string(), "-1".to_string()];
        assert_eq!(0, part_2(&input));
    }

    #[test]
    fn test_part_2_2() {
        let input: Vec<String> = vec![
            "+3".to_string(),
            "+3".to_string(),
            "+4".to_string(),
            "-2".to_string(),
            "-4".to_string(),
        ];
        assert_eq!(10, part_2(&input));
    }

    #[test]
    fn test_part_2_3() {
        let input: Vec<String> = vec![
            "-6".to_string(),
            "+3".to_string(),
            "+8".to_string(),
            "+5".to_string(),
            "-6".to_string(),
        ];
        assert_eq!(5, part_2(&input));
    }

    #[test]
    fn test_part_2_4() {
        let input: Vec<String> = vec![
            "+7".to_string(),
            "+7".to_string(),
            "-2".to_string(),
            "-7".to_string(),
            "-4".to_string(),
        ];
        assert_eq!(14, part_2(&input));
    }

}
