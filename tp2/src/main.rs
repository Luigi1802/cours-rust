use std::io;

struct CompteBancaire {
    nom: String,
    solde: f32,
}

impl CompteBancaire {
    fn afficher_solde(&self) {
        println!("| Le solde de votre compte '{}' est de {:.2} €.", self.nom, self.solde);
    }

    fn retirer(&mut self, montant: f32) {
        if montant > self.solde {
            println!("| Fonds insuffisants !");
        } else {
            self.solde -= montant;
            println!("| Retrait de {:.2} € effectué. Nouveau solde : {:.2} €.", montant, self.solde);
        }
    }

    fn deposer(&mut self, montant: f32) {
        if montant <= 0.0 {
            println!("| Le montant doit être positif.");
        } else {
            self.solde += montant;
            println!("| Dépôt de {:.2} € effectué. Nouveau solde : {:.2} €.", montant, self.solde);
        }
    }
}

fn main() {
    let mut comptes = vec![
        CompteBancaire {
            nom: "Compte courant".to_string(),
            solde: 1000.0,
        },
        CompteBancaire {
            nom: "Livret A".to_string(),
            solde: 5000.0,
        },
    ];

    boucle_principale(&mut comptes);
}

fn boucle_principale(comptes: &mut Vec<CompteBancaire>) {
    loop {
        println!("\n| Sélectionnez un compte :");
        for (i, compte) in comptes.iter().enumerate() {
            println!("| - {}. {}", i + 1, compte.nom);
        }
        println!("| - 0. Quitter");

        let choix = lire_entree("Votre choix :");

        if choix == 0 {
            println!("| Au revoir !");
            break;
        }

        if let Some(compte_selectionne) = comptes.get_mut(choix - 1) {
            afficher_options();
            executer_option(compte_selectionne);
        } else {
            println!("| Choix invalide.");
        }
    }
}

fn afficher_options() {
    let options = ["Afficher solde", "Retrait", "Dépôt", "Retour au choix du compte"];

    println!("| Menu :");
    for (i, option) in options.iter().enumerate() {
        println!("| - {}. {}", i + 1, option);
    }
}

fn executer_option(compte: &mut CompteBancaire) {
    loop {
        let choix = lire_entree("Veuillez saisir un numéro de votre choix :");

        match choix {
            1 => compte.afficher_solde(),
            2 => {
                let montant = lire_montant("Montant à retirer :");
                compte.retirer(montant);
            }
            3 => {
                let montant = lire_montant("Montant à déposer :");
                compte.deposer(montant);
            }
            4 => {
                println!("| Retour au menu principal.");
                break;
            }
            _ => println!("| Choix invalide."),
        }

        afficher_options();
    }
}

fn lire_entree(message: &str) -> usize {
    let mut saisie = String::new();
    println!("| {}", message);
    io::stdin().read_line(&mut saisie).expect("Erreur de lecture");

    match saisie.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("| Entrée invalide.");
            999
        }
    }
}

fn lire_montant(message: &str) -> f32 {
    let mut saisie = String::new();
    println!("| {}", message);
    io::stdin().read_line(&mut saisie).expect("Erreur de lecture");

    match saisie.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => {
            println!("| Montant invalide.");
            lire_montant(message)
        }
    }
}
