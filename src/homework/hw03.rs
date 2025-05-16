fn main() {
    const WIDTH: usize = 50; // Ширина
    const HEIGHT: usize = 20; // Висота

    let mut result = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0 || y == HEIGHT - 1 {
                result.push('*');
            } else if x == 0 || x == WIDTH - 1 {
                result.push('*');
            } else if x == y || x == WIDTH - y - 1 {
                result.push('*');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    print!("{}", result);
}