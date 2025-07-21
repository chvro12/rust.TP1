## Les variables mutables

Par défaut, une variable ne peut pas changer de valeur.  Pour la rendre modifiable, il faut ajouter `mut` :
```rust
let mut solde = 100.0;
solde = solde + 50.0;
```
Ça permet d’éviter de modifier une variable par erreur.

---

## Le snake_case

En Rust, on écrit les noms de variables et de fonctions en minuscules, avec des underscores pour séparer les mots (ex : `age_papa`, `say_hello`).  C’est une convention à respecter pour que le code soit lisible et « rustique ».

---

## Les structures (struct) et méthodes

On peut créer ses propres types avec `struct`.  Par exemple, pour un compte bancaire :
```rust
struct CompteBancaire {
    nom: String,
    solde: f64,
}
```
On peut ensuite ajouter des méthodes à la structure avec `impl` :
```rust
impl CompteBancaire {
    fn afficher(&self) { /* ... */ }
    fn deposer(&mut self, montant: f64) { /* ... */ }
    fn retirer(&mut self, montant: f64) { /* ... */ }
    fn fermer(self) { /* ... */ }
    fn renommer(self, nouveau_nom: String) -> CompteBancaire { /* ... */ }
}
```
J’ai compris la différence entre `&self` (emprunt immuable), `&mut self` (emprunt mutable) et `self` (consommation de l’objet).

---

## Fonctions

On définit une fonction avec `fn`.  Exemple :
```rust
fn addition(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
```
On peut aussi passer des références, comme `&str` pour les chaînes de caractères.

---

## Les conditions et les boucles

Les conditions s’écrivent comme en C ou Python :
```rust
if nombre % 2 == 0 {
    println!("Pair");
} else {
    println!("Impair");
}
```
Plusieurs types de boucles :
- `for i in 1..=10 { ... }` (intervalle inclusif)
- `while compteur < 4 { ... }`
- `loop { ... }` (boucle infinie, on sort avec `break`)

---

## Tableaux, vecteurs et itérations

Les tableaux ont une taille fixe, les vecteurs (`Vec`) sont dynamiques.  On peut itérer sur un tableau ou un vecteur avec `.iter()` et obtenir l’index avec `.enumerate()` :
```rust
for (i, nom) in noms.iter().enumerate() {
    println!("Nom {} : {}", i, nom);
}
```
Pratique pour afficher un menu ou lister des éléments.

---

## Les menus et la gestion des choix

On peut afficher un menu et demander à l’utilisateur de saisir un choix, puis utiliser ce choix pour exécuter une action.  Il faut bien vérifier que l’entrée de l’utilisateur est valide (parsing, bornes, etc.).

---

## Le match

Le `match` permet de faire des tests sur une variable, un peu comme un switch en C :
```rust
match nombre {
    1 => println!("Un"),
    2 => println!("Deux"),
    _ => println!("Autre nombre"),
}
```

---

## Points bonus réalisés

- Ajout d’une méthode `renommer` qui renvoie un nouveau compte avec le nom changé
- Interdiction de déposer un montant négatif
- Utilisation d’un `Vec<CompteBancaire>` pour gérer plusieurs comptes, avec `.iter()` et `.enumerate()` pour lister les comptes

---

**En résumé**, ce TP m’a permis de mieux comprendre la syntaxe de Rust, la gestion des variables, des structures, des fonctions, des boucles, et la façon d’organiser un programme.  J’ai aussi vu comment interagir avec l’utilisateur et gérer des collections de données.
