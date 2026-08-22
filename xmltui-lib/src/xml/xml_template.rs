use std::collections::HashMap;

use roxmltree::Node;

use crate::{app::app_doc::chroot, util::file::read_file_in_chroot_with_extension, xml::{attrs::attr_result, xml_util::template_from_node}};


pub fn templates_from_parent( node : Option<Node>, xml : &str ) -> anyhow::Result<HashMap<String, String>>
{
    let mut ret = HashMap::new();

    if node.is_none() { return Ok( ret ) };

    let node = node.unwrap();

    for child in node.children()
    {
        add_template( child, &mut ret, xml )?;
    }

    Ok( ret )
}

fn add_template( node : Node, templates : &mut HashMap<String, String>, xml : &str ) -> anyhow::Result<()>
{
    if node.tag_name().name() != "template" { return Ok( () ) };

    if node.has_attribute( "src" )
    {
        add_templates_from_file( node.attribute( "src" ).unwrap(), templates, xml )
    }
    else
    {
        add_template_from_code_node( node, templates, xml )
    }
}

fn add_templates_from_file( path : &str, templates : &mut HashMap<String, String>, xml : &str ) -> anyhow::Result<()>
{
    let str_templates = read_file_in_chroot_with_extension( path, chroot(), "xml" )?;

    if str_templates.trim() == "" { return Ok( () ) };

    let doc = roxmltree::Document::parse(str_templates.as_str() )?;

    if doc.root_element().tag_name().name() != "templates" { return Ok( () ); }

    let new_templates = templates_from_parent( Some( doc.root_element() ), xml )?;

    for ( key, templ ) in new_templates
    {
        templates.insert( key, templ );
    }

    Ok( () )
}

fn add_template_from_code_node( node : Node, templates : &mut HashMap<String, String>, xml : &str ) -> anyhow::Result<()>
{
    let name = attr_result( node, "name" )?;

    let template = template_from_node( node, xml ).unwrap_or( "".to_string() );

    if template.trim() == "" { return Ok( () ) };

    templates.insert( name, template );

    Ok( () )
}