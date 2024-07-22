use gdal::vector::{Layer, LayerAccess};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Fields {
    pub geom_field: Vec<String>,
    pub fields: Vec<(String, u32)>,
}

impl From<&Layer<'_>> for Fields {
    fn from(layer: &Layer<'_>) -> Self {
        let fields: Vec<(String, u32)> = layer
            .defn()
            .fields()
            .map(|f| (f.name(), f.field_type()))
            .collect();
        let geom_field: Vec<String> = layer.defn().geom_fields().map(|g| g.name()).collect();
        Self { geom_field, fields }
    }
}
