use std::{f64::consts::PI, ops::{Add, Mul}};

#[derive(Debug)]
pub struct Point{
    x:f64,
    y:f64,
    z:f64,
}

impl Point{
    pub fn new(x:f64,y:f64,z:f64) -> Self{
        Self{
            x,
            y,
            z
        }
    }
    fn magnitude(&self) -> f64{
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    pub fn unit_vector(&self) -> Point{
        let mag = self.magnitude();
        print!("{mag}");
        Point::new( self.x/mag, self.y/mag, self.z/mag ) 
    }

    pub fn unit_vector_projected(&self) -> Point{
        let mag = Point::new(self.x,self.y,0.0).magnitude();
        print!("{mag}");
        Point::new( self.x/mag, self.y/mag, self.z) 
    }
    
    pub fn dot_product(&self,other:&Point) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn angle_between(pt1:&Point,pt2:&Point) -> f64{
        let dot = pt1.dot_product(pt2);
        let mag1 = pt1.magnitude();
        let mag2 = pt2.magnitude();
        let cos_theta = dot / (mag1 * mag2);
        cos_theta.acos()
    }

    pub fn surrounding_points(pt1:&Point,pt2:&Point,pt3:&Point,radius:f64) -> (Point,Point){
        let vec21 = Point::new(pt2.x - pt1.x,pt2.y - pt1.y,pt2.z - pt1.z);
        let vec23 = Point::new(pt2.x - pt3.x,pt2.y - pt3.y,pt2.z - pt3.z);
        let angle_between = Point::angle_between(&vec21,&vec23);

        if (angle_between * 100.0).round() / 100.0 == (PI * 100.0).round() / 100.0 {
            return (
                Point::new(-pt2.y,pt2.x,pt2.z).unit_vector() * -radius + pt2,
                Point::new(-pt2.y,pt2.x,pt2.z).unit_vector() * radius + pt2,
            )
        }
        else {
            
        }

    }
}
impl PartialEq for Point{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && 
        self.y == other.y &&
        self.z == other.z
    }
}

impl Mul<f64> for Point{
    type Output = Point;
    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl<'a,'b> Add<&'b Point> for &'a Point{
    type Output = Point;
    fn add(self, rhs: &'b Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
#[cfg(test)]
mod tests{
    use core::f64;

    use super::*;

    #[test]
    fn test_unit_vec() {
        let pt = Point::new(1.0,1.0,0.0);
        let val = pt.unit_vector();
        
        assert!((val.x - 0.7071).abs() < 0.0001);
        assert!((val.y - 0.7071).abs() < 0.0001);
        assert!((val.z - 0.0).abs() < 0.0001);
    }
    #[test]
    fn test_unit_vec_projected(){
        let pt = Point::new(1.0,1.0,10.0);
        let val = pt.unit_vector_projected();
        println!("{val:?}");
        assert!((val.x - 0.7071).abs() < 0.0001);
        assert!((val.y - 0.7071).abs() < 0.0001);
        assert!((val.z - 10.0).abs() < 0.0001);
    }
    #[test]
    fn test_dot(){
        let pt1 = Point::new(1.0,2.0,3.0);
        let pt2 = Point::new(4.0,-5.0,6.0);
        
        let result = pt1.dot_product(&pt2);
        assert!((result - 12.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_angle_between(){
        let pt1 = Point::new(1.0,0.0,0.0);
        let pt2 = Point::new(-1.0,0.0,0.0);

        assert!(Point::angle_between(&pt1,&pt2) - f64::consts::PI < f64::EPSILON);
    }

}