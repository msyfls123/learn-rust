#![feature(proc_macro_hygiene)]

use wasm_bindgen::prelude::*;
use web_sys;

use css_rs_macro::css;
use virtual_dom_rs::prelude::*;

use std::cell::{RefCell, Cell};
use std::rc::Rc;

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

struct Store {
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
}

pub struct HomeView {
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


#[wasm_bindgen]
struct App {
  dom_updater: DomUpdater,
  view: HomeView
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new () -> App {
        let start_view = html! { <div> Hello </div> };

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let store = Rc::new(RefCell::new(Store {click_count: Rc::new(Cell::new(0))}));
        let mut dom_updater = DomUpdater::new_append_to_mount(start_view, &body);

        let view = HomeView::new(store);

        dom_updater.update(view.render());

        App { dom_updater, view }
    }
    pub fn render(&mut self) {
      self.dom_updater.update(self.view.render());
    }
}

static MY_COMPONENT_CSS: &'static str = css!{r#"
:host {
    font-size: 24px;
    font-weight: bold;
}
"#};

static _MORE_CSS: &'static str = css!{r#"
.big {
  font-size: 30px;
}

.blue {
  color: blue;
}
"#};