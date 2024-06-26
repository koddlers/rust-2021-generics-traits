pub mod using_generics_to_reduce_code_duplication {
    fn print_vector(vec: &[i32], label: &str) {
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

        print_vector(&ints1, "Integers list 1");
        print_vector(&ints2, "Integers list 2");
        print_vector(&ints3, "Integers list 3");
    }
}