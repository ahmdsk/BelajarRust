pub fn data_type() {
    // integer types
    let uint8: u8 = 255;
    let int8: i32 = 1000000000;
    // let uint16: u16 = 255;
    // let uint32: u32 = 255;
    // let uint64: u64 = 255;

    println!("{uint8}");
    println!("{int8}");

    // tuple
    let tup = (200, 2.4, 1);
    println!("{}", tup.0);

    // array
    let arr1 = [1, 2, 3, 4, 5];
    println!("{}", arr1[0]);

    let arr2 = [5; 5];
    println!("{}", arr2[4]);
}