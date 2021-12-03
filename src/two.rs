pub fn first(input: String) -> i32 {
    let mut hori = 0;
    let mut depth = 0;
    for line in input.lines() {
        let value: i32 = line.split(' ').last().unwrap().parse().unwrap();
        match &line[..1] {
            "f" => hori += value,
            "d" => depth += value,
            "u" => depth -= value,
            _ => (),
        }
    }

    hori * depth
}
