fn main() {

    let mut v: Vec<i32> = Vec::new();
    let mut vc = vec![1, 2, 3, 4, 5];
    v.push(5);
    v.push(10);
    vc.push(10);

    let third: &i32 = &vc[2]; // panic on out of bound error
    let slice = &vc[1..=3];
    println!("{:?}", third);
    println!("{:?}", slice);

    match vc.get(2) { // None on out of bound error
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}
