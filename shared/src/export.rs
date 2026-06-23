use crate::brick::{
    h1_base::BrickH1Base, h1_control::BrickH1Control, h2_base::BrickH2Base,
    h2_control::BrickH2Control, h3_base::BrickH3Base,
};
use crate::color::ALL_COLOR_SCHEMES;
use crate::common::{BrickRenderable, SVGRenderable};
use crate::tutorial::Tutorial;

pub fn render_bricks_zip_bytes(
    bricks: &[(&'static str, &'static str)],
    tile_width: u32,
) -> Result<Vec<u8>, String> {
    use std::io::{Cursor, Write};

    use zip::CompressionMethod;
    use zip::write::{FileOptions, ZipWriter};

    let tile_width = tile_width.max(1);
    let mut zip = ZipWriter::new(Cursor::new(Vec::<u8>::new()));
    let opts = FileOptions::<()>::default().compression_method(CompressionMethod::Stored);

    for (path, json) in bricks {
        let brick = serde_json::from_str::<Box<dyn BrickRenderable>>(json)
            .map_err(|e| format!("{path}: {e}"))?;
        let png = brick
            .to_pixmap(tile_width)
            .map_err(|e| format!("{path}: {e}"))?
            .encode_png()
            .map_err(|e| e.to_string())?;

        let mut name = (*path).to_string();
        if let Some(stripped) = name.strip_suffix(".json") {
            name = stripped.to_string();
        }
        name.push_str(".png");

        zip.start_file(name, opts).map_err(|e| e.to_string())?;
        zip.write_all(&png).map_err(|e| e.to_string())?;
    }

    zip.finish()
        .map_err(|e| e.to_string())
        .map(|c| c.into_inner())
}

pub fn tutorial_svg(tutorial: &Tutorial) -> Result<String, String> {
    let mut content = String::new();
    let mut y_offset = 0_u32;
    let mut max_width = 0_u32;

    for brick in &tutorial.content {
        let svg = brick.to_svg();
        let (width, height) = parse_svg_viewbox_dimensions(&svg).unwrap_or((0, 0));
        max_width = max_width.max(width);
        content.push_str(&position_svg_root(svg, y_offset));
        y_offset += height;
    }

    let width = max_width.max(1);
    let height = y_offset.max(1);
    Ok(format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" width=\"{width}\" height=\"{height}\">{content}</svg>",
    ))
}

pub fn render_ninepatch_zip_bytes() -> Result<Vec<u8>, String> {
    use std::io::{Cursor, Write};

    use zip::CompressionMethod;
    use zip::write::{FileOptions, ZipWriter};

    let mut zip = ZipWriter::new(Cursor::new(Vec::<u8>::new()));
    let opts = FileOptions::<()>::default().compression_method(CompressionMethod::Stored);

    for density in DENSITIES {
        for scheme in ALL_COLOR_SCHEMES.iter() {
            for kind in BRICK_KINDS {
                let size = density.size_for(*kind);
                let svg = kind.svg_with_color(scheme.clone());

                let mut pixmap = render_svg_with_padding(&svg, size.width, size.height, 1)?;
                apply_ninepatch_markers(&mut pixmap, size.width, size.height);

                let png = pixmap.encode_png().map_err(|e| e.to_string())?;
                let file_name = format!(
                    "drawable-{}/{}_{}.9.png",
                    density.name,
                    kind.file_stem(),
                    slugify(&scheme.name)
                );

                zip.start_file(file_name, opts).map_err(|e| e.to_string())?;
                zip.write_all(&png).map_err(|e| e.to_string())?;
            }
        }
    }

    zip.finish()
        .map_err(|e| e.to_string())
        .map(|c| c.into_inner())
}

fn parse_svg_viewbox_dimensions(svg: &str) -> Option<(u32, u32)> {
    let marker = "viewBox=\"0 0 ";
    let start = svg.find(marker)? + marker.len();
    let end = svg[start..].find('"')?;
    let mut parts = svg[start..start + end].split_whitespace();
    let width = parts.next()?.parse::<f32>().ok()?;
    let height = parts.next()?.parse::<f32>().ok()?;
    Some((width.ceil() as u32, height.ceil() as u32))
}

fn position_svg_root(mut svg: String, y_offset: u32) -> String {
    if let Some(open_end) = svg.find('>') {
        svg.insert_str(open_end, &format!(" x=\"0\" y=\"{}\"", y_offset));
    }
    svg
}

#[derive(Clone, Copy)]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Clone, Copy)]
struct DensitySpec {
    name: &'static str,
    h1_base: Size,
    h2_base: Size,
    h3_base: Size,
    h1_control: Size,
    h2_control: Size,
}

impl DensitySpec {
    fn size_for(&self, kind: BrickKind) -> Size {
        match kind {
            BrickKind::H1Base => self.h1_base,
            BrickKind::H2Base => self.h2_base,
            BrickKind::H3Base => self.h3_base,
            BrickKind::H1Control => self.h1_control,
            BrickKind::H2Control => self.h2_control,
        }
    }
}

