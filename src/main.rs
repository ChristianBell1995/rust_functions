fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let z = five();

    println!("The value of z is: {}", z);

    let a = plus_one(z);

    println!("The value of a is: {}", a);
}

// Difference between statements and expressions in Rust

// This function:
// let x = (let y = 6);
// Will produce an error because you can't assign values as statements
// IF YOU END A LINE IN A ; THEN IT BECOMES A STATEMENT NOT AN EXPRESSION
// An expression can be used to set a value but a statement can't.


// Functions return their last value implicitly
// you can also use the return keyword to return early
// It also declares the type that it returns

fn five() -> i32 {
  5
}

// if the function here had a semi-colon after it 'num + 1;' there would be an error as the fn would return an empty tuple instead of a num

fn plus_one(num: i32) -> i32 {
  num + 1
}
