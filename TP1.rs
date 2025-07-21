use std::io;

struct CompteBancaire {
    nom: String,
    solde: f64,
}

fn main() {
    let mut comptes: Vec<CompteBancaire> = Vec::new();
    comptes.push(CompteBancaire { nom: String::from("Compte principal"), solde: 1000.0 });
    loop {
        println!("\nMenu :");
        println!("1. Afficher solde");
        println!("2. Retrait");
        println!("3. Liste des comptes");
        println!("4. Quitter");
        print!("Choisissez une option : ");
        io::Write::flush(&mut io::stdout()).unwrap();
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        match choix.trim() {
            "1" => {
                println!("Solde du compte principal : {:.2} €", comptes[0].solde);
            },
            "2" => {
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
                if montant > comptes[0].solde {
                    println!("Fonds insuffisants.");
                } else {
                    comptes[0].solde -= montant;
                    println!("Retrait effectué. Nouveau solde : {:.2} €", comptes[0].solde);
                }
            },
            "3" => {
                println!("Liste des comptes :");
                for (i, compte) in comptes.iter().enumerate() {
                    println!("{}. {} (solde: {:.2} €)", i+1, compte.nom, compte.solde);
                }
            },
            "4" => {
                println!("Au revoir !");
                break;
            },
            _ => println!("Option invalide. Veuillez réessayer."),
        }
    }
}
