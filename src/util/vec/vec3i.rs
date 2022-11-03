use crate::util::vec::CoordinateSpace;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub struct Vec3i<SPACE: CoordinateSpace> {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    _phantom: PhantomData<SPACE>,
}
impl<SPACE: CoordinateSpace> Vec3i<SPACE> {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
            _phantom: Default::default(),
        }
    }
}

impl<SPACE: CoordinateSpace> Copy for Vec3i<SPACE> {}
impl<SPACE: CoordinateSpace> Clone for Vec3i<SPACE> {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            _phantom: Default::default(),
        }
    }
}

impl<SPACE: CoordinateSpace> Eq for Vec3i<SPACE> {}
impl<SPACE: CoordinateSpace> PartialEq<Self> for Vec3i<SPACE> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl<SPACE: CoordinateSpace> Hash for Vec3i<SPACE> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl<SPACE: CoordinateSpace> Display for Vec3i<SPACE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("({}, {}, {})", self.x, self.y, self.z))
    }
}

impl<SPACE: CoordinateSpace> std::ops::Neg for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn neg(self) -> Self::Output {
        let lhs = self;
        (|a: Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(-a.x, -a.y, -a.z) })(lhs)
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<&Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn add(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x + b.x, a.y + b.y, a.z + b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn add(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x + b.x, a.y + b.y, a.z + b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<&Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn add(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x + b.x, a.y + b.y, a.z + b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn add(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x + b.x, a.y + b.y, a.z + b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::AddAssign<&Vec3i<SPACE>> for Vec3i<SPACE> {
    fn add_assign(&mut self, rhs: &Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x += b.x;
                    a.y += b.y;
                    a.z += b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::AddAssign<Vec3i<SPACE>> for Vec3i<SPACE> {
    fn add_assign(&mut self, rhs: Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x += b.x;
                    a.y += b.y;
                    a.z += b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<&Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn sub(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x - b.x, a.y - b.y, a.z - b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn sub(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x - b.x, a.y - b.y, a.z - b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<&Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn sub(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x - b.x, a.y - b.y, a.z - b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn sub(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x - b.x, a.y - b.y, a.z - b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::SubAssign<&Vec3i<SPACE>> for Vec3i<SPACE> {
    fn sub_assign(&mut self, rhs: &Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x -= b.x;
                    a.y -= b.y;
                    a.z -= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::SubAssign<Vec3i<SPACE>> for Vec3i<SPACE> {
    fn sub_assign(&mut self, rhs: Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x -= b.x;
                    a.y -= b.y;
                    a.z -= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<&Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn mul(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x * b.x, a.y * b.y, a.z * b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn mul(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x * b.x, a.y * b.y, a.z * b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<&Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn mul(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x * b.x, a.y * b.y, a.z * b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn mul(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x * b.x, a.y * b.y, a.z * b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::MulAssign<&Vec3i<SPACE>> for Vec3i<SPACE> {
    fn mul_assign(&mut self, rhs: &Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x *= b.x;
                    a.y *= b.y;
                    a.z *= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::MulAssign<Vec3i<SPACE>> for Vec3i<SPACE> {
    fn mul_assign(&mut self, rhs: Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x *= b.x;
                    a.y *= b.y;
                    a.z *= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<&Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn div(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x / b.x, a.y / b.y, a.z / b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn div(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x / b.x, a.y / b.y, a.z / b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<&Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn div(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x / b.x, a.y / b.y, a.z / b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn div(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x / b.x, a.y / b.y, a.z / b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::DivAssign<&Vec3i<SPACE>> for Vec3i<SPACE> {
    fn div_assign(&mut self, rhs: &Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x /= b.x;
                    a.y /= b.y;
                    a.z /= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::DivAssign<Vec3i<SPACE>> for Vec3i<SPACE> {
    fn div_assign(&mut self, rhs: Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x /= b.x;
                    a.y /= b.y;
                    a.z /= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<&i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn add(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x + b, a.y + b, a.z + b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x + b, a.y + b, a.z + b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<&i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn add(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x + b, a.y + b, a.z + b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x + b, a.y + b, a.z + b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::AddAssign<&i32> for Vec3i<SPACE> {
    fn add_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x += b;
                    a.y += b;
                    a.z += b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::AddAssign<i32> for Vec3i<SPACE> {
    fn add_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x += b;
                    a.y += b;
                    a.z += b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<&i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn sub(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x - b, a.y - b, a.z - b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x - b, a.y - b, a.z - b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<&i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn sub(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x - b, a.y - b, a.z - b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x - b, a.y - b, a.z - b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::SubAssign<&i32> for Vec3i<SPACE> {
    fn sub_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x -= b;
                    a.y -= b;
                    a.z -= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::SubAssign<i32> for Vec3i<SPACE> {
    fn sub_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x -= b;
                    a.y -= b;
                    a.z -= b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<&i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn mul(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x * b, a.y * b, a.z * b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x * b, a.y * b, a.z * b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<&i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn mul(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x * b, a.y * b, a.z * b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x * b, a.y * b, a.z * b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::MulAssign<&i32> for Vec3i<SPACE> {
    fn mul_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x *= b;
                    a.y *= b;
                    a.z *= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::MulAssign<i32> for Vec3i<SPACE> {
    fn mul_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x *= b;
                    a.y *= b;
                    a.z *= b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<&i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn div(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x / b, a.y / b, a.z / b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x / b, a.y / b, a.z / b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<&i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn div(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x / b, a.y / b, a.z / b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x / b, a.y / b, a.z / b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::DivAssign<&i32> for Vec3i<SPACE> {
    fn div_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x /= b;
                    a.y /= b;
                    a.z /= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::DivAssign<i32> for Vec3i<SPACE> {
    fn div_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x /= b;
                    a.y /= b;
                    a.z /= b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<&Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitand(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x & b.x, a.y & b.y, a.z & b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitand(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x & b.x, a.y & b.y, a.z & b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<&Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitand(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x & b.x, a.y & b.y, a.z & b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitand(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x & b.x, a.y & b.y, a.z & b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAndAssign<&Vec3i<SPACE>> for Vec3i<SPACE> {
    fn bitand_assign(&mut self, rhs: &Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x &= b.x;
                    a.y &= b.y;
                    a.z &= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAndAssign<Vec3i<SPACE>> for Vec3i<SPACE> {
    fn bitand_assign(&mut self, rhs: Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x &= b.x;
                    a.y &= b.y;
                    a.z &= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<&Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitor(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x | b.x, a.y | b.y, a.z | b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitor(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x | b.x, a.y | b.y, a.z | b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<&Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitor(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x | b.x, a.y | b.y, a.z | b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitor(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x | b.x, a.y | b.y, a.z | b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOrAssign<&Vec3i<SPACE>> for Vec3i<SPACE> {
    fn bitor_assign(&mut self, rhs: &Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x |= b.x;
                    a.y |= b.y;
                    a.z |= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOrAssign<Vec3i<SPACE>> for Vec3i<SPACE> {
    fn bitor_assign(&mut self, rhs: Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x |= b.x;
                    a.y |= b.y;
                    a.z |= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<&Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shl(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x << b.x, a.y << b.y, a.z << b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shl(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x << b.x, a.y << b.y, a.z << b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<&Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shl(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x << b.x, a.y << b.y, a.z << b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shl(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x << b.x, a.y << b.y, a.z << b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShlAssign<&Vec3i<SPACE>> for Vec3i<SPACE> {
    fn shl_assign(&mut self, rhs: &Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x <<= b.x;
                    a.y <<= b.y;
                    a.z <<= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShlAssign<Vec3i<SPACE>> for Vec3i<SPACE> {
    fn shl_assign(&mut self, rhs: Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x <<= b.x;
                    a.y <<= b.y;
                    a.z <<= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<&Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shr(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x >> b.x, a.y >> b.y, a.z >> b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<Vec3i<SPACE>> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shr(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x >> b.x, a.y >> b.y, a.z >> b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<&Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shr(self, rhs: &Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x >> b.x, a.y >> b.y, a.z >> b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<Vec3i<SPACE>> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shr(self, rhs: Vec3i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(a.x >> b.x, a.y >> b.y, a.z >> b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShrAssign<&Vec3i<SPACE>> for Vec3i<SPACE> {
    fn shr_assign(&mut self, rhs: &Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x >>= b.x;
                    a.y >>= b.y;
                    a.z >>= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShrAssign<Vec3i<SPACE>> for Vec3i<SPACE> {
    fn shr_assign(&mut self, rhs: Vec3i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &Vec3i<SPACE>| -> () {
                {
                    a.x >>= b.x;
                    a.y >>= b.y;
                    a.z >>= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Not for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn not(self) -> Self::Output {
        let lhs = self;
        (|a: Vec3i<SPACE>| -> Vec3i<SPACE> { Vec3i::new(!a.x, !a.y, !a.z) })(lhs)
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<&i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitand(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x & b, a.y & b, a.z & b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitand(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x & b, a.y & b, a.z & b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<&i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitand(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x & b, a.y & b, a.z & b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitand(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x & b, a.y & b, a.z & b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAndAssign<&i32> for Vec3i<SPACE> {
    fn bitand_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x &= b;
                    a.y &= b;
                    a.z &= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAndAssign<i32> for Vec3i<SPACE> {
    fn bitand_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x &= b;
                    a.y &= b;
                    a.z &= b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<&i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitor(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x | b, a.y | b, a.z | b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitor(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x | b, a.y | b, a.z | b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<&i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitor(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x | b, a.y | b, a.z | b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn bitor(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x | b, a.y | b, a.z | b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOrAssign<&i32> for Vec3i<SPACE> {
    fn bitor_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x |= b;
                    a.y |= b;
                    a.z |= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOrAssign<i32> for Vec3i<SPACE> {
    fn bitor_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x |= b;
                    a.y |= b;
                    a.z |= b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<&i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shl(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x << b, a.y << b, a.z << b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shl(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x << b, a.y << b, a.z << b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<&i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shl(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x << b, a.y << b, a.z << b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shl(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x << b, a.y << b, a.z << b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShlAssign<&i32> for Vec3i<SPACE> {
    fn shl_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x <<= b;
                    a.y <<= b;
                    a.z <<= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShlAssign<i32> for Vec3i<SPACE> {
    fn shl_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x <<= b;
                    a.y <<= b;
                    a.z <<= b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<&i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shr(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x >> b, a.y >> b, a.z >> b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<i32> for &Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shr(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x >> b, a.y >> b, a.z >> b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<&i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shr(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x >> b, a.y >> b, a.z >> b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<i32> for Vec3i<SPACE> {
    type Output = Vec3i<SPACE>;
    fn shr(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec3i<SPACE>, b: &i32| -> Vec3i<SPACE> { Vec3i::new(a.x >> b, a.y >> b, a.z >> b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShrAssign<&i32> for Vec3i<SPACE> {
    fn shr_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x >>= b;
                    a.y >>= b;
                    a.z >>= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShrAssign<i32> for Vec3i<SPACE> {
    fn shr_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec3i<SPACE>, b: &i32| -> () {
                {
                    a.x >>= b;
                    a.y >>= b;
                    a.z >>= b;
                }
            })(lhs, &rhs);
        }
    }
}
