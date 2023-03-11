use crate::particle_filter::PartcleFilter;

#[derive(Debug, Clone, PartialEq)]
pub struct SinglePoint {
    y: f64,
    particle: usize,
    sigma: f64,
    alpha: f64,
    likelihood: Vec<f64>,
    filtered: f64,
}

impl PartcleFilter<f64> for SinglePoint {
    fn resampling(&self) {
        todo!()
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
