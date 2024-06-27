#![allow(unused)]

mod module_02;
mod module_03;

use module_02::using_generics_to_reduce_code_duplication;
use module_02::using_generics_to_reduce_code_duplication_v2;

fn main() {
    // using_generics_to_reduce_code_duplication::why_do_we_need_generics();
    // using_generics_to_reduce_code_duplication::structs();
    // using_generics_to_reduce_code_duplication_v2::methods();
    using_generics_to_reduce_code_duplication_v2::enums();
}
