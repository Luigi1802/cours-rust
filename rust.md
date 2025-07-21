21/07
Rust 
possible de faire du web avec des frameworks
.rs

Cargo 
cree un projet(cargo new [nom_projet]
cargo build  : compile le projet mode debug par defaut
cargo check   : vérifie le code sans produire de binaire
cargo update :  met à jour les dépendances
cargo doc --open  : gènère la docutmentation et l'ouvre dans le navigateur Web
compiler du code rust
cargo run : compile et exécute le projet
lancer des tests
gestionnaire de dépendances

Cargo.toml-> fichier pour les dépendances (comme package.json)

let pour declarer une variable
: type devant le nom pour préciser le type

u32 non signé i32 signé, f32 float etc

fn pour déclarer une fonction (declarer type params et retour)

pour le for, exemple : 
for i in 1..=10 -> "..=" -> de 1 à 10 INCLUS
".." EXCLUS

for voiture in voitures pour boucler dans un tableau
for (index, voiture) in voitures.iter().enumerate() pour avoir index et valeur

iter crée un iterateur sur la collection sans le consommer
enumerate utilise le iter pour faire sur séquence index, valeur

io:stdin().read_line([variable_de_recup]) -> recup input utilisateur

&mut pour rendre une variable modifiable

pour les tableaux on peut préciser le type et le nb d'éléments 
let tab: [i32;4] = [1, 34, 123, 83 ]

_ devant une variable pour éviter le warning variable non utilisée

loop pour créer une boucle qui ne s'arrete qu'avec un break

struct : structure de donnée personnalisée, on accède aux attributs par un "." struct.attribut

switch : match avec _ pour le default

impl : fonctions associées pour des struct
