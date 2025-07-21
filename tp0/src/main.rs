fn main() {
    let name: &str = "Luigui";
    let age: u32 = 21; // entier non signé de 32 bits (valeurs positives)
    let dads_age = 70; // rust comprend que c'est un entier non signé par défaut
    // i32 est un entier signé de 32 bits (valeurs positives et négatives)
    let temperature: f32 = 27.5; // température en degrés Celsius (float 32 bits)
    println!("Hello, world!");
    println!("My name is {} and I am {} years old.", name, age);
    println!("My dad's age is {}.", dads_age);
    println!("The temperature is {} degrees Celsius.", temperature);

    // Appel de la fonction add
    let sum = add(5, 10);
    println!("The sum of 5 and 10 is {}.", sum);

    hello(name);

    let nombre: i32 = 42; // entier signé de 32 bits
    if nombre % 2 == 0 {
        println!("{} est un nombre pair.", nombre);
    } else {
        println!("{} est un nombre impair.", nombre);
    }

    for i in 1..5 {
        println!("Iteration number: {}", i);
    }

    let voitures = ["Opel", "Renault", "Peugeot"];
    for voiture in voitures {
        println!("Voiture: {}", voiture);
    }
    for (index, voiture) in voitures.iter().enumerate() {
        println!("Voiture {}: {}", index + 1, voiture);
    }

    let options = ["Afficher solde","Retrait","Liste comptes","Quitter"];

    println!("Menu:");
    for ( i,option) in options.iter().enumerate(){
    // afficher chaque option et on commence par 1 
    println!("{}.{}", i+1, option); 
    }

    println!("Veuillez saisir un numéro de votre choix:");

    let mut choix = String::new();
    stdin().read_line(&mut choix).expect("Attention erreur de lecture");
    
    let choix:usize = match choix.trim().parse(){
    Ok(num) => num,
    Err(_)=> {
        println!("Veuillez saisir un numero valide");
        return;
    }
    };

    if choix == 0 || choix > options.len(){
    println!(" choix hors système !! limite système ");
    } else {
    println!("Vous avez sélectionné : {}", options[choix-1]);
    // ici on peut exécuter une action selon choix dans options 
    }
}

fn add(x: i32, y: i32) -> i32 {
    return x + y
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}