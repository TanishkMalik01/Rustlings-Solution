// Booleans (`bool`)

fn main() {
    let is_morning = true;
    let mut is_evening = true;
    if is_morning {
        println!("Good morning!");
        is_evening = false;
    }

    // TODO: Define a boolean variable with the name `is_evening` before the `if` statement below.
    // The value of the variable should be the negation (opposite) of `is_morning`.
    // let …
    if is_evening {
        println!("Good evening!");
    }
}
