use std::fmt::Display;

fn main() {
    let number_list = vec![10, 20, 30, 1000, 40];

    let result = largest(&number_list);
    println!("The largest number: {}", result);

    let char_list = vec!['y', 'm', 'k', 'a'];
    let result = largest(&char_list);
    println!("The largest char: {}", result);

    let p = Point { x: 5, y: 10.0 };
    let p2 = Point { x: 'c', y: "hell" };
    let p3 = p.mixup(p2);

    println!("{}, {}", p3.x, p3.y);

    let pair = Pair::new(100, 200);
    pair.cmp_display();

    let str1 = String::from("longer than str2");
    let res;
    {
        let str2 = String::from("hogehogehogehogehoge");

        res = longer(str1.as_str(), str2.as_str());
    }
    println!("The longer str is {}", res);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
