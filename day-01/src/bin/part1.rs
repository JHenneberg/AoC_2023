use std::fs;

fn main() {
    let input = fs::read_to_string("./input_day-01.txt").unwrap();
    println!("{}", process_part1(&input));
}

fn process_part1(input: &str) -> String {
    todo!("Implement part 1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {}
}
