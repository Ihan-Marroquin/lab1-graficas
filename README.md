# 🖼️ Relleno de Polígonos con Raylib en Rust

Proyecto de laboratorio que dibuja y rellena polígonos con coordenadas dadas, utilizando el algoritmo de barrido (Scanline) y trazado de bordes con Bresenham. El proyecto utiliza **Rust** y la biblioteca **raylib**.

---

## 📌 Características

- Relleno de polígonos de 4 o más puntos
- Soporte para agujeros internos (no se pintan)
- Bordes trazados con el algoritmo de Bresenham
- Salida exportada como imagen PNG (`out.png`)

---

## 📁 Estructura del proyecto

```
poligonos/
├── src/
│ ├── framebuffer.rs # Framebuffer personalizado con Raylib
│ ├── line.rs # Algoritmo de Bresenham para dibujar líneas
│ ├── main.rs # Punto de entrada del programa
│ └── poligono.rs # Lógica para escaneo y relleno de polígonos
├── .gitignore # Ignora archivos binarios y de compilación
├── Cargo.toml # Dependencias del proyecto
├── Cargo.lock # Estado exacto de versiones al compilar
└── out.png # Imagen de salida generada por el programa
```


---

## ⚙️ Ejecución

### 1. Clonar el repositorio

```
git clone https://github.com/tu-usuario/proyecto-poligonos.git
cd proyecto-poligonos
```

### 2. Ejecutar
```
cargo run
```
La salida será una imagen `out.png` con los polígonos rellenados y sus bordes celestes.

--- 

## 🛠️ Dependencias
- ### `raylib v5.5.1`

--- 

## 📄 Notas
- El archivo `.gitignore` evita subir código compilado y carpetas como /target
- La imagen `out.png` es compatible con cualquier visor de imágenes

--- 
## 👾 `out.png`
<img width="800" height="600" alt="out" src="https://github.com/user-attachments/assets/30478027-aa9f-46f2-8e92-333dc7a7aabf" />

---
## 🤖 Autor
### Ihan Marroquin - 23108
