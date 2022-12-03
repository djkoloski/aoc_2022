use solve::Grouped;

fn part_one(input: &Grouped<i32>) -> i32 {
    input.groups.iter().map(|g| g.iter().sum()).max().unwrap()
}

fn part_two(input: &Grouped<i32>) -> i32 {
    let mut totals = input
        .groups
        .iter()
        .map(|g| g.iter().sum())
        .collect::<Vec<_>>();
    totals.sort();
    totals.iter().rev().take(3).sum()
}

fn main() {
    solve::main(part_one, part_two);
}
