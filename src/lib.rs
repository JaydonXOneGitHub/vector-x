pub use crate::vector2::*;
pub use crate::vector3::*;

pub mod vector2;
pub mod vector3;

#[test]
pub fn try_parts() {
    let v1: Vector2<i32> = (9, 8).into();
}