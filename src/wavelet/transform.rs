use crate::FilterParams;

pub fn transform_1d(input: &[u16], filter: &FilterParams) -> Vec<u16> {
    let mut output = vec![0u16; input.len()];
    let (low, high) = output.split_at_mut(input.len() / 2);

    low_pass_filter(low, input);
    high_pass_filter(high, low, input, filter);

    output
}

fn low_pass_filter(data: &mut [u16], input: &[u16]) {
    let compute = |n1: usize, n2: usize| (input[n1] + input[n2]) / 2;
    let (last, elements) = data.split_last_mut().unwrap();

    for (idx, el) in elements.iter_mut().enumerate() {
        let n1 = 2 * idx;
        let n2 = (2 * idx) + 1;
        *el = compute(n1, n2);
    }

    let last_idx = (input.len() - 1) / 2;
    *last = match input.len() & 1 == 1 {
        true => *input.last().unwrap(),
        false => compute(2 * last_idx, (2 * last_idx) + 1),
    };
}

fn high_pass_filter(data: &mut [u16], low: &[u16], input: &[u16], filter: &FilterParams) {
    let compute = |n1: usize, n2: usize| (input[n1] - input[n2]) / 2;
    let (last, elements) = data.split_last_mut().unwrap();

    for (idx, el) in elements.iter_mut().enumerate() {
        let n1 = 2 * idx;
        let n2 = (2 * idx) + 1;
        *el = compute(n1, n2);
    }

    let last_idx = (input.len() - 1) / 2;
    *last = match input.len() & 1 == 1 {
        true => 0,
        false => compute(2 * last_idx, (2 * last_idx) + 1),
    };

    let r = |n: usize| (low[n - 1] - low[n]) as f32;
    let (alpha_neg, alpha_zero, alpha_one, beta) = filter.to_params();
    let compute = |n: usize| {
        let n1 = alpha_neg * r(n - 1);
        let n2 = alpha_zero * r(n);
        let n3 = alpha_one * r(n + 1);
        let n4 = beta * elements[n + 1] as f32;

        (n1 + n2 + n3 - n4 + 0.5) as u16
    };

    elements[1] -= match filter {
        FilterParams::C => {
            let n1 = r(1) / 4.0;
            let n2 = 3.0 / 4.0 * r(2);
            let n3 = elements[2] as f32 / 4.0;
            (n1 + n2 - n3 + 0.5) as u16
        }
        _ => compute(1),
    };
    elements[0] -= r(1) as u16 / 4;

    //TODO: implement for rest of values. maybe without allocating?
}

#[cfg(test)]
mod tests {
    #[test]
    fn low_pass() {
        let input = [0u16, 2, 3, 5, 6, 11, 2, 112, 552];
        let mut data = vec![0u16; input.len() / 2];
        super::low_pass_filter(&mut data, &input);
    }
}
