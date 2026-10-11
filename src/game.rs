mod rhb;
use crate::{
    browser, 
    engine::{Rect, self, Game, Renderer, KeyState, Point},
    game::rhb::RedHatBoy,
};
use std::{
    collections::HashMap, 
    // thread::current
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::Deserialize;
// use serde_wasm_bindgen::from_value;
use web_sys::HtmlImageElement;
// use web_sys::{console};

#[derive(Deserialize, Clone)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Deserialize, Clone)]
struct Cell {
    frame: SheetRect,
}

#[derive(Deserialize, Clone)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}

pub struct WalkTheDog {
    image: Option<HtmlImageElement>,
    sheet: Option<Sheet>,
    frame: u8,
    position: Point,
    rhb: Option<RedHatBoy>,
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog { 
            image: None, 
            sheet: None, 
            frame: 0,
            position: Point { x: 0, y: 0},
            rhb: None, 
        }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let json = browser::fetch_json("rhb.json").await?;
        let sheet: Option<Sheet> = serde_wasm_bindgen::from_value(json)?;
        let image = Some(engine::load_image("rhb.png").await?);

        let rhb = Some(RedHatBoy::new(
            sheet.clone().ok_or_else(|| anyhow!("No Sheet Present"))?,
            image.clone().ok_or_else(|| anyhow!("No image Present"))?,
        ));

        Ok(Box::new(WalkTheDog {
            image: image.clone(),
            sheet,
            position: self.position,
            frame: self.frame,
            rhb,
        }))
    }
    // async fn initialize(&self) -> Result<Box<dyn Game>> {
    //     let json = browser::fetch_json("rhb.json").await?;
    //     let sheet: Option<Sheet> = serde_wasm_bindgen::from_value(json)?;
    //     let image = Some(engine::load_image("rhb.png").await?);
                        
    //     Ok(Box::new(WalkTheDog {
    //         image: image.clone(),
    //         // sheet: Some(sheet),
    //         // sheet: Some(sheet).clone(),
    //         sheet,
    //         position: self.position,
    //         frame: self.frame,
    //         rhb: Some(RedHatBoy::new(
    //             sheet.clone().ok_or_else(|| anyhow!("No Sheet Present"))?,
    //             image.clone().ok_or_else(|| anyhow!("No image Present"))?,
    //         )),
    //     }))
    // }
    fn update(&mut self, keystate: &KeyState) {
        let mut velocity = Point { x: 0, y: 0 };
        if keystate.is_pressed("ArrowDown") {
            velocity.y += 3;
        }

        if keystate.is_pressed("ArrowUp") {
            velocity.y -= 3;
        }

        if keystate.is_pressed("ArrowRight") {
            velocity.x += 3;
        }

        if keystate.is_pressed("ArrowLeft") {
            velocity.x -= 3;
        }

        self.position.x += velocity.x;
        self.position.y += velocity.y;

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
                    x: self.position.x.into(),
                    y: self.position.y.into(),
                    width: sprite.frame.w.into(),
                    height: sprite.frame.h.into(),
                },
            );
        });
    }
}

