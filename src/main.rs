#![allow(unused)]

mod module_02;
mod module_03;
mod module_04;

use module_02::using_generics_to_reduce_code_duplication;
use module_02::using_generics_to_reduce_code_duplication_v2;
use module_03::defining_shared_behaviour_with_traits;
use module_03::defining_shared_behaviour_with_traits_v2;
use module_03::defining_shared_behaviour_with_traits_v3;
use module_03::defining_shared_behaviour_with_traits_v4;
use module_04::using_advanced_traits;


fn main() {
    // Module 02 - Using Generics to Reduce Code Duplication
    // using_generics_to_reduce_code_duplication::why_do_we_need_generics();
    // using_generics_to_reduce_code_duplication::structs();
    // using_generics_to_reduce_code_duplication_v2::methods();
    // using_generics_to_reduce_code_duplication_v2::enums();

    // Module 03 - Defining Shared Behaviour with Traits
    // defining_shared_behaviour_with_traits::defining_a_trait();
    // defining_shared_behaviour_with_traits_v2::default_implementations();
    // defining_shared_behaviour_with_traits_v3::traits_as_parameters_and_trait_bounds();
    // defining_shared_behaviour_with_traits_v4::traits_as_return_types();

    // Module 04 - Using Advanced Traits
    // using_advanced_traits::associated_types();
    using_advanced_traits::default_generic_type_parameters_and_operator_overloading();
}
