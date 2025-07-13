use std::ops::Neg;

fn main() {


    let x = -123;

    let is_neg = x < 0;
    let mut org = (x.clone() as i32).abs();
    let mut result = 0;

    while org > 0{
        let digit = org % 10;
        if result > (i32::MAX - digit) / 10 {
            println!("{}",0);
        }
        result = result * 10 + digit;
        org = org / 10;
    }

    if is_neg {
        println!("{}", result.neg());
    }else{
        println!("{}", result);
    }

}
