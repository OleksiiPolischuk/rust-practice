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
