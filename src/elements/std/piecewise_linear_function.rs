#[derive(Debug)]
pub struct PiecewiseLinearFunction {
    points: Vec<(f32, f32)>,
}

impl PiecewiseLinearFunction {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn add_pair(&mut self, x: f32, y: f32) {
        self.points.push((x, y));
        self.points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }

    pub fn get_value(&self, x: f32) -> f32 {
        // Keine Punkte vorhanden
        if self.points.is_empty() {
            return 0.0;
        }

        // Randfall, kleiner als die vorhandenen Werte
        if x <= self.points.first().unwrap().0 {
            return self.points.first().unwrap().1;
        }

        // Randfall, größer als die vorhandenen Werte
        if x >= self.points.last().unwrap().0 {
            return self.points.last().unwrap().1;
        }

        for i in 0..self.points.len() - 1 {
            let (x0, y0) = self.points[i];
            let (x1, y1) = self.points[i + 1];
            if x0 <= x && x <= x1 {
                if x == x0 {
                    return y0;
                }
                if x == x1 {
                    return y1;
                }
                // Lineare Interpolation
                return y0 + (x - x0) * (y1 - y0) / (x1 - x0);
            }
        }
        0.0
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
