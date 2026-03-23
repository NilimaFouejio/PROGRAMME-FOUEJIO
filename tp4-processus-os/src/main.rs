/// TP 4 — Modélisation de Processus OS
// Pratiquez : structs, enums, pattern matching, Option<T>, Result<T, E>

// ============================================================
// TYPES DE BASE (ne pas modifier)
// ============================================================

#[derive(Debug, Clone, PartialEq)]
enum EtatProcessus {
    Prêt,
    EnExécution { cpu_id: u8 },
    Bloqué { raison: String },
    Terminé { code_retour: i32 },
    Zombie,
}

#[derive(Debug, Clone, PartialEq)]
enum Priorité {
    TrèsFaible,
    Faible,
    Normale,
    Haute,
    TrèsHaute,
    TempsRéel(u8), // niveau 0-99
}

#[derive(Debug)]
struct Processus {
    pid: u32,
    nom: String,
    état: EtatProcessus,
    priorité: Priorité,
    mémoire_ko: u64,
    pid_parent: Option<u32>,
}

#[derive(Debug)]
struct GestionnaireProcessus {
    processus: Vec<Processus>,
    prochain_pid: u32,
}

// ============================================================
// MÉTHODES À IMPLÉMENTER
// ============================================================

impl GestionnaireProcessus {
    /// Crée un nouveau gestionnaire vide (prochain_pid = 1)
    fn nouveau() -> Self {
        GestionnaireProcessus {
            processus: Vec::new(),
            prochain_pid: 1,
        }
    }

    /// Crée un processus avec état Prêt, l'ajoute à la liste et retourne son PID
    fn créer_processus(
        &mut self,
        nom: String,
        priorité: Priorité,
        mémoire_ko: u64,
        pid_parent: Option<u32>,
    ) -> u32 {
        let pid = self.prochain_pid;
        self.prochain_pid += 1;
        let processus = Processus {
            pid,
            nom,
            état: EtatProcessus::Prêt,
            priorité,
            mémoire_ko,
            pid_parent,
        };
        self.processus.push(processus);
        pid
    }

    /// Cherche un processus par PID, retourne None s'il n'existe pas
    fn trouver(&self, pid: u32) -> Option<&Processus> {
        self.processus.iter().find(|p| p.pid == pid)
    }

    /// Change l'état d'un processus. Retourne Err si le PID est introuvable.
    fn changer_état(&mut self, pid: u32, nouvel_état: EtatProcessus) -> Result<(), String> {
        if let Some(p) = self.processus.iter_mut().find(|p| p.pid == pid) {
            p.état = nouvel_état;
            Ok(())
        } else {
            Err(format!("Processus avec PID {} introuvable", pid))
        }
    }

    /// Somme de la mémoire utilisée par tous les processus
    fn mémoire_totale_utilisée(&self) -> u64 {
        self.processus.iter().map(|p| p.mémoire_ko).sum()
    }

    /// Passe le processus en Terminé { code_retour: 0 }, retourne le code ou Err
    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        if let Some(p) = self.processus.iter_mut().find(|p| p.pid == pid) {
            p.état = EtatProcessus::Terminé { code_retour: 0 };
            Ok(0)
        } else {
            Err(format!("Processus avec PID {} introuvable", pid))
        }
    }

    /// Affiche un résumé de tous les processus
    fn afficher_résumé(&self) {
        println!("{:-<70}", "");
        println!("{:^70}", "RÉSUMÉ DES PROCESSUS");
        println!("{:-<70}", "");
        println!(
            "{:<8} {:<20} {:<20} {:<12} {:<8}",
            "PID", "NOM", "ÉTAT", "PRIORITÉ", "MÉMOIRE (ko)"
        );
        for p in &self.processus {
            let état_str = match &p.état {
                EtatProcessus::Prêt => "Prêt".to_string(),
                EtatProcessus::EnExécution { cpu_id } => format!("Exécution (CPU {})", cpu_id),
                EtatProcessus::Bloqué { raison } => format!("Bloqué ({})", raison),
                EtatProcessus::Terminé { code_retour } => format!("Terminé ({})", code_retour),
                EtatProcessus::Zombie => "Zombie".to_string(),
            };
            let priorité_str = match &p.priorité {
                Priorité::TrèsFaible => "TrèsFaible".to_string(),
                Priorité::Faible => "Faible".to_string(),
                Priorité::Normale => "Normale".to_string(),
                Priorité::Haute => "Haute".to_string(),
                Priorité::TrèsHaute => "TrèsHaute".to_string(),
                Priorité::TempsRéel(n) => format!("Réel({})", n),
            };
            println!(
                "{:<8} {:<20} {:<20} {:<12} {:<8}",
                p.pid, p.nom, état_str, priorité_str, p.mémoire_ko
            );
        }
        println!("{:-<70}", "");
        println!("Mémoire totale utilisée : {} ko", self.mémoire_totale_utilisée());
        println!("{:-<70}", "");
    }
}

