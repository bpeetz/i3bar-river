use std::collections::HashMap;

use serde::Deserialize;

use crate::color::Color;

use super::{Font, Layer, OutputOverrides, Position, WmConfig};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct OldConfig {
    pub command: Option<String>,

    // font and size
    pub font: Font,
    pub height: u32,
    pub margin_top: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
    pub margin_right: i32,
    pub separator_width: f64,
    pub tags_r: f64,
    pub tags_padding: f64,
    pub tags_margin: f64,
    pub blocks_r: f64,
    pub blocks_overlap: f64,

    // misc
    pub position: Position,
    pub layer: Layer,
    pub invert_touchpad_scrolling: bool,
    pub start_hidden: bool,

    // The toplevel palette
    // colors
    pub background: Color,
    pub color: Color,
    pub separator: Color,
    pub tag_fg: Color,
    pub tag_bg: Color,
    pub tag_focused_fg: Color,
    pub tag_focused_bg: Color,
    pub tag_urgent_fg: Color,
    pub tag_urgent_bg: Color,
    pub tag_inactive_fg: Color,
    pub tag_inactive_bg: Color,

    // Additional shown stuff
    pub hide_inactive_tags: bool,
    pub show_tags: bool,
    pub show_layout_name: bool,
    pub blend: bool,
    pub show_mode: bool,

    // wm-specific
    pub wm: WmConfig,

    // overrides
    pub output: HashMap<String, OutputOverrides>,
}

impl From<super::Config> for OldConfig {
    fn from(base: super::Config) -> Self {
        Self {
            command: base.command,
            font: base.font,
            margin_top: base.margin_top,
            height: base.height,
            margin_bottom: base.margin_bottom,
            margin_left: base.margin_left,
            margin_right: base.margin_right,
            separator_width: base.separator_width,
            tags_r: base.tags_r,
            tags_padding: base.tags_padding,
            tags_margin: base.tags_margin,
            blocks_r: base.blocks_r,
            blocks_overlap: base.blocks_overlap,
            position: base.position,
            layer: base.layer,
            invert_touchpad_scrolling: base.invert_touchpad_scrolling,
            start_hidden: base.start_hidden,
            background: base.theme.focused.background,
            color: base.theme.focused.color,
            separator: base.theme.focused.separator,
            tag_fg: base.theme.focused.tag_fg,
            tag_bg: base.theme.focused.tag_bg,
            tag_focused_fg: base.theme.focused.tag_focused_fg,
            tag_focused_bg: base.theme.focused.tag_focused_bg,
            tag_urgent_fg: base.theme.focused.tag_urgent_fg,
            tag_urgent_bg: base.theme.focused.tag_urgent_bg,
            tag_inactive_fg: base.theme.focused.tag_inactive_fg,
            tag_inactive_bg: base.theme.focused.tag_inactive_bg,
            hide_inactive_tags: base.theme.focused.hide_inactive_tags,
            show_tags: base.theme.focused.show_tags,
            show_layout_name: base.theme.focused.show_layout_name,
            blend: base.theme.focused.blend,
            show_mode: base.theme.focused.show_mode,
            wm: base.wm,
            output: base.output,
        }
    }
}

impl From<OldConfig> for super::Config {
    fn from(base: OldConfig) -> Self {
        let focused = super::Palette {
            background: base.background,
            color: base.color,
            separator: base.separator,
            tag_fg: base.tag_fg,
            tag_bg: base.tag_bg,
            tag_focused_fg: base.tag_focused_fg,
            tag_focused_bg: base.tag_focused_bg,
            tag_urgent_fg: base.tag_urgent_fg,
            tag_urgent_bg: base.tag_urgent_bg,
            tag_inactive_fg: base.tag_inactive_fg,
            tag_inactive_bg: base.tag_inactive_bg,
            hide_inactive_tags: base.hide_inactive_tags,
            show_tags: base.show_tags,
            show_layout_name: base.show_layout_name,
            blend: base.blend,
            show_mode: base.show_mode,
        };

        Self {
            command: base.command,
            theme: super::theme::Theme {
                focused,
                unfocused: focused,
            },
            font: base.font,
            height: base.height,
            margin_top: base.margin_top,
            margin_bottom: base.margin_bottom,
            margin_left: base.margin_left,
            margin_right: base.margin_right,
            separator_width: base.separator_width,
            tags_r: base.tags_r,
            tags_padding: base.tags_padding,
            tags_margin: base.tags_margin,
            blocks_r: base.blocks_r,
            blocks_overlap: base.blocks_overlap,
            position: base.position,
            layer: base.layer,
            invert_touchpad_scrolling: base.invert_touchpad_scrolling,
            start_hidden: base.start_hidden,
            wm: base.wm,
            output: base.output,
        }
    }
}

impl Default for OldConfig {
    fn default() -> Self {
        Self::from(super::Config::default())
    }
}
