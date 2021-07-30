
pub mod sinbad;

fn main() {
    let backend = "dynamic1";
    let depth = 10;
    let gp = "/var/tmp/cfg_ds/20.acc";
    let lp = "/home/krish/kv/sinbad/bin/general.lex";
    let duration: usize = 5;
    let sin = sinbad_rs::sinbad()
        .expect("Unable to create a SinBAD instance!");
    let res = sinbad_rs::invoke(&sin, duration, backend, depth, gp, lp);
    match res {
        Ok(amb) => { println!("{} => {}", gp, amb) },
        Err(e) => {
            eprintln!("Error:\n{}", e.to_string());
        }
    }
}