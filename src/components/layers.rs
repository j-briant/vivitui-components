use std::{borrow::BorrowMut, time::Instant};

use color_eyre::eyre::Result;
use gdal::{
    vector::{Layer, LayerAccess},
    Dataset, LayerIterator,
};
use ratatui::{prelude::*, widgets::*};

use super::{Component, Focus, FocusableWidget};
use crate::{action::Action, data::LayerInfo, tui::Frame};

#[derive(Debug, Focus, Clone)]
pub struct LayerList {
    pub layerinfos: Vec<LayerInfo>,
    is_focused: bool,
    state: ListState,
}

impl LayerList {
    pub fn new(dataset: Dataset) -> Self {
        let layerinfos = LayerInfo::from_dataset(&dataset);
        let state = ListState::default().with_selected(Some(0));
        Self {
            layerinfos,
            is_focused: true,
            state,
        }
    }

    fn names(&self) -> Vec<String> {
        let names: Vec<String> = self.layerinfos.iter().map(|l| l.name.clone()).collect();
        names
    }

    fn layer_idx(&self) -> usize {
        self.state.selected().unwrap_or(0)
    }

    /*     fn layer(&self) -> Layer {
        self.dataset
            .layer(self.state.selected().unwrap_or(0) as isize)
            .unwrap()
    } */

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.layerinfos.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // Select the previous item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.layerinfos.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl FocusableWidget for LayerList {}

impl Component for LayerList {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        if let Action::UpLayer = action {
            self.previous();
            Ok(Some(Action::PassLayerInfo(
                self.layerinfos[self.layer_idx()].clone(),
            )))
        } else if let Action::DownLayer = action {
            self.next();
            Ok(Some(Action::PassLayerInfo(
                self.layerinfos[self.layer_idx()].clone(),
            )))
        } else {
            Ok(None)
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Ratio(1, 4), // take the full high of the available space
            ])
            .split(rect);

        let rect = rects[0];

        let mut block = Block::default()
            .title(block::Title::from("Layer list").alignment(Alignment::Left))
            .borders(Borders::ALL);

        if self.is_focused {
            block = block.border_set(symbols::border::DOUBLE);
        }

        let l = List::new(self.names())
            .block(block)
            .highlight_symbol(">> ")
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED),
            );

        f.render_stateful_widget(l, rect, &mut self.state);
        Ok(())
    }
}
