#![allow(dead_code)]

pub mod file_info;

pub mod print_type{
    pub fn print<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
