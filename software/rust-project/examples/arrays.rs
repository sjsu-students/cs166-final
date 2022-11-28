#[allow(unused_variables)]

fn main() {
    let array = [1, 2, 3, 4, 5]; // array type is [i32; 5]

    // WILL NOT COMPILE BECAUSE COMPILER IS DOING BOUNDS CHECKS
    // array[5] = 6;
}
