pub mod defining_shared_behaviour_with_traits {
    struct Dog {
        name: String,
    }

    struct Cat {
        name: String,
    }

    struct Jellyfish {
        name: String,
    }

    struct Butterfly {
        name: String,
    }

    trait Animal {
        fn talk(&self) -> String;
    }

    impl Animal for Dog {
        fn talk(&self) -> String {
            "Aaawooooo".to_string()
        }
    }

    impl Animal for Cat {
        fn talk(&self) -> String {
            "Meow".to_string()
        }
    }

    impl Animal for Jellyfish {
        fn talk(&self) -> String {
            "I can't talk".to_string()
        }
    }

    impl Animal for Butterfly {
        fn talk(&self) -> String {
            "I can't talk".to_string()
        }
    }

    pub fn defining_a_trait() {
        let dog = Dog {
            name: "Cujo".to_string()
        };

        let cat = Cat {
            name: "Coco".to_string()
        };

        let jelly = Jellyfish {
            name: "Jelly".to_string()
        };

        let butterfly = Butterfly {
            name: "Monarch".to_string()
        };

        println!("{} says {}", dog.name, dog.talk());
        println!("{} says {}", cat.name, cat.talk());
        println!("{} says {}", jelly.name, jelly.talk());
        println!("{} says {}", butterfly.name, butterfly.talk());
    }
}