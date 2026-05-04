## SNAP-SHELL : Linux Sandboxing CLI
nap-Shell est un moteur de conteneurisation léger conçu pour isoler l'exécution de
commandes sur Linux. Développé en Rust, il combine les technologies Namespaces,
OverlayFS et Pivot Root pour créer une bulle d'exécution éphémère, protégeant ainsi le
système hôte contre toute modification accidentelle ou malveillante.

## Installation
Nécessite un noyau Linux moderne (5.4+) et l'outil de gestion Rust Cargo.
```bash
# Cloner le projet
git clone https://github.com/TON_PSEUDO/snap-shell.git
cd snap-shell

# Compiler en mode release pour de meilleures performances
cargo build --release
```
## Fonctionnalités

  - Isolation FS (OverlayFS) : Toutes les écritures sont redirigées vers une couche temporaire. Le système hôte reste intact.

  - Pivot Root : Neo est enfermé dans une nouvelle racine, rendant le système hôte invisible.

  - Network Sandboxing : Possibilité de couper totalement l'accès réseau à l'intérieur de la bulle.

  - Auto-Cleanup : Suppression automatique de toutes les traces (fichiers créés, montages) à la fermeture.
## Utilisation
L'outil propose plusieurs niveaux d'isolation via une interface simple :
## 1. Mode Standard (Bunker Fichiers)

Idéal pour tester des scripts qui modifient des fichiers système sans risque.
```bash
sudo ./target/release/snap-shell
```
## 2. Mode Paranoïaque (Isolation Totale)

Désactive le réseau en plus de l'isolation des fichiers.
```bash
sudo ./target/release/snap-shell --no-net
```
## 3. Aide et Options
```bash
./target/release/snap-shell --help
```





