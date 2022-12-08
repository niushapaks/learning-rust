use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    guessing_algorithm();
}

fn guessing_algorithm(){
    println!("!شماره را پیدا کنید");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //    println!(".است {secret_number} شماری مخفی");

    println!(".لطفآ شماره را بنویسید");
    loop {

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("!لطفآ ی شماره بنویسید");

        //        println!(".انتخاب کردید {guess} شماری");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("!بالا تر"),
            Ordering::Greater => println!("!کوتچیک تر"),
            Ordering::Equal => {
                println!("!برنده شدید");
                guessing_algorithm();
//                break;
            }
        }
    }
}