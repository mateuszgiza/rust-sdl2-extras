use std::ops;
use std::time::Duration;
use floating_duration::TimeAsFloat;

#[derive(Default)]
pub struct DeltaTime {
    pub elapsed: Duration
}

impl ops::Mul<DeltaTime> for f32 {
    type Output = f32;

    fn mul(self, rhs: DeltaTime) -> Self {
        self * rhs.elapsed.as_fractional_secs() as f32
    }
}

impl ops::Mul<f32> for DeltaTime {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<DeltaTime> for f32 {
    type Output = f32;

    fn div(self, rhs: DeltaTime) -> Self {
        self / rhs.elapsed.as_fractional_secs() as f32
    }
}

impl ops::Div<f32> for DeltaTime {
    type Output = f32;

    fn div(self, rhs: f32) -> Self::Output {
        self.elapsed.as_fractional_secs() as f32 / rhs
    }
}