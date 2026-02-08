use std::fmt::Display;

use crate::Vector2;

/// A struct that comprises of three elements
pub struct Vector3<T> {
    one: T,
    two: T,
    three: T
}

impl<T> Vector3<T> {
    pub fn new(one: T, two: T, three: T) -> Self {
        return Self {
            one: one,
            two: two,
            three: three
        };
    }
}

impl<T> Vector3<T> {
    pub fn get_one(&self) -> &T {
        return &self.one;
    }

    pub fn get_one_mut(&mut self) -> &mut T {
        return &mut self.one;
    }

    pub fn get_two(&self) -> &T {
        return &self.two;
    }

    pub fn get_two_mut(&mut self) -> &mut T {
        return &mut self.two;
    }

    pub fn get_three(&self) -> &T {
        return &self.three;
    }

    pub fn get_three_mut(&mut self) -> &mut T {
        return &mut self.three;
    }
}

impl<T: Default> Default for Vector3<T> {
    fn default() -> Self {
        return Self {
            one: T::default(),
            two: T::default(),
            three: T::default()
        };
    }
}

impl<T: Clone> Clone for Vector3<T> {
    fn clone(&self) -> Self {
        return Self {
            one: self.one.clone(),
            two: self.two.clone(),
            three: self.three.clone()
        };
    }
}

impl<T: Copy> Copy for Vector3<T> {}

impl<T> Into<(T, T, T)> for Vector3<T> {
    fn into(self) -> (T, T, T) {
        return (self.one, self.two, self.three);
    }
}

impl<T> Into<Vector3<T>> for (T, T, T) {
    fn into(self) -> Vector3<T> {
        return Vector3::new(self.0, self.1, self.2);
    }
}

impl<T: PartialEq> PartialEq for Vector3<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.one == other.one && 
            self.two == other.two &&
            self.three == other.three;
    }
    
    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl<T> Into<(Vector2<T>, T)> for Vector3<T> {
    fn into(self) -> (Vector2<T>, T) {
        return (Vector2::new(self.one, self.two), self.three);
    }
}

impl<T> Into<Vector3<T>> for (Vector2<T>, T) {
    fn into(self) -> Vector3<T> {
        let (v1, v2) = self.0.into();
        return Vector3::new(v1, v2, self.1);
    }
}

impl<T: Display> Display for Vector3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "[one: {}, two: {}, three: {}", self.one, self.two, self.three);
    }
}