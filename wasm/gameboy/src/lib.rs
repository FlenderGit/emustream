use base::{BaseConsole, Screen};
use rusty_boy_core::{gameboy, keypad::{Key, KeyEvent}};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct Gameboy {
    gameboy: gameboy::Gameboy,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum GameboyKey {
    Left = "left",
    Up = "up",
    Right = "right",
    Down = "down",
    A = "a",
    B = "b",
    Select = "select",
    Start = "start",
}

#[wasm_bindgen]
impl Gameboy {
    #[wasm_bindgen(constructor)]
    pub fn new(rom: Vec<u8>, skip_checksum: bool) -> Result<Gameboy, JsValue> {
        gameboy::Gameboy::new_from_data(&rom, skip_checksum)
            .map(|gameboy| Gameboy { gameboy })
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen]
    pub fn render(&mut self, delta_time: f32) {
        self.gameboy.render(delta_time);
    }

    /* pub fn get_screen(&self) -> (u32, u32) {
        (160, 144)
    } */

    #[wasm_bindgen]
    pub fn get_screen_ptr(&self) -> *const u8 {
        self.gameboy.get_screen_ptr()
    }

    #[wasm_bindgen]
    pub fn press_key(&mut self, key: GameboyKey) {
        if let Ok(key) = Key::try_from(key) {
            self.gameboy.update_input(KeyEvent::Press(key));
        }
    }

    #[wasm_bindgen]
    pub fn release_key(&mut self, key: GameboyKey) {
        if let Ok(key) = Key::try_from(key) {
            self.gameboy.update_input(KeyEvent::Release(key));
        }
    }
}

impl TryFrom<GameboyKey> for Key {
    type Error = ();

    fn try_from(key: GameboyKey) -> Result<Self, Self::Error> {
        match key {
            GameboyKey::Left => Ok(Key::Left),
            GameboyKey::Up => Ok(Key::Up),
            GameboyKey::Right => Ok(Key::Right),
            GameboyKey::Down => Ok(Key::Down),
            GameboyKey::A => Ok(Key::A),
            GameboyKey::B => Ok(Key::B),
            GameboyKey::Select => Ok(Key::Select),
            GameboyKey::Start => Ok(Key::Start),
            GameboyKey::__Invalid => Err(()),
        }
    }
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}
