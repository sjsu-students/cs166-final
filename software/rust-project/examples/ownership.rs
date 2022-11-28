struct StringOwner {
    string: String,
}

fn main() {
    let string_data: String = "Hello, World!".into();

    let owner = StringOwner {
        string: string_data,
    };

    // println!("{}", string_data); // WILL NOT COMPILE BECAUSE DATA IS NO LONGER OWNED BY `string_data`
    println!("{}", owner.string); // this works because data is `owned by owner`
}
