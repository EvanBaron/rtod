### Pôles

| Pôle       | Périmètre                            | Responsable |
| ---------- | ------------------------------------ | ----------- |
| Pipeline   | caméra, affichage, boucle temps réel | ?           |
| Modèle     | réseau, entraînement, évaluation     | ?           |
| Suivi & 3D | tracker, profondeur, repères         | ?           |
| API        | modèle de données, TCP               | ?           |

### Phase 1 : Pipeline sans IA

- Mise en place le dépôt (workspace cargo, CI, conventions)
- Capture caméra
- Affichage du flux
- Dessin de bounding-boxes (données factices)
- Publication des détections factices via l'API TCP

### Phase 2 : Baseline

- Charger un détecteur pré-entraîné (RT-DETR ou YOLO) dans le pipeline
- Mesurer FPS, latence et mAP
- Évaluer la cible téléphone : soit embarqué, soit flux envoyé au GPU (la latence réseau tient-elle dans le budget de 50 ms ?)

### Phase 3 : Suivi & 3D

- Tracker (Kalman + algorithme hongrois, type ByteTrack)
- Profondeur estimée (Depth Anything V2 métrique) derrière un trait, pour brancher une OAK-D plus tard
- Calibration caméra et passage au repère robot
- Requête "où est X ?"

### Phase 4 : Modèle Homemade

- Petit DETR (backbone pré-entraîné) entraîné sur les classes de la démo
- Comparaison avec la baseline

### Phase 5 : Experimentation

- Mémoire Titans entre les frames
- État récurrent temporel sur les queries
- Sous-objets : re-détection sur crop pleine résolution
- Apprentissage en direct par prototypes, puis piste ATLAS
