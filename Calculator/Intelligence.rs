// ।। ॐ नमः शिवाय ।। \\
/*
fn main() {
  println!("5 × 3 = {}.", (calc(5.0, 3.0, "×")));
  println!("15 ÷ 3 = {}.", (calc(15.0, 3.0, "÷")));
  println!("75 + 53 = {}.", (calc(75.0, 53.0, "+")));
  println!("5 - 63 = {}.", (calc(5.0, 63.0, "-")));
}
*/

use wasm_bindgen::prelude::*; // Import the WASM bridge tools

// attribute for WASM & public so that JS can read it \\
#[wasm_bindgen]
pub fn calc(n1: f32, n2: f32, op: &str) -> f32 {
  // << in-built method >> \\
  // << works like JS switch statement >> \\
  match op {
    "+" => n1 + n2,
    "-" => n1 - n2,
    "×" => n1 * n2,
    "÷" => n1 / n2,
    _ => 0.0 // safety fallback;
  }
}