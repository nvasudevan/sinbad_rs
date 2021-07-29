
pub mod sinbad;

fn main() {
    let backend = "dynamic1";
    let depth = 10;
    let gp = "/var/tmp/cfg_ds/20.acc";
    let lp = "/home/krish/kv/sinbad/bin/general.lex";
    let duration: usize = 10;
    let res = sinbad_rs::sinbad(backend.to_owned(), depth, gp, lp, duration);
    match res {
        Ok(amb) => { println!("{} => {}", gp, amb) },
        Err(e) => {
            eprintln!("Error:\n{}", e.to_string());
        }
    }
}