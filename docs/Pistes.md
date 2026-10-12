Pistes de recherche classées par phase de [Taches](Taches.md).

### Outils Rust (toutes phases)

- **Burn** : [livre officiel](https://burn.dev/books/burn/), tenseurs, modules, backends, entraînement
- **Modèles Burn prêts à l'emploi** : [tracel-ai/models](https://github.com/tracel-ai/models), dont YOLOX (détection) et ResNet (backbone du petit DETR)
- **Import ONNX** : [burn-onnx](https://github.com/tracel-ai/burn-onnx), qui génère du code Rust à partir d'un `.onnx` dans `build.rs`. Voir la [liste des opérateurs supportés](https://github.com/tracel-ai/burn-onnx/blob/main/SUPPORTED-ONNX-OPS.md) (`GridSample`, utilisé par RT-DETR, y figure)
- **Caméra** : [nokhwa](https://github.com/l1npengtul/nokhwa), voir le dossier `examples/`
- **Affichage** : [minifb](https://github.com/emoon/rust_minifb), voir le dossier `examples/`
- **Réseau** : [chapitre serveur web multithread du Rust Book](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html), un serveur TCP avec `std::net`

### Phase 2 : Baseline

Objectif : un détecteur pré-entraîné qui tourne dans le pipeline, le plus vite possible.

- **RT-DETR** (arXiv 2304.08069) : [dépôt officiel](https://github.com/lyuwenyu/RT-DETR), avec le script d'export ONNX `rtdetrv2_pytorch/tools/export_onnx.py`. Il existe aussi une [version Hugging Face](https://github.com/huggingface/transformers/tree/main/src/transformers/models/rt_detr), plus lisible
- **RF-DETR** : [roboflow/rf-detr](https://github.com/roboflow/rf-detr), un DETR temps réel récent sous Apache-2.0 avec export ONNX. C'est une alternative si RT-DETR pose problème

### Phase 3 : Suivi & 3D

**Suivi**

- **ByteTrack** (arXiv 2110.06864) : [dépôt officiel](https://github.com/FoundationVision/ByteTrack), dont le cœur est dans `yolox/tracker/byte_tracker.py`
- **Filtre de Kalman** : [kalmanfilter.net](https://kalmanfilter.net/), un tutoriel pas à pas avec exemples numériques
- **Lecture en Rust** : [trackforge](https://docs.rs/trackforge) implémente déjà ByteTrack en Rust. À lire pour comparer, pas à ajouter comme dépendance

**Profondeur** (monoculaire pour l'instant, OAK-D possible ensuite, voir [Contraintes](Contraintes.md))

- Estimation : [Depth Anything V2](https://github.com/DepthAnything/Depth-Anything-V2) (arXiv 2406.09414) a des petits modèles exportables en ONNX. Prendre une variante **métrique** (intérieur) : la version de base donne une profondeur relative, inutilisable pour le repère robot
- OAK-D plus tard : la profondeur vient du capteur, il reste la calibration. Son SDK (DepthAI) est en C++ sans crate Rust officielle, à valider avec l'équipe le moment venu

**Format de l'API**

- [vision_msgs (ROS 2)](https://github.com/ros-perception/vision_msgs), notre référence pour [API](API.md)

**Intégration robot (bonus)**

- [Zenoh](https://zenoh.io/docs/getting-started/first-app/) et ses [exemples Rust](https://github.com/eclipse-zenoh/zenoh/tree/main/examples) : publication et requêtes, découverte automatique sur le réseau
- [rmw_zenoh](https://github.com/ros2/rmw_zenoh) : Zenoh comme transport de ROS 2, pour parler à un robot type ROS
- Il suffirait d'ajouter une implémentation Zenoh du trait de transport de `rtod-api` (voir [API](API.md)), sans toucher au pipeline

### Phase 4 : Petit DETR

- **DETR** (arXiv 2005.12872), pour comprendre les object queries et le matching hongrois de la loss
- **RT-DETR**, la recette d'entraînement à suivre ([Modele](Modele.md)). Le DETR d'origine converge trop lentement
- **Backbone** : ResNet dans [tracel-ai/models](https://github.com/tracel-ai/models), avec des poids pré-entraînés
- **Données** : [COCO](https://cocodataset.org/), restreint aux classes de la démo (super-catégorie *appliance*, voir [Contraintes](Contraintes.md))

### Phase 5 : Expériences

**Mémoire Titans**

- Titans (arXiv [2501.00663](https://arxiv.org/abs/2501.00663)) : une mémoire neuronale (petit MLP) mise à jour à l'inférence, d'autant plus fort que l'entrée est « surprenante » (gradient de la loss de mémoire), avec momentum et oubli
- Le papier propose trois façons de brancher la mémoire : comme contexte (MAC), comme porte (MAG) ou comme couche (MAL). MAC est le plus naturel ici : les queries du décodeur lisent la mémoire comme des tokens en plus
- Implémentation de référence : [lucidrains/titans-pytorch](https://github.com/lucidrains/titans-pytorch), classe `NeuralMemory`
- Le module est le même que pour la piste ATLAS (voir plus bas) : un seul portage Burn sert aux deux expériences
- Pourquoi pas Mamba : MambaOut (arXiv 2405.07992) montre que le SSM apporte peu sur une image seule

**État temporel (CfC)**

- CfC (arXiv 2106.13898) : [code des auteurs](https://github.com/raminmh/CfC)
- [ncps](https://github.com/mlech26l/ncps) : la bibliothèque des mêmes auteurs. La cellule CfC fait quelques dizaines de lignes et se porte facilement

**Sous-objets**

- Dataset [PACO](https://github.com/facebookresearch/paco) (arXiv 2301.01795), annoté en objets et parties (ex. poignées). Vérifier qu'il couvre les poignées de frigo, sinon annoter nous-mêmes quelques centaines d'images

**Apprentissage en direct : prototypes (à faire en premier)**

- Prototypical Networks (arXiv 1703.05175) : [code](https://github.com/jakesnell/prototypical-networks)
- Le principe : on moyenne l'embedding de quelques exemples pour obtenir le prototype de la classe, puis on classe un objet selon le prototype le plus proche. C'est simple, sans gradient, et ça sert de référence à battre pour ATLAS

### ATLAS : de la théorie à l'implémentation

ATLAS (arXiv 2505.23735) n'a **pas de code officiel** et a été conçu pour du texte. Pour le comprendre, il faut remonter la famille de modèles dont il est la dernière étape.

**Progression conseillée**

1. **TTT** (arXiv 2407.04620) : l'idée de base. L'état caché est un petit modèle (linéaire ou MLP) mis à jour par descente de gradient à chaque token. [Code officiel](https://github.com/test-time-training/ttt-lm-pytorch) (TTT-Linear, TTT-MLP)
2. **Titans** (arXiv [2501.00663](https://arxiv.org/abs/2501.00663)) : ajoute le momentum et un oubli (decay). Implémentation de référence : [lucidrains/titans-pytorch](https://github.com/lucidrains/titans-pytorch), surtout la classe `NeuralMemory`
3. **Miras** (arXiv 2504.13173), des mêmes auteurs : le cadre qui unifie TTT, Titans et ATLAS (objectif de mémoire, oubli, optimiseur). À lire pour comprendre _pourquoi_ ATLAS fait ses choix
4. **ATLAS** : [atlas-torch](https://github.com/danielquintas8/atlas-torch), une implémentation communautaire (MIT) partie de titans-pytorch, qui ajoute l'Omega rule, les features polynomiales et Muon. Le code clé est dans `neural_memory.py`
5. **Muon** : [article de Keller Jordan](https://kellerjordan.github.io/posts/muon/) et [code](https://github.com/KellerJordan/Muon). L'itération de Newton-Schulz (fonction `newtonschulz5`) tient en une dizaine de lignes

**L'algorithme en bref** (sections 3 et 5 du papier)

- **Mémoire** `M` : un MLP à 2 couches avec résiduel, `M(x) = x + W1 σ(W2 x)`
- **Omega rule** : au lieu d'apprendre seulement le dernier couple (clé, valeur), on minimise l'erreur sur une fenêtre glissante des `c` derniers couples : `Σ γ_i ‖M(φ(k_i)) − v_i‖²`, où `γ_i ∈ [0,1]` pondère chaque couple
- **Mise à jour** (Muon) :
  - `S_t = θ_t S_{t-1} + ∇loss` (momentum)
  - `M_t = α_t M_{t-1} − η_t NewtonSchulz_5(S_t)` (oubli `α`, pas `η`)
- **φ** : features polynomiales sur les clés, pour augmenter la capacité de la mémoire sans ajouter de paramètres

**Adaptation à notre cas** (hypothèse à valider, ce n'est pas dans le papier)

- Clé `k` : l'embedding d'un objet (sortie d'une query du décodeur DETR)
- Valeur `v` : l'embedding de sa classe
- Montrer un nouvel objet au système revient à écrire quelques couples (k, v) dans la mémoire. La fenêtre `c` correspond aux derniers exemples montrés
- Pour prédire : `M(k)` est comparé aux embeddings de classes, comme pour les prototypes
- **Conseil** : implémenter d'abord TTT-Linear (mise à jour en une ligne), puis ajouter le momentum et l'oubli (Titans), puis l'Omega rule et Muon. Comparer aux prototypes à chaque étape
