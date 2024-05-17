use bytemuck::{Pod, Zeroable};
use glam::{Mat3, Vec2};

#[derive(Pod, Zeroable, Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct TransformData {
    pub origin: Vec2,
    pub scale: f32,
    pub angle: f32,
}

impl TransformData {
    pub const ZERO: Self = TransformData {
        origin: Vec2::ZERO,
        scale: 0.0,
        angle: 0.0,
    };

    pub const NAN: Self = TransformData {
        origin: Vec2::NAN,
        scale: f32::NAN,
        angle: f32::NAN,
    };

    pub fn with_scale(self, scale: f32) -> Self {
        TransformData {
            origin: self.origin,
            scale,
            angle: self.angle,
        }
    }
}

// Rotation deformers seem pretty simple. I think they're
// a subset of affine transformations, including rotation,
// translation, scale, and reflection. We can just offload
// all the hard work to glam.

pub fn apply_rotation_deformer(
    data: &TransformData,
    base_angle: f32,
    points_to_transform: &mut [Vec2],
) {
    let transform_matrix = Mat3::from_scale_angle_translation(
        Vec2::splat(data.scale),
        (base_angle + data.angle).to_radians(),
        data.origin,
    );

    for i in points_to_transform {
        *i = transform_matrix.transform_point2(*i);
    }
}

// Figures out how movement of a parent deformer changes the angle of a child deformer.
pub fn calculate_rotation_deformer_angle<F>(origin: Vec2, fudge_factor: f32, transform: F) -> f32
where
    F: Fn(Vec2) -> Vec2,
{
    let direction = Vec2::NEG_Y * fudge_factor;
    let transformed_origin = transform(origin);
    let transformed_direction = transform(origin + direction);
    let ret = transformed_direction - transformed_origin;

    if ret.is_finite() && ret != Vec2::ZERO {
        direction.angle_between(ret).to_degrees()
    } else {
        0.0
    }
}
