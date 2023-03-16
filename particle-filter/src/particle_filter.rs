use num_traits::{Float, Zero};

pub trait PartcleFilter<F>
where
    F: Float + Zero,
{
    fn resampling(&self, weights: &[f64]);
    fn set_likelihood(&mut self, observed: &[F]);
    fn get_filtered(&self) -> F;
}
