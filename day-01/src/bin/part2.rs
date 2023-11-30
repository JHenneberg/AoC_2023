use std::fs;

fn main() {
    let input = fs::read_to_string("./input_day-01.txt").unwrap();
    println!("{}", process_part2(&input));
}

fn process_part2(input: &str) -> String {
    todo!("Implement part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part2() {}
}
