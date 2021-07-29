
pub mod sinbad;

fn main() {
    let backend = "dynamic1";
    let depth = 10;
    let gp = "/home/krish/kv/sinbad/bin/amb2.acc";
    let lp = "/home/krish/kv/sinbad/bin/general.lex";
    let res = sinbad_rs::sinbad(backend.to_owned(), depth, gp, lp);
    match res {
        Ok(out) => {
            println!("[ret: {}]\nout: {}\nerr: {}",
                     out.r_code.unwrap(), out.out, out.err
            )
        },
        Err(e) => {
            eprintln!("Error:\n{}", e.to_string());
        }
    }
}