use serde::de::*;
use std::borrow::Cow;
use unit::{TelemetryError, TelemetryErrorKind};

/// Represents a heading in degrees.
pub struct Heading(u16);

#[derive(Deserialize)]
pub struct HeadingPayload<'a> {
    heading: &'a str,
    measurement: &'a str,
}

impl Heading {
    fn from_payload<'a>(payload: &'a HeadingPayload<'a>) -> Result<Heading, TelemetryError> {
        let measurement = payload.measurement.to_lowercase();
        
        // Thease measurement types are pure, industrial grade guesswork, obviously. I am also
        // not necessarily in love with this method of converting, because I worry it's probably 
        // not the fastest way to go. Maybe we could even have some way of registering new 
        // conversions without redeploying the app. I would be fine with that.
        match &*measurement {
            "degrees" => unimplemented!(),
            "radians" => unimplemented!(),
            "compass" => unimplemented!(),

            _ => Err(TelemetryError::unknown_measurement(payload.measurement))
        }
    }
}
