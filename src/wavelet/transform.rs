use crate::FilterParams;

fn low_pass_filter(data: &mut [u32], input: &[u8]) {
    let odd = input.len() & 1 == 1;
    let (last, elements) = data.split_last_mut().unwrap();

    for (idx, el) in elements.iter_mut().enumerate() {
        let n1 = input[2 * idx] as u32;
        let n2 = input[(2 * idx) + 1] as u32;

        *el = (n1 + n2) / 2;
    }

    *last = if odd {
        *input.last().unwrap() as u32
    } else {
        let n1 = input[input.len() - 2] as u32;
        let n2 = input[input.len() - 1] as u32;

        (n1 + n2) / 2
    };
}

fn high_pass_filter(data: &mut [u32], low_outputs: &[u32], input: &[u8], filter: &FilterParams) {
    let odd = input.len() & 1 == 1;
    let (a_neg, a0, a1, b) = filter.to_params();
    let (last, elements) = data.split_last_mut().unwrap();

    /* START PHASE 1 */

    for (idx, el) in elements.iter_mut().enumerate() {
        let n1 = input[2 * idx] as u32;
        let n2 = input[(2 * idx) + 1] as u32;

        *el = n1 - n2;
    }

    *last = if odd {
        0
    } else {
        let n1 = input[input.len() - 2] as u32;
        let n2 = input[input.len() - 1] as u32;

        n1 - n2
    };

    let r: Vec<u32> = low_outputs
        .iter()
        .enumerate()
        .skip(1)
        .map(|(idx, _)| low_outputs[idx - 1] - low_outputs[idx])
        .collect();

    /* END PHASE 1 */

    elements[0] -= r[1] / 4;

    if *filter == FilterParams::C {
        let n1 = r[1] / 4;
        let n2 = r[2] * 3 / 8;
        let n3 = elements[2] / 4;

        elements[1] -= ((n1 + n2 - n3) as f32 + 0.5) as u32;
    }

    *last -= if odd {
        let idx = input.len() - 1;
        let n1 = a_neg * r[idx - 1] as f32;
        let n2 = a0 * r[idx] as f32;
        let n3 = a1 * r[idx + 1] as f32;
        let n4 = b * elements[idx + 1] as f32;

        (n1 + n2 + n3 - n4 + 0.5) as u32
    } else {
        r[input.len() / 2 - 1] / 4
    };
}

pub fn transform_1d(input: &[u8], filter: &FilterParams) -> Vec<u32> {
    let mut output: Vec<u32> = input.iter().copied().map(|el| el as u32).collect();
    let (low, high) = output.split_at_mut(input.len() / 2);

    low_pass_filter(low, input);
    high_pass_filter(high, low, input, filter);

    output
}
