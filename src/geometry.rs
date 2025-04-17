#![allow(unused)]
use std::fmt::{self, Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use glam::{Mat4, Vec2, Vec3, Vec4};

/// Marker traits representing different coordinate spaces.
/// These are used with `Point<S>` for type-level space tracking.
pub mod space {
    /// Marker trait for all coordinate spaces
    pub trait Space: 'static {}

    /// Local (parent-relative) coordinate space
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    pub struct Local;
    impl Space for Local {}

    /// World (canvas/scene) coordinate space
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    pub struct World;
    impl Space for World {}

    /// Screen (window/viewport) coordinate space
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    pub struct Screen;
    impl Space for Screen {}
}

/// Represents a point in 3D space with strong type-checking for coordinate spaces
///
/// The `S` type parameter is a zero-sized type that acts as a marker for which
/// coordinate space this point belongs to (Local, World, or Screen). This prevents
/// accidentally mixing coordinates from different spaces.
#[derive(Clone, Copy, PartialEq)]
pub struct Point<S: space::Space> {
    /// The underlying 3D vector representing this point's coordinates
    pub vec: Vec3,
    /// Phantom data marker to associate this point with a specific coordinate space
    _space: PhantomData<S>,
}

/// Type alias for points in local (parent-relative) coordinate space
///
/// Local points are only meaningful relative to a parent, or another reference point in the same space.
/// To obtain an absolute/world position, combine a `LocalPoint` with the associated transformation matrix.
///
/// Example usage: node positions within a parent group.
///
/// Note: All points include a z coordinate to support future 3D use, though it is currently unused.
pub type LocalPoint = Point<space::Local>;

/// Type alias for points in world (canvas/scene) coordinate space
///
/// World points are anchored to the origin of the overall scene or canvas,
/// and represent absolute positions independent of any local hierarchy.
///
/// Example usage: direct canvas or scene-space elements.
///
/// Note: All points include a z coordinate to support future 3D use, though it is currently unused.
pub type WorldPoint = Point<space::World>;

/// Type alias for points in screen (window/viewport) coordinate space
///
/// Screen points are defined relative to the application window or viewport,
/// and should not be interchanged with LocalPoint or WorldPoint coordinates.
/// Intended for overlay elements such as menus, tooltips, or drag indicators.
///
/// Example usages: context menus, drag indicators, tooltips, etc.
///
/// Note: All points include a z coordinate to support future 3D use, though it is currently unused.
pub type ScreenPoint = Point<space::Screen>;

// Common implementation for all point types
impl<S: space::Space> Point<S> {
    /// Creates a new point with the given coordinates
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            vec: Vec3::new(x, y, z),
            _space: PhantomData,
        }
    }

    /// Creates a new point with the given coordinates (z defaults to 0.0)
    pub fn new_2d(x: f32, y: f32) -> Self {
        Self::new(x, y, 0.0)
    }

    /// Creates a new point from a Vec3
    pub fn from_vec3(vec: Vec3) -> Self {
        Self {
            vec,
            _space: PhantomData,
        }
    }

    /// Creates a new point from a Vec2 (z defaults to 0.0)
    pub fn from_vec2(vec: Vec2) -> Self {
        Self::from_vec3(Vec3::new(vec.x, vec.y, 0.0))
    }

    /// Get the x coordinate
    pub fn x(&self) -> f32 {
        self.vec.x
    }

    /// Get the y coordinate
    pub fn y(&self) -> f32 {
        self.vec.y
    }

    /// Get the z coordinate
    pub fn z(&self) -> f32 {
        self.vec.z
    }

    /// Set the x coordinate
    pub fn set_x(&mut self, x: f32) {
        self.vec.x = x;
    }

    /// Set the y coordinate
    pub fn set_y(&mut self, y: f32) {
        self.vec.y = y;
    }

    /// Set the z coordinate
    pub fn set_z(&mut self, z: f32) {
        self.vec.z = z;
    }

    /// Get a Vec2 (x, y) representation of this point
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.vec.x, self.vec.y)
    }

    /// Get a Vec3 (x, y, z) representation of this point
    pub fn as_vec3(&self) -> Vec3 {
        self.vec
    }

    /// Get a Vec4 (x, y, z, 1.0) representation of this point, suitable for matrix transforms
    pub fn as_vec4(&self) -> Vec4 {
        Vec4::new(self.vec.x, self.vec.y, self.vec.z, 1.0)
    }

    /// Returns a new point with the same coordinates but in a different space
    ///
    /// This is an escape hatch and should be used with caution!
    /// Normally, you should use explicit transform operations to convert between spaces.
    #[inline]
    pub fn into_space<T: space::Space>(self) -> Point<T> {
        Point {
            vec: self.vec,
            _space: PhantomData,
        }
    }

    /// Create a point at the origin (0, 0, 0)
    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Create a point with all components set to zero
    pub fn zero() -> Self {
        Self::origin()
    }

    /// Linearly interpolate between this point and another
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        Self {
            vec: self.vec.lerp(other.vec, t),
            _space: PhantomData,
        }
    }

    /// Calculate the distance between this point and another
    pub fn distance(&self, other: &Self) -> f32 {
        self.vec.distance(other.vec)
    }

    /// Calculate the squared distance between this point and another
    /// (more efficient than distance when only comparing distances)
    pub fn distance_squared(&self, other: &Self) -> f32 {
        self.vec.distance_squared(other.vec)
    }

    /// Returns a point with all components rounded to the nearest integer
    pub fn round(&self) -> Self {
        Self {
            vec: Vec3::new(self.vec.x.round(), self.vec.y.round(), self.vec.z.round()),
            _space: PhantomData,
        }
    }

    /// Returns a point with all components rounded toward zero
    pub fn trunc(&self) -> Self {
        Self {
            vec: Vec3::new(self.vec.x.trunc(), self.vec.y.trunc(), self.vec.z.trunc()),
            _space: PhantomData,
        }
    }

    /// Returns a point with all components rounded down
    pub fn floor(&self) -> Self {
        Self {
            vec: Vec3::new(self.vec.x.floor(), self.vec.y.floor(), self.vec.z.floor()),
            _space: PhantomData,
        }
    }

    /// Returns a point with all components rounded up
    pub fn ceil(&self) -> Self {
        Self {
            vec: Vec3::new(self.vec.x.ceil(), self.vec.y.ceil(), self.vec.z.ceil()),
            _space: PhantomData,
        }
    }
}

