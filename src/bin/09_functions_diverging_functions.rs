#![allow(dead_code)]

fn foo() {
    panic!("This call never returns.");
}

fn some_fn() {
    ()
}

fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
        // Notice that the return type of this match expression must be u32
        // because of the type of the "addition" variable.
        let addition = match i%2 == 1 {
            // The "i" variable is of type u32, which is perfectly fine.
            true => i,
            // On the other hand, the "continue" expression does not return
            // u32, but it is still fine, because it never returns and therefore
            // does not violate the type requirements of the match expression.
            false => continue,
        };
        acc += addition;
    }
    acc
}

fn main() {
    let _a: () = some_fn();
    println!("This function returns and you can see this line.");

    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
    
    /* let x = panic!("This call never returns.");
    println!("You will never see this line!"); */
}