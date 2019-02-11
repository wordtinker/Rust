trait InTrait {}
trait OutTrait {}

struct In {}
struct Out {}

impl OutTrait for Out {}

fn foo(arg: impl InTrait) -> impl OutTrait {
    Out {}
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn but_if_types_are_different(val: bool) -> Box<dyn Fn(i32) -> i32> {
    if val {
        Box::new(|x| x + 1)
    } else {
        Box::new(|x| x + 1)
    }
}

fn main() {
    println!("Hello, world!");
}
