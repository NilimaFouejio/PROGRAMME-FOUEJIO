// TP 2 — Gestion Mémoire & Ownership
//
// ⚠️  RÈGLE : vous NE POUVEZ PAS utiliser de références (&) dans cet exercice.
//
// Partie A : corrigez les 2 programmes (expliquez l'erreur en commentaire)
// Partie B : implémentez le gestionnaire de tâches avec ownership

// ============================================================
// PARTIE A — Débogage d'erreurs d'ownership
// ============================================================

// Programme 1 : pourquoi ce code ne compile-t-il pas ? Corrigez-le.
#[allow(dead_code)]
fn programme_1() {
    let v = vec![1, 2, 3];
    let v2 = v.clone();       // copie profonde, v reste valide
    println!("Longueur : {}", v.len());
    // VOTRE EXPLICATION : v est un Vec<i32> qui ne possède pas le trait Copy. L’affectation let v2 = v déplace la propriété du vecteur de v vers v2. Après ce déplacement, v n’est plus utilisable. L’appel à v.len() tente d’emprunter une valeur qui a été déplacée.
}

// Programme 2 : corrigez SANS utiliser clone()
#[allow(dead_code)]
fn somme(v: &Vec<i32>) -> i32 {  
    v.iter().sum()
}
#[allow(dead_code)]
fn programme_2() {
    let nombres = vec![1, 2, 3, 4, 5];
    let s = somme(&nombres);       // ← on passe une référence
    println!("Somme : {}, Vecteur : {:?}", s, nombres);
    // VOTRE EXPLICATION : la fonction somme prend possession du vecteur. Après l’appel, nombres n’est plus disponible dans main. La solution sans clone() consiste à modifier la signature de somme pour qu’elle emprunte le vecteur plutôt que de le prendre
    // VOTRE CORRECTION : modifier la signature de somme()
}

// ============================================================
// PARTIE B — Gestionnaire de tâches
// Une tâche = tuple (titre: String, priorité: u8, complète: bool)
// ============================================================

fn creer_tache(titre: String, priorite: u8) -> (String, u8, bool) {
    // TODO : retourner un tuple avec complète = false
    (titre, priorite, false) 
    
}
fn afficher_tache(tache: (String, u8, bool)) -> (String, u8, bool) {
     // TODO : afficher "[ ] Titre (priorité: N)" ou "[x] Titre (priorité: N)"
    println!("Tâche : {} | Priorité : {} | Complétée : {}", tache.0, tache.1, tache.2);
    tache   
}

fn marquer_complete(tache: (String, u8, bool)) -> (String, u8, bool) {
    // TODO : retourner la tâche avec complète = true
    (tache.0, tache.1, true)
}

fn extraire_titre(tache: (String, u8, bool)) -> String {
    // TODO : retourner uniquement le titre (le reste est consommé)
    tache.0
}

fn main() {
     programme_1();
    programme_2();
    // Testez vos fonctions ici
    // ⚠️  Attention à l'ownership lors des appels successifs !
    // Exemple :
    // let t = creer_tache(String::from("Apprendre Rust"), 1);
    // let t = marquer_complete(t);
    // afficher_tache(t);
    let tache = creer_tache(String::from("Apprendre Rust"), 1);
    let tache = afficher_tache(tache);
    let tache = marquer_complete(tache);
    let tache = afficher_tache(tache);

    let titre = extraire_titre(tache);
    println!("Titre extrait : {}", titre);
}
