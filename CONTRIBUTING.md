# Contribuer à RTOD

## Avant de commencer

Lire [docs/Objectifs.md](docs/Objectifs.md) et [docs/Architecture.md](docs/Architecture.md).

## Workflow

- Pas de commit direct sur `main` : une branche par tâche et par personne (`@evanbaron/capture/webcam`, `@evanbaron/model/baseline`…), puis une merge request relue par au moins une personne.
- Commits courts et ciblés, message à l'impératif : `capture: lire les frames de la webcam`.

## Avant d'ouvrir une merge request

```sh
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Les trois doivent passer sans erreur.
TODO: setup potentiel d'un workflow pour automatiser ces commandes.

## Où mettre le code

- Chaque crate a un rôle unique (voir [docs/Architecture.md](docs/Architecture.md)). En cas de doute, demander avant de créer un module.
- Les types partagés (`Detection`, etc.) vont dans `rtod-api`.
- `src/main.rs` reste le plus minimal possible, la logique d'assemblage va dans `src/lib.rs`.

## Tests

| Type                    | Emplacement                                    | Exemple                                      |
| ----------------------- | ---------------------------------------------- | -------------------------------------------- |
| Unitaire                | `#[cfg(test)] mod tests` dans le fichier testé | une fonction de calcul de boîte              |
| Intégration d'une crate | `crates/<crate>/tests/*.rs`                    | l'API publique de `rtod-track`               |
| End-to-end              | `tests/*.rs` à la racine                       | le pipeline complet sur des données factices |

Chaque crate doit être testable seule, avec des données factices : pas de caméra ni de GPU requis pour `cargo test`.

## Dépendances

Le projet limite volontairement les crates externes ([docs/Contraintes.md](docs/Contraintes.md)) :

1. Proposer la crate à l'équipe et l'ajouter au tableau « Crates validées ».
2. Déclarer sa version une seule fois dans `[workspace.dependencies]` du `Cargo.toml` racine.
3. L'utiliser dans une crate avec `nom.workspace = true`.

Même principe pour une nouvelle crate du workspace : on l'ajoute à `members` et à la doc d'architecture.

## Documentation

- Toute fonction ou type public a un commentaire `///`.
- Une décision d'architecture ou d'API est notée dans `docs/` dans la même merge request que le code.

## Licence

Sauf mention contraire explicite, toute contribution soumise au projet est publiée sous la double licence MIT OR Apache-2.0, sans conditions supplémentaires.
