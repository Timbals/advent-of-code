use std::collections::HashMap;

pub fn solve_first(input: &str) -> usize {
    let devices = input
        .lines()
        .map(|line| {
            let (device, outputs) = line.split_once(": ").unwrap();
            let outputs = outputs.split(' ').collect::<Vec<_>>();
            (device, outputs)
        })
        .collect::<HashMap<_, _>>();

    let mut stack = vec!["you"];
    let mut count = 0;
    while let Some(device) = stack.pop() {
        if device == "out" {
            count += 1;
        }

        for &output in devices.get(device).iter().copied().flatten() {
            stack.push(output);
        }
    }

    count
}

pub fn solve_second(input: &str) -> usize {
    let mut next_index = 0;
    let mut device_indices = HashMap::new();
    let device_to_outputs = input
        .lines()
        .map(|line| {
            let (device, outputs) = line.split_once(": ").unwrap();
            let outputs = outputs.split(' ').collect::<Vec<_>>();

            device_indices.entry(device).or_insert_with(|| {
                let index = next_index;
                next_index += 1;
                index
            });
            for &v in &outputs {
                device_indices.entry(v).or_insert_with(|| {
                    let index = next_index;
                    next_index += 1;
                    index
                });
            }

            (device, outputs)
        })
        .collect::<HashMap<_, _>>();
    let mut devices = vec![vec![]; next_index];
    for (k, v) in device_to_outputs {
        devices[device_indices[k]] =
            v.into_iter().filter_map(|v| device_indices.get(v).copied()).collect();
    }
    let mut reverse_devices = vec![vec![]; devices.len()];
    for (device, outputs) in devices.iter().enumerate() {
        for &output in outputs {
            reverse_devices[output].push(device);
        }
    }

    let svr = device_indices["svr"];
    let out = device_indices["out"];
    let dac = device_indices["dac"];
    let fft = device_indices["fft"];

    let mut topological_sort = Vec::with_capacity(devices.len());
    let mut no_remaining_edges = vec![out];
    let mut remaining_edges = vec![usize::MAX; devices.len()];
    for (k, v) in devices.iter().enumerate() {
        remaining_edges[k] = v.len();
    }
    while let Some(device) = no_remaining_edges.pop() {
        topological_sort.push(device);
        for &input in &reverse_devices[device] {
            remaining_edges[input] -= 1;
            if remaining_edges[input] == 0 {
                no_remaining_edges.push(input);
            }
        }
    }

    let mut cache = vec![vec![0; 4]; devices.len()];
    cache[out][0b11] = 1;
    for device in topological_sort {
        for &output in &devices[device] {
            if device == dac {
                cache[device][0b00] += cache[output][0b01];
                cache[device][0b10] += cache[output][0b11];
            } else if device == fft {
                cache[device][0b00] += cache[output][0b10];
                cache[device][0b01] += cache[output][0b11];
            } else {
                cache[device][0b00] += cache[output][0b00];
                cache[device][0b01] += cache[output][0b01];
                cache[device][0b10] += cache[output][0b10];
                cache[device][0b11] += cache[output][0b11];
            }
        }
    }

    cache[svr][0b00]
}

#[test]
pub fn sample() {
    assert_eq!(5, solve_first(include_str!("sample1.txt")));
    assert_eq!(2, solve_second(include_str!("sample2.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(555, solve_first(input));
    assert_eq!(502447498690860, solve_second(input));
}
