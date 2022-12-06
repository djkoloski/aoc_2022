fn first_disjoint_substring(input: &[u8], window: usize) -> Option<usize> {
    let mut counts = [0; 26];
    let mut unique = 0;

    for n in 0..input.len() {
        let i = (input[n] - b'a') as usize;
        if counts[i] == 0 {
            unique += 1;
        }
        counts[i] += 1;
        if n >= window {
            let o = (input[n - window] - b'a') as usize;
            if counts[o] == 1 {
                unique -= 1;
            }
            counts[o] -= 1;
        }
        if unique == window {
            return Some(n + 1);
        }
    }

    None
}

fn solve_part_one(input: &String) -> usize {
    first_disjoint_substring(input.as_bytes(), 4).unwrap()
}

fn solve_part_two(input: &String) -> usize {
    first_disjoint_substring(input.as_bytes(), 14).unwrap()
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}
