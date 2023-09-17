use std::{fs, path::{Path, PathBuf}};

#[path = "texmf.rs"] mod texmf;

use crate::resource::{fetch_resource, ResourceLocation};

pub struct LocalFile<'a> {
    pub cls_path: String,
    pub template_path: String,
    pub resource_location: &'a ResourceLocation,
    pub out_dir: String,
    pub out_file: String,
}

impl LocalFile<'_> {
    fn out_file(&self) -> PathBuf {
        let mut path = PathBuf::from(&self.out_dir);
        path.push(&self.out_file);
        path
    }
}

fn write_template(file: LocalFile) {
    // Make template in target dir
    let out_file = file.out_file();

    // Check that we are not overwriting a file!
    if out_file.exists() {
        panic!("Failed to mktex: file exists")
    }

    // Write the template file to the specified directory
    let tmpl_contents = fetch_resource(
        file.template_path.as_str(),
        &file.resource_location
    );
    fs::write(out_file, tmpl_contents).unwrap();
}

pub fn write_cls(file: LocalFile) {
    let file_name = Path::new(&file.cls_path).file_name()
        .unwrap().to_str().unwrap();

    // Need to move file to local texmf if possible
    if !texmf::resource_in_local_texmf(file_name) {
        // Write file to local texmf directory
        let local_dir = texmf::texmf_local_resources();
        let contents = fetch_resource(
            file.cls_path.as_str(),
            &file.resource_location
        );
        fs::write(local_dir.join(&file_name), contents).unwrap();
    }

    write_template(file);
}
