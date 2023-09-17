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
mod file;

use config::*;
use resource::{fetch_resource, ResourceLocation};
use file::LocalFile;

// TODO:
//   - add version history to readme
//   - fix broken pipe: https://stackoverflow.com/a/65760807/12069968
//   - fix -c freeze crash
//   - warn if -l passed without -c or something (-l only relevant with other things)
//   - do not allow freeze with other options
//   - allow freeze options (e.g., don't assume the user wants to use freeze with -c)
//   - add support for teamer class
//   - more idiomatic result handling
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
//   - add option to just print the location of local texmf dir

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
            let cls = LocalFile {
                cls_path: CLS_RESOURCE.to_string(),
                template_path: TMPL_RESOURCE.to_string(),
                resource_location: &resource_location,
                out_dir: cli.dir.unwrap().to_string(),
                out_file: cli.file.unwrap().to_string(),
            };
            file::write_cls(cls);
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
