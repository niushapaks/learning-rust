fn main() {

    const TROIS_HEURES_EN_SECONDES: u32 = 60 * 60 * 3;

//    let mut x = 5;
//    println!("La valeur de x est : {}", x);
//    x = 6;
//    println!("La valeur de x est : {}", x);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("La valeur de x dans la portée interne est : {}", x);
    }

    println!("La valeur de x est : {}", x);

    let espaces = "   ";
    let espaces = espaces.len();

}
