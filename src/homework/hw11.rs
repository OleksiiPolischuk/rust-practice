use rand::Rng;


pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}


pub fn min_adjacent_sum(data: &[i32]) -> (usize, i32, i32) {
    let mut min_sum = i32::MAX;
    let mut index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            index = i;
        }
    }

    (index, data[index], data[index + 1])
}


pub fn display_adjacent_min(data: &[i32], idx: usize, a: i32, b: i32) {
    // індекси
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    // дані
    print!("data:   [");
    for (i, val) in data.iter().enumerate() {
        if i < data.len() - 1 {
            print!("{}, ", val);
        } else {
            print!("{}", val);
        }
    }
    println!("]");

    // стрілка
    print!("indexes:");
    for i in 0..data.len() {
        if i == idx {
            print!("\\__");
        } else if i == idx + 1 {
            print!(" __/");
        } else {
            print!("    ");
        }
    }
    println!();

    // результат
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        a,
        b,
        a + b,
        idx,
        idx + 1
    );
}