use clap::Parser;
use xmltui_lib::run_app;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    root : String,
    #[arg(long)]
    path : Option<String>,
}

fn main() -> anyhow::Result<()>
{
    let args = Args::parse();

    if args.root.trim() == ""
    {
        panic!( "El argumento root es obligatorio" );
    }

    let path = match args.path.as_ref()
    {
        Some( p ) if p.trim() != "" => p,
        _ => "index.xml"
    };

    run_app( &args.root, path )
}
