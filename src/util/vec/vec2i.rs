use crate::util::vec::CoordinateSpace;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub struct Vec2i<SPACE: CoordinateSpace> {
    pub x: i32,
    pub z: i32,
    pub _phantom: PhantomData<SPACE>,
}
impl<SPACE: CoordinateSpace> Vec2i<SPACE> {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x,
            z,
            _phantom: Default::default(),
        }
    }
}

impl<SPACE: CoordinateSpace> Copy for Vec2i<SPACE> {}
impl<SPACE: CoordinateSpace> Clone for Vec2i<SPACE> {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            z: self.z,
            _phantom: Default::default(),
        }
    }
}

impl<SPACE: CoordinateSpace> Eq for Vec2i<SPACE> {}
impl<SPACE: CoordinateSpace> PartialEq<Self> for Vec2i<SPACE> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.z == other.z
    }
}
impl<SPACE: CoordinateSpace> Hash for Vec2i<SPACE> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.z.hash(state);
    }
}

impl<SPACE: CoordinateSpace> Display for Vec2i<SPACE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("({}, {})", self.x, self.z))
    }
}

impl<SPACE: CoordinateSpace> std::ops::Neg for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn neg(self) -> Self::Output {
        let lhs = self;
        (|a: Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(-a.x, -a.z) })(lhs)
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<&Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn add(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x + b.x, a.z + b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn add(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x + b.x, a.z + b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<&Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn add(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x + b.x, a.z + b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn add(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x + b.x, a.z + b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::AddAssign<&Vec2i<SPACE>> for Vec2i<SPACE> {
    fn add_assign(&mut self, rhs: &Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x += b.x;
                    a.z += b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::AddAssign<Vec2i<SPACE>> for Vec2i<SPACE> {
    fn add_assign(&mut self, rhs: Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x += b.x;
                    a.z += b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<&Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn sub(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x - b.x, a.z - b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn sub(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x - b.x, a.z - b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<&Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn sub(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x - b.x, a.z - b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn sub(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x - b.x, a.z - b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::SubAssign<&Vec2i<SPACE>> for Vec2i<SPACE> {
    fn sub_assign(&mut self, rhs: &Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x -= b.x;
                    a.z -= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::SubAssign<Vec2i<SPACE>> for Vec2i<SPACE> {
    fn sub_assign(&mut self, rhs: Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x -= b.x;
                    a.z -= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<&Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn mul(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x * b.x, a.z * b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn mul(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x * b.x, a.z * b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<&Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn mul(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x * b.x, a.z * b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn mul(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x * b.x, a.z * b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::MulAssign<&Vec2i<SPACE>> for Vec2i<SPACE> {
    fn mul_assign(&mut self, rhs: &Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x *= b.x;
                    a.z *= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::MulAssign<Vec2i<SPACE>> for Vec2i<SPACE> {
    fn mul_assign(&mut self, rhs: Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x *= b.x;
                    a.z *= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<&Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn div(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x / b.x, a.z / b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn div(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x / b.x, a.z / b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<&Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn div(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x / b.x, a.z / b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn div(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x / b.x, a.z / b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::DivAssign<&Vec2i<SPACE>> for Vec2i<SPACE> {
    fn div_assign(&mut self, rhs: &Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x /= b.x;
                    a.z /= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::DivAssign<Vec2i<SPACE>> for Vec2i<SPACE> {
    fn div_assign(&mut self, rhs: Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x /= b.x;
                    a.z /= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<&i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn add(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x + b, a.z + b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x + b, a.z + b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<&i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn add(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x + b, a.z + b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Add<i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x + b, a.z + b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::AddAssign<&i32> for Vec2i<SPACE> {
    fn add_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x += b;
                    a.z += b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::AddAssign<i32> for Vec2i<SPACE> {
    fn add_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x += b;
                    a.z += b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<&i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn sub(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x - b, a.z - b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x - b, a.z - b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<&i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn sub(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x - b, a.z - b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Sub<i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x - b, a.z - b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::SubAssign<&i32> for Vec2i<SPACE> {
    fn sub_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x -= b;
                    a.z -= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::SubAssign<i32> for Vec2i<SPACE> {
    fn sub_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x -= b;
                    a.z -= b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<&i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn mul(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x * b, a.z * b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x * b, a.z * b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<&i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn mul(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x * b, a.z * b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Mul<i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x * b, a.z * b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::MulAssign<&i32> for Vec2i<SPACE> {
    fn mul_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x *= b;
                    a.z *= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::MulAssign<i32> for Vec2i<SPACE> {
    fn mul_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x *= b;
                    a.z *= b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<&i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn div(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x / b, a.z / b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x / b, a.z / b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<&i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn div(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x / b, a.z / b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Div<i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x / b, a.z / b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::DivAssign<&i32> for Vec2i<SPACE> {
    fn div_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x /= b;
                    a.z /= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::DivAssign<i32> for Vec2i<SPACE> {
    fn div_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x /= b;
                    a.z /= b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<&Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitand(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x & b.x, a.z & b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitand(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x & b.x, a.z & b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<&Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitand(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x & b.x, a.z & b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitand(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x & b.x, a.z & b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAndAssign<&Vec2i<SPACE>> for Vec2i<SPACE> {
    fn bitand_assign(&mut self, rhs: &Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x &= b.x;
                    a.z &= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAndAssign<Vec2i<SPACE>> for Vec2i<SPACE> {
    fn bitand_assign(&mut self, rhs: Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x &= b.x;
                    a.z &= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<&Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitor(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x | b.x, a.z | b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitor(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x | b.x, a.z | b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<&Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitor(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x | b.x, a.z | b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitor(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x | b.x, a.z | b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOrAssign<&Vec2i<SPACE>> for Vec2i<SPACE> {
    fn bitor_assign(&mut self, rhs: &Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x |= b.x;
                    a.z |= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOrAssign<Vec2i<SPACE>> for Vec2i<SPACE> {
    fn bitor_assign(&mut self, rhs: Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x |= b.x;
                    a.z |= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<&Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shl(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x << b.x, a.z << b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shl(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x << b.x, a.z << b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<&Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shl(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x << b.x, a.z << b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shl(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x << b.x, a.z << b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShlAssign<&Vec2i<SPACE>> for Vec2i<SPACE> {
    fn shl_assign(&mut self, rhs: &Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x <<= b.x;
                    a.z <<= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShlAssign<Vec2i<SPACE>> for Vec2i<SPACE> {
    fn shl_assign(&mut self, rhs: Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x <<= b.x;
                    a.z <<= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<&Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shr(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x >> b.x, a.z >> b.z) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<Vec2i<SPACE>> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shr(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x >> b.x, a.z >> b.z) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<&Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shr(self, rhs: &Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x >> b.x, a.z >> b.z) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<Vec2i<SPACE>> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shr(self, rhs: Vec2i<SPACE>) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(a.x >> b.x, a.z >> b.z) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShrAssign<&Vec2i<SPACE>> for Vec2i<SPACE> {
    fn shr_assign(&mut self, rhs: &Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x >>= b.x;
                    a.z >>= b.z;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShrAssign<Vec2i<SPACE>> for Vec2i<SPACE> {
    fn shr_assign(&mut self, rhs: Vec2i<SPACE>) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &Vec2i<SPACE>| -> () {
                {
                    a.x >>= b.x;
                    a.z >>= b.z;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Not for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn not(self) -> Self::Output {
        let lhs = self;
        (|a: Vec2i<SPACE>| -> Vec2i<SPACE> { Vec2i::new(!a.x, !a.z) })(lhs)
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<&i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitand(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x & b, a.z & b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitand(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x & b, a.z & b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<&i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitand(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x & b, a.z & b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAnd<i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitand(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x & b, a.z & b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAndAssign<&i32> for Vec2i<SPACE> {
    fn bitand_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x &= b;
                    a.z &= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitAndAssign<i32> for Vec2i<SPACE> {
    fn bitand_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x &= b;
                    a.z &= b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<&i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitor(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x | b, a.z | b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitor(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x | b, a.z | b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<&i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitor(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x | b, a.z | b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOr<i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn bitor(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x | b, a.z | b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOrAssign<&i32> for Vec2i<SPACE> {
    fn bitor_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x |= b;
                    a.z |= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::BitOrAssign<i32> for Vec2i<SPACE> {
    fn bitor_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x |= b;
                    a.z |= b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<&i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shl(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x << b, a.z << b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shl(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x << b, a.z << b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<&i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shl(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x << b, a.z << b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shl<i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shl(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x << b, a.z << b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShlAssign<&i32> for Vec2i<SPACE> {
    fn shl_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x <<= b;
                    a.z <<= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShlAssign<i32> for Vec2i<SPACE> {
    fn shl_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x <<= b;
                    a.z <<= b;
                }
            })(lhs, &rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<&i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shr(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x >> b, a.z >> b) })(lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<i32> for &Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shr(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x >> b, a.z >> b) })(lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<&i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shr(self, rhs: &i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x >> b, a.z >> b) })(&lhs, rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::Shr<i32> for Vec2i<SPACE> {
    type Output = Vec2i<SPACE>;
    fn shr(self, rhs: i32) -> Self::Output {
        let lhs = self;
        {
            (|a: &Vec2i<SPACE>, b: &i32| -> Vec2i<SPACE> { Vec2i::new(a.x >> b, a.z >> b) })(&lhs, &rhs)
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShrAssign<&i32> for Vec2i<SPACE> {
    fn shr_assign(&mut self, rhs: &i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x >>= b;
                    a.z >>= b;
                }
            })(lhs, rhs);
        }
    }
}

impl<SPACE: CoordinateSpace> std::ops::ShrAssign<i32> for Vec2i<SPACE> {
    fn shr_assign(&mut self, rhs: i32) {
        let lhs = self;
        {
            (|a: &mut Vec2i<SPACE>, b: &i32| -> () {
                {
                    a.x >>= b;
                    a.z >>= b;
                }
            })(lhs, &rhs);
        }
    }
}
