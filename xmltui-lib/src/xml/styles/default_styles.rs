use ratatui::style::{Modifier, Style};

pub fn default_normal_style() -> Style
{
    Style::default()
}

pub fn default_focus_style() -> Style
{
    Style::default().add_modifier( Modifier::REVERSED ).bold()
}

pub fn default_link_normal_style() -> Style
{
    Style::default().underlined()
}

pub fn default_link_focus_style() -> Style
{
    default_focus_style()
}