use focusable::{Focus, FocusContainer};
use std::path::PathBuf;

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use gdal::Dataset;
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc; // 0.17.1

use crate::{
    action::Action,
    components::{
        extent::Extent, fields::Fields, fps::FpsCounter, home::Home, layers::LayerList, srs::Srs,
        Component, FocusableComponents, FocusableWidget,
    },
    config::Config,
    data,
    mode::Mode,
    tui,
};

pub struct App {
    pub config: Config,
    //pub dataset: Dataset,
    pub components: FocusableComponents, //Vec<Box<dyn FocusableWidget>>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub mode: Mode,
    //pub focusable_components: FocusableComponents,
    pub last_tick_key_events: Vec<KeyEvent>,
}

impl App {
    pub fn new(dataset: Dataset) -> Result<Self> {
        //let dataset = data::dataset(path).unwrap();
        let home = Home::new();
        let fps = FpsCounter::default();
        let layers = LayerList::new(dataset);
        let srs = Srs::from_layerinfo(&layers.layerinfos[0]);
        let extent = Extent::from_layerinfo(&layers.layerinfos[0]);
        let fields = Fields::from_layerinfo(&layers.layerinfos[0]);
        let config = Config::new()?;
        let mode = Mode::LayerList;
        /*         let focusable_components = FocusableComponents {
            children: vec![
                layers.clone().boxed(),
                srs.clone().boxed(),
                extent.clone().boxed(),
                fields.clone().boxed(),
            ],
        }; */
        Ok(Self {
            //dataset,
            components: FocusableComponents {
                children: vec![
                    //Box::new(home),
                    //Box::new(fps),
                    Box::new(layers),
                    Box::new(srs),
                    Box::new(extent),
                    Box::new(fields),
                ],
            },
            should_quit: false,
            should_suspend: false,
            config,
            mode,
            //focusable_components,
            last_tick_key_events: Vec::new(),
        })
    }

    /* fn set_mode(&mut self) {
        let index = self
            .focusable_mode
            .iter()
            .position(|&m| m == self.mode)
            .unwrap();

        if index < self.focusable_mode.len() - 1 {
            self.mode = self.focusable_mode[index + 1]
        } else {
            self.mode = self.focusable_mode[0]
        }
    } */

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        let mut tui = tui::Tui::new()?;
        //.tick_rate(self.tick_rate)
        //.frame_rate(self.frame_rate);
        // tui.mouse(true);
        tui.enter()?;

        for component in self.components.children.iter_mut() {
            component.register_action_handler(action_tx.clone())?;
        }

        for component in self.components.children.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }

        for component in self.components.children.iter_mut() {
            component.init(tui.size()?)?;
        }

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    tui::Event::Quit => action_tx.send(Action::Quit)?,
                    tui::Event::Tick => action_tx.send(Action::Tick)?,
                    tui::Event::Render => action_tx.send(Action::Render)?,
                    tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
                    tui::Event::Key(key) => {
                        // If key is tab we always switch mode
                        if key.code == KeyCode::Tab {
                            action_tx.send(Action::NextFocusableMode)?;
                        } else if key.code == KeyCode::BackTab {
                            action_tx.send(Action::PreviousFocusableMode)?;
                        } else if let Some(keymap) = self.config.keybindings.get(&self.mode) {
                            if let Some(action) = keymap.get(&vec![key]) {
                                log::info!("Got action: {action:?}");
                                action_tx.send(action.clone())?;
                            } else {
                                // If the key was not handled as a single key action,
                                // then consider it for multi-key combinations.
                                self.last_tick_key_events.push(key);

                                // Check for multi-key combinations
                                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                                    log::info!("Got action: {action:?}");
                                    action_tx.send(action.clone())?;
                                }
                            }
                        };
                    }
                    _ => {}
                }
                for component in self.components.children.iter_mut() {
                    if let Some(action) = component.handle_events(Some(e.clone()))? {
                        action_tx.send(action)?;
                    }
                }
            }

            while let Ok(action) = action_rx.try_recv() {
                if action != Action::Tick && action != Action::Render {
                    log::debug!("{action:?}");
                }
                match action {
                    Action::Tick => {
                        self.last_tick_key_events.drain(..);
                    }
                    Action::Quit => self.should_quit = true,
                    Action::Suspend => self.should_suspend = true,
                    Action::Resume => self.should_suspend = false,
                    Action::NextFocusableMode => {
                        if self.components.children.last().unwrap().is_focused() {
                            self.components.focus_first();
                        } else {
                            self.components.focus_next();
                        }
                    }
                    Action::PreviousFocusableMode => {
                        if self.components.children.first().unwrap().is_focused() {
                            self.components.focus_last();
                        } else {
                            self.components.focus_previous();
                        }
                    }
                    Action::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        tui.draw(|f| {
                            for component in self.components.children.iter_mut() {
                                let r = component.draw(f, f.size());
                                if let Err(e) = r {
                                    action_tx
                                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                        .unwrap();
                                }
                            }
                        })?;
                    }
                    Action::Render => {
                        tui.draw(|f| {
                            for component in self.components.children.iter_mut() {
                                let r = component.draw(f, f.size());
                                if let Err(e) = r {
                                    action_tx
                                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                        .unwrap();
                                }
                            }
                        })?;
                    }
                    _ => {}
                }
                for component in self.components.children.iter_mut() {
                    if let Some(action) = component.update(action.clone())? {
                        action_tx.send(action)?
                    };
                }
            }
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                tui = tui::Tui::new()?;
                //.tick_rate(self.tick_rate)
                //.frame_rate(self.frame_rate);
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }
}
