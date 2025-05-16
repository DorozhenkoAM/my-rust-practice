//типу рандомний генератор
fn gen_fake_random_vector(n: usize) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut seed: u64 = 12345;

    for _ in 0..n {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345) % 1_000_000;  //якщо чесно то навіть не уявляю як робити генератор випадкових числ
        let num = 10 + (seed % 90) as i32;                                //випадковий як вибори в караїни терориста
        vec.push(num);
    }

    vec
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, min_sum)
}

fn print_data(data: &[i32], min_index: usize) {
    // Індекси та значення, що може бути кращим.
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();
    print!("data:   ");
    for n in data {
        print!("{:>3},", n);
    }
    println!();

    // знак
    print!("indexes:");
    for i in 0..data.len() {
        if i == min_index {
            print!("\\__");
        } else if i == min_index + 1 {
            print!(" __/");
        } else {
            print!("    ");
        }
    }
    println!();

    // результвт
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_index],
        data[min_index + 1],
        data[min_index] + data[min_index + 1],
        min_index,
        min_index + 1
    );
}

fn main() {
    let data = gen_fake_random_vector(20);
    let (min_index, _) = min_adjacent_sum(&data);
    print_data(&data, min_index);
}