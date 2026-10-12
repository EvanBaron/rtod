# RTOD : Real-Time Object Detection

Détection, suivi et localisation 3D d'objets (et de leurs sous-objets) dans un flux caméra en temps réel, en Rust.

## Prérequis

- Rust ≥ 1.85 (édition 2024), via [rustup](https://rustup.rs)
- Une caméra (webcam suffit pour le développement)
- Linux : bibliothèques X11 ou libxkbcommon pour l'affichage

Sous NixOS, `nix-shell` (ou `direnv allow`) fournit l'environnement.

## Compiler et lancer

```sh
cargo build                 # tout le workspace
cargo run                   # lance le binaire rtod
cargo test --workspace      # tous les tests
cargo doc --workspace --open
```

## Structure

```
rtod/
├── Cargo.toml        # workspace + binaire principal `rtod`
├── src/              # assemblage du pipeline (lib.rs) et point d'entrée (main.rs)
├── crates/
│   ├── rtod-api/       # types partagés + protocole de l'API
│   ├── rtod-capture/   # caméra
│   ├── rtod-model/     # réseau de neurones
│   ├── rtod-track/     # suivi
│   ├── rtod-geometry/  # profondeur, repère robot
│   └── rtod-viewer/    # affichage debug
├── tests/            # tests end-to-end
├── prototype/        # prototype Python (YOLO + OpenCV), référence uniquement
└── docs/             # conception : objectifs, architecture, API, modèle
```

Le rôle de chaque crate et les règles de dépendance sont décrits dans [docs/Architecture.md](docs/Architecture.md).

## Documentation

Point d'entrée : [docs/Index.md](docs/Index.md).

- [Objectifs](docs/Objectifs.md) · [Contraintes](docs/Contraintes.md) · [Tâches](docs/Taches.md)
- [Architecture](docs/Architecture.md) · [Modèle](docs/Modele.md) · [API](docs/API.md) · [Pistes de recherche](docs/Pistes.md)

## Prototype Python

`prototype/main.py` fait tourner YOLOv8 sur la webcam, pour servir de référence visuelle avant la version Rust.

```sh
nix-shell                   # crée .venv et installe prototype/requirements.txt
python prototype/main.py    # `q` pour quitter
```

Les poids (`*.pt`) ne sont pas track par github, Ultralytics télécharge `yolov8n.pt` au premier lancement.

## Contribuer

Voir [CONTRIBUTING.md](CONTRIBUTING.md).

## Licence

Au choix, sous l'une des deux licences suivantes :

- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT ([LICENSE-MIT](LICENSE-MIT))
