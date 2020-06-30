#![feature(proc_macro_hygiene)]

use wasm_bindgen::prelude::*;
use web_sys;

use css_rs_macro::css;
use virtual_dom_rs::prelude::*;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde_json;

// Expose globals from JS for things such as request animation frame
// that web sys doesn't seem to have yet
//
// TODO: Remove this and use RAF from Rust
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.request_animation_frame
#[wasm_bindgen]
extern "C" {
    pub type GlobalJS;

    pub static global_js: GlobalJS;

    #[wasm_bindgen(method)]
    pub fn update(this: &GlobalJS);
}

#[derive(Serialize, Deserialize)]
pub struct Store {
    click_count: Rc<Cell<u32>>,
}

impl Store {
    fn click_count(&self) -> u32 {
        self.click_count.get()
    }
    fn increment_click(&mut self) {
        web_sys::console::log_1(&"Updating state".into());
        global_js.update();
        self.click_count.set(self.click_count.get() + 1);
    }
    pub fn from_json(state_json: &str) -> Self {
        serde_json::from_str(state_json).unwrap()
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

struct HomeView {
    store: Rc<RefCell<Store>>,
}

impl HomeView {
    fn new(store: Rc<RefCell<Store>>) -> HomeView {
        HomeView { store }
    }
}

impl View for HomeView {
    fn render(&self) -> VirtualNode {
        let store = Rc::clone(&self.store);

        let click_count = self.store.borrow().click_count();
        let click_count = &*click_count.to_string();


        html! {
          // Use regular Rust comments within your html
          <div class="big blue">
            /* Interpolate values using braces */
            <strong>{ click_count }</strong>

            <button
              class=MY_COMPONENT_CSS
              onclick=move |_event: web_sys::Event| {
                web_sys::console::log_1(&"Button Clicked!".into());
                store.borrow_mut().increment_click();
              }
            >
              // No need to wrap text in quotation marks (:
              Click me and check your console
            </button>
          </div>
        }
    }
}

pub struct App {
    view: HomeView,
    pub store: Rc<RefCell<Store>>,
}

impl App {
    pub fn new(json: &str) -> App {
        let store = Rc::new(RefCell::new(Store::from_json(json)));

        let view = HomeView::new(Rc::clone(&store));

        App { view, store }
    }
    pub fn render(&self) -> VirtualNode {
        self.view.render()
    }
}

static MY_COMPONENT_CSS: &'static str = css! {r#"
:host {
    font-size: 24px;
    font-weight: bold;
}
"#};

static _MORE_CSS: &'static str = css! {r#"
.big {
  font-size: 30px;
  font-weight: bold;
}

.blue {
  color: blue;
}
"#};
