#![allow(unused)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);


    // use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);


    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);


    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 4, 5, 6, 7];
    let mut sum = 0;
    for i in &v {
        sum += i;
    }
    println!("average: {}", sum);
    println!("average: {}", sum / (v.len() as i32));

    let mut v2: Vec<i32> = Vec::new();
    for &i in &v {
        if v2.len() == 0 {
            v2.push(i);
        } else if v2[v2.len() - 1] <= i {
            v2.push(i);
        } else {
            for j in 0..v2.len() {
                if i < v2[j] {
                    v2.insert(j, i);
                    break;
                } 
            }
        }
    }
    println!("middle: {:?}", v2);
    println!("middle: {}", v2[v2.len() / 2]);

    let mut map: HashMap<i32, i32> = HashMap::new();
    for &i in &v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut number = 0;
    for (&key, &count) in &map {
        if count > max {
            number = key;
        }
    }
    println!("middle: {:?}", map);
    println!("often: {}", number);

}
