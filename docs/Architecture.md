### Pipeline

```
Caméra -> Prétraitement -> Modèle -> Suivi -> Projection 3D -> API (TCP)
                                                    └-> Affichage (debug)
```

Chaque frame traverse le pipeline une fois. Budget total visé : < 50 ms à 30 FPS.

### Modules (workspace cargo)

| Crate           | Rôle                                                |
| --------------- | --------------------------------------------------- |
| `rtod-capture`  | Lecture caméra, conversion en tenseur               |
| `rtod-model`    | Réseau de neurones, chargement des poids, inférence |
| `rtod-track`    | Tracker, identifiants stables                       |
| `rtod-geometry` | Profondeur, calibration, repère caméra → robot      |
| `rtod-api`      | Types partagés, protocole et serveur de l'API (TCP) |
| `rtod-viewer`   | Affichage du flux et des boîtes                     |
| `rtod`          | Binaire principal qui assemble le tout              |

Les crates sont dans `crates/`, le binaire `rtod` à la racine (`src/lib.rs` assemble le pipeline, `src/main.rs` le lance).

### Dépendances entre crates

```
                    rtod (binaire)
   ┌────────┬─────────┼─────────┬──────────┐
capture   model     track    geometry    viewer
   └────────┴─────────┼─────────┴──────────┘
                   rtod-api
```

- Toutes les crates dépendent de `rtod-api`, jamais les unes des autres : seul `rtod` les assemble
- Les versions des dépendances externes sont déclarées une seule fois dans `[workspace.dependencies]` du `Cargo.toml` racine

### Règles

- Les types de données (`Detection`, etc.) vivent dans `rtod-api` et sont partagés partout
- Chaque module est testable seul (données factices en entrée)
- Capture et inférence sur des threads séparés : on jette les vieilles frames plutôt que d'accumuler du retard

### Tests

| Type                    | Emplacement                                    |
| ----------------------- | ---------------------------------------------- |
| Unitaire                | `#[cfg(test)] mod tests` dans le fichier testé |
| Intégration d'une crate | `crates/<crate>/tests/`                        |
| Bout en bout            | `tests/` à la racine                           |

### Sous-objets

1. Détection de l'objet parent (frigo)
2. Crop de sa zone dans la frame **pleine résolution**
3. Seconde passe du modèle sur le crop → parties (poignée), liées par `parent_id`
