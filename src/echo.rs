pub fn echo(inputs: &Vec<String>, upper: &bool) {
    if upper == &true {
        for input in inputs {
            print!("{} ", input.to_uppercase());
        }
    } else {
        for input in inputs {
            print!("{} ", input);
        }
    }

    println!();
}
