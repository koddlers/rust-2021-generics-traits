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
}