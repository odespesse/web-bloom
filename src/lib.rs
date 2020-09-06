mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use index_bloom::index::Index;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn load_search_engine(index_dump: String) {
    utils::set_panic_hook();
    let mut index = Index::new();
    index.index("toto".to_string(), "toto");
    index.index("toto2".to_string(), "toto");
    index.index("titi".to_string(), "titi");
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let search_field = document.get_element_by_id("search_field").expect("Could not get search field");
    let submit = document.get_element_by_id("submit_search").expect("Could not get submit");
    let search_results = document.get_element_by_id("search_results").expect("Could not get results list");

    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyEvent| {
        search_results.set_inner_html("");
        let search_value = document.get_element_by_id("search_field").expect("Could not get results list").dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
        let hits = index.search(&search_value);
        match hits {
            Some(documents) => {
                let list = document.create_element("ul").unwrap();
                search_results.append_child(&list).unwrap();
                for doc in documents {
                    let item = document.create_element("li").unwrap();
                    item.set_inner_html(doc);
                    list.append_child(&item).unwrap();
                }
            },
            None => {
                let message = document.create_element("p").unwrap();
                message.set_inner_html("Nothing found");
                search_results.append_child(&message).unwrap();
            }
        }
    }) as Box<dyn FnMut(_)>);
    search_field.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref()).unwrap();
    submit.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

