use std::collections::HashMap;

use roxmltree::Node;

use crate::{app::app_doc::chroot, code::executor::{Executor, ExecutorArg, ExecutorBuilder, ExecutorEnv, ExecutorEnvVar}, util::file::read_file_in_chroot_with_extension};

pub fn code_from_parent( node : Option<Node> ) -> anyhow::Result<HashMap<String, Executor>>
{
    let mut ret = HashMap::new();

    if node.is_none() { return Ok( ret ) };

    let node = node.unwrap();

    for child in node.children()
    {
        add_executors( child, &mut ret )?;
    }

    Ok( ret )
}

fn add_executors( node : Node, executors : &mut HashMap<String, Executor> ) -> anyhow::Result<()>
{
    if node.tag_name().name() != "code" { return Ok( () ) };

    if node.has_attribute( "src" )
    {
        add_executors_from_file( node.attribute( "src" ).unwrap(), executors )
    }
    else
    {
        add_executor_from_code_node( node, executors )
    }
}

fn add_executors_from_file( path : &str, executors : &mut HashMap<String, Executor> ) -> anyhow::Result<()>
{
    let str_executors = read_file_in_chroot_with_extension( path, chroot(), "xml" )?;

    if str_executors.trim() == "" { return Ok( () ) };

    let doc = roxmltree::Document::parse(str_executors.as_str() )?;

    if doc.root_element().tag_name().name() != "executors" { return Ok( () ); }

    let new_executors = code_from_parent( Some( doc.root_element() ) )?;

    for ( key, exec ) in new_executors
    {
        executors.insert( key, exec );
    }

    Ok( () )
}

fn add_executor_from_code_node( node : Node, executors : &mut HashMap<String, Executor> ) -> anyhow::Result<()>
{
    let name = code_attr( node, "name" )?;

    let mut builder = ExecutorBuilder::new().command( code_attr( node, "command" )? );

    for children in node.children()
    {
        match children.tag_name().name()
        {
            "args" => builder = args( children, builder )?,
            "envs" => builder = envs( children, builder )?,
            _ => continue
        }
    }

    executors.insert( name, builder.build()? );

    Ok( () )
}

fn code_attr( node : Node, attr : &str ) -> anyhow::Result<String>
{
    Ok( 
        node.attribute( attr )
        .ok_or( anyhow::Error::msg( format!( "attribute {} required in code node", attr ) ) )?
        .to_string() 
    )
}

fn envs( node : Node, mut builder : ExecutorBuilder ) -> anyhow::Result<ExecutorBuilder>
{
    for n in node.children()
    {
        match n.tag_name().name()
        {
            "env" =>
            {
                let name = code_attr( n, "name" )?;

                let value = n.text().unwrap_or( "" ).to_string();

                builder = builder.env( ExecutorEnv::Var( ExecutorEnvVar::new( name, value ) ) );
            },
            "env-data" =>
            {
                let name = code_attr( n, "name" )?;

                builder = builder.env( ExecutorEnv::Data( name ) );
            },
            "env-value" =>
            {
                let name = code_attr( n, "name" )?;

                builder = builder.env( ExecutorEnv::Value( name ) );
            },
            "st-env" =>
            {
                let name = code_attr( n, "name" )?;

                builder = builder.env( ExecutorEnv::State( name ) );
            },
            _ => {}
        }
    }

    Ok( builder )
}

fn args( node : Node, mut builder : ExecutorBuilder ) -> anyhow::Result<ExecutorBuilder>
{
    for n in node.children()
    {
        match n.tag_name().name()
        {
            "arg" =>
            {
                match n.text()
                {
                    Some( s ) if s.trim() != "" => 
                    {
                        builder = builder.arg( ExecutorArg::Text( s.trim().to_string() ) );
                    },
                    _ => {}
                }
            },
            "arg-data" =>
            {
                let name = code_attr( n, "name" )?;

                builder = builder.arg( ExecutorArg::Data( name ) );
            },
            "arg-value" =>
            {
                let name = code_attr( n, "name" )?;

                builder = builder.arg( ExecutorArg::Value( name ) );
            },
            "st-arg" =>
            {
                let name = code_attr( n, "name" )?;

                builder = builder.arg( ExecutorArg::State( name ) );
            },
            _ => {}
        }
    }

    Ok( builder )
}