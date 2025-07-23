fn main() {
    let x = 5;

    let x = x + 1;

    const Y:i32 = 69;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");

        println!("Does inner works like gate? Yay, Y now is: {Y}");
    }

    println!("The value of x is: {x}");
    println!("The const Y is: {Y}");
}
