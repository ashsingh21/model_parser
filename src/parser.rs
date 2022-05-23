use clap::Parser;


/// convert wavefront object files to yaml for dront-art
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// input_path of the wavefront object file
    #[clap(short, long)]
    input_path: String,

    /// path for the output file
    #[clap(short, long)]
    output_path: String
}