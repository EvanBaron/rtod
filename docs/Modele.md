### Baseline

Détecteur pré-entraîné (RT-DETR ou YOLO) pour avoir un pipeline fonctionnel tôt et une référence de performance.

### Notre modèle : petit DETR

- Backbone CNN pré-entraîné (ResNet-18/34)
- Encodeur + décodeur transformer, object queries
- Partir des recettes RT-DETR / Deformable DETR : le DETR d'origine converge trop lentement (~500 epochs COCO)

### Expériences

| Expérience            | Idée                                                                                                          | Hypothèse                                                               |
| --------------------- | ------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| Mémoire Titans        | Les queries lisent une mémoire neuronale long terme (Titans), mise à jour à l'inférence selon la « surprise » | Se souvient des objets occultés ou sortis du champ, coût O(1) par frame |
| État temporel         | Chaque query garde un état récurrent (CfC) d'une frame à l'autre                                              | Meilleur suivi et stabilité, coût O(1) par frame                        |
| Queries conditionnées | Initialiser les queries depuis le label demandé ("frigo")                                                     | Mode recherche plus rapide et plus précis                               |

Encodeur SSM (Mamba) écarté : peu adapté à une image seule (cf. MambaOut, arXiv 2405.07992).

### Apprentissage en direct

1. Prototypes : un embedding moyen par classe, ajouter une classe = ajouter un vecteur
2. Piste ATLAS / Titans : mémoire mise à jour à l'inférence (recherche ouverte, conçu pour le texte)

### Évaluation

- Détection : mAP (protocole COCO)
- Suivi : MOTA / HOTA
- Temps réel : FPS, latence de bout en bout
- Toujours comparer à la baseline sur le même matériel
