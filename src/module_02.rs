pub mod using_generics_to_reduce_code_duplication {
    // fn print_vector<T: std::fmt::Display>(vec: &[T], label: &str) {
    //     print!("{}: ", label);
    //     for value in vec {
    //         print!("{} ", value)
    //     }
    //     println!();
    // }

    // The same as the above generic function
    fn print_vector<T>(vec: &[T], label: &str)
        where T: std::fmt::Display + PartialOrd {
        print!("{}: ", label);
        for value in vec {
            print!("{} ", value)
        }
        println!();
    }

    pub fn why_do_we_need_generics() {
        let ints1 = vec![1, 2, 3, 4, 5];
        let ints2 = vec![6, 7, 8, 9, 10];
        let ints3 = vec![11, 12, 13, 14, 15];

        let floats = vec![1.2, 2.3, 3.4, 4.5, 5.6];
        let strings = vec!["hello", "world", "how", "are", "you?"];

        print_vector(&ints1, "Integers list 1");
        print_vector(&ints2, "Integers list 2");
        print_vector(&ints3, "Integers list 3");

        print_vector(&floats, "Floats list");
        print_vector(&strings, "Strings list");
    }

    struct Coordinate<T, U> {
        lat: T,
        lon: U,
    }

    pub fn structs() {
        let location1 = Coordinate {
            lat: 40.712776,
            lon: -74.005974,
        };

        let location2 = Coordinate {
            lat: 40,
            lon: -74.005974,
        };

        println!("Location 1: ({}, {})", location1.lat, location1.lon);
        println!("Location 2: ({}, {})", location2.lat, location2.lon);
    }
}

pub mod using_generics_to_reduce_code_duplication_v2 {
    struct Coordinate<T> {
        lat: T,
        lon: T,
    }

    impl<T: std::fmt::Display> Coordinate<T> {
        fn print(&self) {
            println!("Latitude: {}, Longitude: {}", self.lat, self.lon);
        }
    }

    impl Coordinate<f64> {
        fn distance(&self, other: &Coordinate<f64>) -> f64 {
            let lat1 = self.lat.to_radians();
            let lon1 = self.lon.to_radians();

            let lat2 = other.lat.to_radians();
            let lon2 = other.lon.to_radians();

            let dlat = lat2 - lat1;
            let dlon = lon2 - lon1;

            let a = (dlat / 2.0).sin().powi(2) + lat1.cos()
                * lat2.cos() * (dlon / 2.0).sin().powi(2);
            let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

            3961.0 * c
        }
    }

    pub fn methods() {
        let new_york = Coordinate { lat: 40.712776, lon: -74.005974 };
        let los_angeles = Coordinate { lat: 34.0499998, lon: -118.249999 };

        new_york.print();
        los_angeles.print();
        println!("Distance between New York and Los Angeles: {}", new_york.distance(&los_angeles));

        let loc = Coordinate { lat: 34, lon: -118 };
        loc.print();
    }
}