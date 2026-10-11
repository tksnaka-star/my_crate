use crate::{browser, engine::{Rect, self, Game, Renderer}};
use std::{collections::HashMap, thread::current};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use web_sys::HtmlImageElement;
use web_sys::{console};

#[derive(Deserialize)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Deserialize)]
struct Cell {
    frame: SheetRect,
}

#[derive(Deserialize)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}

pub struct WalkTheDog {
    image: Option<HtmlImageElement>,
    sheet: Option<Sheet>,
    frame: u8,
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog { 
            image: None, 
            sheet: None, 
            frame: 0 
        }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let json = browser::fetch_json("rhb.json").await?;

        let sheet: Sheet = serde_wasm_bindgen::from_value(json)?;

        let image = Some(engine::load_image("rhb.png").await?);
                        
        Ok(Box::new(WalkTheDog {
            image,
            sheet: Some(sheet),
            frame: self.frame,
        }))
    }
    fn update(&mut self) {
        if self.frame < 23 {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
    }

    fn draw(&self, renderer: &Renderer) {
        let current_sprite = (self.frame / 3) + 1;
        // console::log_1(&wasm_bindgen::JsValue::from_f64(current_sprite as f64));
        let frame_name = format!("Run ({}).png", current_sprite);
        // context.clear_rect(0.0, 0.0, 600.0, 600.0);
        let sprite = self
            .sheet
            .as_ref()
            .and_then(|sheet| sheet.frames.get(&frame_name))
            .expect("Cell not found");

        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });
        self.image.as_ref().map(|image| {
            renderer.draw_image(
                &image,
                &Rect {
                    x: sprite.frame.x.into(),
                    y: sprite.frame.y.into(),
                    width: sprite.frame.w.into(),
                    height: sprite.frame.h.into(),            
                },
                &Rect {
                    x: 300.0,
                    y: 300.0,
                    width: sprite.frame.w.into(),
                    height: sprite.frame.h.into(),
                },
            );
        });
    }
}

