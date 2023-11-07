use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy)]
pub struct Colour {
    pub a: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub fn is_transparent(&self) -> bool {
        self.a == 0.0
    }

    pub fn rgba(&self) -> String {
        let red = self.r * 255.0;
        let green = self.g * 255.0;
        let blue = self.b * 255.0;
        let alpha = self.a;
        return format!("rgba({red},{green},{blue},{alpha})");
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ColourStop {
    pub position: f32,
    pub color: Colour,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_transparent() {
        assert!(Colour { a: 0.0, r: 0.0, g: 0.0, b: 0.0 }.is_transparent());
        assert!(!Colour { a: 1.0, r: 0.0, g: 0.0, b: 0.0 }.is_transparent());
    }

    #[test]
    fn rgba() {
        assert_eq!(Colour { a: 0.0, r: 0.0, g: 0.0, b: 0.0 }.rgba(), "rgba(0,0,0,0)");
        assert_eq!(Colour { a: 0.5, r: 0.1, g: 0.2, b: 0.3 }.rgba(), "rgba(25.5,51,76.5,0.5)");
    }
}