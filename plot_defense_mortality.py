import matplotlib.pyplot as plt
import re

def parse_scratch_data(file_path):
    """Parse les données du fichier scratch.txt"""
    defense_values = []
    mortality_probs = []

    with open(file_path, 'r', encoding='utf-8') as file:
        for line in file:
            # Chercher les lignes avec le format "Défense: X, Probabilité de mort: Y%"
            match = re.search(r'Défense:\s*([\d.]+),\s*Probabilité de mort:\s*([\d.]+)%', line)
            if match:
                defense = float(match.group(1))
                mortality = float(match.group(2))
                defense_values.append(defense)
                mortality_probs.append(mortality)

    return defense_values, mortality_probs

def create_defense_mortality_plot():
    """Crée la courbe défense vs probabilité de mort"""
    # Chemin du fichier scratch.txt
    scratch_file = r'C:\Users\axfal\AppData\Roaming\JetBrains\PyCharmCE2024.3\scratches\scratch.txt'

    # Parser les données
    defense, mortality = parse_scratch_data(scratch_file)

    # Créer le graphique
    plt.figure(figsize=(12, 8))
    plt.scatter(defense, mortality, alpha=0.6, s=30, color='blue', edgecolors='navy', linewidth=0.5)

    # Personnalisation du graphique
    plt.xlabel('Défense', fontsize=12)
    plt.ylabel('Probabilité de mort (%)', fontsize=12)
    plt.title('Relation entre Défense et Probabilité de Mort', fontsize=14, fontweight='bold')
    plt.grid(True, alpha=0.3)

    # Améliorer l'apparence
    plt.tight_layout()

    # Afficher quelques statistiques
    print(f"Nombre de points de données: {len(defense)}")
    print(f"Défense minimale: {min(defense):.1f}")
    print(f"Défense maximale: {max(defense):.1f}")
    print(f"Probabilité de mort minimale: {min(mortality):.3f}%")
    print(f"Probabilité de mort maximale: {max(mortality):.3f}%")

    # Sauvegarder et afficher
    plt.savefig('defense_mortality_plot.png', dpi=300, bbox_inches='tight')
    plt.show()

if __name__ == "__main__":
    create_defense_mortality_plot()
