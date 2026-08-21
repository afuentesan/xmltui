use std::str::FromStr;

use ratatui::style::{Color, Style};

fn default_bg() -> Color
{
    Color::from_str( "#000000" ).unwrap()
}

fn default_bg_focus() -> Color
{
    Color::from_str( "#333333" ).unwrap()
}

fn default_fg() -> Color
{
    Color::from_str( "#ffffff" ).unwrap()
}

pub fn default_normal_style() -> Style
{
    Style::default().fg( default_fg() ).bg( default_bg() )
}

pub fn default_focus_style() -> Style
{
    Style::default().fg( default_fg() ).bg( default_bg_focus() )
}

pub fn default_link_normal_style() -> Style
{
    Style::default().fg( default_fg() ).bg( default_bg() ).underlined()
}

pub fn default_link_focus_style() -> Style
{
    Style::default().fg( default_fg() ).bg( default_bg_focus() )
}