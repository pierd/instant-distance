use super::metric::Metric;
use super::simple::{CosineMetric, DotProductMetric, EuclidMetric};

pub type VectorElementType = f32;



/// Sequential number of modification, applied to segment
pub type ScoreType = f32;

/// Type of internal tags, build from payload
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
/// Distance function types used to compare vectors
pub enum Distance {
    // <https://en.wikipedia.org/wiki/Cosine_similarity>
    Cosine,
    // <https://en.wikipedia.org/wiki/Euclidean_distance>
    Euclid,
    // <https://en.wikipedia.org/wiki/Dot_product>
    Dot,
}

impl Distance {
    pub fn preprocess_vector(
        &self,
        vector: &[VectorElementType],
    ) -> Option<Vec<VectorElementType>> {
        match self {
            Distance::Cosine => CosineMetric::preprocess(vector),
            Distance::Euclid => EuclidMetric::preprocess(vector),
            Distance::Dot => DotProductMetric::preprocess(vector),
        }
    }

    pub fn postprocess_score(&self, score: ScoreType) -> ScoreType {
        match self {
            Distance::Cosine => CosineMetric::postprocess(score),
            Distance::Euclid => EuclidMetric::postprocess(score),
            Distance::Dot => DotProductMetric::postprocess(score),
        }
    }

    pub fn distance_order(&self) -> Order {
        match self {
            Distance::Cosine | Distance::Dot => Order::LargeBetter,
            Distance::Euclid => Order::SmallBetter,
        }
    }

    /// Checks if score satisfies threshold condition
    pub fn check_threshold(&self, score: ScoreType, threshold: ScoreType) -> bool {
        match self.distance_order() {
            Order::LargeBetter => score > threshold,
            Order::SmallBetter => score < threshold,
        }
    }
}

pub enum Order {
    LargeBetter,
    SmallBetter,
}
