use std::collections::HashSet;

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn test_data() -> Vec<Rectangle> {
    let r1 = Rectangle {
        a: Point { x: 2, y: 9 },
        b: Point { x: 5, y: 3 },
    };

    let r2 = Rectangle {
        a: Point { x: 1, y: 8 },
        b: Point { x: 11, y: 6 },
    };

    let r3 = Rectangle {
        a: Point { x: 9, y: 10 },
        b: Point { x: 13, y: 2 },
    };

    let rectangles = vec![r1, r2, r3];
    return rectangles;
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut cells = HashSet::new();

    for rect in xs {
        let mut left = 0;
        let mut right = 0;
        let mut top = 0;
        let mut bottom = 0;

        if rect.a.x < rect.b.x {
            left = rect.a.x;
            right = rect.b.x;
        } else {
            left = rect.b.x;
            right = rect.a.x;
        }

        if rect.a.y < rect.b.y {
            top = rect.a.y;
            bottom = rect.b.y;
        } else {
            top = rect.b.y;
            bottom = rect.a.y;
        }

        let mut x = left;
        while x < right {
            let mut y = top;
            while y < bottom {
                cells.insert((x, y));
                y = y + 1;
            }
            x = x + 1;
        }
    }

    let count = cells.len();
    return count as i32;
}

fn area_occupied_test() {
    let rects = test_data();
    let result = area_occupied(&rects);
    println!("Occupied area: {}", result);
    assert_eq!(result, 60);
}

fn main() {
    area_occupied_test();
}
