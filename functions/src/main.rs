fn main() {
    println!("Hello, world!");
    another_function();
    print_number(12);
    print_two_numbers(1, 2);
}

fn print_two_numbers(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn print_number(number: i32) {
    println!("The value of number is {}", number);
}

fn another_function() {
    println!("Another function.");
}