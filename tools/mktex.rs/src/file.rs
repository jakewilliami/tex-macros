use std::{fs, path::{Path, PathBuf}};

#[path = "config.rs"] mod config;
#[path = "texmf.rs"] mod texmf;

use crate::resource::{fetch_resource, ResourceLocation};

pub struct LocalTemplate<'a> {
    pub template_path: String,
    pub out_dir: &'a String,
    pub out_file: &'a String,
}

pub struct LocalResource<'a> {
    pub resource_path: String,
    pub resource_location: &'a ResourceLocation,
    pub template: Option<LocalTemplate<'a>>,
}

impl LocalTemplate<'_> {
    fn out_file(&self) -> PathBuf {
        let mut path = PathBuf::from(&self.out_dir);
        path.push(&self.out_file);
        path
    }
}

fn write_template(file: LocalResource) {
    let template = file.template.unwrap();

    // Make template in target dir
    let out_file = template.out_file();

    // Check that we are not overwriting a file!
    if out_file.exists() {
        panic!("Failed to mktex: file exists")
    }

    // Write the template file to the specified directory
    let tmpl_contents = fetch_resource(
        template.template_path.as_str(),
        &file.resource_location
    );
    fs::write(out_file, tmpl_contents).unwrap();
}

pub fn write_resource(file: LocalResource) {
    let file_name = Path::new(&file.resource_path)
        .strip_prefix(config::RESOURCE_PARENT)
        .unwrap().to_path_buf();

    // Need to move file to local texmf if possible
    if !texmf::resource_in_local_texmf(&file_name) {
        // Ensure parent path exists
        let mut local_path = texmf::texmf_local_resources();
        let file_parent = &file_name.parent();
        if let Some(file_parent) = file_parent {
            local_path.push(&file_parent)
        }
        if !local_path.exists() {
            fs::create_dir_all(&local_path).unwrap();
        }

        // Append file name to local resource path
        local_path.push(file_name.file_name().unwrap());

        // Write file to local texmf directory
        let contents = fetch_resource(
            file.resource_path.as_str(),
            &file.resource_location
        );
        fs::write(local_path, contents).unwrap();
    }

    if file.template.is_some() {
        write_template(file);
    }
}
