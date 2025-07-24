use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use chrono::Local;

struct FileManager;

impl FileManager {
    // Création d'un fichier
    fn create_file(path: &str) {
        match File::create(path) {
            Ok(_) => println!("Fichier '{}' créé avec succès.", path),
            Err(e) => println!("Erreur lors de la création : {}", e),
        }
    }
    // Lecture d'un fichier
    fn read_file(path: &str) {
        match fs::read_to_string(path) {
            Ok(content) => println!("Contenu de '{}':\n{}", path, content),
            Err(e) => println!("Erreur de lecture : {}", e),
        }
    }
    // Écriture dans un fichier (ajout)
    fn write_to_file(path: &str, content: &str) {
        match OpenOptions::new().write(true).append(true).open(path) {
            Ok(mut file) => {
                let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
                let line = format!("{} {}\n", timestamp, content);
                if let Err(e) = file.write_all(line.as_bytes()) {
                    println!("Erreur d'écriture : {}", e);
                } else {
                    println!("Ligne ajoutée à '{}'.", path);
                }
            }
            Err(e) => println!("Erreur à l'ouverture du fichier : {}", e),
        }
    }
    // Remplacement complet du contenu d'un fichier
    fn overwrite_file(path: &str, content: &str) {
        match File::create(path) {
            Ok(mut file) => {
                let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
                let line = format!("{} {}\n", timestamp, content);
                if let Err(e) = file.write_all(line.as_bytes()) {
                    println!("Erreur d'écriture : {}", e);
                } else {
                    println!("Contenu de '{}' remplacé avec horodatage.", path);
                }
            }
            Err(e) => println!("Erreur à l'ouverture du fichier : {}", e),
        }
    }
    // Suppression d'un fichier
    fn delete_file(path: &str) {
        match fs::remove_file(path) {
            Ok(_) => println!("Fichier '{}' supprimé définitivement.", path),
            Err(e) => println!("Erreur lors de la suppression : {}", e),
        }
    }
}

fn print_menu() {
    // Affichage du menu principal
    println!("\n--- Gestionnaire de fichiers ---");
    println!("1. Créer un fichier");
    println!("2. Lire un fichier");
    println!("3. Écrire dans un fichier (append)");
    println!("4. Remplacer le contenu du fichier");
    println!("5. Supprimer un fichier");
    println!("6. Quitter");
    print!("Choisissez une option : ");
    let _ = io::stdout().flush();
}

fn read_input() -> String {
    // Lecture de l'entrée utilisateur
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erreur de lecture");
    buffer.trim().to_string()
}

fn main() {
    loop {
        print_menu();
        let choice = read_input();

        match choice.as_str() {
            "1" => {
                println!("Nom du fichier à créer :");
                let filename = read_input();
                // Appel de la méthode pour créer un fichier
                FileManager::create_file(&filename);
            }
            "2" => {
                println!("Nom du fichier à lire :");
                let filename = read_input();
                // Appel de la méthode pour lire un fichier
                FileManager::read_file(&filename);
            }
            "3" => {
                println!("Nom du fichier où écrire :");
                let filename = read_input();
                println!("Texte à ajouter :");
                let content = read_input();
                // Appel de la méthode pour écrire dans un fichier
                FileManager::write_to_file(&filename, &content);
            }
            "4" => {
                println!("Nom du fichier à modifier :");
                let filename = read_input();
                println!("Nouveau contenu :");
                let content = read_input();
                // Appel de la méthode pour remplacer le contenu d'un fichier
                FileManager::overwrite_file(&filename, &content);
            }
            "5" => {
                println!("Nom du fichier à supprimer :");
                let filename = read_input();
                // Appel de la méthode pour supprimer un fichier
                FileManager::delete_file(&filename);
            }
            "6" => {
                // Sortie de boucle
                println!("Au revoir !");
                break;
            }
            _ => println!("Option invalide."),
        }
        // Attente volontaire pour que l'utilisateur voie le résultat
        println!("Appuyez sur Entrée pour continuer...");
        let _ = read_input();
    }
}