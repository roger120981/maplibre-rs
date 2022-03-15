use crate::layer::StyleLayer;
use crate::source::Source;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Style {
    pub version: u16,
    pub name: String,
    pub metadata: HashMap<String, String>,
    pub sources: HashMap<String, Source>,
    pub layers: Vec<StyleLayer>,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            version: 8,
            name: "Default Style".to_string(),
            metadata: Default::default(),
            sources: Default::default(),
            layers: vec![
                StyleLayer {
                    id: "transportation".to_string(),
                    typ: "line".to_string(),
                    maxzoom: None,
                    minzoom: None,
                    metadata: None,
                    paint: None,
                    source: None,
                    source_layer: Some("transportation".to_string()),
                },
                StyleLayer {
                    id: "building".to_string(),
                    typ: "fill".to_string(),
                    maxzoom: None,
                    minzoom: None,
                    metadata: None,
                    paint: None,
                    source: None,
                    source_layer: Some("building".to_string()),
                },
                StyleLayer {
                    id: "water".to_string(),
                    typ: "fill".to_string(),
                    maxzoom: None,
                    minzoom: None,
                    metadata: None,
                    paint: None,
                    source: None,
                    source_layer: Some("water".to_string()),
                },
                StyleLayer {
                    id: "boundary".to_string(),
                    typ: "line".to_string(),
                    maxzoom: None,
                    minzoom: None,
                    metadata: None,
                    paint: None,
                    source: None,
                    source_layer: Some("boundary".to_string()),
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        // language=JSON
        let style_json_str = r##"
        {
          "version": 8,
          "name": "Test Style",
          "metadata": {},
          "sources": {
            "openmaptiles": {
              "type": "vector",
              "url": "https://maps.tuerantuer.org/europe_germany/tiles.json"
            }
          },
          "layers": [
            {
              "id": "background",
              "type": "background",
              "paint": {"background-color": "rgb(239,239,239)"}
            },
            {
              "id": "transportation",
              "type": "line",
              "source": "openmaptiles",
              "source-layer": "transportation",
              "paint": {
                "line-color": "#3D3D3D"
              }
            },
            {
              "id": "boundary",
              "type": "line",
              "source": "openmaptiles",
              "source-layer": "boundary",
              "paint": {
                "line-color": "#3D3D3D"
              }
            },
            {
              "id": "building",
              "minzoom": 14,
              "maxzoom": 15,
              "type": "fill",
              "source": "openmaptiles",
              "source-layer": "building",
              "paint": {
                "line-color": "#3D3D3D"
              }
            }
          ]
        }
        "##;

        let style: Style = serde_json::from_str(&style_json_str).unwrap();
    }
}
