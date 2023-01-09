use std::io;

fn main() {
    let supposition: isize = "-42".parse().expect("Ce n'est pas un nombre !");

    println!("supposition : {}", supposition);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("La valeur de y est : {:#?}", tup);

    let a = [1, 2, 3, 4, 5];

    println!("Veuillez entrer un indice de tableau.");

    let mut indice = String::new();

    io::stdin()
        .read_line(&mut indice)
        .expect("Échec de la lecture de l'entrée utilisateur");

    let indice: usize = indice
        .trim()
        .parse()
        .expect("L'indice entré n'est pas un nombre");

    let element = a[indice];

    println!(
            "La valeur de l'élément d'indice {} est : {}",
        indice, element
    );
}
