pub fn first(input: &str) -> i32 {
    let (mut horizontal, mut depth) = (0, 0);
    for line in input.lines() {
        let value: i32 = line.split(' ').last().unwrap().parse().unwrap();
        match &line[..1] {
            "f" => horizontal += value,
            "d" => depth += value,
            "u" => depth -= value,
            _ => (),
        }
    }

    horizontal * depth
}

pub fn second(input: &str) -> i32 {
    let (mut horizontal, mut depth, mut aim) = (0, 0, 0);
    for line in input.lines() {
        let value: i32 = line.split(' ').last().unwrap().parse().unwrap();
        match &line[..1] {
            "f" => {
                horizontal += value;
                depth += aim * value;
            }
            "d" => aim += value,
            "u" => aim -= value,
            _ => (),
        }
    }

    horizontal * depth
}
