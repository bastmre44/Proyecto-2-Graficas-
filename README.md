# Lover House: Before the Fire / In Flames

Diorama interactivo realizado con **raytracing en CPU**, Rust y Raylib. La escena reconstruye de forma estilizada una casa de nueve habitaciones inspirada en la estética de *Lover House* y permite alternar entre su versión normal y su versión incendiándose.

> Proyecto académico sin fines comerciales. No incluye fotografías, música, logotipos ni recursos oficiales. Toda la geometría y las texturas procedurales fueron creadas para este proyecto.

## Características

- Diorama formado exclusivamente por cubos y prismas rectangulares.
- Nueve habitaciones con muebles, cortinas, lámparas, ventanas, puerta, escaleras y techo escalonado.
- Diorama exterior con terreno, jardín, árboles, setos, flores y camino de piedra.
- Estanque refractivo con borde de piedra y plantas flotantes.
- Faroles con vidrio y luces cálidas distribuidos por el jardín.
- Dos estados dentro de la misma escena: **Before the Fire** e **In Flames**.
- Cámara orbital, zoom y rotación automática.
- Rayos primarios, intersección rayo-caja y selección de la colisión más cercana.
- Luces puntuales, sombras, iluminación difusa y brillo especular.
- Cinco familias principales de materiales con texturas y parámetros propios.
- Reflexión recursiva en metal y superficies pulidas.
- Refracción en las ventanas de vidrio, con índice de refracción `1.52`.
- Skybox procedural distinto para el estado normal y el estado incendiado.
- Fuego emisivo, luces cálidas, humo procedural y superficies quemadas.
- Render rápido mientras la cámara se mueve y refinamiento automático al detenerse.
- Cálculo distribuido entre los núcleos disponibles del procesador.
- Framebuffer interno escalado con filtro bilineal.

## Controles

| Tecla | Acción |
|---|---|
| `A` / `D` o flechas laterales | Rotar alrededor del diorama |
| `W` / `S` o flechas verticales | Acercar o alejar la cámara, incluyendo primeros planos |
| Rueda del mouse | Zoom |
| `1` | Mostrar la casa normal |
| `2` | Mostrar la casa incendiándose |
| `F` | Alternar directamente entre los dos estados |
| `Espacio` | Activar o detener la rotación automática |
| `R` | Reiniciar la cámara |
| `Esc` | Cerrar el programa |

## Requisitos

- Rust estable 1.78 o posterior.
- Cargo.
- Windows 10/11, Linux o macOS con soporte para OpenGL.
- CMake y un compilador de C/C++ disponibles para compilar Raylib si el sistema lo requiere.

Raylib es la única dependencia de ejecución. Los vectores, intersecciones, materiales, texturas, reflexión, refracción, iluminación y skybox están implementados dentro del proyecto; no se utiliza un motor 3D ni una librería de raytracing.

## Ejecución

Desde la raíz del proyecto:

```powershell
cargo run --release
```

La primera compilación puede tardar mientras Cargo descarga y construye Raylib. Siempre debe utilizarse `--release`: el raytracing se calcula en CPU y la versión de depuración es considerablemente más lenta.



## Materiales

Cada familia posee su propia función de textura procedural y parámetros diferentes. Los valores exactos se encuentran en `src/material.rs`.

| Familia | Textura | Albedo | Specular | Transparencia | Reflectividad |
|---|---|---|---:|---:|---:|
| Pared pintada | Ruido fino de pintura | Depende de la habitación | 0.15 | 0.00 | 0.04 |
| Papel tapiz | Franjas onduladas | Depende de la era | 0.10 | 0.00 | 0.02 |
| Madera | Vetas direccionales | Marrón o teñido | 0.24 | 0.00 | 0.08 |
| Metal | Cuadrícula de paneles | Negro/dorado | 0.90 | 0.00 | 0.58 |
| Vidrio | Ondulación suave | Azul muy claro | 0.85 | 0.78 | 0.16 |



## Cómo funciona

Por cada píxel del framebuffer se lanza un rayo desde la cámara. El programa busca la caja más cercana que intersecta ese rayo y calcula su color considerando textura, luces, sombras y propiedades del material. Los rayos secundarios implementan reflejos y refracción hasta una profundidad máxima de tres rebotes.

La casa se construye una sola vez. Los objetos con `burning_only` se omiten en el estado normal y se activan al mostrar el incendio. Así ambos estados conservan exactamente la misma geometría y posición de cámara.

Durante el movimiento se calculan bloques de píxeles para mantener una respuesta rápida. Al soltar las teclas, el programa vuelve a calcular automáticamente una imagen de `480 × 270` con todos sus detalles. El zoom puede acercarse hasta la fachada para mostrar materiales, muebles y ventanas en el video.

## Estructura

```text
src/
├── main.rs        # Ventana, controles, framebuffer y HUD
├── camera.rs      # Cámara orbital y generación de rayos
├── framebuffer.rs # Buffer de color
├── geometry.rs    # Rayos, impactos e intersección con cubos
├── material.rs    # Materiales y texturas procedurales
├── math.rs        # Vector Vec3 y operaciones matemáticas
├── raytracer.rs   # Iluminación, sombras, reflexión, refracción y skybox
└── scene.rs       # Construcción de la casa, muebles, fuego y luces

```




### Video del diorama


[![Video pendiente]([\[https://img.youtube.com/vi/ID_DEL_VIDEO/0.jpg\](https://youtu.be/B6Rawiwi2lQ?si=935UOPbh0raAN8et)](https://youtu.be/B6Rawiwi2lQ?si=935UOPbh0raAN8et))](https://www.youtube.com/watch?v=ID_DEL_VIDEO)

## Capturas



1. `normal.png`: vista frontal de la casa normal.
2. `fire.png`: la misma vista con el incendio activado.
3. `reflection.png`: ángulo donde se aprecie el metal reflectante.
4. `refraction.png`: acercamiento a una ventana de vidrio.



|