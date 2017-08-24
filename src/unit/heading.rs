use unit::TelemetryError;

/// A heading in degrees.
pub struct Heading(f64);

type HeadingResult = Result<Heading, TelemetryError>;

#[derive(Deserialize)]
pub struct HeadingPayload {
    heading: String,
    measurement: String,
}

impl Heading {
    fn from_payload<'a>(payload: HeadingPayload) -> Result<Heading, TelemetryError> {
        let measurement = payload.measurement.to_lowercase();
        
        // Thease measurement types are pure, industrial grade guesswork, obviously. I am also
        // not necessarily in love with this method of converting, because I worry it's probably 
        // not the fastest way to go. Maybe we could even have some way of registering new 
        // conversions without redeploying the app. I would be fine with that.
        match &*measurement {
            "degrees" => parse_degrees(payload.heading),
            "radians" => parse_radians(payload.heading),
            "compass" => parse_compass(payload.heading),

            _ => Err(TelemetryError::unknown_measurement(payload.measurement))
        }
    }
}

fn parse_degrees(s: String) -> HeadingResult {
    let value: f64 = s.parse().map_err(|e| {
        TelemetryError::parse_with_cause("unable to parse degrees as heading", s.into(), e)
    })?;

    Ok(Heading(value))
}

fn parse_radians(s: String) -> HeadingResult {
    let value: f64 = s.parse().map_err(|e| {
        TelemetryError::parse_with_cause("unable to parse radians as heading", s.into(), e)
    })?;

    Ok(Heading(value * 57.2958))
}

fn parse_compass(s: String) -> HeadingResult {
    let lowercase = s.to_lowercase();

    match &*lowercase {
        "n" | "north" => Ok(Heading(0.0)),
        "nne" | "north by northeast" => Ok(Heading(22.5)),
        "ne" | "northeast" => Ok(Heading(45.0)),
        "ene" | "east by northeast" => Ok(Heading(67.5)),

        "e" | "east" => Ok(Heading(90.0)),
        "ese" | "east by southeast" => Ok(Heading(112.5)),
        "se" | "southeast" => Ok(Heading(135.0)),
        "sse" | "south by southeast" => Ok(Heading(157.5)),
        
        "s" | "south" => Ok(Heading(180.0)),
        "ssw" | "south by southwest" => Ok(Heading(202.5)),
        "sw" | "southwest" => Ok(Heading(225.0)),
        "wsw" | "west by southwest" => Ok(Heading(247.5)),

        "w" | "west" => Ok(Heading(270.0)),
        "wnw" | "west by northwest" => Ok(Heading(292.5)),
        "nw" | "northwest" => Ok(Heading(315.0)),
        "nnw" | "north by northwest" => Ok(Heading(337.5)),

        _ => Err(TelemetryError::parse("unable to parse compass direction as heading", s)),
    }
}
