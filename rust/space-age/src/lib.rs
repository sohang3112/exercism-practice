const SECONDS_IN_YEAR: u64 = 31557600;  

#[derive(Debug)]
pub struct Duration { time_seconds: u64 }

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { time_seconds: s }
    }
}

pub trait Planet {
    // convert a duration ({:?}) to the number of years on this planet for that duration
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) / 0.2408467
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) / 0.61519726
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.time_seconds as f64 / SECONDS_IN_YEAR as f64
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) / 1.8808158
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) / 11.862615
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) / 29.447498
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) / 84.016846
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) / 164.79132
    }
}
