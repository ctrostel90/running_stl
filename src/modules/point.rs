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
}
impl PartialEq for Point{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && 
        self.y == other.y &&
        self.z == other.z
    }
}
#[cfg(test)]
mod tests{
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
}