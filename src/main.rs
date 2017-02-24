extern crate rustc_version;
extern crate rayon;
extern crate env_logger;
extern crate rusoto;
extern crate serde_json;

use std::path::Path;
use std::io::Write;
use std::fs::{File, rename};
use std::fs;
use rusoto::botocore::Service as BotocoreService;
use std::io::BufReader;
use rusoto::generator::generate_source;


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
	service_name=service_name.replace("-","_"));

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

fn main() {
    let _ = env_logger::init();
    let services_dir = "services";
    let services_path = Path::new(&services_dir).to_owned();

    let service_paths = fs::read_dir("botocore/botocore/data").unwrap();

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

const BOTOCORE_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/botocore/botocore/data/");

pub struct Service {
    pub name: String,
    protocol_date: String,
}

impl Service {
    pub fn new<S>(name: S, protocol_date: S) -> Self
        where S: Into<String> {
        Service {
            name: name.into(),
            protocol_date: protocol_date.into(),
        }
    }
}

pub fn generate(service: Service, output_path: &Path) -> i32 {
    let botocore_destination_path = output_path.join(format!("{}/src/{}_botocore.rs", service.name, service.name));
    let serde_destination_path = output_path.join(format!("{}/src/{}.rs", service.name, service.name));
    let botocore_service_data_path = Path::new(BOTOCORE_DIR)
        .join(format!("{}/{}/service-2.json", service.name, service.protocol_date));

    let needs_serde = botocore_generate(botocore_service_data_path.as_path(),
                                        botocore_destination_path.as_path());

    // only pass the generated code through serde if we actually need JSON serialization
    //if needs_serde {
    //    serde_generate(botocore_destination_path.as_path(),
    //                   serde_destination_path.as_path());
    //} else {
        rename(botocore_destination_path, serde_destination_path).unwrap();
    //}

    return 1;

}

fn botocore_generate(input_path: &Path, output_path: &Path) -> bool {
    let input_file = File::open(input_path).expect(&format!(
        "{:?} not found",
        input_path,
    ));

    let service_data_as_reader = BufReader::new(input_file);

    let service: BotocoreService = serde_json::from_reader(service_data_as_reader).expect(&format!(
        "Could not convert JSON in {:?} to Service",
        input_path,
    ));

    match generate_source(&service, output_path) {
        Ok(()) => {},
        _ => panic!("Failed to write file at {:?}", output_path)
    }

    match &service.metadata.protocol[..] {
        "json" | "rest-json" => true,
        _ => false,
    }
}


fn serde_generate(source: &Path, destination: &Path) {
    ::std::fs::copy(source, destination).expect(&format!(
        "Failed to copy {:?} to {:?}",
        source,
        destination,
    ));
}
