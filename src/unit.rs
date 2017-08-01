/// Represents a speed in meters/sec.
pub struct Speed(f64);

/// Represents an altitude in meters.
pub struct Altitude(f64);

/// Represents a rate of climb (positive or negative) in meters/sec.
pub struct RateOfClimb(f64);

/// Represents a heading in degrees.
pub struct Heading(f64);

/// Represents control surface deployment as +/- from 0.
pub struct Deployment(i8);

/// Represents a percent (for throttle) as [0..100].
pub struct Throttle(u8);
