extern crate wasm_bindgen;

use std::f64::consts::PI;
use std::fmt;
use std::cmp;
use std::ops::{Add, Mul};

use wasm_bindgen::prelude::*;

type Coord = i16;

const CELL_SIZE: Coord = 20;

mod list;
use list::List;
mod step;
use step::Step;

struct StepStore {
    min_distance: usize,
    steps: Vec<Vec<Step>>,
}

impl StepStore {
    fn new() -> StepStore {
        StepStore {
            min_distance: usize::max_value(),
            steps: Vec::new(),
        }
    }

    fn apply_final_distance(&mut self, distance: usize) {
        self.min_distance = cmp::min(self.min_distance, distance);
        if self.min_distance < self.steps.len() {
            self.steps.drain(distance..);
        }
    }

    fn push(&mut self, element: Step) {
        let insert_len = element.len();
        if insert_len >= self.min_distance {
            // no need to insert, this is too long anyway
            return;
        }
        while self.steps.get(insert_len).is_none() {
            self.steps.push(Vec::new());
        }
        self.steps[insert_len].push(element);
    }

    fn pop(&mut self) -> Option<Step> {
        self.steps.iter_mut().find(|e| !e.is_empty()).and_then(|e| e.pop())
    }
}

pub fn compute_final_paths(first_root: Point, boundaries: &[(Coord, Coord)]) -> Vec<Vec<Point>> {
    let mut points = StepStore::new();
    points.push(Step::from_point(first_root));

    let mut final_paths = Vec::new();
    while let Some(element) = points.pop() {
        let min = -40;
        let max = -min;

        if let Some(pv) = element.last_vector() {
            for dx in -1..=1 {
                'dy: for dy in -1..=1 {
                    let v = Vector2 { x: pv.x + dx, y: pv.y + dy };
                    // zero vectors can't help us
                    if v.x == 0 && v.y == 0 { continue; }

                    handle_vector(
                        &mut points,
                        &mut final_paths,
                        &element,
                        v,
                        &boundaries,
                    );
                }
            }
        } else {
            for dx in min..=max {
                'p: for dy in min..=max {
                    let v = Vector2 {
                        x: dx,
                        y: dy,
                    };
                    handle_vector(
                        &mut points,
                        &mut final_paths,
                        &element,
                        v,
                        &boundaries,
                    );
                }
            }
        }
    }

    final_paths
}

fn handle_vector(
    points: &mut StepStore,
    final_paths: &mut Vec<Vec<Point>>,
    element: &Step,
    v: Vector2,
    boundaries: &[(Coord, Coord)],
) {
    match is_vector_valid(boundaries, element.position(), v) {
        Ok(()) => {
            points.push(element.with_vector(v));
        }
        Err(segment) => {
            let end = Segment {
                from: Point {
                    x: 31,
                    y: 30,
                },
                to: Point {
                    x: 24,
                    y: 30,
                },
            };
            if segment == end {
                points.apply_final_distance(element.len());
                final_paths.push(element.with_vector(v).into_points());
            }
        }
    }
}

fn is_vector_valid(boundaries: &[(Coord, Coord)], root: Point, v: Vector2) -> Result<(), Segment> {
    let mut last_boundary = boundaries.last().cloned().unwrap();
    let anchored = v.anchor_at(root);
    for &boundary in boundaries {
        let segment = Segment {
            from: Point {
                x: last_boundary.0,
                y: last_boundary.1,
            },
            to: Point {
                x: boundary.0,
                y: boundary.1,
            },
        };
        if anchored.intersects(segment) {
            return Err(segment);
        }
        last_boundary = boundary;
    }
    Ok(())
}

