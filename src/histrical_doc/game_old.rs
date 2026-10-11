use std::collections::HashMap;

use crate::{
    browser, 
    engine::{Game, Renderer, Rect},
};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use web_sys::HtmlImageElement;

// pub struct WalkTheDog;

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let sheet: Sheet = browser::fetch_json("rhb.json")
            .await
            .expect("Could not fetch rhb.json");
        let sheet: Sheet = from_value(json)
            .expect("Could not convert rhb.json into a Sheet structure");

        let image = engine::load_image("rhb.png")
            .await
            .expect("Could not load rhb.png");

        Ok(Box::new(WalkTheDog {
            image: Some(image),
            sheet: Some(sheet),
            frame: Some(frame),
        }))
    }
    fn update(&mut self) {}
    fn draw(&self, renderer: &Renderer) {}
}

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
    frame: HashMap<String, Cell>
}

pub struct WalkTheDog {
    image: Option<HtmlImageElement>,
    sheet: Option<Sheet>,
    frame: u8,
}

fn draw(&self, renderer: &Renderer) {
    let frame_name = format!("Run ({}).png", frame + 1);
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
            &self.image,
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