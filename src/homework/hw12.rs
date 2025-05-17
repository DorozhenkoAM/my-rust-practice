fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let n = shipments.len() as u32;
    let total: u32 = shipments.iter().sum();

    if total % n != 0 {
        return None;
    }

    let average = total / n;
    let mut moves = 0;

    for &load in shipments.iter() {
        if load > average {
            moves += (load - average) as usize;
        }
    }

    Some(moves)
}

fn main() {
    let shipments1 = vec![8, 2, 2, 4, 4];
    let shipments2 = vec![9, 4, 7, 2, 9];

    match count_permutation(&shipments1) {
        Some(moves) => println!("1. Average cargo for 5 ships: {}", moves),
        None => println!("1. impossible to divide equally"),
    }

    match count_permutation(&shipments2) {
        Some(moves) => println!("2. Average cargo for 5 ships: {}", moves),
        None => println!("2. impossible to divide equally"),
    }
}