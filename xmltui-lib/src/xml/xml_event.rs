use roxmltree::Node;

use crate::rtml::util::rtml_event::{RTMLCallback, RTMLCallbackAction, RTMLCallbackCommand, RTMLEvent};


pub fn parse_event_attrs( node : Node, id : &str  ) -> anyhow::Result<Vec<RTMLEvent>>
{
    let mut ret = vec![];

    if let Some( enter ) = node.attribute( "enter" ) && enter.trim() != ""
    {
        ret.push( parse_enter_event( node, enter, id )? );
    }
    
    Ok( ret )
}

fn parse_enter_event( node : Node, value : &str, id : &str ) -> anyhow::Result<RTMLEvent>
{
    let mut data_from = vec![];

    if let Some( from ) = node.attribute( "enter-data" )
    {
        from.split( "," ).for_each(
            | s |
            {
                let s = s.trim();

                if s != ""
                {
                    data_from.push( s.to_string() );
                }
            }
        );
    }
    else
    {
        data_from.push( id.to_string() );    
    }

    Ok(
        RTMLEvent::Enter( RTMLCallback::Command(
                RTMLCallbackCommand::new( value.to_string(), data_from ),
                parse_callback_action( node, "enter" )?
            )
        )
    )
}

fn parse_callback_action( node : Node, prefix : &str ) -> anyhow::Result<RTMLCallbackAction>
{
    if let Some( parent_id ) = node.attribute( format!( "{prefix}-replace-childs" ).as_str() ) && parent_id.trim() != ""
    {
        Ok( RTMLCallbackAction::ReplaceChilds( parent_id.to_string() ) )
    }
    else if let Some( node_id ) = node.attribute( format!( "{prefix}-replace-node" ).as_str() ) && node_id.trim() != ""
    {
        Ok( RTMLCallbackAction::ReplaceNode( node_id.to_string() ) )
    }
    else if let Some( node_id ) = node.attribute( format!( "{prefix}-change-value" ).as_str() ) && node_id.trim() != ""
    {
        Ok( RTMLCallbackAction::ChangeValue( node_id.to_string() ) )
    }
    else
    {
        Ok( RTMLCallbackAction::None )    
    }
}