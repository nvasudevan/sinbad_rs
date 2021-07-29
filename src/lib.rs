pub(crate) mod sinbad;
use sinbad::SinBAD;
use std::env;
use std::env::VarError;
use crate::sinbad::{SinBADError, SinBADOutput};

const SINBAD_CMD: &str = "/home/krish/kv/sinbad/src/sinbad";
const ACCENT_DIR: &str = "/home/krish/kv/accent";

pub fn sinbad(backend: String, depth: usize, gp: &str, lp: &str)
    -> Result<SinBADOutput, SinBADError> {
    let sinbad_cmd = env::var("SINBAD_CMD")?;
    let accent_dir = env::var("ACCENT_DIR")?;
    let sin = SinBAD::new(
        sinbad_cmd, accent_dir, backend, depth, None
    );

    sin.invoke(gp, lp)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
