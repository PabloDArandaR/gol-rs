#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        assert!(r <= 1.0 && r >= 0.0);
        assert!(g <= 1.0 && g >= 0.0);
        assert!(b <= 1.0 && b >= 0.0);
        assert!(a <= 1.0 && a >= 0.0);
        Self {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}
