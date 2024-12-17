use serde::Serialize;

use crate::model::HexStr;

#[derive(Serialize)]
pub(super) struct TextMateRule {
    scope: Vec<String>,
    settings: TextMateRuleSettings,
}

#[derive(Serialize)]
pub(super) struct TextMateRuleSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontStyle")]
    font_style: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    foreground: Option<HexStr>,
}

impl TextMateRule {
    pub(super) fn new(
        scope: &[&str],
        font_style: Option<&str>,
        foreground: Option<HexStr>,
    ) -> Self {
        let font_style = font_style.map(|font_style| font_style.to_string());
        Self {
            scope: scope.iter().map(|v| v.to_string()).collect(),
            settings: TextMateRuleSettings {
                font_style,
                foreground,
            },
        }
    }
}
