use roxmltree::Node;


pub fn template_from_inner_node( node : Node, xml : &str ) -> Option<String>
{
    for node in node.children()
    {
        if let Some( t ) = template_from_node( node, xml )
        {
            return Some( t );
        }

        // if node.tag_name().name() != "template" { continue; }
        
        // return match ( node.first_child(), node.last_child() ) 
        // {
        //     ( Some( primer_hijo ), Some( ultimo_hijo ) ) => 
        //     {
        //         let inicio = primer_hijo.range().start;
        //         let fin = ultimo_hijo.range().end;

        //         if xml[ inicio..fin ].trim() == "" 
        //         { 
        //             None 
        //         }
        //         else
        //         {
        //             Some( xml[ inicio..fin ].to_string() )    
        //         }
        //     }
        //     _ => None
        // }
    }

    None
}

pub fn template_from_node( node : Node, xml : &str ) -> Option<String>
{
    if node.tag_name().name() != "template" { return None; }
        
    return match ( node.first_child(), node.last_child() ) 
    {
        ( Some( primer_hijo ), Some( ultimo_hijo ) ) => 
        {
            let inicio = primer_hijo.range().start;
            let fin = ultimo_hijo.range().end;

            if xml[ inicio..fin ].trim() == "" 
            { 
                None 
            }
            else
            {
                Some( xml[ inicio..fin ].to_string() )    
            }
        }
        _ => None
    }
}

// Este de momento no lo uso así que lo comento
// pub fn template_from_node( node : Node, xml : &str ) -> String
// {
//     xml[ node.range() ].to_string()
// }