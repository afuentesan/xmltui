use ratatui::style::Style;

use crate::rtml::util::rtml_style::RTMLStyleTemplate;

pub type TextLine = Vec<( String, Option<Style>, Option<RTMLStyleTemplate> )>;
pub type TextLines = Vec<TextLine>;