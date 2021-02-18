fn main() {
    println!("Hello, world!");

    // integer types 
    let a: i8 = 125;
    let b: u8 = 250;
    let c: i16 = 257;
    // all types have their signed counterpart
    let d: u16 = 17000;
    let e: u32 = 3453245234;
    let f: u64 = 3452345234523;
    let g: u128 = 5345345345432425;
    let h: usize = 234;

    // hex value
    let hex = 0xf5;
    // octal value
    let oct = 0o17;
    // binary value
    let bin = 0b1111_0101;
    // byte value (u8)
    let byte = b'A';

    // bool
    let t = true;
    let f = false;

    // char
    let ch = 'z';

    // Compount types
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1); // the types can be infered 
    let (x, y, z) = tup; // destructing 
    // indexing 
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    // array
    let arr = [1, 2, 3, 4, 5, 6];
    let arr2: [i32; 5] = [1 ,2 , 3, 4, 5];
    let arr3 = [3; 5]; // equal to [3, 3, 3, 3, 3]
    // indexing
    let first = arr[0];
    let second = arr[1];
}
