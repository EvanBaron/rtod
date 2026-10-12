### Objectif principal

Détecter, localiser et suivre plusieurs objets dans un flux caméra en temps réel, et rendre ces informations accessibles à un autre système.

### Fonctionnalités

- Détection multi-objets et multi-occurrences (plusieurs frigos = plusieurs détections)
- Suivi : chaque objet garde un identifiant stable entre les frames
- Position 3D dans le repère du robot (pas seulement une boîte en pixels)
- Mode recherche : "où est le frigo ?" → réponse ciblée
- Affichage des bounding-boxes (debug / démo)

### Bonus

- Sous-objets : détecter les parties d'un objet (poignée du frigo) avec un lien parent → enfant
- Apprentissage en direct : ajouter un nouvel objet sans réentraîner (prototypes, puis piste ATLAS)

### Démo finale visée

Caméra pointée sur une cuisine → le système répond à la requête "frigo" avec sa position, puis à "poignée" avec un point de préhension.
