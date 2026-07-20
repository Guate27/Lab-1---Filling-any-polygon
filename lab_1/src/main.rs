mod framebuffer;
mod line;
mod bmp;
mod polygon;

use crate::framebuffer::Framebuffer;
use crate::bmp::{WriteBmp, WritePng};
use crate::polygon::{Polygon, fill_polygon, fill_polygon_with_holes};

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    // Poligono 1 
    let poly1: Vec<(i32, i32)> = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    // Poligono 2 
    let poly2: Vec<(i32, i32)> = vec![
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    //  Poligono 3 
    let poly3: Vec<(i32, i32)> = vec![
        (377, 249), (411, 197), (436, 249),
    ];

    // Poligono 4 
    let poly4: Vec<(i32, i32)> = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36),
        (676, 37), (660, 52), (750, 145), (761, 179), (672, 192),
        (659, 214), (615, 214), (632, 230), (580, 230), (597, 215),
        (552, 214), (517, 144), (466, 180),
    ];

    // Poligono 5 (agujero dentro del Poligono 4 no se pinta)
    let poly5: Vec<(i32, i32)> = vec![
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];

    // Pintado de polígonos con algoritmo scanline fill
    fill_polygon(&mut framebuffer, &poly1, 0xFFFF00); // amarillo
    fill_polygon(&mut framebuffer, &poly2, 0x0000FF); // azul
    fill_polygon(&mut framebuffer, &poly3, 0xFFFFFF); // blanco

    // El Poligono 4 se rellena tomando en cuenta el Poligono 5 como agujero
    fill_polygon_with_holes(&mut framebuffer, &poly4, &[&poly5], 0xFF0000); // rojo

    // aristas dibujadas con Bresenham
    Polygon::new(poly1).draw_edges(&mut framebuffer, 0xFFFFFF);
    Polygon::new(poly2).draw_edges(&mut framebuffer, 0xFFFFFF);
    Polygon::new(poly3).draw_edges(&mut framebuffer, 0xFFFFFF);
    Polygon::new(poly4).draw_edges(&mut framebuffer, 0xFFFFFF);
    Polygon::new(poly5).draw_edges(&mut framebuffer, 0xFFFFFF); // solo el borde, sin relleno

    let _ = framebuffer.render_buffer("output.bmp");
    if let Err(e) = framebuffer.render_png("out.png") {
        eprintln!("Error al generar out.png: {e}");
    }

    println!("Framebuffer renderizado en output.bmp y out.png");
}
