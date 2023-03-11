use num_traits::{Float, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2d<F>
where
    F: Float + Zero,
{
    x: F,
    y: F,
    theta: F,
}

impl<F> Point2d<F>
where
    F: Float + Zero,
{
    pub fn norm(&self) -> F {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<F> std::ops::Add for Point2d<F>
where
    F: Float + Zero,
{
    type Output = Point2d<F>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x * self.theta.cos(),
            y: self.y + rhs.y * self.theta.sin(),
            theta: self.theta + rhs.theta,
        }
    }
}
