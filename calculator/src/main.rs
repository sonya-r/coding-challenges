use calculator::Calculator;

fn main() {
    let number = Calculator::from_args();

    println!("{number:?}")
}
