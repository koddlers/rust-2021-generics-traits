pub mod using_advanced_traits {
    trait Incrementor {
        type Item;

        fn increment(&mut self) -> Self::Item;
    }

    struct Counter {
        count: u32
    }

    impl Incrementor for Counter {
        type Item = u32;

        fn increment(&mut self) -> Self::Item {
            self.count += 1;
            self.count
        }
    }

    pub fn associated_types() {
        let mut counter = Counter { count: 0};
        for _ in 0..5 {
            println!("Counter: {}", counter.increment());
        }
    }
}