#[derive(PartialEq, Copy, Clone)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Vector2> for Point {
    type Output = Point;

    fn add(self, other: Vector2) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<Coord> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: Coord) -> Vector2 {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Rectangle {
    base_a: Point,
    base_b: Point,
    height: f64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Segment {
    from: Point,
    to: Point,
}

impl Segment {
    fn intersects(self, other: Self) -> bool {
        // If our start and end lie to the left of the start and end x on the other segment, there
        // is no way that the two intersect
        if self.from.x < other.from.x && self.from.x < other.to.x &&
            self.to.x < other.from.x && self.to.x < other.to.x {
            return false;
        }

        // Same as before, except with y.
        if self.from.y < other.from.y && self.from.y < other.to.y &&
            self.to.y < other.from.y && self.to.y < other.to.y {
            return false;
        }

        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
        let x1 = self.from.x;
        let y1 = self.from.y;
        let x2 = self.to.x;
        let y2 = self.to.y;
        let x3 = other.from.x;
        let y3 = other.from.y;
        let x4 = other.to.x;
        let y4 = other.to.y;
        let t1 = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
        let t2 = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        let u1 = (x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3);
        let u2 = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        let t = t1 as f64 / t2 as f64;
        let u = -1.0 * u1 as f64 / u2 as f64;
        0.0 <= t && t <= 1.0 && 0.0 <= u && u <= 1.0
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Vector2 {
    pub x: Coord,
    pub y: Coord,
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

impl Vector2 {
    pub fn anchor_at(self, p: Point) -> Segment {
        Segment {
            from: p,
            to: p + self,
        }
    }

    fn draw(&self, anchor: Point, ctx: &mut CanvasRenderingContext2D, color: &str) {
        let seg = self.anchor_at(anchor);
        ctx.begin_path();
        ctx.move_to(seg.from.x, seg.from.y);
        ctx.line_to(seg.to.x, seg.to.y);
        ctx.arc(seg.to.x as usize, seg.to.y as usize, 3.0, 0.0, 2.0 * PI, false);
        ctx.set_stroke_style(color);
        ctx.set_line_width(1.0);
        ctx.stroke();
    }
}

#[wasm_bindgen]
pub struct Universe {
    pub width: usize,
    pub height: usize,
    ctx: CanvasRenderingContext2D,
    boundaries: Vec<(Coord, Coord)>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(ctx: CanvasRenderingContext2D) -> Self {
        Universe {
            height: 60,
            width: 60,
            ctx,
            boundaries: vec![
                (20, 29),
                (16, 23),
                (16, 19),
                (13, 17),
                (10, 19),
                (10, 31),
                (5, 31), // start
                (5, 20),
                (6, 17),
                (8, 14),
                (12, 12),
                (16, 13),
                (19, 15),
                (21, 22),
                (24, 25),
                (29, 26),
                (31, 30),
                (24, 30), // finish (31, 30) -> (24, 30)
            ],
        }
    }

    pub fn clicked(&mut self, _row: usize, _col: usize, _x: usize, _y: usize) -> bool {
        false
    }

    pub fn draw_grid(&mut self) {
        self.ctx.begin_path();
        self.ctx
        // todo: 1.0 / window.devicePixelRatio
            .set_line_width(0.5);
        self.ctx.set_stroke_style("#CCCCCC");

        // vertical lines
        for i in 0..=self.width as Coord {
            self.ctx.move_to(i * CELL_SIZE, 0);
            self.ctx.line_to(i * CELL_SIZE, CELL_SIZE * self.height as Coord);
        }

        // horizontal lines
        for i in 0..=self.height as Coord {
            self.ctx.move_to(0, i * CELL_SIZE);
            self.ctx.line_to(CELL_SIZE * self.width as Coord, i * CELL_SIZE);
        }

        self.ctx.stroke();

        // segmented boundaries
        self.ctx.begin_path();
        self.ctx.set_line_width(2.0);
        self.ctx.set_stroke_style("#c00000");

        self.ctx.move_to(24 * CELL_SIZE, 30 * CELL_SIZE);

        for boundary in &self.boundaries {
            self.ctx.line_to(boundary.0 * CELL_SIZE, boundary.1 * CELL_SIZE);
        }

        self.ctx.stroke();
    }

    fn draw_for_root(&mut self, first_root: Point) {
        let points = compute_final_paths(first_root, &self.boundaries);

        for path in &points {
            let mut prev_pos = first_root;
            for &cur_pos in path {
                let step = Vector2 { x: cur_pos.x - prev_pos.x, y: cur_pos.y - prev_pos.y };
                (step * CELL_SIZE).draw(
                    Point { x: prev_pos.x * CELL_SIZE, y: prev_pos.y * CELL_SIZE },
                    &mut self.ctx,
                    "red",
                );
                prev_pos = cur_pos;
            }
        }
    }

    pub fn draw_points(&mut self) {
        for x in 6..=9 {
            let root = Point { x, y: 30 };
            self.draw_for_root(root);
        }
    }
}

#[wasm_bindgen]
extern "C" {
    pub type CanvasRenderingContext2D;
    #[wasm_bindgen(method, js_name = beginPath)]
    fn begin_path(this: &CanvasRenderingContext2D);
    #[wasm_bindgen(method, setter = lineWidth)]
    fn set_line_width(this: &CanvasRenderingContext2D, width: f64);
    #[wasm_bindgen(method, setter = strokeStyle)]
    fn set_stroke_style(this: &CanvasRenderingContext2D, style: &str);
    #[wasm_bindgen(method, js_name = moveTo)]
    fn move_to(this: &CanvasRenderingContext2D, x: Coord, y: Coord);
    #[wasm_bindgen(method, js_name = lineTo)]
    fn line_to(this: &CanvasRenderingContext2D, x: Coord, y: Coord);
    #[wasm_bindgen(method)]
    fn arc(
        this: &CanvasRenderingContext2D,
        x: usize,
        y: usize,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    );
    #[wasm_bindgen(method)]
    fn stroke(this: &CanvasRenderingContext2D);
}
