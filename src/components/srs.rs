use focusable::Focus;
use std::default;

use color_eyre::eyre::Result;
use gdal::vector::{Layer, LayerAccess};
use layout::Size;
use ratatui::{prelude::*, widgets::*};
use tui_scrollview::{self, ScrollView, ScrollViewState};

use super::{Component, FocusableWidget};
use crate::{action::Action, data::LayerInfo, tui::Frame};

#[derive(Debug, Default, Clone, Focus)]
pub struct Srs {
    pub name: String,
    pub wkt: String,
    pub proj4: String,
    pub is_focused: bool,
    pub state: ScrollViewState,
}

impl Srs {
    /* pub fn new(layer: &Layer) -> Self {
        match layer.spatial_ref() {
            Some(srs) => Self {
                name: srs.name().unwrap_or_default(),
                wkt: srs.to_pretty_wkt().unwrap_or_default(),
                proj4: srs.to_proj4().unwrap_or_default(),
            },
            None => Self::default(),
        }
    } */

    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_layerinfo(li: &LayerInfo) -> Self {
        Srs {
            name: li.srs.name.clone(),
            wkt: li.srs.wkt.clone(),
            proj4: li.srs.proj4.clone(),
            is_focused: false,
            state: Default::default(),
        }
    }

    pub fn line_count(&self) -> u16 {
        format!("{}\n{}\n{}\n", self.name, self.wkt, self.proj4)
            .lines()
            .count() as u16
    }

    pub fn line_width(&self) -> u16 {
        let line: String = format!("{}\n{}\n{}\n", self.name, self.wkt, self.proj4);
        let result = line
            .lines()
            //.filter_map(|l| Some(l))
            .max_by(|x, y| x.len().cmp(&y.len()));
        result.unwrap().len() as u16
    }
}

impl FocusableWidget for Srs {}

impl Component for Srs {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        if let Action::PassLayerInfo(li) = action {
            self.name = li.srs.name;
            self.proj4 = li.srs.proj4;
            self.wkt = li.srs.wkt;
        } else if let Action::ScrollDown = action {
            self.state.scroll_down();
        } else if let Action::ScrollUp = action {
            self.state.scroll_up();
        } else if let Action::ScrollLeft = action {
            self.state.scroll_left();
        } else if let Action::ScrollRight = action {
            self.state.scroll_right();
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
            .constraints(vec![
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(rects[1]);

        let rect = inner_rects[0];

        let mut block = Block::default()
            .title(block::Title::from("Srs").alignment(Alignment::Right))
            .borders(Borders::ALL);

        if self.is_focused {
            block = block.border_set(symbols::border::DOUBLE);
        }

        let srs_view = Paragraph::new(format!(
            "name: {}\nwkt: {}\nproj4: {}\n",
            self.name, self.wkt, self.proj4
        ))
        .block(block);
        /*   let l = List::new(self.items.clone())
        .block(block)
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
        ); */

        //f.render_widget(scrollview, rect);

        let mut scroll_view = ScrollView::new(Size::new(self.line_width(), self.line_count()));
        scroll_view.render_widget(
            srs_view,
            Rect::new(0, 0, self.line_width() - 1, self.line_count()),
        );
        f.render_stateful_widget(scroll_view, rect, &mut self.state);
        Ok(())
    }
}
