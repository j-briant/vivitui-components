use focusable::Focus;
use std::default;

use color_eyre::eyre::Result;
use gdal::{
    spatial_ref::SpatialRef,
    vector::{Layer, LayerAccess},
};
use layout::Size;
use ratatui::widgets::canvas::{Canvas, Map, MapResolution, Rectangle};
use ratatui::{prelude::*, widgets::*};

use super::{Component, FocusableWidget};
use crate::{action::Action, data::LayerInfo, tui::Frame};

#[derive(Debug, Focus, Clone)]
pub struct PositionMap {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub is_focused: bool,
}

impl PositionMap {
    pub fn from_layerinfo(li: &LayerInfo) -> Self {
        let reproj = li.extent.clone().reproject(
            SpatialRef::from_proj4(&li.srs.proj4).unwrap(),
            SpatialRef::from_epsg(4326).unwrap(),
        );
        Self {
            xmin: reproj.xmin,
            xmax: reproj.xmax,
            ymin: reproj.ymin,
            ymax: reproj.ymax,
            is_focused: false,
        }
    }
}

impl FocusableWidget for PositionMap {}

impl Component for PositionMap {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        if let Action::PassLayerInfo(li) = action {
            let reproj = li.extent.clone().reproject(
                SpatialRef::from_proj4(&li.srs.proj4).unwrap(),
                SpatialRef::from_epsg(4326).unwrap(),
            );
            self.xmin = reproj.xmin;
            self.xmax = reproj.xmax;
            self.ymin = reproj.ymin;
            self.ymax = reproj.ymax;
        };
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Fill(1)])
            .split(rect);

        let rect = rects[1];

        let mut block = Block::default()
            .title(block::Title::from("Position Map").alignment(Alignment::Left))
            .borders(Borders::ALL);

        if self.is_focused {
            block = block.border_set(symbols::border::DOUBLE);
        }

        let map = Canvas::default()
            .block(block)
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .paint(|ctx| {
                ctx.draw(&Map {
                    resolution: MapResolution::High,
                    color: Color::White,
                });
                ctx.draw(&Rectangle {
                    x: self.ymin,
                    y: self.xmin,
                    width: self.ymax - self.ymin,
                    height: self.xmax - self.xmin,
                    color: Color::Red,
                });
            })
            .marker(Marker::Braille);
        f.render_widget(map, rect);
        Ok(())
    }
}
