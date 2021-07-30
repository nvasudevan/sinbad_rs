pub(crate) mod sinbad;

use sinbad::SinBAD;
use std::env;
use crate::sinbad::SinBADError;

pub fn sinbad() -> Result<SinBAD, SinBADError> {
    let timeout_cmd = env::var("TIMEOUT_CMD")?;
    let sinbad_cmd = env::var("SINBAD_CMD")?;
    let accent_dir = env::var("ACCENT_DIR")?;
    let sin = SinBAD::new(timeout_cmd, sinbad_cmd, accent_dir);

    Ok(sin)
}

pub fn invoke(sin: &SinBAD, duration: usize, backend: &str, depth: usize, gp: &str, lp: &str)
              -> Result<bool, SinBADError> {
    let res = sin.invoke(duration, backend, depth, gp, lp)?;
    Ok(res.is_amb())
}


