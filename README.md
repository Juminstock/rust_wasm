<h1 align="center">Rust + WebAssembly (WASM)</h1>
<p>
  Acá te explicaré cómo podemos conectar el lenguaje de programación Rust con el navegador, ésto para utilizar funcionabilidades del web browser haciendo uso de WASM. Para hacer la conexión, hagamos los siguientes pasos:
  <ul>
    <li>Inserta el siguiente comando en la terminal: <code>cargo new --lib <em>nombre_de_tu_proyecto</em></code></li>
    <li>En el fichero <strong>Cargo.toml</strong> agrega esto:
      <ol>
        <li>Una sección llamada: <code>[lib]</code></li>
        <li>Debajo de esa sección: <code>crate-type = ["cdylib", "rlib"]</code></li>
        <li>En la sección <strong>[dependencies]</strong> agrega: <code>wasm-bindgen = "0.2.82"</code></li>
      </ol>
    </li>
    <li>
      Por último, en la terminal usa este comando: <code>wasm-pack build --target web</code> Esto lo que hará será crearte carpetas y archivos necesarios para utilizar WASM.
    </li>
  </ul>
</p>

Fichero Cargo.toml

```r
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2.82"
```
Por último, se ha creado una pagina html que hace uso de Rust y WASM.

index.html

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Mi primer proyecto en WASM</title>
    <script type="module">
        import init, {saludar} from "./pkg/rust_wasm.js"
        init().then(()=> {
            saludar("Ignacio");
        });
    </script>
</head>
<body>
</body>
</html>
