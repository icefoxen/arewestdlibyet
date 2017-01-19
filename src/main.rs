extern crate cargo;
use cargo::core::Registry;
use std::path;

fn main() {
    let config = cargo::util::config::Config::default().unwrap();
    //cargo::ops::search("ggez", &config, None, 10).unwrap();
    let p = path::PathBuf::from(".");
    let w = cargo::core::Workspace::new(&p, &config);
    let mut registry = cargo::core::registry::PackageRegistry::new(&config).unwrap();

    let source_id = cargo::core::source::SourceId::crates_io(&config).unwrap();
    //let target_package_id = cargo::core::package_id::PackageId::new("ggez", "*", &source_id).unwrap();
    //let package_set = registry.get(&[target_package_id]);
    
    let dependency = cargo::core::dependency::Dependency::parse_no_deprecated("ggez", None, &source_id).unwrap();
    let summaries = registry.query(&dependency).unwrap();

    for s in summaries.iter() {
        println!("Package summary: {:#?}", s);
    }
    //let r = summaries.len();
    //println!("Hello, world: {:?}", r);
}