// Debug implementation for nicer output
impl<S: space::Space> Debug for Point<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let space_name = if std::any::TypeId::of::<S>() == std::any::TypeId::of::<space::Local>() {
            "Local"
        } else if std::any::TypeId::of::<S>() == std::any::TypeId::of::<space::World>() {
            "World"
        } else if std::any::TypeId::of::<S>() == std::any::TypeId::of::<space::Screen>() {
            "Screen"
        } else {
            "Unknown"
        };

        write!(
            f,
            "{}Point({}, {}, {})",
            space_name,
            self.x(),
            self.y(),
            self.z()
        )
    }
}

// Display implementation for user-friendly output
impl<S: space::Space> Display for Point<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

// Implement Default for all point types (creates origin points)
impl<S: space::Space> Default for Point<S> {
    fn default() -> Self {
        Self::origin()
    }
}

// Implement mathematical operators for points in the same space

impl<S: space::Space> Add for Point<S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            vec: self.vec + rhs.vec,
            _space: PhantomData,
        }
    }
}

impl<S: space::Space> AddAssign for Point<S> {
    fn add_assign(&mut self, rhs: Self) {
        self.vec += rhs.vec;
    }
}

impl<S: space::Space> Sub for Point<S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            vec: self.vec - rhs.vec,
            _space: PhantomData,
        }
    }
}

impl<S: space::Space> SubAssign for Point<S> {
    fn sub_assign(&mut self, rhs: Self) {
        self.vec -= rhs.vec;
    }
}

impl<S: space::Space> Mul<f32> for Point<S> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            vec: self.vec * rhs,
            _space: PhantomData,
        }
    }
}

impl<S: space::Space> MulAssign<f32> for Point<S> {
    fn mul_assign(&mut self, rhs: f32) {
        self.vec *= rhs;
    }
}

impl<S: space::Space> Div<f32> for Point<S> {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            vec: self.vec / rhs,
            _space: PhantomData,
        }
    }
}

impl<S: space::Space> DivAssign<f32> for Point<S> {
    fn div_assign(&mut self, rhs: f32) {
        self.vec /= rhs;
    }
}

impl<S: space::Space> Neg for Point<S> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            vec: -self.vec,
            _space: PhantomData,
        }
    }
}

// TODO: Implement transform methods between spaces, probably using a Transform type
// For now, we'll implement a basic example of Local <-> World conversion

/// A transformation from one coordinate space to another
///
/// This provides type-safe transformations between spaces using matrices.
pub struct Transform<From: space::Space, To: space::Space> {
    /// The underlying 4x4 transformation matrix
    pub matrix: Mat4,
    _phantom: PhantomData<(From, To)>,
}

impl<From: space::Space, To: space::Space> Transform<From, To> {
    /// Create a new transformation with the given matrix
    pub fn new(matrix: Mat4) -> Self {
        Self {
            matrix,
            _phantom: PhantomData,
        }
    }

    /// Create an identity transformation (no change)
    pub fn identity() -> Self {
        Self::new(Mat4::IDENTITY)
    }

    /// Apply this transformation to a point in the source space,
    /// returning a point in the destination space
    pub fn transform_point(&self, point: Point<From>) -> Point<To> {
        let result = self.matrix.transform_point3(point.vec);
        Point {
            vec: result,
            _space: PhantomData,
        }
    }

