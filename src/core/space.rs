use core::vector3::Vector3;

pub struct Space {
    t: f64,
    size: Vector3,
    positions: Vec<Vector3>,
    velocities: Vec<Vector3>,
}

impl Space {
    pub fn new(size: Vector3) -> Space {
        Space {
            t: 0.0,
            size: size,
            positions: Vec::new(),
            velocities: Vec::new()
        }
    }

    pub fn push_particle(&mut self,
                        position: Vector3,
                        velocity: Vector3) {
        self.positions.push(position);
        self.velocities.push(velocity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let size = Vector3::new(10.0, 11.0, 12.0);
        let space = Space::new(size);
        assert_eq!(0.0, space.t);
        assert_eq!(size, space.size);
        assert_eq!(0, space.positions.len());
        assert_eq!(0, space.velocities.len());
    }

    #[test]
    fn test_push_particle() {
        let size = Vector3::new(10.0, 11.0, 12.0);
        let mut space = Space::new(size);
        space.push_particle(Vector3::new(5.0, 5.0, 5.0),
                            Vector3::new(1.0, 2.0,-3.0));
        assert_eq!(1, space.positions.len());
        assert_eq!(Vector3::new(5.0, 5.0, 5.0),
                   space.positions[0]);
        assert_eq!(1, space.velocities.len());
        assert_eq!(Vector3::new(1.0, 2.0,-3.0),
                   space.velocities[0]);
    }
}
