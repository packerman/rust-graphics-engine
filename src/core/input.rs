use std::{cell::RefCell, collections::HashMap, rc::Rc};

use futures::channel::{self, mpsc::UnboundedReceiver};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlCanvasElement, KeyboardEvent};

enum KeyEvent {
    Up(KeyboardEvent),
    Down(KeyboardEvent),
}

pub struct KeyState {
    pressed: HashMap<String, KeyboardEvent>,
}

impl KeyState {
    pub fn new() -> KeyState {
        KeyState {
            pressed: HashMap::new(),
        }
    }

    pub fn is_pressed(&self, code: &str) -> bool {
        self.pressed.contains_key(code)
    }

    fn set_pressed(&mut self, code: &str, event: KeyboardEvent) {
        self.pressed.insert(code.into(), event);
    }

    fn set_released(&mut self, code: &str) {
        self.pressed.remove(code);
    }
}

pub struct KeyboardInput {
    receiver: UnboundedReceiver<KeyEvent>,
}

impl KeyboardInput {
    pub fn prepare(canvas: &HtmlCanvasElement) -> KeyboardInput {
        let (keyevent_sender, keyevent_receiver) = channel::mpsc::unbounded();
        let keydown_sender = Rc::new(RefCell::new(keyevent_sender));
        let keyup_sender = Rc::clone(&keydown_sender);
        let onkeydown = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let result = keydown_sender
                .borrow_mut()
                .start_send(KeyEvent::Down(event));
            if let Err(error) = result {
                error!("Cannot send key down event: {:#?}", error);
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        let onkeyup = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let result = keyup_sender.borrow_mut().start_send(KeyEvent::Up(event));
            if let Err(error) = result {
                error!("Cannot send key up event: {:#?}", error);
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        canvas.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
        canvas.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
        onkeydown.forget();
        onkeyup.forget();
        KeyboardInput {
            receiver: keyevent_receiver,
        }
    }

    pub fn process(&mut self, state: &mut KeyState) {
        loop {
            match self.receiver.try_next() {
                Ok(None) => break,
                Err(_error) => break,
                Ok(Some(event)) => match event {
                    KeyEvent::Up(event) => state.set_released(&event.code()),
                    KeyEvent::Down(event) => state.set_pressed(&event.code(), event),
                },
            }
        }
    }
}
