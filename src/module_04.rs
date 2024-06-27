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
}