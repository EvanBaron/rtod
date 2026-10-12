### Contraintes

- Tout en Rust (inférence + pipeline temps réel)
- Le moins de crates possible : chaque nouvelle crate est validée par l'équipe

### Crates validées

| Besoin                   | Crate                              |
| ------------------------ | ---------------------------------- |
| Réseau de neurones / GPU | `burn` (alternative : `candle`)    |
| Caméra                   | `nokhwa` (alternative : `cameras`) |
| Affichage                | `minifb`                           |
| Sérialisation            | `serde`, `serde_json`              |

### Décisions prises

- Cible principale : GPU NVIDIA. Le téléphone reste une option (embarqué, ou envoi du flux vers le GPU), à évaluer en phase 2
- Objectifs temps réel : ≥ 30 FPS, latence < 50 ms, en 640×480
- Entraînement en Rust avec `burn`
- Classes de la démo : la super-catégorie COCO _appliance_ (`refrigerator`, `oven`, `microwave`, `sink`, `toaster`), avec la partie `handle` pour les sous-objets. C'est un point de départ simple à tester. Le système vise à terme des objets quelconques, donc aucune liste de classes n'est codée en dur (elle vient des métadonnées du modèle)
- Architecture type DETR (object queries) pour la baseline
- Caméra monoculaire + estimation de profondeur. Une OAK-D pourra venir ensuite : la source de profondeur (estimée ou capteur) est derrière un trait de `rtod-geometry`, interchangeable sans toucher au pipeline
- API maison en TCP (`std::net`) + JSON (`serde_json`), (voir [API](API.md))
- Encodeur SSM (Mamba) écarté, peu adapté à une image seule. On expérimente à la place une mémoire de type Titans, comparée à la baseline (voir [Modele](Modele.md))
