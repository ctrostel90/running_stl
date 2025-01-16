pub struct StlSlice{
    gps:f32,
    top_left:f32,
    top_right:f32,
    bottom_left:f32,
    bottom_right:f32
}

impl StlSlice{
    pub fn new(gps:f32)->Self{
        Self { gps: gps, top_left: 0.0, top_right: 0.0, bottom_left: 0.0, bottom_right: 0.0 }
    }
}
