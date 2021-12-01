pub fn first(input: String) -> i32 {
    let input = preprocess(input);
    count_increased(1, input)
}

pub fn second(input: String) -> i32 {
    let input = preprocess(input);
    count_increased(3, input)
}

fn count_increased(width: usize, data: Vec<i32>) -> i32 {
    data.windows(width)
        .map(|w| -> i32 { w.iter().sum() })
        .fold((0, None), |(count, last), w| match last {
            Some(last) if w > last => (count + 1, Some(w)),
            _ => (count, Some(w)),
        })
        .0
}

fn preprocess(input: String) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}
