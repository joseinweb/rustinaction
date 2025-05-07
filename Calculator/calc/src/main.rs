fn main() {

    println!("Hello, I am a calculator!");
    let x = 10;
    let y=2;
    let sum = addition::add(x, y);
    let diff = substraction::diff(x, y);
    let div = divide::divide(x, y);
    let product = product::mul(x, y);
    println!("The sum of {} and {} is {}", x, y, sum);
    println!("The difference of {} and {} is {}", x, y, diff);
    println!("The division of {} and {} is {}", x, y, div);
    println!("The multiplication of {} and {} is {}", x, y, product);
    let lucky_num = luckynum::gen_lucky_num();
    println!("The lucky number is {}", lucky_num);
}
