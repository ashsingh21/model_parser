mod wavefront_parser;

fn main() {
    let document = wavefront_parser::wavefront_obj_parser("cube.obj").expect("File reading failed!");
    let serialized_document = serde_yaml::to_string(&document).expect("cant serailize");
    wavefront_parser::save_to_disk("generated.yaml", &serialized_document).expect("error saving to disk");
}

