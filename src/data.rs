use extent::Extent;
use gdal::{
    errors::GdalError,
    spatial_ref::SpatialRef,
    vector::{Geometry, Layer, LayerAccess},
    Dataset, DatasetOptions, DriverManager, GdalOpenFlags,
};
use geo::{coord, Point};
//use geo::{BoundingRect, GeometryCollection};
//use geodesy::prelude::*;
//use geozero::ToGeo;
use fields::Fields;
use serde::{Deserialize, Serialize};
use srs::Srs;
use std::{borrow::BorrowMut, path::PathBuf};
use strum::Display;

pub mod extent;
pub mod fields;
pub mod srs;

lazy_static::lazy_static! {
    static ref DRIVERS: Vec<String> = {
        DriverManager::register_all();
        let count = DriverManager::count();
        let mut list: Vec<String> = vec![];
        for i in 0..count {
            if let Ok(d) = DriverManager::get_driver(i) {
            list.push(d.short_name())
            }
        }
        list
    };
}

lazy_static::lazy_static! {
    static ref DRIVERS_STR: Vec<&'static str> = {
        let v: Vec<&str> = DRIVERS.iter().map(|s| s.as_str()).collect();
        v
    };
}

fn get_dataset_options() -> DatasetOptions<'static> {
    DatasetOptions {
        open_flags: GdalOpenFlags::GDAL_OF_VECTOR,
        allowed_drivers: Some(&DRIVERS_STR),
        open_options: None,
        sibling_files: None,
    }
}

pub fn dataset(p: PathBuf) -> Result<Dataset, GdalError> {
    Dataset::open_ex(p, get_dataset_options())
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LayerInfo {
    pub name: String,
    pub extent: Extent,
    pub srs: Srs,
    pub fields: Fields,
    pub feature_number: u64,
    #[serde(skip)]
    pub geometries: Vec<geo::Geometry>,
}

impl From<&mut Layer<'_>> for LayerInfo {
    fn from(layer: &mut Layer) -> Self {
        let geometries: Vec<geo::Geometry> = layer
            .features()
            .map(|g| {
                g.geometry()
                    .unwrap()
                    .to_geo()
                    .unwrap_or(geo::Geometry::Point(geo::Point(coord! { x: 0., y: 0. })))
            })
            .collect();
        Self {
            name: layer.name(),
            extent: Extent::from(&*layer),
            fields: Fields::from(&*layer),
            srs: Srs::from(&*layer),
            feature_number: layer.feature_count(),
            geometries,
        }
    }
}

impl LayerInfo {
    pub fn from_dataset(dataset: &Dataset) -> Vec<Self> {
        dataset
            .layers()
            .map(|mut l| LayerInfo::from(l.borrow_mut()))
            .collect()
    }
}
