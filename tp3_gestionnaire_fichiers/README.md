# TP3 – Gestionnaire de fichiers en Rust

## Sujet

Créer un programme Rust en ligne de commande permettant de :
- Lire un fichier
- Écrire dans un fichier
- Modifier un fichier
- Supprimer un fichier

Le tout en utilisant :
- Les boucles (`loop`, `while`, `for`)
- Le mot-clé `match`
- Les notions d'ownership et membership
- Les structures (`struct`) et leurs méthodes associées (`impl`)
- La gestion de la date avec la bibliothèque `chrono`

## Lancer le projet

```bash
cd tp3_gestionnaire_fichiers
cargo run
```

## Fonctionnalités

- **Lire un fichier** : Affiche le contenu d'un fichier texte.
- **Écrire dans un fichier** : Crée ou écrase un fichier avec le contenu saisi.
- **Modifier un fichier** : Remplace le contenu d'un fichier existant.
- **Supprimer un fichier** : Efface définitivement un fichier.
- **Afficher la date** : Affiche la date et l'heure actuelles (UTC).

## Notions Rust illustrées

### 1. Struct et impl
- `struct User` : structure représentant un utilisateur.
- `struct FileManager` : structure principale du gestionnaire, qui possède un membre `user` (membership).
- `impl FileManager` : méthodes associées à la structure, illustrant l'encapsulation et la réutilisation du code.

### 2. Ownership et Membership
- **Ownership** : Les valeurs (ex : `String`) ont un propriétaire unique. Le passage de variables à des fonctions/méthodes transfère ou emprunte la propriété selon le cas.
- **Membership** : Un champ d'une structure appartient à cette structure (ex : `user` dans `FileManager`).

### 3. Boucles
- Utilisation de `loop` pour le menu principal (répétition jusqu'à quitter).
- Possibilité d'utiliser `for` ou `while` pour d'autres traitements (ex : itérations sur des collections).

### 4. Match
- Utilisé pour traiter le choix de l'utilisateur dans le menu, de façon claire et sûre.

### 5. Chrono
- Utilisation de la bibliothèque `chrono` pour afficher la date et l'heure actuelles lors des opérations.

### 6. Gestion des fichiers
- Utilisation du module standard `std::fs` pour lire, écrire, modifier et supprimer des fichiers.

## Exemple d'utilisation

```
--- Gestionnaire de fichiers ---
1. Lire un fichier
2. Écrire dans un fichier
3. Modifier un fichier
4. Supprimer un fichier
5. Afficher la date
6. Quitter
Veuillez saisir votre choix :
```

Chaque opération demande le chemin du fichier et, si besoin, le contenu à écrire/modifier.

---

**Ce TP permet de mettre en pratique les concepts fondamentaux de Rust dans un contexte concret de gestion de fichiers.** 