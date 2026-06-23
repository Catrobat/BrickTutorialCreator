use std::collections::BTreeMap;

use crate::interfaces::brick::BrickState;

#[derive(Clone, PartialEq)]
pub struct CatalogEntry {
    pub path: &'static str,
    pub label: String,
    pub brick: Option<BrickState>,
}

#[derive(Clone, PartialEq)]
pub struct CatalogGroup {
    pub name: String,
    pub entries: Vec<CatalogEntry>,
}

pub fn render_all_bricks_zip_bytes(tile_width: u32) -> Result<Vec<u8>, String> {
    shared::export::render_bricks_zip_bytes(crate::generated::brick_catalog::BRICKS, tile_width)
}

pub fn catalog_groups() -> Vec<CatalogGroup> {
    let mut groups: BTreeMap<String, Vec<CatalogEntry>> = BTreeMap::new();
    for (path, json) in crate::generated::brick_catalog::BRICKS {
        let key = group_key_from_path(path).to_string();
        let brick = match BrickState::from_json(json) {
            Ok(brick) => Some(brick),
            Err(e) => {
                web_sys::console::error_1(&format!("{path}: {e}").into());
                None
            }
        };
        groups.entry(key).or_default().push(CatalogEntry {
            path,
            label: label_from_path(path),
            brick,
        });
    }

    groups
        .into_iter()
        .map(|(name, mut entries)| {
            entries.sort_by(|a, b| a.path.cmp(b.path));
            CatalogGroup { name, entries }
        })
        .collect()
}

fn group_key_from_path(path: &str) -> &str {
    path.split('/').next().unwrap_or("(root)")
}

fn label_from_path(path: &str) -> String {
    let file = path.rsplit_once('/').map(|(_, f)| f).unwrap_or(path);
    file.strip_suffix(".json").unwrap_or(file).replace('_', " ")
}
