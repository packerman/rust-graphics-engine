use na::SVector;

pub struct BoundecBox<const N: usize> {
    min: SVector<f32, N>,
    max: SVector<f32, N>,
}

impl<const N: usize> BoundecBox<N> {
    pub fn new() -> Self {
        Self {
            min: [f32::INFINITY; N].into(),
            max: [f32::NEG_INFINITY; N].into(),
        }
    }

    pub fn update_bounds(&mut self, point: &[f32]) {
        for (i, item) in point.iter().enumerate().take(N) {
            self.min[i] = self.min[i].min(point[i]);
            self.max[i] = self.max[i].max(point[i]);
        }
    }

    pub fn is_empty(&self) -> bool {
        let mut s = 0_u8;
        for i in 0..N {
            s += u8::from(self.min[i] < self.max[i]);
            if s > 1 {
                return true;
            }
        }
        false
    }

    pub fn center_point(&self) -> SVector<f32, N> {
        (self.min + self.max) / 2.0
    }
}
