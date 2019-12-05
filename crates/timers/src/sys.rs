//! Raw bindings to the Javascript APIs we need, namely set(Timeout|Interval) and clear(Timeout|Interval).
//! Depending on how rustwasm/wasm-bindgen#1046 is resolved, we may be able to remove this at a later date.

use js_sys::Function;
use wasm_bindgen::prelude::*;

pub fn queue_microtask(handler: &Function) {
    global::queue_microtask(handler)
}

pub fn set_timeout(handler: &Function, timeout: i32) -> i32 {
    global::set_timeout(handler, timeout)
}

pub fn clear_timeout(token: i32) {
    global::clear_timeout(token)
}

pub fn set_interval(handler: &Function, timeout: i32) -> i32 {
    global::set_interval(handler, timeout)
}

pub fn clear_interval(token: i32) {
    global::clear_interval(token)
}

pub fn request_animation_frame(handler: &Function) -> f64 {
    window::request_animation_frame(handler)
}

pub fn cancel_animation_frame(token: f64) {
    window::cancel_animation_frame(token)
}

// See rustwasm/gloo#96 for more context on why we need these workarounds and polyfills.
#[wasm_bindgen(inline_js = "
export const global = (function () {
    if (typeof globalThis !== 'undefined') { return globalThis; }
    if (typeof self !== 'undefined') { return self; }
    if (typeof window !== 'undefined') { return window; }
    if (typeof global !== 'undefined') { return global; }
    throw new Error('unable to locate global object');
})();

if (typeof global.queueMicrotask !== 'function') {
    global.queueMicrotask = function (callback) {
        Promise.resolve()
            .then(callback)
            .catch(e => setTimeout(() => { throw e; }));
    };
}
")]
extern "C" {
    type global;

    #[wasm_bindgen(js_name = "queueMicrotask", static_method_of = global)]
    fn queue_microtask(handler: &Function);

    #[wasm_bindgen(js_name = "setTimeout", static_method_of = global)]
    fn set_timeout(handler: &Function, timeout: i32) -> i32;

    #[wasm_bindgen(js_name = "clearTimeout", static_method_of = global)]
    fn clear_timeout(token: i32);

    #[wasm_bindgen(js_name = "setInterval", static_method_of = global)]
    fn set_interval(handler: &Function, timeout: i32) -> i32;

    #[wasm_bindgen(js_name = "clearInterval", static_method_of = global)]
    fn clear_interval(token: i32);
}

#[wasm_bindgen]
extern "C" {
    type window;

    #[wasm_bindgen(js_name = "requestAnimationFrame", static_method_of = window)]
    fn request_animation_frame(handler: &Function) -> f64;

    #[wasm_bindgen(js_name = "cancelAnimationFrame", static_method_of = window)]
    fn cancel_animation_frame(token: f64);
}
