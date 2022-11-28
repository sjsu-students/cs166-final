fn main() {
    let mut string: String = "Hello".into();

    let same_string: &mut String;
    unsafe {
        let string_ptr = string.as_mut_ptr() as *mut String;
        same_string = &mut *string_ptr;
    }

    // It can be done to have multiple mutable access, but it is unsafe
    println!("{}", string);
    println!("{}", same_string);
}
