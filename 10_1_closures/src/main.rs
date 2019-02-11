use std::thread;
use std::time::Duration;
use std::collections::HashMap;

trait Value {
    fn value(&mut self, arg: u32) -> u32;
}

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    values: HashMap<u32, u32>
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }
}

impl<T> Value for Cacher<T> where T: Fn(u32) -> u32 {
    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    // heavy function
    let sleep = 2;
    let mut cache =  Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(sleep));
        num
    });

    generate_workout(&mut cache, 20, 3);
    generate_workout(&mut cache, 20, 3);

    generate_workout(&mut cache, 30, 3);
    generate_workout(&mut cache, 30, 3);

    generate_workout(&mut cache, 30, 4);
    generate_workout(&mut cache, 30, 4);
}

fn generate_workout(cache: &mut impl Value, intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            cache.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            cache.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                cache.value(intensity)
            );
        }
    }
}
