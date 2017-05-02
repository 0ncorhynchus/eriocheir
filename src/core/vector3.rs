#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vec = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, vec.x);
        assert_eq!(2.0, vec.y);
        assert_eq!(3.0, vec.z);
    }

    #[test]
    fn test_zero() {
        let vec = Vector3::zero();
        assert_eq!(0.0, vec.x);
        assert_eq!(0.0, vec.y);
        assert_eq!(0.0, vec.z);
    }
}
