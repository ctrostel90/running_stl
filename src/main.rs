use quick_xml::de::from_str;
use serde::Deserialize;
use std::{fs::{self, File}, io::{BufWriter, Error, Write}};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TrainingCenterDatabase {
    activities: Activities,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Activities {
    activity: Vec<Activity>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Activity {
    #[serde(rename = "Sport")]
    sport: Option<String>,
    id: String,
    lap: Vec<Lap>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Lap {
    #[serde(rename = "StartTime")]
    start_time: Option<String>,
    total_time_seconds: f64,
    distance_meters: f64,
    maximum_speed: f64,
    calories: i32,
    average_heart_rate_bpm: HeartRateBpm,
    maximum_heart_rate_bpm: HeartRateBpm,
    intensity: String,
    trigger_method: String,
    track: Track,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct HeartRateBpm {
    value: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Track {
    trackpoint: Vec<Trackpoint>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Trackpoint {
    time: String,
    position: Option<Position>,
    altitude_meters: f64,
    distance_meters: f64,
    heart_rate_bpm: HeartRateBpm,
    extensions: Option<Extensions>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Position {
    latitude_degrees: f64,
    longitude_degrees: f64,
}

impl Position {
    pub fn to_cartesian(&self) -> (f64, f64) {
        const EARTH_RADIUS: f64 = 6371000.0;
        //Convert to radians, then project with Equirectangular projection (simpliefied)
        (
            self.latitude_degrees * std::f64::consts::PI / 180.0 * EARTH_RADIUS,
            self.longitude_degrees * std::f64::consts::PI / 180.0 * EARTH_RADIUS,
        )
    }
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Extensions {
    #[serde(rename = "ns3:TPX")]
    tpx: Option<TPX>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TPX {
    speed: f64,
    run_cadence: i32,
}

struct PointWrapper{
    x:f64,
    y:f64,
    z:f64,
    distance:f64,
}

fn find_maximum(points: &Vec<PointWrapper>) -> (f64, f64, f64) {
    let (max_x, max_y, max_z) = points.iter().fold(
        (f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY), 
        |(max_x, max_y, max_z), point| {
            (
                max_x.max(point.x),
                max_y.max(point.y),
                max_z.max(point.z),
            )
        }
    );
    
    (max_x, max_y, max_z)
}

fn find_minimum(points: &Vec<PointWrapper>) -> (f64, f64, f64) {
    let (min_x, min_y, min_z) = points.iter().fold(
        (f64::INFINITY, f64::INFINITY, f64::INFINITY), 
        |(min_x, min_y, min_z), point| {
            (
                min_x.min(point.x),
                min_y.min(point.y),
                min_z.min(point.z),
            )
        }
    );
    
    (min_x, min_y, min_z)
}

fn scale_point(to_scale:f64,old_min:f64,old_max:f64,new_min:f64,new_max:f64) -> f64{
    new_min + ((to_scale - old_min) * (new_max - new_min)) / (old_max - old_min)
}

fn main() -> Result<(), Error>{
    let xml_data = fs::read_to_string("/home/trostelc/rust/running_stl/data/race.tcx")
        .expect("Failed to read XML file");

    // Deserialize the XML into the `TrainingCenterDatabase` struct
    let result: TrainingCenterDatabase = from_str(&xml_data).expect("Failed to deserialize XML");
    let mut pointlist: Vec<PointWrapper> = Vec::new();
    result.activities.activity[0].lap.iter().for_each(|lap| {
        lap.track.trackpoint.iter().for_each(|point|{
            match &point.position{
                Some(pos)=> {
                    let (x,y) = pos.to_cartesian();
                    pointlist.push(PointWrapper{
                        x: x,
                        y: y,
                        z: point.altitude_meters,
                        distance: point.distance_meters,
                    });
                },
                None => (),
            };
        });
    });

    let (min_x,min_y,min_z) = find_minimum(&pointlist);
    let (max_x,max_y,max_z) = find_maximum(&pointlist);
    //Need to calculate the correct aspect ratio
    let aspect_ratio = (max_x-min_x) / (max_y - min_y);
    let (mut new_width, mut new_height) = (400.0,400.0);
    if aspect_ratio > 1.0{
        //width is bigger than height
        new_height /= aspect_ratio;
    }
    else{
        //height is bigger than width
        new_width /= aspect_ratio;
    }
    let mut scaled_points:Vec<(f64,f64,f64)> = Vec::new();
    let (new_min_x,new_max_x) = (-new_width/ 2.0,new_width / 2.0);
    let (new_min_y,new_max_y) = (-new_height / 2.0,new_height / 2.0);
    
    pointlist.iter().for_each(|point| scaled_points.push((
        scale_point(point.x, min_x, max_x, new_min_x, new_max_x),
        scale_point(point.y, min_y, max_y, new_min_y, new_max_y),
        point.z
    )));

    let f = File::create("/home/trostelc/rust/running_stl/output/output.txt")?;
    let mut writer = BufWriter::new(f);
    for tuple in scaled_points{
        writeln!(writer,"{},{},{};",tuple.0,tuple.1,tuple.2)?;
    }
    
    

    Ok(())
}