// ============================================================
// MAIN
// ============================================================

fn main() {
    let mut gp = GestionnaireProcessus::nouveau();

    // Créer init (PID 1, sans parent)
    let pid_init = gp.créer_processus(String::from("init"), Priorité::Haute, 1024, None);

    // Créer quelques processus enfants
    let pid_shell = gp.créer_processus(
        String::from("bash"),
        Priorité::Normale,
        2048,
        Some(pid_init),
    );
    let pid_app = gp.créer_processus(
        String::from("mon_app"),
        Priorité::TempsRéel(10),
        8192,
        Some(pid_shell),
    );

    // Changer des états
    match gp.changer_état(pid_shell, EtatProcessus::EnExécution { cpu_id: 1 }) {
        Ok(()) => println!("bash en exécution sur CPU 1"),
        Err(e) => eprintln!("Erreur : {}", e),
    }

    // Tuer un processus
    match gp.tuer_processus(pid_app) {
        Ok(code) => println!("mon_app terminé avec code {}", code),
        Err(e) => eprintln!("Erreur : {}", e),
    }

    // Afficher le résumé
    gp.afficher_résumé();

    println!("Mémoire totale : {} ko", gp.mémoire_totale_utilisée());
}

// ============================================================
// TESTS UNITAIRES
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nouveau() {
        let gp = GestionnaireProcessus::nouveau();
        assert_eq!(gp.processus.len(), 0);
        assert_eq!(gp.prochain_pid, 1);
    }

    #[test]
    fn test_créer_processus() {
        let mut gp = GestionnaireProcessus::nouveau();
        let pid = gp.créer_processus(String::from("test"), Priorité::Normale, 100, None);
        assert_eq!(pid, 1);
        assert_eq!(gp.processus.len(), 1);
        assert_eq!(gp.prochain_pid, 2);
        let p = gp.trouver(pid).unwrap();
        assert_eq!(p.nom, "test");
        assert!(matches!(p.état, EtatProcessus::Prêt));
    }

    #[test]
    fn test_trouver() {
        let mut gp = GestionnaireProcessus::nouveau();
        let pid = gp.créer_processus(String::from("test"), Priorité::Normale, 100, None);
        assert!(gp.trouver(pid).is_some());
        assert!(gp.trouver(999).is_none());
    }

    #[test]
    fn test_changer_état() {
        let mut gp = GestionnaireProcessus::nouveau();
        let pid = gp.créer_processus(String::from("test"), Priorité::Normale, 100, None);
        assert!(gp.changer_état(pid, EtatProcessus::EnExécution { cpu_id: 2 }).is_ok());
        let p = gp.trouver(pid).unwrap();
        assert!(matches!(p.état, EtatProcessus::EnExécution { cpu_id: 2 }));
        assert!(gp.changer_état(999, EtatProcessus::Prêt).is_err());
    }

    #[test]
    fn test_mémoire_totale() {
        let mut gp = GestionnaireProcessus::nouveau();
        gp.créer_processus(String::from("p1"), Priorité::Normale, 100, None);
        gp.créer_processus(String::from("p2"), Priorité::Normale, 200, None);
        assert_eq!(gp.mémoire_totale_utilisée(), 300);
    }

    #[test]
    fn test_tuer_processus() {
        let mut gp = GestionnaireProcessus::nouveau();
        let pid = gp.créer_processus(String::from("test"), Priorité::Normale, 100, None);
        assert!(gp.tuer_processus(pid).is_ok());
        let p = gp.trouver(pid).unwrap();
        match &p.état {
            EtatProcessus::Terminé { code_retour } => assert_eq!(*code_retour, 0),
            _ => panic!("État incorrect"),
        }
        assert!(gp.tuer_processus(999).is_err());
    }
}