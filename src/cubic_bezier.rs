#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct CubicBezier {
    p1: Vec2,
    p2: Vec2,
    lookup_table: Vec<(f32, f32, f32)>, // (t, x, y)
}

impl CubicBezier {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        let num_samples = 1000;
        let mut lookup_table = Vec::with_capacity(num_samples);

        // ルックアップテーブルを生成
        for i in 0..=num_samples {
            let t = i as f32 / num_samples as f32;
            let point = CubicBezier::calculate_bezier(p1, p2, t);
            lookup_table.push((t, point.x, point.y));
        }

        Self {
            p1,
            p2,
            lookup_table,
        }
    }

    pub fn solve_y(&self, x: f32) -> Option<f32> {
        // ルックアップテーブルから最も近いtを初期値として選択
        let mut closest_t = 0.0;
        let mut closest_distance = f32::MAX;

        for &(t, table_x, _) in &self.lookup_table {
            let distance = (table_x - x).abs();
            if distance < closest_distance {
                closest_distance = distance;
                closest_t = t;
            }
        }

        // ニュートン法で収束させる
        let mut t = closest_t;
        let epsilon = 1e-6; // 収束判定のための閾値
        let max_iterations = 100; // ニュートン法の最大試行回数

        for _ in 0..max_iterations {
            let current_x = self.bezier(t).x;
            let derivative = self.bezier_derivative(t).x;
            let delta = current_x - x;

            if delta.abs() < epsilon {
                return Some(self.bezier(t).y);
            }

            t -= delta / derivative;

            if t < 0.0 || t > 1.0 {
                return None;
            }
        }

        None
    }

    // Bezier曲線の座標を計算
    fn bezier(&self, t: f32) -> Vec2 {
        CubicBezier::calculate_bezier(self.p1, self.p2, t)
    }

    // ベジェ曲線の微分
    fn bezier_derivative(&self, t: f32) -> Vec2 {
        let p0 = Vec2 { x: 0.0, y: 0.0 };
        let p3 = Vec2 { x: 1.0, y: 1.0 };
        let one_minus_t = 1.0 - t;

        let x = 3.0 * one_minus_t.powi(2) * (self.p1.x - p0.x)
            + 6.0 * one_minus_t * t * (self.p2.x - self.p1.x)
            + 3.0 * t.powi(2) * (p3.x - self.p2.x);

        let y = 3.0 * one_minus_t.powi(2) * (self.p1.y - p0.y)
            + 6.0 * one_minus_t * t * (self.p2.y - self.p1.y)
            + 3.0 * t.powi(2) * (p3.y - self.p2.y);

        Vec2 { x, y }
    }

    // ベジェ曲線の座標を計算（静的メソッド）
    fn calculate_bezier(p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
        let p0 = Vec2 { x: 0.0, y: 0.0 };
        let p3 = Vec2 { x: 1.0, y: 1.0 };
        let one_minus_t = 1.0 - t;

        let x = one_minus_t.powi(3) * p0.x
            + 3.0 * one_minus_t.powi(2) * t * p1.x
            + 3.0 * one_minus_t * t.powi(2) * p2.x
            + t.powi(3) * p3.x;

        let y = one_minus_t.powi(3) * p0.y
            + 3.0 * one_minus_t.powi(2) * t * p1.y
            + 3.0 * one_minus_t * t.powi(2) * p2.y
            + t.powi(3) * p3.y;

        Vec2 { x, y }
    }
}
