use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use codegen::Scope;

mod recipe;

const MINECRAFT_VERSION: &str = "1.19.3";

const DATA_PATH: &str = "minecraft-data/data";

fn get_latest_data_paths() -> HashMap<String, String> {
    let mut data_paths: serde_json::Value = serde_json::from_reader(
        File::open(Path::new(DATA_PATH).join("dataPaths.json")).unwrap()
    ).unwrap();

    serde_json::from_value(data_paths["pc"][MINECRAFT_VERSION].take()).unwrap()
}

fn load_raw_resource(resource: &str, data_paths: &HashMap<String, String>) -> String {
    //let resource = 

    "".into()
}

fn main() {
    println!("{:?}", get_latest_data_paths());

    //let recipes = Scope::new()

    //recipes.import(path, ty)
}
