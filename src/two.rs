use std::iter::Map;
use std::str::Lines;

pub fn first(input: &str) -> i32 {
    let (horizontal, depth) = preprocess(input).fold((0, 0), |(h, d), (c, v)| match c {
        "f" => (h + v, d),
        "d" => (h, d + v),
        "u" => (h, d - v),
        _ => (h, d),
    });

    horizontal * depth
}

pub fn second(input: &str) -> i32 {
    let (horizontal, depth, _) = preprocess(input).fold((0, 0, 0), |(h, d, a), (c, v)| match c {
        "f" => (h + v, d + a * v, a),
        "d" => (h, d, a + v),
        "u" => (h, d, a - v),
        _ => (h, d, a),
    });

    horizontal * depth
}

fn preprocess<'a>(input: &'a str) -> Map<Lines<'_>, fn(&'a str) -> (&str, i32)> {
    input.lines().map(|l| {
        (
            &l[..1],
            l.split(' ').last().unwrap().parse::<i32>().unwrap(),
        )
    })
}
