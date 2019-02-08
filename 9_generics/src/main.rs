fn main() {
    // At compile time Rust performs monomorphization: turns generic code into specific code
    // by filling in the concrete types that are used when compiled.
    // Generics are as fast as monomorphized functions would be. 

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// Generics in function
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// Generics in structs
struct Point<T> {
    x: T,
    y: T,
}

// in method defenitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Generics in enums
enum Option<T> {
    Some(T),
    None,
}
