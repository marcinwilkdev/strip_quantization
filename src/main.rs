use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "strip_quantization")]
struct Opt {
    #[structopt(short, long)]
    input: String,
    #[structopt(short, long)]
    output: String,
    #[structopt(short, long)]
    colors: u32,
}


fn main() {
    let opt = Opt::from_args();

    strip_quantization::perform_quantization(opt.colors, &opt.input, &opt.output);
}
