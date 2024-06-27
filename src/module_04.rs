pub mod using_advanced_traits {
    trait Incrementor {
        type Item;

        fn increment(&mut self) -> Self::Item;
    }

    struct Counter {
        count: u32,
    }

    impl Incrementor for Counter {
        type Item = u32;

        fn increment(&mut self) -> Self::Item {
            self.count += 1;
            self.count
        }
    }

    pub fn associated_types() {
        let mut counter = Counter { count: 0 };
        for _ in 0..5 {
            println!("Counter: {}", counter.increment());
        }
    }

    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Coordinate {
        lat: f64,
        lon: f64,
    }

    impl Add for Coordinate {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                lat: self.lat + rhs.lat,
                lon: self.lon + rhs.lon,
            }
        }
    }

    pub fn default_generic_type_parameters_and_operator_overloading() {
        let coordinate1 = Coordinate { lat: 1.0, lon: 2.0 };
        let coordinate2 = Coordinate { lat: 3.0, lon: 4.0 };
        let coordinate3 = Coordinate { lat: 4.0, lon: 6.0 };

        // assert_eq!(coordinate1 + coordinate2, coordinate3);

        if coordinate1 + coordinate2 == coordinate3 {
            println!("{}", true);
        } else {
            println!("{}", false);
        }
    }

    trait Printer {
        fn write(&self);
    }

    trait File {
        fn write(&self);
    }

    struct Logger;

    impl Logger {
        fn write(&self) {
            println!("We write this to the screen");
        }
    }

    impl Printer for Logger {
        fn write(&self) {
            println!("Write this to the printer");
        }
    }

    impl File for Logger {
        fn write(&self) {
            println!("Write this to the file");
        }
    }

    pub fn fully_qualified_syntax() {
        let logger = Logger;
        logger.write();

        Printer::write(&logger);
        File::write(&logger);
        Logger::write(&logger);

        <Logger as Printer>::write(&logger);
    }
}

pub mod using_advanced_traits_v2 {
    // Define a supertrait
    trait Speak {
        fn speak(&self);
    }

    // Define a trait that depends on the supertrait
    trait Greet: Speak {
        fn greet(&self) {
            self.speak();    // call a method from supertrait
            println!("Nice to meet you!");
        }
    }

    struct Person {
        name: String,
    }

    // Implement the supertrait `Speak` for the `Person` struct
    impl Speak for Person {
        fn speak(&self) {
            println!("Hello my name is {}", self.name);
        }
    }

    // Implement the dependant trait `Greet` for `Person`
    // Note: If `Person` implements `Greet`, it also has to implement the supertrait `Speak`,
    // commenting out the above implementation will cause a panic
    impl Greet for Person {}

    pub fn supertraits() {
        let person = Person { name: String::from("Shaun") };
        // the following calls both `speak()` from supertrait and `greet()` from the dependant trait
        person.greet();
    }
}