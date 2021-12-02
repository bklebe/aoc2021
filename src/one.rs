pub fn first(input: String) -> usize {
    let input = preprocess(input);
    count_increased(1, input)
}

pub fn second(input: String) -> usize {
    let input = preprocess(input);
    count_increased(3, input)
}

fn count_increased(width: usize, data: Vec<i32>) -> usize {
    data.windows(width)
        .map(|w| -> i32 { w.iter().sum() })
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}

fn preprocess(input: String) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}
