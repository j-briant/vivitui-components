use std::default;

use color_eyre::eyre::Result;
use focusable::Focus;
use gdal::vector::{Layer, LayerAccess};
use layout::Size;
use ratatui::{prelude::*, widgets::*};
use tui_scrollview::{self, ScrollView, ScrollViewState};

use super::{Component, FocusableWidget};
use crate::{action::Action, data::LayerInfo, tui::Frame};

#[derive(Debug, Default, Clone, Focus)]
pub struct Fields {
    pub geom_field: Vec<String>,
    pub fields: Vec<(String, u32)>,
    pub is_focused: bool,
}

impl Fields {
    pub fn from_layerinfo(li: &LayerInfo) -> Self {
        Self {
            geom_field: li.fields.geom_field.clone(),
            fields: li.fields.fields.clone(),
            is_focused: false,
        }
    }
}

impl FocusableWidget for Fields {}

impl Component for Fields {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        if let Action::PassLayerInfo(li) = action {
            self.geom_field = li.fields.geom_field;
            self.fields = li.fields.fields;
        };
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let header_style = Style::default();

        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(Color::LightYellow);

        let header = ["Name", "Type"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 4), Constraint::Ratio(1, 4)])
            .split(rect);

        let inner_rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Ratio(1, 3),
                Constraint::Length(6),
                Constraint::Min(0),
            ])
            .split(rects[1]);

        let rect = inner_rects[2];

        let mut block = Block::default()
            .title(block::Title::from("Fields").alignment(Alignment::Right))
            .borders(Borders::ALL);

        if self.is_focused {
            block = block.border_set(symbols::border::DOUBLE);
        }

        let rows = self.fields.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => Color::Black,
                _ => Color::Gray,
            };
            let item = [&data.0, &data.1.to_string()];
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(Color::LightCyan).bg(color))
                .height(2)
        });

        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(10),
                Constraint::Min(10),
                Constraint::Min(10),
            ],
        )
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .bg(Color::Black)
        .highlight_spacing(HighlightSpacing::Always)
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

        f.render_widget(t, rect);
        Ok(())
    }
}
