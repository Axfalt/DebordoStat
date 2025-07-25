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

## Output
- The script generates a plot showing how the probability changes as the threshold varies.

## Support
For issues or questions, please open an issue in the repository.

