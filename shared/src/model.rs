use crate::brick::factory::{brick_from_base, clone_brick, default_brick, default_y_offset};
use crate::color::ColorScheme;
use crate::common::BrickRenderable;
use crate::types::BrickType;

pub struct BrickModel {
    brick: Box<dyn BrickRenderable>,
}

impl Clone for BrickModel {
    fn clone(&self) -> Self {
        Self {
            brick: clone_brick(self.as_brick()),
        }
    }
}

impl PartialEq for BrickModel {
    fn eq(&self, other: &Self) -> bool {
        self.get_type() == other.get_type() && self.as_brick().deref() == other.as_brick().deref()
    }
}

impl Default for BrickModel {
    fn default() -> Self {
        Self {
            brick: default_brick(BrickType::H1Base),
        }
    }
}

impl std::fmt::Display for BrickModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let brick: Box<&dyn BrickRenderable> = Box::new(self.as_brick());
        let result = serde_json::to_string_pretty(&brick).map_err(|_| std::fmt::Error)?;
        write!(f, "{result}")
    }
}

impl BrickModel {
    pub fn from_brick(brick: &dyn BrickRenderable) -> Self {
        Self {
            brick: clone_brick(brick),
        }
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let brick =
            serde_json::from_str::<Box<dyn BrickRenderable>>(json).map_err(|e| e.to_string())?;
        Ok(Self { brick })
    }

    pub fn get_type(&self) -> BrickType {
        self.brick.get_type()
    }

    pub fn change_type(&mut self, brick_type: BrickType) {
        let mut base = self.as_brick().deref().clone();
        base.offset.1 = default_y_offset(brick_type);
        self.brick = brick_from_base(brick_type, base);
    }

    pub fn change_color(&mut self, color: ColorScheme) {
        self.as_mut_brick().color_scheme = color;
    }

    pub fn change_offset(&mut self, x: f32, y: f32) {
        self.as_mut_brick().offset = (x, y);
    }

    pub fn change_content(&mut self, content: String) {
        self.as_mut_brick().content = content;
    }

    pub fn as_brick(&self) -> &dyn BrickRenderable {
        self.brick.as_ref()
    }

    pub fn as_mut_brick(&mut self) -> &mut dyn BrickRenderable {
        self.brick.as_mut()
    }

    pub fn to_box(&self) -> Box<dyn BrickRenderable> {
        clone_brick(self.as_brick())
    }

    pub fn get_svg(&self) -> String {
        self.as_brick().to_svg()
    }

    pub fn get_png(&self, target_width: u32) -> Result<Vec<u8>, String> {
        self.as_brick()
            .to_pixmap(target_width)?
            .encode_png()
            .map_err(|error| error.to_string())
    }
}
