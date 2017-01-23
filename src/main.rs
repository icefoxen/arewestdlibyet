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
    let target_package_id = cargo::core::package_id::PackageId::new("ggez", "0.2.0", &source_id).unwrap();
    let package_set = registry.get(&[target_package_id.clone()]);
    for p in package_set.package_ids() {
        //let package = package_set.get(p).unwrap();
        //println!("Package: {:?}", package);
        println!("fjdklsafj");
    }
    
    //let dependency = cargo::core::dependency::Dependency::parse_no_deprecated("ggez", None, &source_id).unwrap();
    //let summaries = registry.query(&dependency).unwrap();
    //for s in summaries.iter() {
    //    println!("Package summary: {:#?}", s);
    //}
}
