fn main() {
    let comptes = ["Compte courant", "Livret A"];
    let solde: f32 = 1000.0; // solde du compte
    afficher_options();
    executer_option(solde, &comptes);
}

fn afficher_options() {
    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

    println!("| Menu :");
    for (i, option) in options.iter().enumerate() {
        println!("| - {}. {}", i + 1, option);
    }
}

fn executer_option(solde: f32, comptes: &[&str]) {
    let mut choix = String::new();
    println!("| Veuillez saisir un numéro de votre choix : ");
    std::io::stdin().read_line(&mut choix).expect("Erreur de lecture");

    let choix: usize = match choix.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez saisir un numéro valide.");
            executer_option(solde, comptes);
            return;
        }
    };

    if choix == 0 || choix > 4 {
        println!("Votre choix ne fait pas partie des options.");
        executer_option(solde, comptes);
    } else {
        match choix {
            1 => {
                println!("| Le solde de votre compte est de {} €.", solde);
                executer_option(solde, comptes);
            },
            2 => {
                println!("| Retrait effectué avec succès.");
                executer_option(solde, comptes);
            },
            3 => {
                println!("| Liste des comptes :");
                for compte in comptes.iter() {
                    println!("| - {}", compte);
                }
                executer_option(solde, comptes);
            }
            4 => println!("| Au revoir !"),
            _ => unreachable!(),
        }
        return;
    }
}