use std::borrow::Borrow;
use std::slice::Windows;

pub fn first(input: String) -> i32 {
    let input = preprocess(input);
    count_increased(1, input)
}

pub fn second(input: String) -> i32 {
    let input = preprocess(input);
    count_increased(3, input)
}

fn count_increased(width: usize, data: Vec<i32>) -> i32 {
    let windows: Vec<&[i32]> = data.windows(width).collect();
    let mut count = 0;
    let mut last: i32 = windows[0].iter().sum();
    for w in windows.into_iter().skip(1).map(|w| -> i32 { w.iter().sum() }) {
        if w > last {
            count += 1;
        }
        last = w;
    }
    count
}

fn preprocess(input: String) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}
