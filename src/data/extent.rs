use gdal::{
    spatial_ref::SpatialRef,
    vector::{Geometry, Layer, LayerAccess},
};
use geo::{BoundingRect, GeometryCollection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extent {
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
}

impl Extent {
    pub fn reproject(self, ssr: SpatialRef, dsr: SpatialRef) -> Self {
        let mut new_extent = Geometry::bbox(self.xmin, self.ymin, self.xmax, self.ymax).unwrap();
        new_extent.set_spatial_ref(ssr);
        new_extent.transform_to_inplace(&dsr).unwrap();
        let new_envelope = new_extent.envelope();
        Self {
            xmin: new_envelope.MinX,
            ymin: new_envelope.MinY,
            xmax: new_envelope.MaxX,
            ymax: new_envelope.MaxY,
        }
    }
}

impl From<&Layer<'_>> for Extent {
    fn from(layer: &Layer) -> Self {
        if let Ok(extent) = layer.get_extent() {
            Self {
                xmin: extent.MinX,
                ymin: extent.MinY,
                xmax: extent.MaxX,
                ymax: extent.MaxY,
            }
        } else {
            Self::default()
        }
    }
}

impl From<Vec<geo::Geometry>> for Extent {
    fn from(geometries: Vec<geo::Geometry>) -> Self {
        let multi = GeometryCollection::new_from(geometries);
        let extent = multi.bounding_rect();
        if let Some(extent) = extent {
            Self {
                xmin: extent.min().x,
                ymin: extent.min().y,
                xmax: extent.max().x,
                ymax: extent.max().y,
            }
        } else {
            Self::default()
        }
    }
}
