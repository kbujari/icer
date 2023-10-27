//
//  TODO:
//  - use actual sizes instead of only u32
//

use create::FilterParams;

fn low_pass_filter(data: &mut Vec<u16>, input: &[u8], odd: bool) {
    // helper for filter computation
    let compute = |n1: u8, n2: u8| -> u16 { (n1 as u16 + n2 as u16) / 2 };

    // use helper on all but last element
    input
        .into_iter()
        .enumerate()
        .map(|(idx, _)| (*input.get(idx).unwrap(), *input.get(idx + 1).unwrap_or(&0)))
        .map(|(n1, n2)| compute(n1, n2))
        .for_each(|val| data.push(val));

    // determine last element based on odd or even input length
    let last = if !odd {
        compute(input[input.len() - 2], input[input.len() - 1])
    } else {
        *input.last().unwrap() as u16
    };

    // overwrite last element
    *data.get_mut(input.len() - 1).unwrap() = last;
}

fn high_pass_filter(data: &mut Vec<u16>, input: &[u8], odd: bool, filter: &FilterParams) {
    let compute = |n1: u8, n2: u8| -> u16 { (n1 as u16 - n2 as u16) / 2 };

    // use helper on all but last element
    for idx in 0..(input.len() - 1) {
        data.push(compute(input[idx], input[idx + 1]));
    }
}

pub fn transform_1d(input: &[u8], filter: &FilterParams, is_odd: bool) -> Vec<u16> {
    let mut output = Vec::with_capacity(input.len() * 2);

    low_pass_filter(&mut output, input, is_odd);
    assert_eq!(output.len(), input.len());
    // high_pass_filter(&mut output, input.as_slice(), is_odd, filter);

    output
}
