use std::io;

struct CompteBancaire {
    nom: String,
    solde: f64,
}

impl CompteBancaire {
    fn afficher(&self) {
        println!("Compte de {} : {} €", self.nom, self.solde);
    }

    fn deposer(&mut self, montant: f64) {
        if montant <= 0.0 {
            println!("Impossible de déposer un montant négatif ou nul !");
        } else {
            self.solde += montant;
            println!("+{} € déposés.", montant);
        }
    }

    fn retirer(&mut self, montant: f64) {
        if montant <= 0.0 {
            println!("Impossible de retirer un montant négatif ou nul !");
        } else if self.solde >= montant {
            self.solde -= montant;
            println!("-{} € retirés.", montant);
        } else {
            println!("Solde insuffisant");
        }
    }

    fn fermer(self) {
        println!("Le compte de {} est fermé, dernier solde : {}€", self.nom, self.solde);
    }

    fn renommer(self, nouveau_nom: String) -> CompteBancaire {
        CompteBancaire {
            nom: nouveau_nom,
            solde: self.solde,
        }
    }
}

fn main() {
    let mut comptes = vec![
        CompteBancaire { nom: String::from("Nouredine"), solde: 3000.0 },
        CompteBancaire { nom: String::from("Kevin"), solde: 1500.0 },
    ];

    loop {
        println!("\nMenu :");
        println!("1. Afficher tous les comptes");
        println!("2. Déposer de l'argent");
        println!("3. Retirer de l'argent");
        println!("4. Renommer un compte");
        println!("5. Fermer un compte");
        println!("6. Quitter");
        print!("Votre choix : ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim();

        match choix {
            "1" => {
                println!("Liste des comptes :");
                for (i, compte) in comptes.iter().enumerate() {
                    print!("{}. ", i + 1);
                    compte.afficher();
                }
            }
            "2" => {
                if let Some(i) = selectionner_compte(&comptes) {
                    print!("Montant à déposer : ");
                    io::Write::flush(&mut io::stdout()).unwrap();
                    let mut montant = String::new();
                    io::stdin().read_line(&mut montant).expect("Erreur de lecture");
                    let montant: f64 = match montant.trim().parse() {
                        Ok(val) => val,
                        Err(_) => {
                            println!("Montant invalide.");
                            continue;
                        }
                    };
                    comptes[i].deposer(montant);
                }
            }
            "3" => {
                if let Some(i) = selectionner_compte(&comptes) {
                    print!("Montant à retirer : ");
                    io::Write::flush(&mut io::stdout()).unwrap();
                    let mut montant = String::new();
                    io::stdin().read_line(&mut montant).expect("Erreur de lecture");
                    let montant: f64 = match montant.trim().parse() {
                        Ok(val) => val,
                        Err(_) => {
                            println!("Montant invalide.");
                            continue;
                        }
                    };
                    comptes[i].retirer(montant);
                }
            }
            "4" => {
                if let Some(i) = selectionner_compte(&comptes) {
                    print!("Nouveau nom du compte : ");
                    io::Write::flush(&mut io::stdout()).unwrap();
                    let mut nouveau_nom = String::new();
                    io::stdin().read_line(&mut nouveau_nom).expect("Erreur de lecture");
                    let nouveau_nom = nouveau_nom.trim().to_string();
                    let compte = std::mem::replace(&mut comptes[i], CompteBancaire { nom: String::new(), solde: 0.0 });
                    comptes[i] = compte.renommer(nouveau_nom);
                    println!("Compte renommé !");
                }
            }
            "5" => {
                if let Some(i) = selectionner_compte(&comptes) {
                    let compte = comptes.remove(i);
                    compte.fermer();
                }
            }
            "6" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Option invalide. Veuillez réessayer."),
        }
    }
}

fn selectionner_compte(comptes: &Vec<CompteBancaire>) -> Option<usize> {
    println!("Sélectionnez le numéro du compte :");
    for (i, compte) in comptes.iter().enumerate() {
        println!("{}. {}", i + 1, compte.nom);
    }
    print!("Votre choix : ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).expect("Erreur de lecture");
    match choix.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= comptes.len() => Some(num - 1),
        _ => {
            println!("Numéro de compte invalide.");
            None
        }
    }
}
