use clap::Parser;

mod wavefront_parser;
mod parser;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 3 {
    //     println!("Filepath and output filename are required!");
    //     return;
    // }
    // let filepath = &args[1];
    // let output_path = &args[2];
    // let document = wavefront_parser::wavefront_obj_parser(filepath).expect("File reading failed!");
    // let serialized_document = serde_yaml::to_string(&document).expect("cant serailize");
    // wavefront_parser::save_to_disk(output_path, &serialized_document).expect("error saving to disk");
    let _ = parser::Args::parse();
}

