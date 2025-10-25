# Projet Rust

## Français

Ce projet est une application Rust qui génère une application de simulation d'attaque pour [MyHordes](https://myhordes.de/).
Vous pouvez facilement le compiler et l'utiliser grâce à [mise](https://mise.jdx.dev/), un outil de gestion des environnements de développement et des tâches.

La configuration des simulation est à faire dans le fichier `SimConfig.toml`:

```toml
[CONFIG]
tdg_interval = [ATK_MIN, ATK_MAX]
defense_range = [DEF_MIN, DEF_MAX]
points = 11
min_def = 60
nb_drapo = 39
day = 37
iterations = 10000
```
tdg_interval : intervalle de l'estimation TDG de l'attaque (min, max)
defense_range : intervalle des défenses à simuler (min, max)
min_def : défense minimale de chaque citoyens
nb_drapo : nombre de drapeaux utilisés par les citoyens
day : jour de la simulation
iterations : nombre d'itérations par défense et attaque différentes (10000 recommandé pour avoir des résultats consistants)

### Comment déployer et utiliser

1. **Utiliser le dev container**
   - Ouvrez le projet dans [GitHub Codespaces](https://github.com/features/codespaces)
   ou dans [Visual Studio Code](https://code.visualstudio.com/) avec l'extension "Dev Containers".
   - Toutes les dépendances et outils nécessaires seront installés automatiquement.
   - Vous pouvez alors utiliser le terminal intégré pour exécuter les tâches ci-dessous.

2. **Installer mise**
   - Rendez-vous sur https://mise.jdx.dev/ et suivez les instructions selon votre système d'exploitation (Préinstallé dans le devcontainer).
   - Installez la toolchain: 
     ```sh
     mise install
     ```

3. **Exécuter les tâches**
   - Ouvrez un terminal dans le dossier du projet.
   - Pour lancer une simulation (penser à configurer SimConfig.toml) :
       ```sh
       mise sim:run
       ```
   - Pour afficher les points dans une courbe (après une simulation) :
       ```sh
       mise sim:draw
       ```
   - Pour compiler le projet :
     ```sh
     mise sim:build
     ```
   - Listez les tâches disponibles :
     ```sh
     mise tasks
     ```


### Alternative :
**Télécharger directement les exécutables**
   - Après une publication, rendez-vous sur la page [GitHub Releases](https://github.com/Axfalt/DebordoStat/releases) pour télécharger l'exécutable adapté à votre système.

> **_NOTE:_** Les éxecutables n'incluent pas la partie graph.

---

## English

This project is a Rust application that generates an attack simulation tool for [MyHordes](https://myhordes.de/).
You can easily build and use it thanks to [mise](https://mise.jdx.dev/), a tool for managing development environments and tasks.

Simulation configuration is done in the `SimConfig.toml` file:

```toml
[CONFIG]
tdg_interval = [(ATK_MIN), (ATK_MAX)]
defense_range = [(DEF_MIN), DEF_MAX)]
points = 11
min_def = 60
nb_drapo = 39
day = 37
iterations = 10000
```
tdg_interval: attack TDG estimation interval (min, max)
defense_range: defense values to simulate (min, max)
min_def: minimum defense for each citizen
nb_drapo: number of flags used by citizens
day: simulation day
iterations: number of iterations per defense and attack (10,000 recommended for consistent results)

### How to deploy and use

1. **Use the dev container**
   - Open the project in [GitHub Codespaces](https://github.com/features/codespaces) or in [Visual Studio Code](https://code.visualstudio.com/) with the "Dev Containers" extension.
   - All dependencies and tools will be installed automatically.
   - You can then use the integrated terminal to run the tasks below.

2. **Install mise** 
   - Go to https://mise.jdx.dev/ and follow the instructions for your operating system (Preinstalled in devcontainer).
   - Install toolchain:
     ```sh
     mise install
     ```
3. **Run tasks**
   - Open a terminal in the project folder.
   - To build the project:
     ```sh
     mise sim:build
     ```
   - To run a simulation (make sure to configure SimConfig.toml):
     ```sh
     mise sim:run
     ```
   - To display points in a graph (after a simulation):
       ```sh
       mise sim:draw
       ```
   - List available tasks:
     ```sh
     mise tasks
     ```

### Alternative :
**Download exe from releases**
- After a release, visit the [GitHub Releases](https://github.com/Axfalt/DebordoStat/releases) page to download the executable for your operating system.

> **_NOTE:_** Executables does not include the plot tools.
  

---
