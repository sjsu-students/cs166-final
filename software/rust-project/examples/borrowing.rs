struct StringBorrower<'a> {
    string: &'a mut String,
}

fn main() {
    let mut string_data: String = "Hello, ".into();
    string_data += "World!"; // strings can be concatenated

    let borrower = StringBorrower {
        string: &mut string_data,
    };

    // println!("{}", string_data); // NO LONGER COMPILES BECAUSE MUTABLE ACCESS CANNOT BE SHARED
    println!("{}", borrower.string); // this still works because data is 'borrowed'

    // THIS WILL WORK NOW THOUGH SINCE WE HAVE MUTABLE ACCESS
    *borrower.string += "test";
}
