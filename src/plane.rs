use unit::*;

pub struct Plane {
    controls: Controls,
    speed: Speed,
    climb: RateOfClimb,
    altitude: Altitude,
    heading: Heading,
}

pub struct Controls {
    rudder: Deployment,
    elevator: Deployment,
    aileron: Deployment,
    throttle: Throttle,
}
