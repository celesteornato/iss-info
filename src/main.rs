use reqwest;
use serde_json::Value;
use std::f64::consts::PI;
use std::thread::sleep;
use std::time;
const URL: &str = "https://api.wheretheiss.at/v1/satellites/25544";
const RADIUS: f64 = 6371000.0;

struct Info {
    longitude: f64,
    latitude: f64,
    altitude: f64,
    date: time::Instant,
}

fn main() {
    let mut current_info = process();
    loop {
        sleep(time::Duration::from_millis(1500));
        let new_info = process();
        let angular_speed = f64::sqrt(
            (new_info.longitude - current_info.longitude)
                * (new_info.longitude - current_info.longitude)
                + (new_info.latitude - current_info.latitude)
                    * (new_info.latitude - current_info.latitude),
        ) / (new_info.date - current_info.date).as_secs_f64();
        let angspeed_rad = angular_speed * PI / 180.;
        let speed = angspeed_rad * (RADIUS + new_info.altitude);
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "Longitude: {0}°\nLatitide: {1}°\nAltitude: {2}\nAngular Speed: {angular_speed}°/s | {angspeed_rad} rad/s\nTangential Velocity: {speed} m/s",
            new_info.longitude, new_info.latitude, new_info.altitude
        );
        current_info = new_info;
    }
}

fn get_raw_json() -> Result<Value, reqwest::Error> {
    reqwest::blocking::get(URL)?.json()
}

fn get_lat_long<'a>(input: Value) -> (f64, f64, f64) {
    // NOTE: At this point, we know exactly what the response looks
    // like, this section WILL panic if the API response changes.
    if let (Value::Number(lat), Value::Number(long), Value::Number(alt)) = (
        input.get("latitude").unwrap(),
        input.get("longitude").unwrap(),
        input.get("altitude").unwrap(),
    ) {
        return (
            lat.as_f64().unwrap(),
            long.as_f64().unwrap(),
            alt.as_f64().unwrap(),
        );
    }
    panic!("API WAS CHANGED!!!")
}

fn process() -> Info {
    let body = get_raw_json();
    if let Err(_) = body {
        panic!("{:?}", body);
    };
    let (lat, long, alt) = get_lat_long(body.unwrap());
    Info {
        latitude: lat,
        longitude: long,
        altitude: alt,
        date: time::Instant::now(),
    }
}
