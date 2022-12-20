fn mix(input: &Vec<i64>, times: usize) -> Vec<i64> {
    let len = input.len() as i64;

    let mut derange = (0..input.len() as i64).collect::<Vec<i64>>();
    for _ in 0..times {
        for (i, offset) in input.iter().enumerate() {
            if *offset == 0 {
                continue;
            }

            let old_pos = derange[i];
            let new_pos = (old_pos + offset % (len - 1) + len - 1) % (len - 1);
            if old_pos < new_pos {
                for j in 0..derange.len() {
                    if derange[j] > old_pos && derange[j] <= new_pos {
                        derange[j] = (derange[j] + len - 1) % len;
                    }
                }
            } else {
                for j in 0..derange.len() {
                    if derange[j] >= new_pos && derange[j] < old_pos {
                        derange[j] = (derange[j] + len + 1) % len;
                    }
                }
            }

            derange[i] = new_pos;
        }
    }

    let mut output = vec![0; input.len()];
    for (i, value) in derange.iter().zip(input.iter()) {
        output[*i as usize] = *value;
    }

    output
}

fn solve_part_one(input: &Vec<i64>) -> i64 {
    let deranged = mix(input, 1);
    let zero_pos = deranged.iter().position(|x| *x == 0).unwrap();
    deranged[(zero_pos + 1000) % input.len()] + deranged[(zero_pos + 2000) % input.len()] + deranged[(zero_pos + 3000) % input.len()]
}

fn solve_part_two(input: &Vec<i64>) -> i64 {
    let real_input = input.iter().map(|x| x * 811589153).collect::<Vec<_>>();
    let deranged = mix(&real_input, 10);
    let zero_pos = deranged.iter().position(|x| *x == 0).unwrap();
    deranged[(zero_pos + 1000) % input.len()] + deranged[(zero_pos + 2000) % input.len()] + deranged[(zero_pos + 3000) % input.len()]
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}
