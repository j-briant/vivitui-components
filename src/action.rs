use std::{collections::HashMap, fmt, string::ToString};

use gdal::spatial_ref::SpatialRef;
use gdal::vector::LayerAccess;
use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize, Serialize,
};
use strum::Display;

use crate::data::LayerInfo;

#[derive(Debug, PartialEq, Clone, Serialize, Display, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    Refresh,
    Error(String),
    Help,
    EnterSrs,
    EnterExtent,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    UpLayer,
    DownLayer,
    PassLayerInfo(LayerInfo),
}
