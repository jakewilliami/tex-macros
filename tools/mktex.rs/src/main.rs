use std::fs;
use std::path::Path;
use clap::{
    ArgAction,
    crate_authors,
    crate_version,
    Parser,
};
use home;

mod texmf;

// TODO:
//   - sync
//   - freeze
//   - allow remote pull
//   - class option local with no texmf
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

    // /// Local
    // #[arg(
    //     short = 'l',
    //     long = "local",
    //     action = ArgAction::SetTrue,
    //     num_args = 0,
    //     default_value = true,
    // )]
    // local: Option<bool>,

    // /// Sync
    // #[arg(
    //     short = 's',
    //     long = "sync",
    //     action = ArgAction::SetTrue,
    //     num_args = 0,
    // )]
    // sync: Option<bool>,

    // /// Freeze
    // #[arg(
    //     short = 'f',
    //     long = "freeze",
    //     action = ArgAction::SetTrue,
    //     num_args = 0,
    // )]
    // freeze: Option<bool>,

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
    let tex_macros_dir = home::home_dir().expect("Cannot get home directory")
        .join("projects").join("tex-macros");
    let cli = Cli::parse();

    // Make class file
    if let Some(use_class) = cli.class {
        if use_class {
            let cls_name = "arteacle.cls";
            let cls_file = tex_macros_dir
                .join("general_macros").join("class")
                .join(&cls_name);

            // Need to move class file to local texmf if possible
            if !cls_in_local_texmf(&cls_name) {
                // Ensure that class file exists locally
                if !cls_file.as_path().exists() {
                    panic!("Cannot find arteacle.cls locally");
                }

                // Move class file to local texmf direcotry
                let local_dir = texmf::texmf_local_resources();
                fs::copy(&cls_file, local_dir.join(&cls_name)).unwrap();
            }

            // Make template in target dir
            let tmplt_name = "template_Class_Typesetting.tex";
            let tmplt_file = tex_macros_dir
                .join("templates").join(&tmplt_name);
            let out_file = Path::new(&cli.dir.unwrap()).join(&cli.file.unwrap());

            // Check that we are not overwriting a file!
            if out_file.exists() {
                panic!("Failed to mktex: file exists")
            }

            // Copy the template file to the specified directory
            fs::copy(tmplt_file, out_file).unwrap();
        }
    };
}

fn cls_in_local_texmf(cls_name: &str) -> bool {
    texmf::texmf_local_resources()
        .join(cls_name).as_path().exists()
}