    /// Get the inverse of this transformation
    pub fn inverse(&self) -> Transform<To, From> {
        Transform {
            matrix: self.matrix.inverse(),
            _phantom: PhantomData,
        }
    }
}

/// Type aliases for common transformations
pub type LocalToWorld = Transform<space::Local, space::World>;
pub type WorldToLocal = Transform<space::World, space::Local>;
pub type WorldToScreen = Transform<space::World, space::Screen>;
pub type ScreenToWorld = Transform<space::Screen, space::World>;

// Basic composition (chaining) of transforms
impl<A: space::Space, B: space::Space, C: space::Space> Mul<Transform<B, C>> for Transform<A, B> {
    type Output = Transform<A, C>;

    fn mul(self, rhs: Transform<B, C>) -> Self::Output {
        Transform {
            matrix: rhs.matrix * self.matrix, // Note: matrix multiplication order is reversed
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_constructors() {
        let p1 = LocalPoint::new(1.0, 2.0, 3.0);
        assert_eq!(p1.x(), 1.0);
        assert_eq!(p1.y(), 2.0);
        assert_eq!(p1.z(), 3.0);

        let p2 = WorldPoint::new_2d(4.0, 5.0);
        assert_eq!(p2.x(), 4.0);
        assert_eq!(p2.y(), 5.0);
        assert_eq!(p2.z(), 0.0);

        let p3 = ScreenPoint::from_vec3(Vec3::new(6.0, 7.0, 8.0));
        assert_eq!(p3.x(), 6.0);
        assert_eq!(p3.y(), 7.0);
        assert_eq!(p3.z(), 8.0);
    }

    #[test]
    fn test_point_operations() {
        let p1 = LocalPoint::new(1.0, 2.0, 3.0);
        let p2 = LocalPoint::new(4.0, 5.0, 6.0);

        let sum = p1 + p2;
        assert_eq!(sum.x(), 5.0);
        assert_eq!(sum.y(), 7.0);
        assert_eq!(sum.z(), 9.0);

        let diff = p2 - p1;
        assert_eq!(diff.x(), 3.0);
        assert_eq!(diff.y(), 3.0);
        assert_eq!(diff.z(), 3.0);

        let scaled = p1 * 2.0;
        assert_eq!(scaled.x(), 2.0);
        assert_eq!(scaled.y(), 4.0);
        assert_eq!(scaled.z(), 6.0);

        let divided = p2 / 2.0;
        assert_eq!(divided.x(), 2.0);
        assert_eq!(divided.y(), 2.5);
        assert_eq!(divided.z(), 3.0);
    }

    #[test]
    fn test_transforms() {
        // Create a translation transform that moves points 10 units in x direction
        let local_to_world = LocalToWorld::new(Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0)));

        // Create a local point at origin
        let local = LocalPoint::origin();

        // Transform to world space
        let world = local_to_world.transform_point(local);

        // Should be at (10, 0, 0) in world space
        assert_eq!(world.x(), 10.0);
        assert_eq!(world.y(), 0.0);
        assert_eq!(world.z(), 0.0);

        // Create inverse transform
        let world_to_local = local_to_world.inverse();

        // Transform back to local space
        let local_again = world_to_local.transform_point(world);

        // Should be back at origin
        assert_eq!(local_again.x(), 0.0);
        assert_eq!(local_again.y(), 0.0);
        assert_eq!(local_again.z(), 0.0);
    }

    #[test]
    fn test_transform_composition() {
        // Create a transform that moves points 10 units in x direction
        let t1 = LocalToWorld::new(Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0)));

        // Create a transform that moves points from world to screen (for example, adding 5 to y)
        let t2 = WorldToScreen::new(Mat4::from_translation(Vec3::new(0.0, 5.0, 0.0)));

        // Compose them to get a direct local-to-screen transform
        let t3 = t1 * t2;

        // Create a local point at origin
        let local = LocalPoint::origin();

        // Transform directly to screen space
        let screen = t3.transform_point(local);

        // Should be at (10, 5, 0) in screen space
        assert_eq!(screen.x(), 10.0);
        assert_eq!(screen.y(), 5.0);
        assert_eq!(screen.z(), 0.0);
    }

    #[test]
    fn test_debug_output() {
        let local = LocalPoint::new(1.0, 2.0, 3.0);
        let world = WorldPoint::new(4.0, 5.0, 6.0);
        let screen = ScreenPoint::new(7.0, 8.0, 9.0);

        assert_eq!(format!("{:?}", local), "LocalPoint(1, 2, 3)");
        assert_eq!(format!("{:?}", world), "WorldPoint(4, 5, 6)");
        assert_eq!(format!("{:?}", screen), "ScreenPoint(7, 8, 9)");
    }
}
