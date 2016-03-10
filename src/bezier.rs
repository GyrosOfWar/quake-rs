use vector::Vec2;

pub struct BezierCurve {
    start: Vec2,
    control: Vec2,
    end: Vec2,
}

impl BezierCurve {
    pub fn new(s: Vec2, c: Vec2, e: Vec2) -> BezierCurve {
        BezierCurve {
            start: s,
            control: c,
            end: e,
        }
    }

    pub fn evaluate(&self, t: f32) -> Vec2 {
        let s = 1.0 - t;
        let c0 = s * s;
        let c1 = 2.0 * s * t;
        let c2 = t * t;

        self.start * c0 + self.control * c1 + self.end * c2
    }

    pub fn approximate(&self, num_points: usize) -> Vec<Vec2> {
        let mut t = 0.0;
        let step = 1.0 / num_points as f32;
        let mut pts = vec![];

        while t <= 1.0 {
            pts.push(self.evaluate(t));
            t += step;
        }

        pts
    }
}
