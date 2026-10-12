use serde::{Deserialize, Serialize};

/// Object detected by a frame.
///
/// Depending on needs, this type may change, this is currently only an idea.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Detection {
    /// Stable Id between frames.
    pub track_id: u64,
    /// Parent objet if it's a sub-object.
    pub parent_id: Option<u64>,
    pub label: String,
    pub confidence: f32,
    /// Bouding box `(x, y, w, h)`.
    pub bbox_2d: [f32; 4],
    /// Position in meters.
    pub position_3d: Option<[f32; 3]>,
    /// Grasp points for sub-ojects.
    pub grasp_point: Option<[f32; 3]>,
    /// Moment when the frame was captured.
    pub timestamp_ns: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detection_json_round_trip() {
        let detection = Detection {
            track_id: 1,
            parent_id: None,
            label: "frigo".into(),
            confidence: 0.9,
            bbox_2d: [10.0, 20.0, 100.0, 200.0],
            position_3d: Some([1.0, 0.0, 0.5]),
            grasp_point: None,
            timestamp_ns: 42,
        };

        let json = serde_json::to_string(&detection).unwrap();
        assert_eq!(serde_json::from_str::<Detection>(&json).unwrap(), detection);
    }
}
