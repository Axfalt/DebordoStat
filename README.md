# DebordoStat

## Français

DebordoStat est un simulateur Rust pour estimer la probabilité de mort ("débordement") en fonction de la défense dans [MyHordes](https://myhordes.de/).

### Fonctionnalités et utilisation

1. **Simulation principale (défense -> probabilité de mort)**
   - Configurez `SimConfig.toml` (section `[CONFIG]`).
   - Lancez:
     ```sh
     mise sim:run
     ```
   - Le fichier de sortie est `results.txt`.

2. **Mode d'estimation via observations (`ESTIMATION25`)**
   - Activez la section `[ESTIMATION25]` dans `SimConfig.toml`.
   - Si la section est absente, ce mode est simplement désactivé.
   - Ce mode utilise `observations` pour estimer la distribution d'attaque du jour.
   - Si l'estimation ne trouve aucun match, le programme revient automatiquement à `tdg_interval`.

3. **Option réacteur (`is_reactor_built`)**
   - Dans `[CONFIG]`, mettez:
     ```toml
     is_reactor_built = true
     ```
   - Quand activé, la simulation ajoute un bonus d'attaque aléatoire avant répartition.

4. **Génération du graphe défense / mortalité**
   - Après une simulation, tracez la courbe:
     ```sh
     mise sim:draw
     ```
   - Le script lit `results.txt` et génère `defense_mortality_plot.png`.

5. **Build de l'exécutable**
   - Build standard:
     ```sh
     mise sim:build
     ```
   - Build vers un dossier spécifique:
     ```sh
     mise sim:build --path ./out
     ```

### Exemple de configuration

```toml
[CONFIG]
tdg_interval = [20109, 20861]
defense_range = [20338, 20538]
points = 201
min_def = 46
nb_drapo = 0
day = 34
iterations = 30000
is_reactor_built = false

[ESTIMATION25]
enabled = true
iterations = 1000000
observations = [
  "24:17188-17674",
  "20:16010-16520",
  "15:18200-18690",
]
```

### Signification des paramètres

- `tdg_interval`: intervalle d'attaque utilisé en mode classique.
- `defense_range`: plage des valeurs de défense testées.
- `points`: nombre de points simulés entre min/max de `defense_range`.
- `min_def`: seuil de défense par cible pour compter un débordement.
- `nb_drapo`: nombre de drapeaux pris en compte.
- `day`: jour simulé.
- `iterations`: nombre d'itérations Monte Carlo par point (plus grand = plus stable, plus lent).
- `is_reactor_built`: active/désactive le bonus d'attaque lié au réacteur.
- `ESTIMATION25.enabled`: active/désactive l'estimation basée sur observations.
- `ESTIMATION25.iterations`: nombre d'itérations pour l'estimation.
- `ESTIMATION25.observations`: liste d'observations de la tour de guet. Format: `"ROUND:MIN-MAX"` où `ROUND` est le numéro de tour (1–24), `MIN` et `MAX` sont les bornes lues sur la tour. Exemple : `"24:17188-17674"`.

### Installation / exécution

1. Installez [mise](https://mise.jdx.dev/), puis:
   ```sh
   mise install
   ```
2. Liste des tâches disponibles:
   ```sh
   mise tasks
   ```

### Alternative

Vous pouvez aussi télécharger les exécutables depuis [GitHub Releases](https://github.com/Axfalt/DebordoStat/releases).

> Note: les exécutables publiés n'incluent pas la partie génération de graphe.

---

## English

DebordoStat is a Rust simulator to estimate death probability ("overflow") from defense values in [MyHordes](https://myhordes.de/).

### Features and how to use them

1. **Main simulation (defense -> death probability)**
   - Configure `SimConfig.toml` (`[CONFIG]` section).
   - Run:
     ```sh
     mise sim:run
     ```
   - Output is written to `results.txt`.

2. **Observation-based estimation mode (`ESTIMATION25`)**
   - Enable the `[ESTIMATION25]` section in `SimConfig.toml`.
   - If the section is omitted, this mode is simply disabled.
   - This mode estimates attack distribution from `observations`.
   - If no match is found, the program automatically falls back to `tdg_interval`.

3. **Reactor option (`is_reactor_built`)**
   - In `[CONFIG]`, set:
     ```toml
     is_reactor_built = true
     ```
   - When enabled, a random attack bonus is added before allocation.

4. **Defense/mortality plot generation**
   - After running a simulation:
     ```sh
     mise sim:draw
     ```
   - The script reads `results.txt` and creates `defense_mortality_plot.png`.

5. **Build executable**
   - Standard build:
     ```sh
     mise sim:build
     ```
   - Build to a custom directory:
     ```sh
     mise sim:build --path ./out
     ```

### Example configuration

```toml
[CONFIG]
tdg_interval = [20109, 20861]
defense_range = [20338, 20538]
points = 201
min_def = 46
nb_drapo = 0
day = 34
iterations = 30000
is_reactor_built = false

[ESTIMATION25]
enabled = true
iterations = 1000000
observations = [
  "24:17188-17674",
  "20:16010-16520",
  "15:18200-18690",
]
```

### Parameter reference

- `tdg_interval`: attack interval used in classic mode.
- `defense_range`: defense values range to simulate.
- `points`: number of sampled defense points between min/max.
- `min_def`: per-target defense threshold used for overflow check.
- `nb_drapo`: number of flags applied.
- `day`: simulated day.
- `iterations`: Monte Carlo iterations per point (higher = more stable, slower).
- `is_reactor_built`: enables/disables reactor attack bonus.
- `ESTIMATION25.enabled`: enables/disables observation-based estimator.
- `ESTIMATION25.iterations`: estimator iteration count.
- `ESTIMATION25.observations`: list of watchtower observations. Format: `"ROUND:MIN-MAX"` where `ROUND` is the round number (1–24), `MIN` and `MAX` are the bounds read from the watchtower. Example: `"24:17188-17674"`.

### Setup / run

1. Install [mise](https://mise.jdx.dev/), then:
   ```sh
   mise install
   ```
2. List available tasks:
   ```sh
   mise tasks
   ```

### Alternative

You can download prebuilt executables from [GitHub Releases](https://github.com/Axfalt/DebordoStat/releases).

> Note: published executables do not include graph generation tools.
