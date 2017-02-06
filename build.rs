extern crate rustc_version;
extern crate rusoto_codegen;
extern crate rayon;

use std::path::Path;
use std::io::Write;
use std::fs::File;

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

	let _ = f.write_all(file_contents.as_bytes());
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

	let _ = f.write_all(file_contents.as_bytes());
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
    let services_dir = "services";
    let services_path = Path::new(&services_dir).to_owned();

    let services = services! {
        ["acm", "2015-12-08"],
        ["autoscaling", "2011-01-01"],
        ["cloudformation", "2010-05-15"],
        ["cloudfront", "2016-11-25"],
        ["cloudhsm", "2014-05-30"],
        ["cloudsearch", "2013-01-01"],
        ["cloudtrail", "2013-11-01"],
        ["cloudwatch", "2010-08-01"],
        ["codecommit", "2015-04-13"],
        ["codedeploy", "2014-10-06"],
        ["codepipeline", "2015-07-09"],
        ["cognito-identity", "2014-06-30"],
        ["config", "2014-11-12"],
        ["datapipeline", "2012-10-29"],
        ["devicefarm", "2015-06-23"],
        ["directconnect", "2012-10-25"],
        ["ds", "2015-04-16"],
        ["dynamodb", "2012-08-10"],
        ["dynamodbstreams", "2012-08-10"],
        ["ec2", "2016-11-15"],
        ["ecr", "2015-09-21"],
        ["ecs", "2014-11-13"],
        ["elasticache", "2015-02-02"],
        ["elasticbeanstalk", "2010-12-01"],
        ["elastictranscoder", "2012-09-25"],
        ["elb", "2012-06-01"],
        ["elbv2", "2015-12-01"],
        ["emr", "2009-03-31"],
        ["events", "2015-10-07"],
        ["firehose", "2015-08-04"],
        ["iam", "2010-05-08"],
        ["importexport", "2010-06-01"],
        ["inspector", "2016-02-16"],
        ["iot", "2015-05-28"],
        ["kinesis", "2013-12-02"],
        ["kms", "2014-11-01"],
        ["lambda", "2015-03-31"],
        ["logs", "2014-03-28"],
        ["machinelearning", "2014-12-12"],
        ["marketplacecommerceanalytics", "2015-07-01"],
        ["opsworks", "2013-02-18"],
        ["redshift", "2012-12-01"],
        ["rds", "2014-10-31"],
        ["route53", "2013-04-01"],
        ["route53domains", "2014-05-15"],
        ["s3", "2006-03-01"],
        ["sdb", "2009-04-15"],
        ["sns", "2010-03-31"],
        ["sqs", "2012-11-05"],
        ["ssm", "2014-11-06"],
        ["storagegateway", "2013-06-30"],
        ["swf", "2012-01-25"],
        ["waf", "2015-08-24"],
        ["workspaces", "2015-04-08"]
    };

    let count: usize = services.into_par_iter().map(|service| {
    	let service_name = service.name.to_owned();
		generate_cargo_toml(&service_name, &services_path.clone(), "0.22.0");
		generate_lib_rs(&service_name, &services_path.clone());
    	generate(service, &services_path.clone());

    }).count();
    println!("\nGenerated {:?} services.\n", count);

}
