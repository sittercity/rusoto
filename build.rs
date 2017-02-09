extern crate rustc_version;
extern crate rusoto_codegen;
extern crate rayon;
extern crate env_logger;

use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::fs;

use rusoto_codegen::{Service, generate};
use rayon::prelude::*;

fn generate_lib_rs(service_name: &str, output_path: &Path) {
	let mut f = File::create(&output_path.join(format!("{}/src/lib.rs", service_name))).expect("Could not create lib.rs");
	let file_contents = format!("
		#![crate_name = \"rusoto_{service_name}\"]
		#![crate_type = \"lib\"]
		pub mod {service_name};

		extern crate hyper;
		extern crate rusoto_credential;
		extern crate rusoto_core;
		extern crate xml;
	",
	service_name=service_name);

	f.write_all(file_contents.as_bytes()).unwrap();
}

fn generate_cargo_toml(service_name: &str, output_path: &Path, crate_version: &str) {
	let mut f = File::create(&output_path.join(format!("{}/Cargo.toml", service_name))).expect("Could not create Cargo.toml");

	let file_contents = format!("
[package]
authors = [
    \"Anthony DiMarco <ocramida@gmail.com>\",
    \"Jimmy Cuadra <jimmy@jimmycuadra.com>\",
    \"Matthew Mayer <matthewkmayer@gmail.com>\",
    \"Nikita Pekin <contact@nikitapek.in>\"
]

description = \"AWS SQS\"
documentation = \"http://rusoto.github.io/rusoto/rusoto/index.html\"
keywords = [\"AWS\", \"Amazon\"]
license = \"MIT\"
name = \"rusoto_{service_name}\"
repository = \"https://github.com/rusoto/rusoto\"
version = \"{crate_version}\"

[dependencies]
hyper = \"0.10.0\"
hyper-native-tls = \"0.2.1\"
xml-rs = \"0.3\"

[dependencies.rusoto_core]
path = \"../../rusoto_core\"
version = \"0.22.0\"

[dependencies.rusoto_credential]
path = \"../../credential\"
version = \"0.4.0\"
		",
		service_name=service_name,
		crate_version=crate_version);

	f.write_all(file_contents.as_bytes()).unwrap();
}

// expand to use cfg!() so codegen only gets run for services
// in the features list
macro_rules! services {
    ( $( [$name:expr, $date:expr] ),* ) => {
        {
            let mut services = Vec::new();
            $(
                if cfg!(feature = $name) {
                    services.push(Service::new($name, $date));
                }
            )*
            services
        }
    }
}

fn main() {
    let _ = env_logger::init();
    let services_dir = "services";
    let services_path = Path::new(&services_dir).to_owned();

    let service_paths = fs::read_dir("codegen/botocore/botocore/data").unwrap();

    let mut services = Vec::new();
    let mut dir_builder = std::fs::DirBuilder::new();
    dir_builder.recursive(true);

    for service in service_paths {
    	let path = service.unwrap();
    	if !path.file_type().unwrap().is_dir() {
    		continue;
    	}

    	let service_name = path.file_name().into_string().unwrap();
    	dir_builder.create(format!("services/{}/src", service_name)).unwrap();
    	let dates = fs::read_dir(path.path()).unwrap();
    	let mut date_strs = Vec::new();
    	for date in dates {
    		date_strs.push(date.unwrap().file_name().into_string().unwrap());
    	}
    	date_strs.sort();
    	let date = date_strs.last().unwrap().to_owned();

    	services.push(Service::new(service_name, date))

    }

    let count: usize = services.into_par_iter().map(|service| {
    	let service_name = service.name.to_owned();
        println!("Service: {}", service_name);
		generate_cargo_toml(&service_name, &services_path.clone(), "0.22.0");
		generate_lib_rs(&service_name, &services_path.clone());
    	generate(service, &services_path.clone());

    }).count();
    println!("\nGenerated {:?} services.\n", count);

}
