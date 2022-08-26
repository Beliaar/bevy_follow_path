use bevy::prelude::*;

use crate::follow_path2::path::PathSegment::{CubicBezierCurve, Point, QuadraticBezierCurve};
use crate::follow_path2::vec2_geo_nd::Vec2Geo;

#[cfg(feature = "debug_draw")]
use bevy::{render::mesh::PrimitiveTopology, sprite::MaterialMesh2dBundle};

type Bezier2 = bezier_nd::Bezier<f32, Vec2Geo, 2>;

/// Possible segments to build a 2D path from
///
/// The last point of the segment will always connect to the first point of the next.
pub(crate) enum PathSegment {
    Point(Vec2),
    /// Points of a cubic bezier curve, with 2 control points.
    CubicBezierCurve {
        to: Vec2,
        ctrl1: Vec2,
        ctrl2: Vec2,
        straightness: f32,
    },
    /// Points of a quadratic bezier curve, with a single control point
    QuadraticBezierCurve {
        to: Vec2,
        ctrl: Vec2,
        straightness: f32,
    },
}

/// Contains the data for the path to follow
#[derive(Default, Clone)]
pub struct Path2 {
    /// The list of [Points](bevy::math::f32::Vec2) to follow
    pub points: Vec<Vec2>,
    /// Whether the path circles back to the first point, or not.
    pub is_loop: bool,
}

impl Path2 {
    /// Spawn a [bundle](MaterialMesh2dBundle) for drawing the path
    ///
    /// Returns the [Entity] for the mesh
    #[cfg(feature = "debug_draw")]
    pub fn spawn_mesh(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
    ) -> Entity {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let mut points: Vec<[f32; 3]> = self.points.iter().map(|p| [p.x, p.y, 0.]).collect();
        if self.is_loop {
            points.push(points[0]);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points.clone());
        let mut v_normal = Vec::new();
        let mut v_uv = Vec::new();

        for _ in 0..points.len() {
            v_normal.push([0.0, 1.0, 0.0]);
            v_uv.push([1.0, 1.0]);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, v_normal);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, v_uv);

        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                transform: Transform::default(),
                material: materials.add(ColorMaterial::from(color)),
                ..default()
            })
            .id()
    }
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
    /// using 2 control points and the given straightness
    pub fn add_cubic_bezier_curve(
        &mut self,
        to: Vec2,
        ctrl1: Vec2,
        ctrl2: Vec2,
        straightness: f32,
    ) {
        self.segments.push(CubicBezierCurve {
            to,
            ctrl1,
            ctrl2,
            straightness,
        });
    }

    /// Add a bezier curve from the previous to the specified end [point](bevy::math::f32::Vec2)
    /// using a single control point and the given straightness
    pub fn add_quadratic_bezier_curve(&mut self, to: Vec2, ctrl: Vec2, straightness: f32) {
        self.segments.push(QuadraticBezierCurve {
            to,
            ctrl,
            straightness,
        });
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
                    CubicBezierCurve {
                        to,
                        ctrl1,
                        ctrl2,
                        straightness,
                    } => {
                        let curve = Bezier2::cubic(
                            &Vec2Geo(*last_pos),
                            &Vec2Geo(*ctrl1),
                            &Vec2Geo(*ctrl2),
                            &Vec2Geo(*to),
                        );
                        let points: &mut Vec<Vec2> =
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
                            Bezier2::quadratic(&Vec2Geo(*last_pos), &Vec2Geo(*ctrl), &Vec2Geo(*to));
                        let points: &mut Vec<Vec2> =
                            &mut curve.as_points(*straightness).map(|p| p.0).collect();
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
        Path2 {
            points,
            is_loop: false,
        }
    }

    /// Build a looping [Path](Path2) from the current segments
    pub fn build_looping_path(&self) -> Path2 {
        let points = self.build_points();
        Path2 {
            points,
            is_loop: true,
        }
    }
}
