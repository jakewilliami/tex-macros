// Fetch resource!

use std::fs;
use std::path::Path;
use home;

// https://stackoverflow.com/a/73840814/12069968
// include!("remote.rs");
#[path = "remote.rs"] mod remote;

#[derive(PartialEq)]
pub enum ResourceLocation {
    Local,
    Remote,
}

pub fn fetch_resource(resource: &str, loc: &ResourceLocation) -> String {
    match loc {
        ResourceLocation::Local => fetch_resource_local(resource),
        ResourceLocation::Remote => fetch_resource_remote(resource),
    }
}

fn fetch_resource_local(resource: &str) -> String {
    let resource_dir = home::home_dir().expect("Cannot get home directory")
        .join("projects").join("tex-macros");
    if !resource_dir.as_path().exists() {
        panic!("No local resource path at ~/projects/tex-macros/")
    }

    let resource_path = resource_dir.join(Path::new(resource));
    fs::read_to_string(resource_path).unwrap()
}

fn fetch_resource_remote(resource: &str) -> String {
    remote::get_remote_resource(resource, "master")
}
