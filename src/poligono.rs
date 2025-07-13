use crate::framebuffer::Framebuffer;
use crate::line::line;
use raylib::prelude::*;

fn draw_polygon_edges(fb: &mut Framebuffer, vertices: &[(i32, i32)]) {
    for i in 0..vertices.len() {
        let (x0, y0) = vertices[i];
        let (x1, y1) = vertices[(i + 1) % vertices.len()];
        line(
            fb,
            Vector2::new(x0 as f32, y0 as f32),
            Vector2::new(x1 as f32, y1 as f32),
        );
    }
}

fn scanline_fill(fb: &mut Framebuffer, polygon: &[(i32, i32)], holes: &[Vec<(i32, i32)>]) {
    let edges: Vec<(i32, i32)> = polygon.to_vec();

    let min_y = polygon.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = polygon.iter().map(|&(_, y)| y).max().unwrap();

    for y in min_y..=max_y {
        let mut intersections = vec![];

        for i in 0..edges.len() {
            let (x0, y0) = edges[i];
            let (x1, y1) = edges[(i + 1) % edges.len()];

            if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                let x = x0 + (y - y0) * (x1 - x0) / (y1 - y0);
                intersections.push(x);
            }
        }

        intersections.sort();

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i];
                let x_end = intersections[i + 1];
                for x in x_start..=x_end {
                    fb.set_pixel(x as u32, y as u32);
                }
            }
        }
    }

    for hole in holes {
    fb.set_current_color(Color::RAYWHITE);

    let edges: Vec<(i32, i32)> = hole.to_vec();
    let min_y = hole.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = hole.iter().map(|&(_, y)| y).max().unwrap();

    for y in min_y..=max_y {
        let mut intersections = vec![];
        for i in 0..edges.len() {
            let (x0, y0) = edges[i];
            let (x1, y1) = edges[(i + 1) % edges.len()];

            if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                let x = x0 + (y - y0) * (x1 - x0) / (y1 - y0);
                intersections.push(x);
            }
        }
        intersections.sort();
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i];
                let x_end = intersections[i + 1];
                for x in x_start..=x_end {
                    fb.set_pixel(x as u32, y as u32);
                }
            }
        }
    }
    fb.set_current_color(Color::BLACK);
    }

}

pub fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, Color::RAYWHITE);

    framebuffer.set_current_color(Color::BLACK);

    let poly1 = vec![(165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360),
                     (250, 380), (220, 385), (205, 410), (193, 383)];

    let poly2 = vec![(321, 335), (288, 286), (339, 251), (374, 302)];
    let poly3 = vec![(377, 249), (411, 197), (436, 249)];
    let poly4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180)
    ];

    let poly5 = vec![(682, 175), (708, 120), (735, 148), (739, 170)];

    scanline_fill(&mut framebuffer, &poly1, &[]);
    draw_polygon_edges(&mut framebuffer, &poly1);

    scanline_fill(&mut framebuffer, &poly2, &[]);
    draw_polygon_edges(&mut framebuffer, &poly2);

    scanline_fill(&mut framebuffer, &poly3, &[]);
    draw_polygon_edges(&mut framebuffer, &poly3);

    scanline_fill(&mut framebuffer, &poly4, &[poly5.clone()]);
    draw_polygon_edges(&mut framebuffer, &poly4);
    draw_polygon_edges(&mut framebuffer, &poly5);

    framebuffer.render_to_file("out.png");
}
