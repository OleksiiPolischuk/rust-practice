// Функція рахує мінімальну кількість переміщень
pub fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();

    // Якщо вага не ділиться на кількість кораблів — неможливо вирівняти
    if total as usize % n != 0 {
        return -1;
    }

    let avg = total / n as u32;
    let mut moves = 0;
    let mut balance: i32 = 0;

    for &weight in shipments {
        balance += weight as i32 - avg as i32;
        moves += balance.abs();
    }

    moves as isize
}

// Функція генерує список ваги, яку можна вирівняти
pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![4; n];
    if n >= 2 {
        shipments[0] += 2;
        shipments[1] -= 2;
    }
    shipments
}

// Тести
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_permutation() {
        assert_eq!(count_permutation(&vec![1, 1, 1, 1, 6]), -1);
        assert_eq!(count_permutation(&vec![8, 2, 2, 4, 4]), 4);
        assert_eq!(count_permutation(&vec![9, 3, 7, 2, 9]), 7);
    }

    #[test]
    fn test_gen_shipments() {
        let generated = gen_shipments(5);
        let result = count_permutation(&generated);
        assert!(result >= 0); // Перевірка, що розподіл можливий
    }
}

/*
==================== ВІДПОВІДІ НА ПИТАННЯ ====================

3. Чи завжди можливо всі кораблі забезпечити однаковою кількістю грузу?
Ні, не завжди. Якщо сума вантажу не ділиться на кількість кораблів без остачі, 
то рівномірно розподілити вантаж неможливо.

4. Як буде виглядати сигнатура в іншому випадку?
У випадку, коли розподіл може бути неможливим, функція може повертати isize, де -1 — ознака неможливості:
fn count_permutation(shipments: &Vec<u32>) -> isize

5. Функція генерації Vec<u32>, які можна вирівняти:
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![4; n];
    if n >= 2 {
        shipments[0] += 2;
        shipments[1] -= 2;
    }
    shipments
}
У такому випадку вага все ще рівномірно розподіляється при перерозподілі.

6. Пояснення прикладів:
[8, 2, 2, 4, 4]
total = 20, average = 4
Рухи:
- з 1-го корабля забираємо 4, передаємо по 2 одиниці на 2-й і 3-й → 4 переміщення

[9, 3, 7, 2, 9]
total = 30, average = 6
Рухи:
- з 1-го забираємо 3, віддаємо 2-му
- 3-й забирає 1 і передає 4-му
- 5-й забирає 3 і передає 4-му
→ 7 переміщень

==============================================================
*/
