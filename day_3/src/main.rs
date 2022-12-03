use std::collections::HashSet;

fn find_duplicate(s: &String) -> u8 {
    let bytes = s.as_bytes();
    let first = bytes[..bytes.len() / 2].iter().collect::<HashSet<_>>();
    *bytes[bytes.len() / 2..]
        .iter()
        .find(|c| first.contains(c))
        .unwrap()
}

fn priority(x: u8) -> i32 {
    let x = x as u32;
    if x >= 'a' as u32 && x <= 'z' as u32 {
        (x - 'a' as u32) as i32 + 1
    } else {
        (x - 'A' as u32) as i32 + 27
    }
}

fn solve_part_one(input: &Vec<String>) -> i32 {
    input.iter().map(find_duplicate).map(priority).sum()
}

fn solve_part_two(input: &Vec<String>) -> i32 {
    (0..input.len() / 3)
        .map(|i| {
            let a = input[i * 3].as_bytes().iter().collect::<HashSet<_>>();
            let b = input[i * 3 + 1].as_bytes().iter().collect::<HashSet<_>>();
            let c = input[i * 3 + 2].as_bytes();

            let anb = a.intersection(&b).collect::<HashSet<_>>();
            priority(*c.iter().find(|x| anb.contains(x)).unwrap())
        })
        .sum()
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}
