use crate::FilterParams;

#[inline]
const fn is_odd(len: usize) -> bool {
    len & 1 == 1
}

pub fn wavelet_transform(buf: &mut [u16], input: &[u16], width: usize, filter: FilterParams) {
    buf.chunks_exact_mut(width)
        .for_each(|row| transform_1d(row, input, filter));
}

fn transform_1d(data: &mut [u16], input: &[u16], filter: FilterParams) {
    let half = data.len() / 2;
    let (low, high) = data.split_at_mut(half);

    lowpass(low, input);
    highpass(high, low, input, filter);
}

fn lowpass(data: &mut [u16], input: &[u16]) {
    let compute = |n: usize| (input[2 * n] + input[(2 * n) + 1]) / 2;
    let (last, elements) = data.split_last_mut().unwrap();

    elements
        .iter_mut()
        .enumerate()
        .for_each(|(idx, el)| *el = compute(idx));

    *last = match is_odd(input.len()) {
        true => *input.last().unwrap(),
        false => compute((input.len() - 1) / 2),
    };
}

fn highpass(data: &mut [u16], low: &[u16], input: &[u16], filter: FilterParams) {
    {
        let compute = |n: usize| input[2 * n].saturating_sub(input[(2 * n) + 1]);
        let (last, elements) = data.split_last_mut().unwrap();

        elements
            .iter_mut()
            .enumerate()
            .for_each(|(idx, el)| *el = compute(idx));

        *last = match is_odd(input.len()) {
            true => *input.last().unwrap(),
            false => compute((input.len() - 1) / 2),
        };
    }; // drop references

    let r = |n: usize| (low[n - 1].saturating_sub(low[n])) as f32;
    let (alpha_neg, alpha_zero, alpha_one, beta) = filter.to_params();

    // 'data_val' should be the value of data[n + 1], passed as a parameter to force a cheap copy and
    // avoid allocating in this function.
    // TODO: Look for idiomatic way to accomplish the same thing
    let compute = |n: usize, data_val: u16| {
        let n1 = alpha_neg * r(n - 1);
        let n2 = alpha_zero * r(n);
        let n3 = alpha_one * r(n + 1);
        let n4 = beta * data_val as f32;

        (n1 + n2 + n3 - n4 + 0.5) as u16
    };

    data[0] = data[0].saturating_sub(r(1) as u16 / 4);
    data[1] = data[1].saturating_sub(match filter {
        FilterParams::C => {
            let n1 = r(1) / 4.0;
            let n2 = 3.0 / 4.0 * r(2);
            let n3 = data[2] as f32 / 4.0;
            (n1 + n2 - n3 + 0.5) as u16
        }
        _ => compute(2, data[2]),
    });

    for idx in 2..(data.len() - 1) {
        data[idx] = compute(idx, data[idx + 1]);
    }

    let last_idx = (input.len() / 2) - 1;
    *data.last_mut().unwrap() = match is_odd(input.len()) {
        true => compute(last_idx, data[last_idx + 1]),
        false => r(last_idx) as u16 / 4,
    };
}
