use crate::follow_path2::path::PathSegment::{CubicBezierCurve, Point, QuadraticBezierCurve};
use bevy::prelude::*;
use euclid::Point2D;

#[cfg(feature = "debug_draw")]
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{PathBuilder as LyonPathBuilder, *},
};
use lyon_geom::{CubicBezierSegment, QuadraticBezierSegment};

/// Possible segments to build a 2D path from
///
/// The last point of the segment will always connect to the first point of the next.
pub(crate) enum PathSegment {
    Point(Vec2),
    /// Points of a cubic bezier curve, with 2 control points.
    CubicBezierCurve { to: Vec2, ctrl1: Vec2, ctrl2: Vec2 },
    /// Points of a quadratic bezier curve, with a single control point
    QuadraticBezierCurve { to: Vec2, ctrl: Vec2 },
}

/// Contains the data for the path to follow
#[derive(Default)]
pub struct Path2 {
    /// The list of [Points](bevy::math::f32::Vec2) to follow
    pub points: Vec<Vec2>,
    /// Whether the path circles back to the first point, or not.
    pub is_loop: bool,
}

/// Builder to simplify making [paths](Path2) using segments that are connected to each other
pub struct PathBuilder {
    segments: Vec<PathSegment>,
}

impl PathBuilder {
    /// Create the builder with the first point set to *start*
    pub fn new(start: Vec2) -> Self {
        Self {
            segments: vec![Point(start)],
        }
    }

    /// Add a line from the previous point to the passed [point](bevy::math::f32::Vec2)
    pub fn add_line_to(&mut self, point: Vec2) {
        self.segments.push(Point(point));
    }

    /// Add a bezier curve from the previous to the specified end [point](bevy::math::f32::Vec2)
    /// using 2 control points
    pub fn add_cubic_bezier_curve(&mut self, to: Vec2, ctrl1: Vec2, ctrl2: Vec2) {
        self.segments.push(CubicBezierCurve { to, ctrl1, ctrl2 });
    }

    /// Add a bezier curve from the previous to the specified end [point](bevy::math::f32::Vec2)
    /// using a single control point
    pub fn add_quadratic_bezier_curve(&mut self, to: Vec2, ctrl: Vec2) {
        self.segments.push(QuadraticBezierCurve { to, ctrl });
    }

    /// Build a list of [points](bevy::math::f32::Vec2) from the current segments
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

    /// Build a non looping [Path](Path2) from the current segments
    pub fn build_path(&self) -> Path2 {
        let points = self.build_points();
        Path2 {points, is_loop: false}
    }

    /// Build a looping [Path](Path2) from the current segments
    pub fn build_looping_path(&self) -> Path2 {
        let points = self.build_points();
        Path2 {points, is_loop: true}
    }

    /// Build a [bundle](ShapeBundle) for drawing the path using [bevy_prototype_lyon]
    #[cfg(feature = "debug_draw")]
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
