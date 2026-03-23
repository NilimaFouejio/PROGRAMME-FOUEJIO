// TP 1 — Setup & CLI Simple
// Implémentez une calculatrice en ligne de commande
//
// Usage : ./calculatrice_cli <nombre1> <opérateur> <nombre2>
// Opérateurs : +  -  *  /
// Exemple   : ./calculatrice_cli 10 / 3  =>  10 / 3 = 3.3333

use std::env;
use std::io;          
use std::io::Write; 

/// Effectue le calcul et retourne le résultat ou une erreur
fn calculer(a: f64, op: &str, b: f64) -> Result<f64, String> {
    // TODO : implémenter +  -  *  /
    // TODO : gérer division par zéro -> Err(...)
    // TODO : opérateur inconnu      -> Err(...)
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err(String::from("Division par zéro"))
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("Opérateur inconnu : {}", op)),
    }
}
/// Mode interactif : boucle de saisie des expressions.
fn mode_interactif() {
    println!("Mode interactif – saisissez une expression (ex: 10 + 5)");
    println!("Tapez 'quitter' pour sortir.\n");

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        if stdin.read_line(&mut input).is_err() {
            eprintln!("Erreur de lecture");
            break;
        }

        let line = input.trim();
        if line == "quitter" {
            println!("Au revoir !");
            break;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() != 3 {
            println!("Format invalide. Utilisez : <nombre> <op> <nombre>");
            continue;
        }

        let a: f64 = match tokens[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("'{}' n'est pas un nombre valide", tokens[0]);
                continue;
            }
        };
        let op = tokens[1];
        let b: f64 = match tokens[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("'{}' n'est pas un nombre valide", tokens[2]);
                continue;
            }
        };

        match calculer(a, op, b) {
            Ok(resultat) => println!("{} {} {} = {}", a, op, b, resultat),
            Err(e) => println!("Erreur : {}", e),
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO : vérifier que args.len() == 4, sinon afficher l'usage et quitter
    // TODO : parser args[1] en f64, args[2] comme &str, args[3] en f64
    // TODO : appeler calculer() et afficher "a op b = résultat"
    // TODO : gérer les erreurs avec eprintln! et std::process::exit(1)
    // Si aucun argument, lancer le mode interactif
    if args.len() == 1 {
        mode_interactif();
        return;
    }
    // Sinon, mode avec arguments
    if args.len() != 4 {
        eprintln!("Usage: {} <nombre1> <opérateur> <nombre2>", args[0]);
        eprintln!("Opérateurs : + - * /");
        eprintln!("Ou lancez sans argument pour le mode interactif.");
        std::process::exit(1);
    }
    let a: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("'{}' n'est pas un nombre valide", args[1]);
            std::process::exit(1);
        }
    };
    let op = &args[2];
    let b: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("'{}' n'est pas un nombre valide", args[3]);
            std::process::exit(1);
        }
    };

    match calculer(a, op, b) {
        Ok(resultat) => println!("{} {} {} = {}", a, op, b, resultat),
        Err(e) => {
            eprintln!("Erreur : {}", e);
            std::process::exit(1);
        }
    }
    println!("TODO : implémenter main");
}


// BONUS Partie D — mode interactif
// Lisez les expressions depuis stdin en boucle jusqu'à "quitter"
// Utilisez std::io::{self, BufRead}
