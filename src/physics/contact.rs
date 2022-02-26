use bevy::prelude::*;

pub struct Contact {
    pub penetration: f32,
    pub normal: Vec2,
}

pub fn ball_ball(pos_a: Vec2, radius_a: f32, pos_b: Vec2, radius_b: f32) -> Option<Contact> {
    let ab = pos_b - pos_a;
    let combined_radius = radius_a + radius_b;
    let ab_sqr_len = ab.length_squared();
    if ab_sqr_len < combined_radius * combined_radius {
        let ab_length = ab_sqr_len.sqrt();
        let penetration = combined_radius - ab_length;
        let normal = ab / ab_length;
        Some(Contact {
            normal,
            penetration,
        })
    } else {
        None
    }
}

pub fn ball_box(pos_a: Vec2, radius_a: f32, pos_b: Vec2, size_b: Vec2) -> Option<Contact> {
    let box_to_circle = pos_a - pos_b;
    let box_to_circle_abs = box_to_circle.abs();
    let half_extents = size_b / 2.;
    let corner_to_center = box_to_circle_abs - half_extents;
    let r = radius_a;
    if corner_to_center.x > r || corner_to_center.y > r {
        return None;
    }

    let s = box_to_circle.signum();

    let (n, penetration) = if corner_to_center.x > 0. && corner_to_center.y > 0. {
        // Corner case
        let corner_to_center_sqr = corner_to_center.length_squared();
        if corner_to_center_sqr > r * r {
            return None;
        }
        let corner_dist = corner_to_center_sqr.sqrt();
        let penetration = r - corner_dist;
        let n = corner_to_center / corner_dist * -s;
        (n, penetration)
    } else if corner_to_center.x > corner_to_center.y {
        // Closer to vertical edge
        (Vec2::X * -s.x, -corner_to_center.x + r)
    } else {
        (Vec2::Y * -s.y, -corner_to_center.y + r)
    };

    Some(Contact {
        normal: n,
        penetration,
    })
}

pub fn box_box(pos_a: Vec2, size_a: Vec2, pos_b: Vec2, size_b: Vec2) -> Option<Contact> {
    let half_a = size_a / 2.;
    let half_b = size_b / 2.;
    let ab = pos_b - pos_a;
    let overlap = (half_a + half_b) - ab.abs(); // exploit symmetry
    if overlap.x < 0. || overlap.y < 0. {
        None
    } else if overlap.x < overlap.y {
        // closer to vertical edge
        Some(Contact {
            penetration: overlap.x,
            normal: Vec2::X * ab.x.signum(),
        })
    } else {
        // closer to horizontal edge
        Some(Contact {
            penetration: overlap.y,
            normal: Vec2::Y * ab.y.signum(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_box_clear() {
        assert!(box_box(Vec2::ZERO, Vec2::ONE, Vec2::new(1.1, 0.), Vec2::ONE).is_none());
        assert!(box_box(Vec2::ZERO, Vec2::ONE, Vec2::new(-1.1, 0.), Vec2::ONE).is_none());
        assert!(box_box(Vec2::ZERO, Vec2::ONE, Vec2::new(0., 1.1), Vec2::ONE).is_none());
        assert!(box_box(Vec2::ZERO, Vec2::ONE, Vec2::new(0., -1.1), Vec2::ONE).is_none());
    }

    #[test]
    fn box_box_intersection() {
        assert!(box_box(Vec2::ZERO, Vec2::ONE, Vec2::ZERO, Vec2::ONE).is_some());
        assert!(box_box(Vec2::ZERO, Vec2::ONE, Vec2::new(0.9, 0.9), Vec2::ONE).is_some());
        assert!(box_box(Vec2::ZERO, Vec2::ONE, Vec2::new(-0.9, -0.9), Vec2::ONE).is_some());
    }

    #[test]
    fn box_box_contact_horizontal() {
        let Contact {
            normal,
            penetration,
        } = box_box(Vec2::ZERO, Vec2::ONE, Vec2::new(0.9, 0.), Vec2::ONE).unwrap();

        assert!(normal.x > 0.999);
        assert!(normal.y < 0.001);
        assert!((penetration - 0.1).abs() < 0.001);
    }

    #[test]
    fn box_box_contact_vertical() {
        let Contact {
            normal,
            penetration,
        } = box_box(Vec2::ZERO, Vec2::ONE, Vec2::new(0., 0.9), Vec2::ONE).unwrap();

        assert!(normal.y > 0.999);
        assert!(normal.x < 0.001);
        assert!((penetration - 0.1).abs() < 0.001);
    }
}
