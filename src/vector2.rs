use std::fmt::Display;

use crate::Vector3;

/// A struct that comprises of two elements
pub struct Vector2<T> {
    pub one: T,
    pub two: T
}

impl<T> Vector2<T> {
    pub fn new(one: T, two: T) -> Self {
        return Self {
            one: one,
            two: two
        };
    }
}

impl<T: Default> Default for Vector2<T> {
    fn default() -> Self {
        return Self {
            one: T::default(),
            two: T::default()
        };
    }
}

impl<T: Clone> Clone for Vector2<T> {
    fn clone(&self) -> Self {
        return Self {
            one: self.one.clone(),
            two: self.two.clone()
        };
    }
}

impl<T: Copy> Copy for Vector2<T> {}

impl<T> Into<(T, T)> for Vector2<T> {
    fn into(self) -> (T, T) {
        return (self.one, self.two);
    }
}

impl<T> Into<Vector2<T>> for (T, T) {
    fn into(self) -> Vector2<T> {
        return Vector2::new(self.0, self.1);
    }
}

impl<T: PartialEq> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.one == other.one && self.two == other.two;
    }
    
    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl<T: Default> Into<Vector3<T>> for Vector2<T> {
    fn into(self) -> Vector3<T> {
        return Vector3::new(self.one, self.two, T::default());
    }
}

impl<T: Display> Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "[one: {}, two: {}", self.one, self.two);
    }
}