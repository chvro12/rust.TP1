use std::io;
use chrono::Utc;
use std::fs;
use std::io::Write;

// Structure User
struct User {
    nom: String,
    secu: String,
}

// Structure FileManager
struct FileManager {
    user: User,
}

impl FileManager {
    fn new(user: User) -> Self {
        FileManager { user }
    }

    fn afficher_date(&self) {
        let maintenant = Utc::now();
        println!("Date et heure actuelle : {}", maintenant.format("%d/%m/%Y %H:%M:%S"));
    }

    fn lire_fichier(&self, chemin: &str) {
        match fs::read_to_string(chemin) {
            Ok(contenu) => println!("Contenu du fichier :\n{}", contenu),
            Err(e) => println!("Erreur lors de la lecture : {}", e),
        }
    }

    fn ecrire_fichier(&self, chemin: &str, contenu: &str) {
        match fs::File::create(chemin) {
            Ok(mut fichier) => {
                if let Err(e) = fichier.write_all(contenu.as_bytes()) {
                    println!("Erreur d'écriture : {}", e);
                } else {
                    println!("Fichier écrit avec succès.");
                }
            }
            Err(e) => println!("Erreur lors de la création du fichier : {}", e),
        }
    }

    fn modifier_fichier(&self, chemin: &str, nouveau_contenu: &str) {
        self.ecrire_fichier(chemin, nouveau_contenu);
    }

    fn supprimer_fichier(&self, chemin: &str) {
        match fs::remove_file(chemin) {
            Ok(_) => println!("Fichier supprimé avec succès."),
            Err(e) => println!("Erreur lors de la suppression : {}", e),
        }
    }
}

fn main() {
    let user = User {
        nom: String::from("Kevin"),
        secu: String::from("1234567890"),
    };
    let gestionnaire = FileManager::new(user);

    loop {
        println!("\n--- Gestionnaire de fichiers ---");
        println!("1. Lire un fichier");
        println!("2. Écrire dans un fichier");
        println!("3. Modifier un fichier");
        println!("4. Supprimer un fichier");
        println!("5. Afficher la date");
        println!("6. Quitter");
        println!("Veuillez saisir votre choix :");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim();

        match choix {
            "1" => {
                println!("Chemin du fichier à lire :");
                let mut chemin = String::new();
                io::stdin().read_line(&mut chemin).expect("Erreur de lecture");
                gestionnaire.lire_fichier(chemin.trim());
            }
            "2" => {
                println!("Chemin du fichier à écrire :");
                let mut chemin = String::new();
                io::stdin().read_line(&mut chemin).expect("Erreur de lecture");
                println!("Contenu à écrire :");
                let mut contenu = String::new();
                io::stdin().read_line(&mut contenu).expect("Erreur de lecture");
                gestionnaire.ecrire_fichier(chemin.trim(), contenu.trim());
            }
            "3" => {
                println!("Chemin du fichier à modifier :");
                let mut chemin = String::new();
                io::stdin().read_line(&mut chemin).expect("Erreur de lecture");
                println!("Nouveau contenu :");
                let mut contenu = String::new();
                io::stdin().read_line(&mut contenu).expect("Erreur de lecture");
                gestionnaire.modifier_fichier(chemin.trim(), contenu.trim());
            }
            "4" => {
                println!("Chemin du fichier à supprimer :");
                let mut chemin = String::new();
                io::stdin().read_line(&mut chemin).expect("Erreur de lecture");
                gestionnaire.supprimer_fichier(chemin.trim());
            }
            "5" => {
                gestionnaire.afficher_date();
            }
            "6" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide, veuillez réessayer."),
        }
    }
}
