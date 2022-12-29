#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#![allow(unused)]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

pub fn print_type_of_variable<T>(_: &T) {
    #[cfg(debug_assertions)]
    {
        println!("{}", std::any::type_name::<T>())
    }
}
