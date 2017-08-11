mod altitude;
mod deployment;
mod heading;
mod rate_of_climb;
mod speed;
mod throttle;

pub use self::altitude::*;
pub use self::deployment::*;
pub use self::heading::*;
pub use self::rate_of_climb::*;
pub use self::speed::*;
pub use self::throttle::*;

use std::error;

#[derive(Debug)]
pub struct Cause(Box<error::Error>);

impl<E: error::Error + 'static> From<E> for Cause {
    fn from(error: E) -> Cause {
        Cause(Box::new(error))
    }
}

#[derive(Debug)]
pub struct TelemetryError {
    kind: TelemetryErrorKind,
    cause: Option<Cause>,
    description: &'static str,
}

#[derive(Debug)]
pub enum TelemetryErrorKind {
    UnknownMeasurement(String),
    ParseError(String),
}

impl TelemetryError {
    fn unknown_measurement<T: Into<String>>(measurement: T) -> Self {
        TelemetryError {
            kind: TelemetryErrorKind::UnknownMeasurement(measurement.into()),
            cause: None,
            description: "Unable to convert from measurement",
        }
    }
}
