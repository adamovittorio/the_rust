fn main() {
    // mutability
    let mut mutable = 2;
    println!("this is the original: {}", mutable);
    mutable = 7;
    println!("this is after the mutation: {}", mutable);

    // shadowing
    let _shadow = 1;
    let _shadow = 1 * 10;
    println!("this is shadowing: {}", _shadow);

    // scalars
    let _int: i32 = 1_200;
    let _boolean: bool = true;
    let _float: f32 = 6.0;
    let _charr: char = '😁';


    let tup = (500, 5.6, 'h');
    let (x, _y, _z) = tup;
    println!("x is: {}", x)

}
