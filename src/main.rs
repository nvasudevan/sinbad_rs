use std::path::PathBuf;
use sinbad_rs::SinBADInput;
use structopt::StructOpt;

pub mod sinbad;

#[derive(StructOpt, Debug)]
#[structopt(name = "sin_opt")]
struct SinOpt {
    /// SinBAD backend to run
    #[structopt(short)]
    backend: String,
    /// Threshold depth to apply in SinBAD
    #[structopt(short)]
    depth: usize,
    /// Duration SinBAD to run for
    #[structopt(short = "t")]
    duration: usize,
    /// Path to the grammar file
    #[structopt(parse(from_os_str))]
    gp: PathBuf,
    /// Path to the lex file
    #[structopt(parse(from_os_str))]
    lp: PathBuf,
}

fn main() {
    let sin_opts = SinOpt::from_args();
    println!("=> running sinbad with: {:#?}", sin_opts);
    let sin_input = SinBADInput::new(
        sin_opts.backend,
        sin_opts.depth,
        sin_opts.duration,
    );
    let gp: PathBuf = sin_opts.gp;
    let lp = sin_opts.lp;
    if ! gp.exists() {
        panic!("grammar file {} does not exist!", gp.to_str().unwrap());
    }
    if ! lp.exists() {
        panic!("lex file {} does not exist!", lp.to_str().unwrap());
    }
    let sin = sinbad_rs::sinbad()
        .expect("Unable to create a SinBAD instance!");
    let res = sinbad_rs::invoke(
        &sin, &sin_input, gp.to_str().unwrap(), lp.to_str().unwrap());
    match res {
        Ok(sin_out) => { println!("{} => {}", gp.to_str().unwrap(), sin_out.is_amb()) }
        Err(e) => {
            eprintln!("Error:\n{}", e.to_string());
        }
    }
}