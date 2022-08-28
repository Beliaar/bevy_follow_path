use bevy::prelude::*;

use crate::follow_path3::path::PathSegment::{CubicBezierCurve, Point, QuadraticBezierCurve};
use crate::follow_path3::vec3_geo_nd::Vec3Geo;

#[cfg(feature = "debug_draw")]
use bevy::render::mesh::PrimitiveTopology;

type Bezier3 = bezier_nd::Bezier<f32, Vec3Geo, 3>;

/// Possible segments to build a 3D path from
///
/// The last point of the segment will always connect to the first point of the next.
pub(crate) enum PathSegment {
    Point(Vec3),
    /// Points of a cubic bezier curve, with 2 control points.
    CubicBezierCurve {
        to: Vec3,
        ctrl1: Vec3,
        ctrl2: Vec3,
        straightness: f32,
    },
    /// Points of a quadratic bezier curve, with a single control point
    QuadraticBezierCurve {
        to: Vec3,
        ctrl: Vec3,
        straightness: f32,
    },
}

/// Contains the data for the path to follow
#[derive(Default, Clone)]
pub struct Path3 {
    /// The list of [Points](bevy::math::f32::Vec3) to follow
    pub points: Vec<Vec3>,
    /// Whether the path circles back to the first point, or not.
    pub is_loop: bool,
}

impl Path3 {
    /// Spawn a [bundle](PbrBundle) for drawing the path
    ///
    /// Returns the Entity for the mesh
    #[cfg(feature = "debug_draw")]
    pub fn spawn_mesh(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        color: Color,
    ) -> Entity {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let mut points: Vec<[f32; 3]> = self.points.iter().map(|p| [p.x, p.y, p.z]).collect();
        if self.is_loop {
            points.push(points[0]);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points.clone());
        let mut v_normal = Vec::new();
        let mut v_uv = Vec::new();
        let mut v_color = Vec::new();

        for _ in 0..points.len() {
            v_normal.push(Vec3::ZERO.to_array());
            v_uv.push([1.0, 1.0]);
            v_color.push(color.as_rgba_f32())
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, v_normal);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, v_uv);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_color);

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(mesh),
                transform: Transform::default(),
                ..default()
            })
            .id()
    }
}

/// Builder to simplify making [paths](Path3) using segments that are connected to each other
pub struct PathBuilder {
    segments: Vec<PathSegment>,
}

impl PathBuilder {
    /// Create the builder with the first point set to *start*
    pub fn new(start: Vec3) -> Self {
        Self {
            segments: vec![Point(start)],
        }
    }

    /// Add a line from the previous point to the passed [point](bevy::math::f32::Vec3)
    pub fn add_line_to(&mut self, point: Vec3) {
        self.segments.push(Point(point));
    }

    /// Add a bezier curve from the previous to the specified end [point](bevy::math::f32::Vec3)
    /// using 2 control points and the given straightness
    pub fn add_cubic_bezier_curve(
        &mut self,
        to: Vec3,
        ctrl1: Vec3,
        ctrl2: Vec3,
        straightness: f32,
    ) {
        self.segments.push(CubicBezierCurve {
            to,
            ctrl1,
            ctrl2,
            straightness,
        });
    }

    /// Add a bezier curve from the previous to the specified end [point](bevy::math::f32::Vec3)
    /// using a single control point and the given straightness
    pub fn add_quadratic_bezier_curve(&mut self, to: Vec3, ctrl: Vec3, straightness: f32) {
        self.segments.push(QuadraticBezierCurve {
            to,
            ctrl,
            straightness,
        });
    }

    /// Build a list of [points](bevy::math::f32::Vec3) from the current segments
    pub fn build_points(&self) -> Vec<Vec3> {
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
                    CubicBezierCurve {
                        to,
                        ctrl1,
                        ctrl2,
                        straightness,
                    } => {
                        let curve = Bezier3::cubic(
                            &Vec3Geo(*last_pos),
                            &Vec3Geo(*ctrl1),
                            &Vec3Geo(*ctrl2),
                            &Vec3Geo(*to),
                        );
                        let points: &mut Vec<Vec3> =
                            &mut curve.as_points(*straightness).map(|p| p.0).collect();
                        path_points.append(points);
                        last_pos = to;
                    }
                    QuadraticBezierCurve {
                        to,
                        ctrl,
                        straightness,
                    } => {
                        let curve =
                            Bezier3::quadratic(&Vec3Geo(*last_pos), &Vec3Geo(*ctrl), &Vec3Geo(*to));
                        let points: &mut Vec<Vec3> =
                            &mut curve.as_points(*straightness).map(|p| p.0).collect();
                        path_points.append(points);
                        last_pos = to;
                    }
                }
            }
        }
        path_points
    }

    /// Build a non looping [Path](Path3) from the current segments
    pub fn build_path(&self) -> Path3 {
        let points = self.build_points();
        Path3 {
            points,
            is_loop: false,
        }
    }

    /// Build a looping [Path](Path3) from the current segments
    pub fn build_looping_path(&self) -> Path3 {
        let points = self.build_points();
        Path3 {
            points,
            is_loop: true,
        }
    }
}
