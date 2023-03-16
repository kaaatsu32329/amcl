use crate::particle_filter::PartcleFilter;

use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct SinglePoint {
    y: f64,
    particle: usize,
    sigma: f64,
    alpha: f64,
    likelihood: Vec<f64>,
    resampled: Vec<f64>,
    filtered: f64,
}

impl PartcleFilter<f64> for SinglePoint {
    fn resampling(&mut self, weights: &[f64]) {
        if weights.len() == self.particle {
            self.resampled.clear();

            let mut rng = rand::thread_rng();
            let sum_weight = weights.iter().sum::<f64>();
            let sample_width = sum_weight / self.particle as f64;
            let start_point = rng.gen_range(0f64..sample_width);

            for i in 0..self.particle {
            }
        }
    }

    fn set_likelihood(&mut self, observed: &[f64]) {
        self.likelihood.clear();

        for x in observed {
            self.likelihood.push(
                (-(self.y - *x).powi(2) / (2. * self.sigma.powi(2)))
                    / (2. * std::f64::consts::PI * self.sigma.powi(2)).sqrt(),
            );
        }
    }

    fn get_filtered(&self) -> f64 {
        todo!()
    }
}
