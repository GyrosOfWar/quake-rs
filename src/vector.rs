use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x: x, y: y }
    }

    pub fn dot(self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, t: f32) -> Vec2 {
        Vec2 { x: self.x * t, y: self.y * t }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3 { x: self.x * t, y: self.y * t, z: self.z * t }
    }
}

#[cfg(test)]
mod test {
    use vector::{Vec2, Vec3};

    #[test]
    fn test_vec2_add() {
        let v1 = Vec2::new(5.0, 2.0);
        let v2 = Vec2::new(5.0, 3.0);
        let result = Vec2::new(10.0, 5.0);
        assert_eq!(v1 + v2, result);
    }

    #[test]
    fn test_vec2_mul() {
        let v1 = Vec2::new(5.0, 2.0);
        let t = 2.0;
        let result = Vec2::new(10.0, 4.0);
        assert_eq!(v1 * t, result);
    }

    #[test]
    fn test_vec2_dot() {
        let v1 = Vec2::new(5.0, 2.0);
        let result = 29.0;
        assert_eq!(v1.dot(v1), result);
    }

    #[test]
    fn test_vec3_add() {
        let v1 = Vec3::new(5.0, 3.0, 2.0);
        let v2 = Vec3::new(5.0, 2.0, 3.0);
        let result = Vec3::new(10.0, 5.0, 5.0);
        assert_eq!(v1 + v2, result);
    }

    #[test]
    fn test_vec3_mul() {
        let v1 = Vec3::new(5.0, 3.0, 2.0);
        let t = 2.0;
        let result = Vec3::new(10.0, 6.0, 4.0);
        assert_eq!(v1 * t, result);
    }

    #[test]
    fn test_vec3_dot() {
        let v1 = Vec3::new(5.0, 2.0, 1.0);
        let result = 30.0;
        assert_eq!(v1.dot(v1), result);
    }
}
