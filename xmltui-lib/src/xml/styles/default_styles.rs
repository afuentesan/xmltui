use ratatui::style::{Color, Style};

pub fn default_normal_style() -> Style
{
    Style::default()
}

pub fn default_focus_style( not_focus : &Style ) -> Style
{
    match not_focus.bg.as_ref()
    {
        Some( bg ) =>
        {
            if let Some( new_bg ) = adjust_background( *bg )
            {
                not_focus.bg( new_bg ).bold()
            }
            else
            {
                not_focus.clone().bold()
            }
        },
        None => not_focus.clone().bold()
    }
}

fn adjust_background( color: Color ) -> Option<Color>
{
    let (r, g, b) = match color
    {
        Color::Rgb(r, g, b) => ( r, g, b ),
        
        Color::Red => ( 170, 0, 0 ),
        Color::Green => ( 0, 170, 0 ),
        Color::Yellow => ( 170, 85, 0 ),
        Color::Blue => ( 0, 0, 170 ),
        Color::Magenta => ( 170, 0, 170 ),
        Color::Cyan => ( 0, 170, 170 ),
        Color::Gray => ( 170, 170, 170 ),
        Color::DarkGray => ( 85, 85, 85 ),
        Color::LightRed => ( 255, 85, 85 ),
        Color::LightGreen => ( 85, 255, 85 ),
        Color::LightYellow => ( 255, 255, 85 ),
        Color::LightBlue => ( 85, 85, 255 ),
        Color::LightMagenta => ( 255, 85, 255 ),
        Color::LightCyan => ( 85, 255, 255 ),

        Color::Black => return Some( Color::Rgb( 10, 10, 10 ) ),
        Color::White => return Some( Color::Rgb( 245, 245, 245 ) ),
        Color::Indexed( _ ) | Color::Reset => return None,
    };

    Some( adjust_rgb( r, g, b ) )
}

fn adjust_rgb( r: u8, g: u8, b: u8 ) -> Color 
{
    let luminance = 0.2126 * ( r as f32 ) + 0.7152 * ( g as f32 ) + 0.0722 * ( b as f32 );
    let step = 10;

    if luminance < 10.0 
    {
        return Color::Rgb( r + step, g + step, b + step );
    }
    else if luminance > 245.0
    {
        return Color::Rgb( r - step, g - step, b - step );
    }

    if luminance > 127.5 
    {
        Color::Rgb(
            r.saturating_add( step ),
            g.saturating_add( step ),
            b.saturating_add( step ),
        )
    } 
    else 
    {
        Color::Rgb(
            r.saturating_sub( step ),
            g.saturating_sub( step ),
            b.saturating_sub( step ),
        )
    }
}

pub fn default_link_normal_style() -> Style
{
    Style::default().underlined()
}

pub fn default_link_focus_style( not_focus : &Style ) -> Style
{
    default_focus_style( not_focus )
}