const HEIGHT: usize = 6; // половина ромба
const CHAR: char = '*'; // символ, який використовуємо

pub fn draw_rhombus() {
    let mut output = String::new();

    // Верхня частина ромба
    for i in 0..HEIGHT {
        let spaces = HEIGHT - i - 1;
        let stars = 2 * i + 1;

        output.push_str(&" ".repeat(spaces));
        output.push_str(&CHAR.to_string().repeat(stars));
        output.push('\n');
    }

    // Нижня частина ромба
    for i in (0..HEIGHT - 1).rev() {
        let spaces = HEIGHT - i - 1;
        let stars = 2 * i + 1;

        output.push_str(&" ".repeat(spaces));
        output.push_str(&CHAR.to_string().repeat(stars));
        output.push('\n');
    }

    print!("{}", output);
}
