use std::collections::HashMap;

fn main() {
    // hashmap<K, V>
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name)
        .expect("Wrong key");
    println!("Team: {} -> {}", team_name, score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // Updateting values
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    //
    scores.insert(String::from("Blue"), 25);
    // Insert if key has no value
    scores.entry(String::from("Yellow")).or_insert(500);
    scores.entry(String::from("Blue")).or_insert(500);
    let val = scores.entry(String::from("Green")).or_insert(500);
    // can use this reference to mutate value
    // deref and change
    *val += 1;
    //
    println!("{:?}", scores);
}
