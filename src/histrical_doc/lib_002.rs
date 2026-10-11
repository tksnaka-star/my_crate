#[macro_use]
mod browser;
mod engine;
mod game;

// use std::rc::Rc;
// use std::sync::Mutex;
use std::collections::HashMap;
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{console};

use crate::{
    engine::GameLoop, 
    // engine::Game,
    game::WalkTheDog
};
// use rand::RngExt;

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
     // Your code goes here!
    console_error_panic_hook::set_once();
    // let context = browser::context().expect("Could not get browser context");

    browser::spawn_local(async move {
        let game = WalkTheDog::new();

        // let game = game
        //     .initialize()
        //     .await
        //     .expect("Could not initialize game");

        GameLoop::start(game)
            .await
            .expect("Could not start game loop");
        // let json = browser::fetch_json("rhb.json")
        //     .await
        //     .expect("Could not fetch rhb.json");

        // let sheet: Sheet = from_value(json)
        //     .expect("Could not convert rhb.json into a Sheet structure");

        // let image = engine::load_image("rhb.png")
        //     .await
        //     .expect("Could not load rhb.png");

        // let mut frame = -1;
        // let interval_callback = Closure::wrap(Box::new(move || {
        //     frame = (frame + 1) % 8;
        //     let frame_name = format!("Run ({}).png", frame + 1);
        //     context.clear_rect(0.0, 0.0, 600.0, 600.0);
        //     let sprite = sheet.frames.get(&frame_name).expect("Cell not found");
        //     context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        //         &image,
        //         sprite.frame.x.into(),
        //         sprite.frame.y.into(),
        //         sprite.frame.w.into(),
        //         sprite.frame.h.into(),
        //         300.0,
        //         300.0,
        //         sprite.frame.w.into(),
        //         sprite.frame.h.into(),
        //     );
        // }) as Box<dyn FnMut()>);

        // browser::window()
        //     .unwrap()
        //         .set_interval_with_callback_and_timeout_and_arguments_0(
        //         interval_callback.as_ref().unchecked_ref(),
        //         50,
        // );
        // interval_callback.forget();

    });

    // // ブラウザのアラートで Hello World! を表示
    // console::log_1(&"Hello World from Rust!".into());

    Ok(())
}
