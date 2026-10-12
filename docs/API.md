### Principe

Le système publie ce qu'il voit et répond aux questions.
Format inspiré de ROS 2 `vision_msgs` pour faciliter une intégration future.

### Potentiel modèle de données

```rust
struct Detection {
    track_id: u64,                  // identité stable entre frames
    parent_id: Option<u64>,         // poignée -> frigo
    label: String,
    confidence: f32,
    bbox_2d: [f32; 4],              // pixels (x, y, w, h)
    position_3d: Option<[f32; 3]>,  // mètres, repère robot
    grasp_point: Option<[f32; 3]>,  // sous-objets saisissables
    timestamp_ns: u64,              // instant de capture de la frame
}
```

### Transport

Protocole maison, sans crate réseau : **TCP + JSON**, avec `std::net` et `serde_json`.

- RTOD est le serveur, sur un port configurable. Les clients s'y connectent (robot, outil de debug, script de démo…)
- **Un objet JSON par ligne** (terminé par `\n`), dans les deux sens
- Chaque message a un champ `type`
- Pour tester sans écrire de client : `nc localhost <port>`, puis taper une requête

### Messages

Client → RTOD :

| `type`      | Champs                         | Effet                                                          |
| ----------- | ------------------------------ | -------------------------------------------------------------- |
| `subscribe` | -                              | Recevoir les détections de chaque frame à partir de maintenant |
| `find`      | `id`, `label`, `parent` (opt.) | Détections de ce label dans la dernière frame                  |

RTOD → client :

| `type`        | Champs                       | Quand                                    |
| ------------- | ---------------------------- | ---------------------------------------- |
| `detections`  | `timestamp_ns`, `detections` | À chaque frame, pour les clients abonnés |
| `find_result` | `id`, `detections`           | Réponse à un `find`, avec le même `id`   |
| `error`       | `id` (ou `null`), `message`  | Requête invalide (`id` nul si illisible) |

Exemple d'échange :

```
→ {"type":"find","id":1,"label":"frigo"}
← {"type":"find_result","id":1,"detections":[{"track_id":42,"label":"frigo",...}]}
→ {"type":"find","id":2,"label":"poignee","parent":42}
← {"type":"find_result","id":2,"detections":[{"track_id":57,"parent_id":42,...}]}
→ {"type":"subscribe"}
← {"type":"detections","timestamp_ns":1728380000000000000,"detections":[...]}
```

### Trait de transport

Le pipeline ne connaît pas TCP. `rtod-api` définira un trait, par exemple :

```rust
trait Publisher {
    /// Publie les détections d'une frame. Le serveur garde la dernière
    /// pour répondre aux requêtes `find`.
    fn publish(&mut self, timestamp_ns: u64, detections: &[Detection]);
}
```

Le serveur TCP en est une implémentation. Si un robot arrive un jour, on ajoute une autre implémentation sans toucher au pipeline. Voir [Pistes](Pistes.md), section "Intégration robot".

### Règles

- Sérialisation JSON au début (lisible), format binaire seulement si nécessaire
- Un client lent ne doit jamais bloquer le pipeline : on jette ses vieux messages plutôt que d'attendre
- `timestamp_ns` est obligatoire, un robot bouge entre la capture et la réception
- Les positions sont toujours dans le repère robot, la calibration caméra → robot est un fichier de config
