const WIDTH: usize = 30;
const HEIGHT: usize = 16;

fn main() {
    let mut output = String::new();

    // Верхня межа
    output.push_str(&("*".repeat(WIDTH) + "\n"));

    // Малювання верхньої половини
    for i in 0..(HEIGHT / 2 - 1) {
        output.push('*');
        output.push_str(&" ".repeat(2 * i));
        output.push('*');
        output.push_str(&" ".repeat(WIDTH - 4 - 4 * i));
        output.push('*');
        output.push_str(&" ".repeat(2 * i));
        output.push('*');
        output.push('\n');
    }

    // Центр конверта
    output.push('*');
    output.push_str(&" ".repeat(WIDTH / 2 - 2));
    output.push_str("**");
    output.push_str(&" ".repeat(WIDTH / 2 - 2));
    output.push('*');
    output.push('\n');

    // Нижня половина 
    for i in (0..(HEIGHT / 2 - 1)).rev() {
        output.push('*');
        output.push_str(&" ".repeat(2 * i));
        output.push('*');
        output.push_str(&" ".repeat(WIDTH - 4 - 4 * i));
        output.push('*');
        output.push_str(&" ".repeat(2 * i));
        output.push('*');
        output.push('\n');
    }

    // Нижня межа
    output.push_str(&("*".repeat(WIDTH) + "\n"));

    // Вивід
    print!("{}", output);
}
 
