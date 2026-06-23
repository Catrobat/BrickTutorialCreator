use serde_json::Value;

use crate::{
    brick::factory::clone_brick,
    common::{BrickRenderable, Pixmap},
    model::BrickModel,
};

#[derive(Default)]
pub struct Tutorial {
    pub name: String,
    pub content: Vec<Box<dyn BrickRenderable>>,
}

impl PartialEq for Tutorial {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.content.len() == other.content.len()
            && self
                .content
                .iter()
                .zip(other.content.iter())
                .all(|(a, b)| a.get_type() == b.get_type() && a.deref() == b.deref())
    }
}

impl Clone for Tutorial {
    fn clone(&self) -> Self {
        Tutorial {
            name: self.name.clone(),
            content: self
                .content
                .iter()
                .map(|brick| clone_brick(brick.as_ref()))
                .collect(),
        }
    }
}

impl Tutorial {
    pub fn length(&self) -> usize {
        self.content.len()
    }

    pub fn move_brick(&mut self, from_index: usize, to_index: usize) -> Result<(), String> {
        if from_index >= self.content.len() || to_index >= self.content.len() {
            return Err("Index out of range".to_string());
        }
        let brick = self.content.remove(from_index);
        self.content.insert(to_index, brick);
        Ok(())
    }

    pub fn delete(&mut self, index: usize) {
        if index < self.content.len() {
            self.content.remove(index);
        }
    }

    pub fn to_json(&self) -> String {
        let content = self
            .content
            .iter()
            .map(|brick| serde_json::to_string_pretty(brick).unwrap_or_else(|_| "{}".to_string()))
            .collect::<Vec<_>>();
        serde_json::to_string_pretty(&serde_json::json!({
            "name": self.name,
            "content": content,
        }))
        .unwrap_or_else(|_| "{}".to_string())
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let parsed: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("Failed to parse JSON: {}", e))?;
        let name = parsed
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Missing 'name' field".to_string())?
            .to_string();
        let content_array = parsed
            .get("content")
            .and_then(|v| v.as_array())
            .ok_or_else(|| "Missing 'content' field".to_string())?;

        let content: Result<Vec<Box<dyn BrickRenderable>>, String> = content_array
            .iter()
            .map(Value::as_str)
            .map(|item| {
                let item = item.ok_or_else(|| "Content item is not a string".to_string())?;
                serde_json::from_str::<Box<dyn BrickRenderable>>(item)
                    .map_err(|error| error.to_string())
            })
            .collect();

        Ok(Tutorial {
            name,
            content: content?,
        })
    }
}

