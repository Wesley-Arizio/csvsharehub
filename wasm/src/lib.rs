use wasm_bindgen::prelude::*;

use js_sys::{Array, Object};
use js_sys::Reflect;

// Function to compare objects by key dynamically
fn get_key_value(obj: &Object, key: &str) -> Option<JsValue> {
    Reflect::get(obj, &JsValue::from_str(key)).ok()
}

#[wasm_bindgen]
pub fn sort_items_by_key(key: &str, list: JsValue) -> JsValue {
    // Attempt to convert JsValue to Array and handle gracefully
    let arr = Array::from(&list);

    // Collect elements as Vec<Object>
    let mut objects: Vec<Object> = arr.iter()
        .filter_map(|val| val.dyn_into::<Object>().ok())
        .collect();

    // Sort the Vec<Object> by the specified key
    objects.sort_by(|a, b| {
        let a_key = get_key_value(a, key);
        let b_key = get_key_value(b, key);

        // Compare values as strings (for simplicity)
        a_key.as_ref().and_then(|v| v.as_string()).cmp(&b_key.and_then(|v| v.as_string()))
    });

    // Convert Vec<Object> back to Array
    let sorted_arr = Array::new();
    for obj in objects {
        sorted_arr.push(&obj);
    }

    sorted_arr.into()
}