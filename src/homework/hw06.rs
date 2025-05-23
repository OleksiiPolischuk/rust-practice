fn draw_tree(levels: usize) {
    let mut height = 1;

    for level in 1..=levels {
        for row in 0..level {
            let stars = 2 * row + 1;
            let spaces = levels + levels - row;
            let line = " ".repeat(spaces) + &"*".repeat(stars);
            println!("{}", line);
        }
    }
}

fn main() {
    draw_tree(5);
}