impl Pixmap for Tutorial {
    fn to_pixmap(&self, target_width: u32) -> Result<tiny_skia::Pixmap, String> {
        if self.content.is_empty() {
            return tiny_skia::Pixmap::new(target_width.max(1), 1)
                .ok_or_else(|| "Failed to create empty tutorial pixmap".to_string());
        }

        let pixmaps: Vec<tiny_skia::Pixmap> = self
            .content
            .iter()
            .map(|brick| brick.to_pixmap(target_width))
            .collect::<Result<Vec<_>, _>>()?;

        let total_height_sum: u32 = pixmaps.iter().map(|pm| pm.height()).sum();
        let overlap = (target_width as f32 * 0.02).ceil() as u32;
        let total_height =
            total_height_sum.saturating_sub(overlap * (pixmaps.len().saturating_sub(1) as u32));

        let mut canvas = tiny_skia::Pixmap::new(target_width, total_height.max(1))
            .ok_or_else(|| format!("Failed to create canvas {}x{}", target_width, total_height))?;

        let mut y_offset: u32 = 0;
        for pm in &pixmaps {
            let pm_data = pm.data();
            let pw = pm.width() as usize;
            let ph = pm.height() as usize;
            let canvas_w = canvas.width() as usize;

            for row in 0..ph {
                let dst_row = y_offset as usize + row;
                if dst_row >= canvas.height() as usize {
                    break;
                }

                let src_row_start = row * pw * 4;
                let dst_row_start = dst_row * canvas_w * 4;
                let canvas_data = canvas.data_mut();

                for col in 0..pw {
                    let source = src_row_start + col * 4;
                    let destination = dst_row_start + col * 4;
                    if source + 3 >= pm_data.len() || destination + 3 >= canvas_data.len() {
                        break;
                    }

                    let src_a = pm_data[source + 3] as u32;
                    let inv_a = 255 - src_a;
                    canvas_data[destination] = (pm_data[source] as u32
                        + canvas_data[destination] as u32 * inv_a / 255)
                        as u8;
                    canvas_data[destination + 1] = (pm_data[source + 1] as u32
                        + canvas_data[destination + 1] as u32 * inv_a / 255)
                        as u8;
                    canvas_data[destination + 2] = (pm_data[source + 2] as u32
                        + canvas_data[destination + 2] as u32 * inv_a / 255)
                        as u8;
                    canvas_data[destination + 3] =
                        (src_a + canvas_data[destination + 3] as u32 * inv_a / 255) as u8;
                }
            }

            y_offset += pm.height().saturating_sub(overlap);
        }

        Ok(canvas)
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct TutorialState {
    pub tutorial: Tutorial,
    pub selected_index: Option<usize>,
}

pub fn tutorial_from_brick_models(states: &[BrickModel], name: &str) -> Tutorial {
    Tutorial {
        name: name.to_string(),
        content: states.iter().map(BrickModel::to_box).collect(),
    }
}

pub fn tutorial_png_bytes(states: &[BrickModel], target_width: u32) -> Result<Vec<u8>, String> {
    let tutorial = tutorial_from_brick_models(states, "Exported tutorial");
    let pixmap = tutorial.to_pixmap(target_width)?;
    pixmap.encode_png().map_err(|e| e.to_string())
}

impl TutorialState {
    pub fn add_brick(&mut self, brick: &BrickModel) {
        self.tutorial.content.push(brick.to_box());
        let len = self.tutorial.content.len();
        self.selected_index = Some(len - 1);
    }

    pub fn insert_after_selected(&mut self, bricks: &[BrickModel]) {
        if bricks.is_empty() {
            return;
        }
        let insert_at = match self.selected_index {
            Some(index) => (index + 1).min(self.tutorial.content.len()),
            None => self.tutorial.content.len(),
        };
        let iter = bricks.iter().map(BrickModel::to_box);
        self.tutorial.content.splice(insert_at..insert_at, iter);
    }

    pub fn get_brick_models(&self) -> Vec<BrickModel> {
        self.tutorial
            .content
            .iter()
            .map(|brick| BrickModel::from_brick(brick.as_ref()))
            .collect()
    }

    pub fn apply_changes(&mut self, brick: &BrickModel) -> Result<(), String> {
        let index = self
            .selected_index
            .ok_or_else(|| "No brick selected".to_string())?;
        if index >= self.tutorial.content.len() {
            return Err("Selected index out of range".to_string());
        }
        self.tutorial.content[index] = brick.to_box();
        Ok(())
    }

    pub fn remove_selected(&mut self) {
        let removed_index = self.selected_index;
        let old_len = self.tutorial.content.len();
        if let Some(index) = self.selected_index.filter(|index| *index < old_len) {
            self.tutorial.delete(index);
        } else {
            self.selected_index = None;
            return;
        }
        let len = self.tutorial.content.len();
        self.selected_index = match (removed_index, len) {
            (Some(_index), 0) => None,
            (Some(index), len) => {
                if index + 1 == old_len {
                    Some(len - 1)
                } else {
                    Some(index)
                }
            }
            _ => None,
        };
    }

    pub fn select(&mut self, index: usize) {
        self.selected_index = (index < self.tutorial.content.len()).then_some(index);
    }

    pub fn deselect(&mut self) {
        self.selected_index = None;
    }

    pub fn move_entry(&mut self, from: usize, to: usize) -> Result<(), String> {
        self.tutorial.move_brick(from, to)?;
        self.selected_index = Some(to);
        Ok(())
    }

    pub fn to_json(&self) -> String {
        self.tutorial.to_json()
    }

    pub fn load_json(&mut self, json: &str) -> Result<(), String> {
        let mut imported = Tutorial::from_json(json)?;
        if imported.content.is_empty() {
            return Ok(());
        }
        let insert_at = match self.selected_index {
            Some(index) => (index + 1).min(self.tutorial.content.len()),
            None => self.tutorial.content.len(),
        };
        self.tutorial
            .content
            .splice(insert_at..insert_at, imported.content.drain(..));
        Ok(())
    }

    pub fn get_png_bytes(&self, target_width: u32) -> Result<Vec<u8>, String> {
        let pixmap = self.tutorial.to_pixmap(target_width)?;
        pixmap.encode_png().map_err(|e| e.to_string())
    }

    pub fn get_png(&self, target_width: u32) -> Result<String, String> {
        use base64::Engine;
        let png_bytes = self.get_png_bytes(target_width)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&png_bytes))
    }
}
