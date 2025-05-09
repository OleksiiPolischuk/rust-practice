pub fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    // Нормалізуємо зміщення в межах довжини рядка
    let shift = ((n % len) + len) % len;
    let shift = shift as usize;

    let chars: Vec<char> = s.chars().collect();
    let rotated: String = chars[len as usize - shift..]
        .iter()
        .chain(&chars[..len as usize - shift])
        .collect();

    rotated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.clone(), *n), exp.to_string())
        });
    }
}
