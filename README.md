# WIK-DPS-TP01

Ce projet est un exemple d'application web simple utilisant le framework Actix Web en Rust. Il crée un serveur HTTP qui écoute sur l'adresse "127.0.0.1" et le port "8080". Lorsqu'un utilisateur accède à l'URL "/ping" du serveur, les en-têtes HTTP de la requête sont renvoyés en tant que réponse JSON.

## Prérequis

Avant de pouvoir exécuter cette application, assurez-vous d'avoir les éléments suivants installés sur votre système :

- [Rust](https://www.rust-lang.org/tools/install) : Le langage de programmation Rust.
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) : Le gestionnaire de paquets et de compilation de Rust.

## Exécution de l'application

Pour exécuter l'application, suivez ces étapes :

1. Clonez ce dépôt sur votre système :
   
   "git clone https://github.com/al3XIST/WIK-DPS-TP01.git"
   

2. Accédez au répertoire du projet :

   "cd WIK-DPS-TP01"
   

3. Exécutez l'application à l'aide de Cargo :
   
   "cargo run"

4. Modifier le port d'entrée via la variable "port" par défaut "8081":
   

5. Le serveur Actix Web sera maintenant en cours d'exécution. Vous pouvez accéder à l'URL `http://127.0.0.1:8081/ping` pour obtenir les en-têtes HTTP en format JSON.

## Comment ça fonctionne

L'application utilise Actix Web pour créer un serveur HTTP. Elle définit une route `/ping` qui correspond à la fonction `header`. Lorsque quelqu'un accède à cette route, la fonction `header` est appelée. Cette fonction récupère les en-têtes HTTP de la requête, les convertit en JSON et les renvoie en réponse HTTP.

