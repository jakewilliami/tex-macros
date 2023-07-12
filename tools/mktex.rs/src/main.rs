use clap::{
    ArgAction,
    crate_authors,
    crate_version,
    Parser,
};

#[derive(Parser)]
#[command(
    name = "mktex",
    author = crate_authors!("\n"),
    version = crate_version!(),
    allow_missing_positional = true,
)]
/// Make LaTeX projects with custom macros.
struct Cli {
    // Usage: mktex [h] [gcfbp] [dir] [file name] || mktex [b] [dir]

    /// Output file name
    #[arg(
        action = ArgAction::Set,
        num_args = 0..=1,
        value_name = "file name",
        default_value = "document.tex",
    )]
    file: Option<String>,

    /// Output directory
    #[arg(
        action = ArgAction::Set,
        num_args = 1,
        value_name = "output directory",
        required = true,
    )]
    dir: Option<String>,

    /// Use class
    #[arg(
        short = 'c',
        long = "class",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    class: Option<bool>,
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli.dir);
    println!("{:?}", cli.file);

    // Make class file
    if let Some(use_class) = cli.class {
        if use_class {
            println!("Make class file!")
        }
    };
}
