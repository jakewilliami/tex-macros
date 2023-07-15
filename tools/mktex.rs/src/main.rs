use std::{fs, path::Path};
use clap::{
    ArgAction,
    crate_authors,
    crate_version,
    Parser,
    Subcommand,
};

mod config;
mod freeze;
mod texmf;
mod resource;

use config::*;
use resource::{fetch_resource, ResourceLocation};

// TODO:
//   - More idiomatic result handling
//   - allow freeze to accept commit id
//   - sync/overwrite
//   - class option local with no texmf
//   - freeze more than just class
//   - no-option default?
//   - decouple from tex-macros repo as much as possible
//   - author
//   - general class option?
//   - bibligraphy file option
//   - letter option
//   - formal letter option
//   - beamer option
//   - figure option
//   - poi option

#[derive(Parser)]
#[command(
    name = "mktex",
    author = crate_authors!("\n"),
    version = crate_version!(),
    allow_missing_positional = true,
    subcommand_negates_reqs = true,
)]
/// Make LaTeX projects with custom macros.
struct Cli {
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

    /// Try to use local files rather than remote
    #[arg(
        short = 'l',
        long = "local",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    local: Option<bool>,

    // /// Sync
    // #[arg(
    //     short = 's',
    //     long = "sync",
    //     action = ArgAction::SetTrue,
    //     num_args = 0,
    // )]
    // sync: Option<bool>,

    /// Use class
    #[arg(
        short = 'c',
        long = "class",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    class: Option<bool>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Freeze latest class files
    Freeze,
}

fn main() {
    let cli = Cli::parse();

    let resource_location = if let Some(local) = cli.local {
        if local { ResourceLocation::Local } else { ResourceLocation::Remote }
    } else {
        ResourceLocation::Remote
    };

    // Make class file
    if let Some(use_class) = cli.class {
        if use_class {
            // Need to move class file to local texmf if possible
            if !texmf::resource_in_local_texmf(&CLS_NAME) {
                // Write class file to local texmf directory
                let local_dir = texmf::texmf_local_resources();
                let cls_contents = fetch_resource(CLS_RESOURCE, &resource_location);
                fs::write(local_dir.join(&CLS_NAME), cls_contents).unwrap();
            }

            // Make template in target dir
            let out_file = Path::new(&cli.dir.unwrap()).join(&cli.file.unwrap());

            // Check that we are not overwriting a file!
            if out_file.exists() {
                panic!("Failed to mktex: file exists")
            }

            // Write the template file to the specified directory
            let tmpl_contents = fetch_resource(TMPL_RESOURCE, &resource_location);
            fs::write(out_file, tmpl_contents).unwrap();
        }
    };

    match &cli.command {
        Some(Commands::Freeze) => {
            let cls_contents = fetch_resource(CLS_RESOURCE, &resource_location);
            println!("{}", freeze::expand_input_paths(cls_contents, &resource_location));
        },
        None => {},
    }
}
