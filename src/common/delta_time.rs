use std::ops;
use std::time::Duration;
use floating_duration::TimeAsFloat;

#[derive(Default)]
pub struct DeltaTime {
    pub elapsed: Duration
}

impl<'a> ops::Mul<&'a DeltaTime> for f32 {
    type Output = f32;

    fn mul(self, rhs: &DeltaTime) -> Self {
        self * rhs.elapsed.as_fractional_secs() as f32
    }
}

impl<'a> ops::Mul<f32> for &'a DeltaTime {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl<'a> ops::Div<&'a DeltaTime> for f32 {
    type Output = f32;

    fn div(self, rhs: &DeltaTime) -> Self {
        self / rhs.elapsed.as_fractional_secs() as f32
    }
}

impl<'a> ops::Div<f32> for &'a DeltaTime {
    type Output = f32;

    fn div(self, rhs: f32) -> Self::Output {
        self.elapsed.as_fractional_secs() as f32 / rhs
    }
}