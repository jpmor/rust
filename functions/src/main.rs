fn main() {
    let mut sum: i32 = add_numbers(5, 6);

    println!("The sum squared is: {}", sum * sum);
    println!("The sum halfed is: {}", sum / 2);

    sum += 1;

    println!("The sum plus 1 is: {}", sum);

    let diverge: bool;

    diverge = false;

    if diverge {diverges()}

    let f: fn(i32) -> i32;

    f = plus_one;
    
    println!("The sum plus 1 is: {}", f(sum));
}

fn add_numbers(x: i32, y: i32) -> i32 {
    println!("The sum is: {}", x + y);
    x + y
}

fn diverges() -> ! {
    panic!("This function never returns!");
}

fn plus_one(i: i32) -> i32 {
    i + 1
}
