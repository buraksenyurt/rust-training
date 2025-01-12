mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, meth-leb!");
}
/*
    Aşağıdaki fonksiyon belirli bir x aralığındaki y değerlerini hesaplamaktadır.
    Tipik bir parabol denklemi aslında.
    Parametreleri bu projede index.html sayfasından alacağız.

*/

#[wasm_bindgen]
pub fn calculate_parabola(
    a: f64,
    b: f64,
    c: f64,
    x_min: f64,
    x_max: f64,
    steps: usize,
) -> Vec<f64> {
    let mut points = Vec::new();
    let step_size = (x_max - x_min) / steps as f64;
    let mut x = x_min;

    for _ in 0..=steps {
        let y = a * x * x + b * x + c;
        points.push(x);
        points.push(y);
        x += step_size;
    }

    points
}
