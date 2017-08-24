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

    /// Creates a telemetry parsing error.
    /// 
    /// - `description` a description of the error
    /// - `source` the string being parsed
    fn parse(description: &'static str, source: String) -> Self {
        TelemetryError {
            kind: TelemetryErrorKind::ParseError(source),
            cause: None,
            description,
        }
    }

    /// Creates a telemetry parsing error.
    ///
    /// - `description` a description of the error
    /// - `source` the string being parsed
    /// - `cause` the underlying parsing error
    fn parse_with_cause<E: Into<Cause>>(description: &'static str, source: String, cause: E) -> Self {
        TelemetryError {
            kind: TelemetryErrorKind::ParseError(source),
            cause: Some(cause.into()),
            description,
        }
    }
}
