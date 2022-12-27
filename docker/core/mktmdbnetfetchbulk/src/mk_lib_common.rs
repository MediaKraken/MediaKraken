#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#![allow(unused)]

pub fn print_type_of_variable<T>(_: &T) {
    #[cfg(debug_assertions)]
    {
        println!("{}", std::any::type_name::<T>())
    }
}
