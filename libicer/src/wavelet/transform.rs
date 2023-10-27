//
//  TODO:
//  - use actual sizes instead of only u32
//

use crate::FilterParams;

fn low_pass_filter(data: &mut [u32], input: &[u32]) {
    // helpers for filter computation
    let compute = |n1: u32, n2: u32| -> u32 { (n1 + n2) / 2 };
    let odd = input.len() & 1 == 1;

    let (last, elements) = data.split_last_mut().unwrap();

    // use helper on all but last element
    for (idx, el) in elements.into_iter().enumerate() {
        let n1 = input[2 * idx];
        let n2 = input[(2 * idx) + 1];

        *el = compute(n1, n2);
    }

    // determine last element based on odd or even input length
    *last = if odd {
        *input.last().unwrap()
    } else {
        let n1 = input[input.len() - 2];
        let n2 = input[input.len() - 1];

        compute(n1, n2)
    };
}

fn high_pass_filter(data: &mut [u32], input: &[u32], filter: &FilterParams) {
    let compute = |n1: u32, n2: u32| -> u32 { (n1 - n2) / 2 };
    let odd = input.len() & 1 == 1;
    let (a_neg, a0, a1, b) = filter.to_params();

    // use helper on all but last element
    // for idx in 0..(input.len() - 1) {
    // data.push(compute(input[idx], input[idx + 1]));
    // }
}

pub fn transform_1d(input: &[u32], filter: &FilterParams) -> Vec<u32> {
    let mut output = input.to_vec();
    let (low, high) = output.split_at_mut(input.len() / 2);

    low_pass_filter(low, input);
    high_pass_filter(high, input, filter);

    output
}
