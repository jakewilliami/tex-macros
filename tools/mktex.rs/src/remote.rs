// Fetch resource remotely

use reqwest;

const BASE_URI: &str = "https://raw.githubusercontent.com/jakewilliami/tex-macros/";

// use "master" for tag
pub fn get_remote_resource(resource: &str, tag: &str) -> String {
	let mut uri = BASE_URI.to_owned();
	uri.push('/');
	uri.push_str(tag);
	uri.push('/');
	uri.push_str(resource);
    reqwest::blocking::get(uri).expect("Cannot get remote resource")
		.text().expect("Cannot get text from remote response")
}
