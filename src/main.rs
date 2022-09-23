use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Devinez le nombre entier entre 1 et 100 !");

    let nb_secret = rand::thread_rng().gen_range(1..101);

    let mut nb_essai = 7;

    while nb_essai > 0 {
        println!("Il vous reste : {} essai(s).", nb_essai);
        println!("Veuillez entrer votre nombre s'il vous plait.");

        let mut supposition = String::new();

        io::stdin()
        .read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur.");

        let supposition: u32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        println!("Votre nombre : {}.", supposition);

        match supposition.cmp(&nb_secret) {
            Ordering::Less => println!("Votre supposition est inférieure au nombre secret !"),
            Ordering::Greater => println!("Votre supposition est supérieure au nombre secret !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
        nb_essai -=1; 
    }
    if nb_essai == 0 {
        println!("Malheusement, vous n'avez plus d'essai, le nombre secret était : {}", nb_secret);
    }    
}