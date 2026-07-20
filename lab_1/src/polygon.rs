use crate::framebuffer::Framebuffer;
use crate::line::Line;

//Representa un poligono como una lista de vertices (x, y) en el orden  en que deben conectarse. El poligono se cierra automaticamente
pub struct Polygon {
    pub vertices: Vec<(i32, i32)>,
}

impl Polygon {
    pub fn new(vertices: Vec<(i32, i32)>) -> Self {
        Polygon { vertices }
    }

    // Dibuja las aristas del poligono usando el algoritmo de Bresenham
    pub fn draw_edges(&self, fb: &mut Framebuffer, color: u32) {
        fb.set_current_color(color);
        let n = self.vertices.len();
        if n < 2 {
            return;
        }
        for i in 0..n {
            let (x1, y1) = self.vertices[i];
            let (x2, y2) = self.vertices[(i + 1) % n];
            fb.line(x1 as usize, y1 as usize, x2 as usize, y2 as usize);
        }
    }
}

// Convierte una lista de vertices en una lista de aristas uniendo pares de vertices
fn edges_of(vertices: &[(i32, i32)]) -> Vec<((i32, i32), (i32, i32))> {
    let n = vertices.len();
    let mut edges = Vec::with_capacity(n);
    for i in 0..n {
        edges.push((vertices[i], vertices[(i + 1) % n]));
    }
    edges
}

/// Rellena un poligono usando el algoritmo de scanline fill (regla par-impar).




pub fn fill_polygon_with_holes(
    fb: &mut Framebuffer,

    // outer es el poligono principal a rellenar.
    outer: &[(i32, i32)],

    // holes es una lista de poligonos que estan dentro de outer (por ejemplo, el Poligono 5 dentro del Poligono 4). Al aplicar el algoritmo scanline fill tomando en cuenta las aristas de outer y las aristas de los polígonos interiores esos polígonos interiores quedan sin pintar y representan agujeros dentro de outer
    holes: &[&[(i32, i32)]],
    color: u32,
) {
    let mut edges = edges_of(outer);
    for hole in holes {
        edges.extend(edges_of(hole));
    }

    fill_edges(fb, &edges, color);
}

// Rellena un poligono sin agujeros)
pub fn fill_polygon(fb: &mut Framebuffer, vertices: &[(i32, i32)], color: u32) {
    fill_polygon_with_holes(fb, vertices, &[], color);
}

// Algoritmo de scanline fill
fn fill_edges(fb: &mut Framebuffer, edges: &[((i32, i32), (i32, i32))], color: u32) {
    if edges.is_empty() {
        return;
    }


    let min_y = edges
        .iter()
        .flat_map(|&((_, y1), (_, y2))| [y1, y2])
        .min()
        .unwrap();
    let max_y = edges
        .iter()
        .flat_map(|&((_, y1), (_, y2))| [y1, y2])
        .max()
        .unwrap();


    fb.set_current_color(color);

    for y in min_y..=max_y {
        let mut intersections: Vec<f64> = Vec::new();

        for &((x1, y1), (x2, y2)) in edges {
            // Ignoramos aristas horizontales ya no son parte del area de llenado util para el scanline
            if y1 == y2 {
                continue;
            }

            // Ordenamos la arista para que ya vaya de menor a mayor en y.
            let (y_top, y_bottom, x_top, x_bottom) = if y1 < y2 {
                (y1, y2, x1, x2)
            } else {
                (y2, y1, x2, x1)
            };


            //Intervalo semi-abierto [y_top, y_bottom) para que un vertice compartido entre dos aristas no se cuente dos veces.
            if y >= y_top && y < y_bottom {
                let t = (y - y_top) as f64 / (y_bottom - y_top) as f64;
                let x = x_top as f64 + t * (x_bottom - x_top) as f64;
                intersections.push(x);





            }
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());


        // Pinta entre cada par de intersecciones consecutivas.
        let mut i = 0;
        while i + 1 < intersections.len() {
            let x_start = intersections[i].round() as i32;
            let x_end = intersections[i + 1].round() as i32;

            for x in x_start..=x_end {
                if x >= 0 && y >= 0 {
                    fb.point(x as usize, y as usize);
                }
            }

            i += 2;
        }
    }
}
