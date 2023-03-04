// El asterísco al final para traernos todo lo de la librería
use wasm_bindgen::prelude::*;

// Con esto podemos llamar funciones del navegador
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn saludar(nombre: &str) {
    alert(&format!("Hola {}, ¿cómo estás?", nombre));
}
