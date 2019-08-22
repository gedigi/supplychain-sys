pub fn foo() {
    println!("good code!");
    println!("bad code!");
    println!("evil code!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
