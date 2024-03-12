use crate::primitive::Ellipse;

pub struct Observation {
    pub ellipse: Ellipse,
    pub confidence_2d: f64,
    pub confidence: f64,
    pub timestamp: f64,
}

trait ObservationStorage {
    fn add(&self, observation: Observation);
}