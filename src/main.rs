
pub mod sinbad;

fn main() {
    let backend = "dynamic1";
    let depth = 10;
    let gp = "./grammars/amb.acc";
    let lp = "./grammars/general.lex";
    let duration: usize = 5;
    let sin = sinbad_rs::sinbad()
        .expect("Unable to create a SinBAD instance!");
    let res = sinbad_rs::invoke(&sin, duration, backend, depth, gp, lp);
    match res {
        Ok(sin_out) => { println!("{} => {}", gp, sin_out.is_amb()) },
        Err(e) => {
            eprintln!("Error:\n{}", e.to_string());
        }
    }
}