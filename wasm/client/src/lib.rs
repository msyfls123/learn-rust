use virtual_dom_rs::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys;
use wasm::{App, Store};

#[wasm_bindgen]
pub struct Client {
    app: App,
    dom_updater: DomUpdater,
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
        let dom_updater = DomUpdater::new_replace_mount(app.render(), root_node);
        Client { app, dom_updater }
    }

    pub fn render(&mut self) {
        let vdom = self.app.render();
        self.dom_updater.update(vdom);
    }
}
