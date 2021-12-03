pub fn first(input: &str) -> i32 {
    let (horizontal, depth) = input
        .lines()
        .map(|l| {
            (
                &l[..1],
                l.split(' ').last().unwrap().parse::<i32>().unwrap(),
            )
        })
        .fold((0, 0), |(h, d), (c, v)| match c {
            "f" => (h + v, d),
            "d" => (h, d + v),
            "u" => (h, d - v),
            _ => (h, d),
        });

    horizontal * depth
}

pub fn second(input: &str) -> i32 {
    let (horizontal, depth, _) = input
        .lines()
        .map(|l| {
            (
                &l[..1],
                l.split(' ').last().unwrap().parse::<i32>().unwrap(),
            )
        })
        .fold((0, 0, 0), |(h, d, a), (c, v)| match c {
            "f" => (h + v, d + a * v, a),
            "d" => (h, d, a + v),
            "u" => (h, d, a - v),
            _ => (h, d, a),
        });

    horizontal * depth
}
