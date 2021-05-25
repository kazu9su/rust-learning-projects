use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("hello")),
    ];

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    println!("{}, {}", s2, s3);
    let s1 = String::from("tic");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    println!("{}, {}, {}", s1, s2, s3);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }


    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("green"), 50);
    println!("{:?}", scores);
    let teams = vec![String::from("blue"), String::from("green")];
    let initial_points = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_points.into_iter()).collect();
    println!("{:?}", scores);

    let name = String::from("hoge");
    let value = String::from("fuga");
    let mut map = HashMap::new();
    map.insert(name, value);
    //  println!("{}", name);
    let team = String::from("blue");
    let score = scores.get(&team).unwrap();
    println!("{}", score);

    for (k, v) in &scores {
        println!("{}, {}", k, v);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    let v = vec![1, 2, 3, 4, 5, 5, 5];
    let mode = mode(&v);
    println!("{}", mode);
    let mean = mean(&v);
    println!("{}", mean);
    let median = median(&v);
    println!("{}", median);
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut count = 0;
    let mut ans = 0;
    for (k, v) in &map {
        if v > &count {
            count = *v;
            ans = **k;
        }
    }

    ans
}

fn mean(v: &Vec<i32>) -> i32 {
    let mut sum = 0.0;
    for i in v {
        sum += *i as f64;
    }

    (sum / v.len() as f64) as i32
}

fn median(v: &Vec<i32>) -> i32 {
    let i = (v.len() as f64 / 2.0) as usize;

    match v.get(i) {
        Some(ret) => *ret,
        None => panic!("unexpected"),
    }
}
