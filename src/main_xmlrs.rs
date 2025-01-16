use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

struct XmlEntryIterator<R: BufRead> {
    parser : EventReader<R>,
    depth: u32,
    activity:Activity,
    state: Xml
}

struct Activity {
    Id: String,
    Laps: Vec<Lap>,
}
struct Lap {
    StartTime: String,
    TotalTimeSeconds: f32,
    DistanceMeters: f32,
    MaxSpeed: f32,
    Calories: u16,
    AverageHearRate: u8,
    MaximumHeartRate: u8,
    Track: Vec<TrackPoint>,
}

enum XmlTrackPointReadingState{
    Off,
    Time,
    PositionStart,
    Latitude,
    Longitude,
    PositionEnd,
    Altitude,
    Distance,
    HeartRate,
    TrackPointEnd,
}
enum XmlEntryReadingState{
    Off,
    Comment,
    End,
}
struct TrackPoint {
    Time: String,
    Position: TrackPosition,
    Altitude: f32,
    HeartRate: u8,
}
struct TrackPosition {
    Latitude: f64,
    Longitude: f64,fs
}

fn main() -> std::io::Result<()> {
    let file = File::open("/home/trostelc/rust/running_stl/data/race.tcx")?;
    let file = BufReader::new(file); // Buffering is important for performance

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                //println!("{:spaces$}+{name}", "", spaces = depth * 2);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                //println!("{:spaces$}-{name}", "", spaces = depth * 2);
            }
            Ok(XmlEvent::Characters(text)) => {
                println!("{text}");
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn parse_point(){

    }
}