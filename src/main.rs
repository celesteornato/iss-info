use reqwest;
use serde_json::Value;
use std::f32::consts::PI;
use std::thread::sleep;
use std::time;

const URL: &str = "http://api.open-notify.org/iss-now.json";
const RADIUS: f32 = 6777038.8;

struct Info {
    longitude: f32,
    latitude: f32,
    date: time::Instant,
}

fn main() {
    let mut current_info = Info {
        longitude: 0.,
        latitude: 0.,
        date: time::Instant::now(),
    };
    loop {
        let new_info = process();
        let angular_speed = f32::sqrt(
            (new_info.longitude - current_info.longitude)
                * (new_info.longitude - current_info.longitude)
                + (new_info.latitude - current_info.latitude)
                    * (new_info.latitude - current_info.latitude),
        ) / (new_info.date - current_info.date).as_secs_f32();
        let angspeed_rad = angular_speed * PI / 180.;
        let speed = angspeed_rad * RADIUS;
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "Longitude: {0}°\nLatitide: {1}°\n Angular Speed: {angular_speed}°/s | {angspeed_rad} rad/s\nTangential Velocity: {speed} m/s",
            new_info.longitude, new_info.latitude
        );
        current_info = new_info;
        sleep(time::Duration::from_millis(2000));
    }
}

fn get_raw_json() -> Result<Value, reqwest::Error> {
    reqwest::blocking::get(URL)?.json()
}

fn get_lat_long<'a>(input: Value) -> (String, String) {
    // NOTE: At this point, we know exactly what the response looks
    // like, this section WILL panic if the API response changes.
    let pos = input.get("iss_position").unwrap();
    if let (Value::String(lat), Value::String(long)) =
        (pos.get("latitude").unwrap(), pos.get("longitude").unwrap())
    {
        return (lat.clone(), long.clone());
    }
    panic!("API WAS CHANGED!!!")
}

fn process() -> Info {
    let body = get_raw_json();
    if let Err(_) = body {
        panic!("{:?}", body);
    };
    let (lat, long) = get_lat_long(body.unwrap());
    Info {
        latitude: lat.parse().unwrap(),
        longitude: long.parse().unwrap(),
        date: time::Instant::now(),
    }
}
