fn main() {

    let mut x: i32 = 5;

    x = 10;

    let mut y: i32;

    y = 6;

    let (a, b) = (1, 2);

    {    
        let x = "Hello";

        println!("x: {} y: {} a: {} b: {}", x, y, a, b);

        y = 3;
    }
    
    println!("x: {} y: {} a: {} b: {}", x, y, a, b);

    let (mut a, mut b) = ("A", "B");

    a = "A2";

    let b = b;

    println!("x: {} y: {} a: {} b: {}", x, y, a, b);
}
