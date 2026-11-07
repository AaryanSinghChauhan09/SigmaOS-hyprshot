//! hyprshot model — Hyprland screenshot tool.

#[derive(Debug, Clone)]
pub struct Model {
    pub output: Option<String>,
    pub format: String,
    pub region: Option<(i32, i32, u32, u32)>,
}

impl Default for Model {
    fn default() -> Self { Self { output: None, format: "png".into(), region: None } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn capture(&self) -> String {
        let name = self.output.clone().unwrap_or_else(|| "screenshot".into());
        format!("{}.{}", name, self.format)
    }

    pub fn capture_region(&self, x: i32, y: i32, w: u32, h: u32) -> String {
        format!("{}-{}x{}-{}-{}.{}", self.output.clone().unwrap_or_else(|| "region".into()), w, h, x, y, self.format)
    }

    pub fn window(&self, class: &str) -> String {
        format!("{}.{}", class, self.format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capture() {
        let mut h = Model::new();
        h.output = Some("shot".into());
        assert_eq!(h.capture(), "shot.png");
        assert!(h.capture_region(0, 0, 100, 100).contains("100x100"));
        assert_eq!(h.window("firefox"), "firefox.png");
    }
}
