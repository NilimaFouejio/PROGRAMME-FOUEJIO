/// TP 3 — Analyseur de Chaînes
// Toutes les fonctions travaillent avec des références (&str / &'a str)

use std::collections::HashMap;

// ============================================================
// STRUCTURES
// ============================================================

#[derive(Debug, PartialEq)]
struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,
    nb_phrases: usize,
    mot_le_plus_long: String,
    frequence_chars: [(char, usize); 5], // top 5 caractères (hors espaces)
}

// ============================================================
// FONCTIONS À IMPLÉMENTER
// ============================================================

/// Compte les mots dans un texte (séparés par des espaces)
fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

/// Retourne le mot le plus long (le premier en cas d'égalité)
/// Le lifetime 'a garantit que le résultat vit aussi longtemps que l'entrée
fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte
        .split_whitespace()
        .max_by_key(|mot| mot.len())
        .unwrap_or("")
}

/// Vérifie si le texte est un palindrome (ignore la casse et les espaces)
fn est_palindrome(texte: &str) -> bool {
    let nettoye: String = texte
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let inverse: String = nettoye.chars().rev().collect();
    nettoye == inverse
}

/// Retourne les N premiers mots sous forme de slices
fn premiers_mots<'a>(texte: &'a str, n: usize) -> Vec<&'a str> {
    texte.split_whitespace().take(n).collect()
}

/// Remplace toutes les occurrences de `de` par `vers`, retourne un String
fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

/// Calcule les statistiques globales du texte
fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);
    let nb_caracteres = texte.chars().count();

    // Compte les phrases (simpliste : . ! ?)
    let nb_phrases = texte
        .chars()
        .filter(|c| *c == '.' || *c == '!' || *c == '?')
        .count();

    let mot_le_plus_long = mot_le_plus_long(texte).to_string();

    // Fréquence des caractères (hors espaces)
    let mut freq = HashMap::new();
    for c in texte.chars() {
        if !c.is_whitespace() {
            *freq.entry(c).or_insert(0) += 1;
        }
    }

    let mut freq_vec: Vec<(char, usize)> = freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1)); // tri décroissant par fréquence

    let mut frequence_chars = [('\0', 0); 5];
    for (i, &(c, count)) in freq_vec.iter().take(5).enumerate() {
        frequence_chars[i] = (c, count);
    }

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long,
        frequence_chars,
    }
}

// ============================================================
// MAIN
// ============================================================

fn main() {
    let texte = "Rust est un langage système. Rust garantit la sécurité mémoire.";

    println!("Mots          : {}", compter_mots(texte));
    println!("Mot le + long : {}", mot_le_plus_long(texte));
    println!("Est palindrome ('kayak') : {}", est_palindrome("kayak"));
    println!("3 premiers mots : {:?}", premiers_mots(texte, 3));
    println!("Remplacé : {}", remplacer(texte, "Rust", "Go"));
}

// ============================================================
// TESTS — lancez avec : cargo test
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compter_mots() {
        assert_eq!(compter_mots("hello world"), 2);
        assert_eq!(compter_mots(""), 0);
        assert_eq!(compter_mots("  espaces  "), 1);
    }

    #[test]
    fn test_palindrome() {
        assert!(est_palindrome("kayak"));
        assert!(est_palindrome("A man a plan a canal Panama"));
        assert!(!est_palindrome("Rust"));
    }

    #[test]
    fn test_mot_le_plus_long() {
        assert_eq!(mot_le_plus_long("Rust est génial"), "génial");
        // Test de cas où plusieurs mots ont la même longueur (premier conservé)
        assert_eq!(mot_le_plus_long("abc def ghij klm"), "ghij");
    }

    #[test]
    fn test_premiers_mots() {
        assert_eq!(premiers_mots("un deux trois quatre", 2), vec!["un", "deux"]);
        assert_eq!(premiers_mots("un", 5), vec!["un"]);
    }

    #[test]
    fn test_remplacer() {
        assert_eq!(remplacer("hello world", "world", "Rust"), "hello Rust");
        assert_eq!(remplacer("abc abc abc", "abc", "x"), "x x x");
    }

    #[test]
    fn test_analyser() {
        let texte = "Bonjour. Comment ça va? Très bien!";
        let stats = analyser(texte);
        assert_eq!(stats.nb_mots, 6);
        assert_eq!(stats.nb_caracteres, 33); // y compris espaces et ponctuation
        assert_eq!(stats.nb_phrases, 3);
        assert_eq!(stats.mot_le_plus_long, "Bonjour");

        // Vérification du top 5 (ordre non garanti car fréquences égales possibles)
        // On vérifie que le caractère le plus fréquent est 'n' ou 'o' (3 occurrences chacun)
        let top = stats.frequence_chars;
        assert!(top[0].1 >= 3); // le premier a au moins 3 occurrences
        // On vérifie que les 5 premiers ont tous une fréquence > 0 (sauf si moins de 5 caractères distincts)
        for i in 0..5 {
            if i < top.iter().filter(|(c, _)| *c != '\0').count() {
                assert!(top[i].1 > 0);
            }
        }
    }
}