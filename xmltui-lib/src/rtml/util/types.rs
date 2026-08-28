use ratatui::style::Style;

pub type TextLine = Vec<( String, Option<Style> )>;
pub type TextLines = Vec<TextLine>;