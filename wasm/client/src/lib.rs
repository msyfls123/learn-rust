use percy_dom::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys;
use wasm::{App, Store};

#[wasm_bindgen]
pub struct Client {
    app: App,
    pdom: PercyDom,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_state: &str) -> Client {
        let app = App::new(initial_state);
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let root_node = document.get_element_by_id("wasm-app")
          .unwrap();
        let pdom = PercyDom::new_replace_mount(app.render(), root_node);
        Client { app, pdom }
    }

    pub fn render(&mut self) {
        let vdom = self.app.render();
        self.pdom.update(vdom);
    }
}
