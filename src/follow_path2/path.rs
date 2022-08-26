use crate::follow_path2::path::PathSegment::{CubicBezierCurve, Point, QuadraticBezierCurve};
use bevy::prelude::*;
use euclid::Point2D;

#[cfg(feature = "debug_draw")]
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{PathBuilder as LyonPathBuilder, *},
};
use lyon_geom::{CubicBezierSegment, QuadraticBezierSegment};

pub(crate) enum PathSegment {
    Point(Vec2),
    CubicBezierCurve { to: Vec2, ctrl1: Vec2, ctrl2: Vec2 },
    QuadraticBezierCurve { to: Vec2, ctrl: Vec2 },
}

#[derive(Default)]
pub struct Path2 {
    pub points: Vec<Vec2>,
    pub is_loop: bool,
}

pub struct PathBuilder {
    segments: Vec<PathSegment>,
}

impl PathBuilder {
    pub fn new(start: Vec2) -> Self {
        Self {
            segments: vec![Point(start)],
        }
    }

    pub fn add_line_to(&mut self, point: Vec2) {
        self.segments.push(Point(point));
    }

    pub fn add_cubic_bezier_curve(&mut self, to: Vec2, ctrl1: Vec2, ctrl2: Vec2) {
        self.segments.push(CubicBezierCurve { to, ctrl1, ctrl2 });
    }

    pub fn add_quadratic_bezier_curve(&mut self, to: Vec2, ctrl: Vec2) {
        self.segments.push(QuadraticBezierCurve { to, ctrl });
    }

    pub fn build_points(&self) -> Vec<Vec2> {
        let mut path_points = Vec::new();

        if let Some(segment) = self.segments.first() {
            let mut last_pos = match segment {
                Point(point) => point,
                CubicBezierCurve { .. } | QuadraticBezierCurve { .. } => {
                    panic!("Path has to start with a Point")
                }
            };

            for segment in self.segments.iter().skip(1) {
                match segment {
                    Point(point) => {
                        last_pos = point;
                        path_points.push(*point);
                    }
                    CubicBezierCurve { to, ctrl1, ctrl2 } => {
                        let points: &mut Vec<Vec2> = &mut CubicBezierSegment::<f32> {
                            from: Point2D::new(last_pos.x, last_pos.y),
                            to: Point2D::new(to.x, to.y),
                            ctrl1: Point2D::new(ctrl1.x, ctrl1.y),
                            ctrl2: Point2D::new(ctrl2.x, ctrl2.y),
                        }
                        .flattened(1.)
                        .map(|p| Vec2::new(p.x, p.y))
                        .collect();
                        path_points.append(points);
                        last_pos = to;
                    }
                    QuadraticBezierCurve { to, ctrl } => {
                        let points: &mut Vec<Vec2> = &mut QuadraticBezierSegment::<f32> {
                            from: Point2D::new(last_pos.x, last_pos.y),
                            to: Point2D::new(to.x, to.y),
                            ctrl: Point2D::new(ctrl.x, ctrl.y),
                        }
                        .flattened(1.)
                        .map(|p| Vec2::new(p.x, p.y))
                        .collect();
                        path_points.append(points);
                        last_pos = to;
                    }
                }
            }
        }
        path_points
    }

    pub fn build_path(&self) -> Path2 {
        let points = self.build_points();
        Path2 {points, is_loop: false}
    }

    pub fn build_looping_path(&self) -> Path2 {
        let points = self.build_points();
        Path2 {points, is_loop: true}
    }

    pub fn build_debug_draw_bundle(&self) -> ShapeBundle {
        let mut path_builder = LyonPathBuilder::new();

        if let Some(segment) = self.segments.first() {
            if let Point(last_pos) = segment {
                path_builder.move_to(*last_pos);

                for segment in self.segments.iter().skip(1) {
                    match segment {
                        Point(point) => {
                            path_builder.line_to(*point);
                        }
                        CubicBezierCurve { to, ctrl1, ctrl2 } => {
                            path_builder.cubic_bezier_to(*ctrl1, *ctrl2, *to);
                        }
                        QuadraticBezierCurve { to, ctrl } => {
                            path_builder.quadratic_bezier_to(*ctrl, *to);
                        }
                    }
                }

                let line = path_builder.build();
                let bundle = GeometryBuilder::build_as(
                    &line,
                    DrawMode::Stroke(StrokeMode::new(Color::BLACK, 1.0)),
                    Transform::default(),
                );
                bundle
            } else {
                panic!("Path has to start with a Point") // Panic, because currently this code should not be able to be reached.
            }
        } else {
            panic!("Path cannot be empty") // Panic, because currently this code should not be able to be reached.
        }
    }
}
