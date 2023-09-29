use std::{fs, path::Path, process};
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
use file::{LocalTemplate, LocalResource};

// TODO:
//   - warn when local version not same as remote
//   - add version history to readme
//   - fix broken pipe: https://stackoverflow.com/a/65760807/12069968
//   - fix -c freeze crash
//   - warn if -l passed without -c or something (-l only relevant with other things)
//   - do not allow freeze with other options
//   - allow freeze options (e.g., don't assume the user wants to use freeze with -c)
//   - handle updating of local texmf files
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

    /// Use article class
    #[arg(
        short = 'c',
        long = "class",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    class: Option<bool>,

    /// Use beamer class
    #[arg(
        short = 'b',
        long = "beamer",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    beamer: Option<bool>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Freeze latest class files
    Freeze,
    /// Print local texmf directory
    Texmf,
}

fn main() {
    let cli = Cli::parse();

    let resource_location = if let Some(local) = cli.local {
        if local { ResourceLocation::Local } else { ResourceLocation::Remote }
    } else {
        ResourceLocation::Remote
    };

    // Parse subcommands and exit
    match &cli.command {
        Some(Commands::Freeze) => {
            let cls_contents = fetch_resource(CLS_RESOURCE, &resource_location);
            println!("{}", freeze::expand_input_paths(cls_contents, &resource_location));
            process::exit(0);
        },
        Some(Commands::Texmf) => {
            if let Some(texmf_path) = texmf::texmf() {
                println!("{}", texmf_path.display());
            } else {
                eprintln!("ERROR: Could not find local texmf directory");
            }
            process::exit(0);
        },
        None => {},
    }

    let out_dir = cli.dir.unwrap().to_string();
    let out_file = cli.file.unwrap().to_string();

    // Make class file
    if let Some(use_class) = cli.class {
        if use_class {
            let cls = LocalResource {
                resource_path: CLS_RESOURCE.to_string(),
                resource_location: &resource_location,
                template: Some(LocalTemplate {
                    template_path: TMPL_RESOURCE.to_string(),
                    out_dir: &out_dir,
                    out_file: &out_file,
                }),
            };
            file::write_resource(cls);
        }
    };

    // Make beamer file
    if let Some(use_beamer) = cli.beamer {
        if use_beamer {
            // Custom Beamer theme files
            for file in vec![BMR_THEME_COLOUR, BMR_THEME_INNER, BMR_THEME_OUTER, BMR_THEME_MAIN] {
                let theme_file = Path::new(BMR_THEME_PATH).join(file);
                let sty = LocalResource {
                    resource_path: theme_file.display().to_string(),
                    resource_location: &resource_location,
                    template: None,
                };
                file::write_resource(sty);
            }

            // Main Beamer class file
            let cls = LocalResource {
                resource_path: BMR_RESOURCE.to_string(),
                resource_location: &resource_location,
                template: Some(LocalTemplate {
                    template_path: BMR_TMPL_RESOURCE.to_string(),
                    out_dir: &out_dir,
                    out_file: &out_file,
                }),
            };
            file::write_resource(cls);
        }
    }
}