const DENSITIES: &[DensitySpec] = &[
    DensitySpec {
        name: "ldpi",
        h1_base: Size {
            width: 43,
            height: 35,
        },
        h2_base: Size {
            width: 43,
            height: 55,
        },
        h3_base: Size {
            width: 43,
            height: 73,
        },
        h1_control: Size {
            width: 156,
            height: 49,
        },
        h2_control: Size {
            width: 156,
            height: 68,
        },
    },
    DensitySpec {
        name: "mdpi",
        h1_base: Size {
            width: 56,
            height: 46,
        },
        h2_base: Size {
            width: 56,
            height: 73,
        },
        h3_base: Size {
            width: 56,
            height: 96,
        },
        h1_control: Size {
            width: 208,
            height: 64,
        },
        h2_control: Size {
            width: 208,
            height: 90,
        },
    },
    DensitySpec {
        name: "hdpi",
        h1_base: Size {
            width: 83,
            height: 68,
        },
        h2_base: Size {
            width: 83,
            height: 109,
        },
        h3_base: Size {
            width: 83,
            height: 143,
        },
        h1_control: Size {
            width: 311,
            height: 95,
        },
        h2_control: Size {
            width: 311,
            height: 134,
        },
    },
    DensitySpec {
        name: "xhdpi",
        h1_base: Size {
            width: 111,
            height: 90,
        },
        h2_base: Size {
            width: 111,
            height: 144,
        },
        h3_base: Size {
            width: 111,
            height: 190,
        },
        h1_control: Size {
            width: 414,
            height: 156,
        },
        h2_control: Size {
            width: 413,
            height: 208,
        },
    },
    DensitySpec {
        name: "xxhdpi",
        h1_base: Size {
            width: 164,
            height: 134,
        },
        h2_base: Size {
            width: 164,
            height: 216,
        },
        h3_base: Size {
            width: 164,
            height: 284,
        },
        h1_control: Size {
            width: 620,
            height: 188,
        },
        h2_control: Size {
            width: 620,
            height: 266,
        },
    },
];

#[derive(Clone, Copy)]
enum BrickKind {
    H1Base,
    H2Base,
    H3Base,
    H1Control,
    H2Control,
}

const BRICK_KINDS: &[BrickKind] = &[
    BrickKind::H1Base,
    BrickKind::H2Base,
    BrickKind::H3Base,
    BrickKind::H1Control,
    BrickKind::H2Control,
];

impl BrickKind {
    fn file_stem(self) -> &'static str {
        match self {
            BrickKind::H1Base => "h1_base",
            BrickKind::H2Base => "h2_base",
            BrickKind::H3Base => "h3_base",
            BrickKind::H1Control => "h1_control",
            BrickKind::H2Control => "h2_control",
        }
    }

    fn svg_with_color(self, scheme: crate::color::ColorScheme) -> String {
        match self {
            BrickKind::H1Base => {
                let mut brick = BrickH1Base::default();
                brick.base.color_scheme = scheme;
                brick.to_svg()
            }
            BrickKind::H2Base => {
                let mut brick = BrickH2Base::default();
                brick.base.color_scheme = scheme;
                brick.to_svg()
            }
            BrickKind::H3Base => {
                let mut brick = BrickH3Base::default();
                brick.base.color_scheme = scheme;
                brick.to_svg()
            }
            BrickKind::H1Control => {
                let mut brick = BrickH1Control::default();
                brick.base.color_scheme = scheme;
                brick.to_svg()
            }
            BrickKind::H2Control => {
                let mut brick = BrickH2Control::default();
                brick.base.color_scheme = scheme;
                brick.to_svg()
            }
        }
    }
}

fn render_svg_with_padding(
    svg_string: &str,
    target_width: u32,
    target_height: u32,
    padding: u32,
) -> Result<tiny_skia::Pixmap, String> {
    let tree = usvg::Tree::from_str(svg_string, &crate::common::OPTIONS)
        .map_err(|e| format!("Failed to parse SVG: {e:?}"))?;

    let svg_size = tree.size();
    let svg_width = svg_size.width();
    let svg_height = svg_size.height();
    if svg_width == 0.0 || svg_height == 0.0 {
        return Err("SVG has zero width or height".to_string());
    }

    let pad2 = padding.saturating_mul(2);
    if target_width <= pad2 || target_height <= pad2 {
        return Err(format!(
            "Target too small for padding: {target_width}x{target_height} with padding={padding}"
        ));
    }
    let inner_h = (target_height - pad2) as f32;

    let mut pixmap = tiny_skia::Pixmap::new(target_width, target_height)
        .ok_or_else(|| format!("Failed to create pixmap with {target_width}x{target_height}"))?;

    let scale = inner_h / svg_height;
    let transform = tiny_skia::Transform::from_scale(scale, scale)
        .post_translate(padding as f32, padding as f32);

    let mut pm = pixmap.as_mut();
    resvg::render(&tree, transform, &mut pm);
    Ok(pixmap)
}

fn apply_ninepatch_markers(pixmap: &mut tiny_skia::Pixmap, width: u32, height: u32) {
    if width < 3 || height < 3 {
        return;
    }

    let black = tiny_skia::PremultipliedColorU8::from_rgba(0, 0, 0, 255).unwrap();
    let pixels = pixmap.pixels_mut();
    let w = width as usize;

    let set = |pixels: &mut [tiny_skia::PremultipliedColorU8], x: u32, y: u32| {
        let idx = y as usize * w + x as usize;
        if idx < pixels.len() {
            pixels[idx] = black;
        }
    };

    for x in 1..(width - 1) {
        set(pixels, x, 0);
        set(pixels, x, height - 1);
    }
    for y in 1..(height - 1) {
        set(pixels, 0, y);
        set(pixels, width - 1, y);
    }
}

fn slugify(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if (ch.is_whitespace() || ch == '-' || ch == '_') && !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_normalizes_file_stems() {
        assert_eq!(slugify("Light Orange"), "light_orange");
        assert_eq!(slugify("Wine-Red"), "wine_red");
    }

    #[test]
    fn viewbox_dimensions_parse_simple_svg() {
        assert_eq!(
            parse_svg_viewbox_dimensions("<svg viewBox=\"0 0 10.2 20.8\"></svg>"),
            Some((11, 21))
        );
    }
}
