use serde::Serialize;

use super::text_mate_rule::TextMateRule;
use crate::model::{Color, ColorMap};

#[derive(Serialize)]
pub(super) struct EditorTokenColorCustomizations {
    #[serde(rename = "textMateRules")]
    text_mate_rules: Vec<TextMateRule>,
}

impl EditorTokenColorCustomizations {
    pub(super) fn new(color_map: &ColorMap<super::HexColors>) -> Self {
        Self {
            text_mate_rules: vec![
                TextMateRule::new(
                    &[
                        "comment",
                        "punctuation.definition.comment",
                        "string.comment",
                    ],
                    None,
                    Some(color_map[Color::Gray][0]),
                ),
                TextMateRule::new(
                    &[
                        "constant",
                        "entity.name.constant",
                        "variable.other.constant",
                        "variable.other.enummember",
                        "variable.language",
                    ],
                    None,
                    Some(color_map[Color::Blue][3]),
                ),
                TextMateRule::new(
                    &["entity", "entity.name"],
                    None,
                    Some(color_map[Color::Purple][3]),
                ),
                TextMateRule::new(
                    &["variable.parameter.function"],
                    None,
                    Some(color_map[Color::Gray][2]),
                ),
                TextMateRule::new(&["entity.name.tag"], None, Some(color_map[Color::Green][3])),
                TextMateRule::new(&["keyword"], None, Some(color_map[Color::Red][3])),
                TextMateRule::new(
                    &["storage", "storage.type"],
                    None,
                    Some(color_map[Color::Red][3]),
                ),
                TextMateRule::new(
                    &[
                        "storage.modifier.package",
                        "storage.modifier.import",
                        "storage.type.java",
                    ],
                    None,
                    Some(color_map[Color::Gray][2]),
                ),
                TextMateRule::new(
                    &[
                        "string",
                        "punctuation.definition.string",
                        "string punctuation.section.embedded source",
                    ],
                    None,
                    Some(color_map[Color::Blue][4]),
                ),
                TextMateRule::new(&["support"], None, Some(color_map[Color::Blue][3])),
                TextMateRule::new(
                    &["meta.property-name"],
                    None,
                    Some(color_map[Color::Blue][3]),
                ),
                TextMateRule::new(&["variable"], None, Some(color_map[Color::Orange][3])),
                TextMateRule::new(&["variable.other"], None, Some(color_map[Color::Gray][2])),
                TextMateRule::new(&["invalid.broken"], None, Some(color_map[Color::Red][3])),
                TextMateRule::new(
                    &["invalid.deprecated"],
                    None,
                    Some(color_map[Color::Red][3]),
                ),
                TextMateRule::new(&["invalid.illegal"], None, Some(color_map[Color::Red][3])),
                TextMateRule::new(
                    &["invalid.unimplemented"],
                    None,
                    Some(color_map[Color::Red][3]),
                ),
                TextMateRule::new(
                    &["carriage-return"],
                    Some("italic underline"),
                    Some(color_map[Color::Gray][0]),
                ),
                TextMateRule::new(&["message.error"], None, Some(color_map[Color::Red][3])),
                TextMateRule::new(&["string variable"], None, Some(color_map[Color::Blue][3])),
                TextMateRule::new(
                    &["source.regexp", "string.regexp"],
                    None,
                    Some(color_map[Color::Blue][4]),
                ),
                TextMateRule::new(
                    &[
                        "string.regexp.character-class",
                        "string.regexp constant.character.escape",
                        "string.regexp source.ruby.embedded",
                        "string.regexp string.regexp.arbitrary-repitition",
                    ],
                    None,
                    Some(color_map[Color::Blue][4]),
                ),
                TextMateRule::new(
                    &["string.regexp constant.character.escape"],
                    None,
                    Some(color_map[Color::Green][3]),
                ),
                TextMateRule::new(&["support.constant"], None, Some(color_map[Color::Blue][3])),
                TextMateRule::new(&["support.variable"], None, Some(color_map[Color::Blue][3])),
                TextMateRule::new(
                    &["punctuation.definition.list.begin.markdown"],
                    None,
                    Some(color_map[Color::Orange][3]),
                ),
                TextMateRule::new(
                    &["markup.heading", "markup.heading entity.name"],
                    Some("bold"),
                    Some(color_map[Color::Blue][3]),
                ),
                TextMateRule::new(&["markup.quote"], None, Some(color_map[Color::Green][3])),
                TextMateRule::new(
                    &["markup.italic"],
                    Some("italic"),
                    Some(color_map[Color::Gray][2]),
                ),
                TextMateRule::new(
                    &["markup.bold"],
                    Some("bold"),
                    Some(color_map[Color::Gray][2]),
                ),
                TextMateRule::new(&["markup.underline"], Some("underline"), None),
                TextMateRule::new(&["markup.strikethrough"], Some("strikethrough"), None),
                TextMateRule::new(
                    &["markup.inline.raw"],
                    None,
                    Some(color_map[Color::Blue][3]),
                ),
                TextMateRule::new(
                    &[
                        "markup.deleted",
                        "meta.diff.header.from-file",
                        "punctuation.definition.deleted",
                    ],
                    None,
                    Some(color_map[Color::Red][3]),
                ),
                TextMateRule::new(
                    &[
                        "markup.inserted",
                        "meta.diff.header.to-file",
                        "punctuation.definition.inserted",
                    ],
                    None,
                    Some(color_map[Color::Green][3]),
                ),
                TextMateRule::new(
                    &["markup.changed", "punctuation.definition.changed"],
                    None,
                    Some(color_map[Color::Orange][3]),
                ),
                TextMateRule::new(
                    &["markup.ignored", "markup.untracked"],
                    None,
                    Some(color_map[Color::Gray][0]),
                ),
                TextMateRule::new(
                    &["meta.diff.range"],
                    Some("bold"),
                    Some(color_map[Color::Purple][3]),
                ),
                TextMateRule::new(&["meta.diff.header"], None, Some(color_map[Color::Blue][3])),
                TextMateRule::new(
                    &["meta.separator"],
                    Some("bold"),
                    Some(color_map[Color::Blue][3]),
                ),
                TextMateRule::new(&["meta.output"], None, Some(color_map[Color::Blue][3])),
                TextMateRule::new(
                    &[
                        "brackethighlighter.tag",
                        "brackethighlighter.curly",
                        "brackethighlighter.round",
                        "brackethighlighter.square",
                        "brackethighlighter.angle",
                        "brackethighlighter.quote",
                    ],
                    None,
                    Some(color_map[Color::Gray][3]),
                ),
                TextMateRule::new(
                    &["brackethighlighter.unmatched"],
                    None,
                    Some(color_map[Color::Red][3]),
                ),
                TextMateRule::new(
                    &["constant.other.reference.link", "string.other.link"],
                    Some("underline"),
                    Some(color_map[Color::Blue][4]),
                ),
            ],
        }
    }
}
