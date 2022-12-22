enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Vec2i {
    x: i32,
    y: i32,
}

enum Tile {
    Open,
    Wall,
}

struct Input {
    tiles: Vec<Vec<Tile>>,
}

fn solve_part_one(input: &Vec<i32>) -> solve::Unimplemented {
    solve::Unimplemented
}

fn solve_part_two(input: &Vec<i32>) -> solve::Unimplemented {
    solve::Unimplemented
}

fn main() {
    solve::main(solve_part_one, solve_part_two)
}
