use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T> Vec3<T>
where
    T: Copy + Into<f64> + Add<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    pub fn length_squared(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).into()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Vec3<f64> {
        let length = self.length();
        Vec3 {
            x: self.x.into() / length,
            y: self.y.into() / length,
            z: self.z.into() / length,
        }
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, a: T) -> Self::Output {
        Vec3 {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

pub fn dot<T>(a: &Vec3<T>, b: &Vec3<T>) -> T
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross<T>(a: &Vec3<T>, b: &Vec3<T>) -> Vec3<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    let x = a.y * b.z - b.y * a.z;
    let y = b.x * a.z - a.x * b.z;
    let z = a.x * b.y - b.x * a.y;
    Vec3 { x, y, z }
}

pub type Vec3f = Vec3<f64>;

#[test]
fn test_length() {
    assert_eq!(Vec3::new(1, 0, 0).length_squared(), 1.0);
    assert_eq!(Vec3::new(1, 0, 0).length(), 1.0);
}

#[test]
fn test_normalized() {
    let v = Vec3::new(1, 1, 1);
    let component = 1.0 / 3.0f64.sqrt();
    assert_eq!(v.normalized(), Vec3::new(component, component, component));
}

#[test]
fn test_add() {
    let a = Vec3::new(1, 2, 3);
    let b = Vec3::new(3, 2, 1);
    assert_eq!(a + b, Vec3::new(4, 4, 4));
}

#[test]
fn test_sub() {
    let a = Vec3::new(1, 2, 3);
    let b = Vec3::new(3, 2, 1);
    assert_eq!(a - b, Vec3::new(-2, 0, 2));
}

#[test]
fn test_dot() {
    let a = Vec3::new(1, 2, 3);
    let b = Vec3::new(3, 0, 1);
    assert_eq!(dot(&a, &b), 6);
}

#[test]
fn test_cross() {
    let a = Vec3::new(1, 2, 3);
    let b = Vec3::new(3, 2, 1);
    assert_eq!(cross(&a, &b), Vec3::new(-4, 8, -4));
    assert_eq!(cross(&b, &a), Vec3::new(4, -8, 4));
}
