# DebordoStat

## Project Goal

This project simulates and estimates the probability that a given target receives a certain fraction of attacks in a random allocation scenario. 
It is useful for statistical analysis of attack distributions, such as in gaming or resource allocation contexts.

## Main Script
- `debordo-stats.py`: Python script to run the simulation and plot the probability curve.

## Dependencies

### Python (Local)
- Python 3.8+
- numpy
- matplotlib

#### Installation
1. Install Python from [python.org](https://www.python.org/downloads/).
2. Install dependencies:
   ```bash
   pip install numpy matplotlib
   ```

### Devcontainer
A devcontainer setup allows you to run the project in a reproducible environment using VS Code and Docker.

#### Usage
1. Make sure you have [Docker](https://www.docker.com/) and [VS Code](https://code.visualstudio.com/) installed.
2. Open the project folder in VS Code.
3. When prompted, "Reopen in Container".
4. The container will automatically install Python and all dependencies.

## Running the Script

```bash
python debordo-stats.py
```

This will run the simulation and display a plot of estimated probability vs threshold.

## Customization
- You can adjust parameters such as `targets`, `attacking`, `threshold`, and `iterations` directly in the script for different scenarios.

## Parameters of `estimate_probability`

The main function of the script is `estimate_probability`, which estimates the probability that at least one target receives a given fraction of attacks. Here are its parameters:

- `targets` (int): Number of possible targets. Each attack is randomly assigned to one of these. It depends on the number of players alive and the current day.
- `attacking` (int): Total number of zombie to distribute among the targets. Just keep it close to reality, it doesn't matter much since it only affects the rounding.
- `threshold` (float): The maximum fraction (between 0 and 1) of total attacks that a target can receive to be considered a success (e.g., 0.2 for 20%).
- `nb_drapo` (int): Adjustment factor (default 40), used to scale the number of attacks. The effective number of attacks is `attacking * (nb_drapo/40)`.
- `iterations` (int): Number of simulation runs. Higher values give more accurate probability estimates.

**Return value:**
- Returns a float between 0 and 1, representing the estimated probability that at least one target receives at least the specified fraction of attacks.

You can adjust these parameters when calling the function to fit your scenario.

## Output
- The script generates a plot showing how the probability changes as the threshold varies.

## Support
For issues or questions, please open an issue in the repository.
