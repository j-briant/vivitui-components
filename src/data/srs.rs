use gdal::vector::{Layer, LayerAccess};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Srs {
    pub name: String,
    pub wkt: String,
    pub proj4: String,
}

impl Srs {
    pub fn line_count(&self) -> usize {
        format!("{}\n{}\n{}\n", self.name, self.wkt, self.proj4)
            .lines()
            .count()
    }
}

impl From<&Layer<'_>> for Srs {
    fn from(layer: &Layer<'_>) -> Self {
        match layer.spatial_ref() {
            Some(srs) => Self {
                name: srs.name().unwrap_or_default(),
                wkt: srs.to_pretty_wkt().unwrap_or_default(),
                proj4: srs.to_proj4().unwrap_or_default(),
            },
            None => Self::default(),
        }
    }
}
