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
        println!("{} says {}\n", butterfly.name, butterfly.talk());
    }
}

pub mod defining_shared_behaviour_with_traits_v2 {
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
        fn talk(&self) -> String {
            "I can't talk".to_string()
        }
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

    impl Animal for Jellyfish {}

    impl Animal for Butterfly {}

    pub fn default_implementations() {
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

        println!("With default implementation:");
        println!("{} says {}", dog.name, dog.talk());
        println!("{} says {}", cat.name, cat.talk());
        println!("{} says {}", jelly.name, jelly.talk());
        println!("{} says {}\n", butterfly.name, butterfly.talk());
    }
}

pub mod defining_shared_behaviour_with_traits_v3 {
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
        fn talk(&self) -> String {
            "I can't talk".to_string()
        }
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

    impl Animal for Jellyfish {}

    impl Animal for Butterfly {}

    fn make_dog_talk(dog: &Dog) {
        println!("This dog says: {}", dog.talk());
    }

    fn make_animal_talk(animal: &(impl Animal + std::fmt::Display)) {
        println!("This animal says: {}", animal.talk());
    }

    fn make_generic_talk<T: Animal>(animal: &T) {
        println!("This animal says: {}", animal.talk());
    }

    fn make_generic_talk_v2<T>(animal: &T) where T: Animal + std::fmt::Display {
        println!("This animal says: {}", animal.talk());
    }

    fn make_animal_zoo(
        animal1: &impl Animal,
        animal2: &impl Animal,
        animal3: &impl Animal,
        animal4: &impl Animal,
    ) {}

    // courtesy of copilot
    impl std::fmt::Display for Cat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Cat named {}", self.name)
        }
    }

    pub fn traits_as_parameters_and_trait_bounds() {
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

        make_dog_talk(&dog);

        make_animal_talk(&cat);
        // make_animal_talk(&jelly);
        // make_animal_talk(&butterfly);

        make_generic_talk(&dog);
        make_generic_talk_v2(&cat)
    }
}

pub mod defining_shared_behaviour_with_traits_v4 {
    struct Dog {
        name: String,
    }

    struct Cat {
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

    fn return_talking_animal() -> impl Animal {
        Cat {
            name: "Cujo".to_string()
        }
    }

    fn return_talking_animal_box(dog: bool) -> Box<dyn Animal> {
        if dog {
            Box::new(Dog {
                name: "Cujo".to_string(),
            })
        } else {
            Box::new(Cat {
                name: "Felix".to_string()
            })
        }
    }

    pub fn traits_as_return_types() {
        let animal = return_talking_animal();
        println!("{}", animal.talk());

        let dog = return_talking_animal_box(true);
        println!("{}", dog.talk());

        let cat = return_talking_animal_box(false);
        println!("{}", cat.talk());
    }
}