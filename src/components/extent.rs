use std::default;

use color_eyre::eyre::Result;
use gdal::vector::{Layer, LayerAccess};
use layout::Size;
use ratatui::{prelude::*, widgets::*};
use tui_scrollview::{self, ScrollView, ScrollViewState};

use super::Component;
use crate::{action::Action, data::LayerInfo, tui::Frame};

#[derive(Debug, Default, Clone)]
pub struct Extent {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub focus: bool,
}

impl Extent {
    pub fn from_layerinfo(li: &LayerInfo) -> Self {
        Self {
            xmin: li.extent.xmin,
            xmax: li.extent.xmax,
            ymin: li.extent.ymin,
            ymax: li.extent.ymax,
            focus: false,
        }
    }
}

impl Component for Extent {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        if let Action::PassLayerInfo(li) = action {
            self.xmin = li.extent.xmin;
            self.xmax = li.extent.xmax;
            self.ymin = li.extent.ymin;
            self.ymax = li.extent.ymax;
        } else if let Action::EnterExtent = action {
            self.focus = true;
        };
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 4), Constraint::Ratio(1, 4)])
            .split(rect);

        let inner_rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Ratio(1, 3), Constraint::Length(6)])
            .split(rects[1]);

        let rect = inner_rects[1];

        let mut block = Block::default()
            .title(block::Title::from("Extent").alignment(Alignment::Right))
            .borders(Borders::ALL);

        if self.focus {
            block = block.border_set(symbols::border::DOUBLE);
        }

        let xmin_line = Line::from(format!("xmin: {}", self.xmin));
        let xmax_line = Line::from(format!("xmax: {}", self.xmax));
        let ymin_line = Line::from(format!("ymin: {}", self.ymin));
        let ymax_line = Line::from(format!("ymax: {}", self.ymax));

        let extent = Paragraph::new(vec![xmin_line, xmax_line, ymin_line, ymax_line]).block(block);
        /*   let l = List::new(self.items.clone())
        .block(block)
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
        ); */

        //f.render_widget(scrollview, rect);

        f.render_widget(extent, rect);
        Ok(())
    }
}
