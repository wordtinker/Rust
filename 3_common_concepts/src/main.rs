fn main() {
    // immutable
    let x = 5;
    // shadowing, temporal mutability
    let x = x + 1;
    println!("The value of x: {}", x);
    // mutable
    let mut y = 6;
    println!("The value of y: {}", y);
    y = 7;
    println!("The value of y: {}", y);
    // constant is immmutable ever
    const MAX_POINTS: u32 = 100_000;
    println!("The value of const: {}", MAX_POINTS);
    // Scalar types: integers, floating point numbers
    // booleans, characters
    let i: i32 = 32;
    let ui: u32 = 32;
    let f = 2.0; // f64
    let f32: f32 = 3.0; // f32
    let tr = true;
    let fs: bool = false; // can we explicitly convert?
    let chr = 'z'; // unicode
    // Compound types
    // 1.0 Tuple
    let tup = (500, 6.4, true);
    let (_, second, _) = tup; // deconstruction
    println!("The value inside the tuple is: {}", second);
    println!("The first value of the tuple is: {}", tup.0); // direct access
    // 2.0 Array
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // Function call
    another_function(10);
    another_function(five());
    another_function(plus_one(5));
    another_function(conditional(1));
    another_function(conditional(7));
    another_function(conditional(20));
    loopy();
    while_loop();
	for_loop();
}

// Declaring a function
fn another_function(x: i32) {
    println!("Another function. {}", x);
}

fn five() -> i32 {
    5 // the last line implicitly returns
}

fn plus_one(x: i32) -> i32 {
    // return 10; // can explicitly return early
    x + 1
}

fn conditional(x: i32) -> i32 {
    // expression if
    // if ecaluates to some result and then function expression
    // returns that result
    if x < 5 {
        x
    } else if x < 10 {
        x + 1
    } else {
        x + 2
    }
}

fn loopy() {
    let mut counter = 0;

    let result = loop {
        println!("loop");
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
