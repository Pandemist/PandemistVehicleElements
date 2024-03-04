#[derive(Default, Debug)]
pub struct PiecewiseLinearFunction {
    points: Vec<(f32, f32)>,
}

impl PiecewiseLinearFunction {
    pub fn new() -> Self {
        PiecewiseLinearFunction { points: Vec::new() }
    }

    pub fn add_pair(&mut self, x: f32, y: f32) {
        self.points.push((x, y));
        self.points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }

    pub fn get_value(&self, x: f32) -> f32 {
        if self.points.len() < 2 {
            return 0.0;
        }

        let index = match self
            .points
            .binary_search_by(|&(point_x, _)| point_x.partial_cmp(&x).unwrap())
        {
            Ok(index) => index,
            Err(index) => index,
        };

        match index {
            0 => self.points[0].1,
            len if len == self.points.len() => self.points[len - 1].1,
            _ => {
                let (x1, y1) = self.points[index - 1];
                let (x2, y2) = self.points[index];
                // Interpolation zwischen zwei Punkten
                let delta_x = x2 - x1;
                if delta_x != 0.0 {
                    let weight1 = (x2 - x) / delta_x;
                    let weight2 = 1.0 - weight1;
                    y1 * weight2 + y2 * weight1
                } else {
                    y1 // Wenn delta_x gleich 0 ist, kein Bedarf für Interpolation
                }
            }
        }
    }
}

#[test]
fn test_piecewise_linear_function() {
    let mut piecewise_linear = PiecewiseLinearFunction::new();

    // Punkte hinzufügen
    piecewise_linear.add_pair(0.0, 1.0);
    piecewise_linear.add_pair(2.0, 3.0);
    piecewise_linear.add_pair(5.0, 2.0);

    // Werte überprüfen
    assert_eq!(piecewise_linear.get_value(0.0), 1.0);
    assert_eq!(piecewise_linear.get_value(1.0), 2.0);
    assert_eq!(piecewise_linear.get_value(4.0), 2.5);
    assert_eq!(piecewise_linear.get_value(6.0), 2.0);
}
