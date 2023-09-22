use clap;


#[derive(clap::Parser, Clone)]
pub struct Args {
    #[arg(short, long, default_value = "dataset")]
    pub root_dir: String,
    
    #[arg(short, long)]
    pub timeit: bool,

    #[arg(long)]
    pub trackit: bool,
}
