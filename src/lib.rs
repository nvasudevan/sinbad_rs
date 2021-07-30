pub mod sinbad;

use sinbad::SinBAD;
use std::env;
use crate::sinbad::SinBADError;

pub fn sinbad() -> Result<SinBAD, SinBADError> {
    let timeout_cmd = env::var("TIMEOUT_CMD")
        .map_err(|_|
            SinBADError::new("Environment variable TIMEOUT_CMD is not set".to_owned())
        )?;
    let sinbad_cmd = env::var("SINBAD_CMD")
        .map_err(|_|
            SinBADError::new("Environment variable SINBAD_CMD is not set".to_owned())
        )?;
    let accent_dir = env::var("ACCENT_DIR")
        .map_err(|_|
            SinBADError::new("Environment variable ACCENT_DIR is not set".to_owned())
        )?;
    let sin = SinBAD::new(timeout_cmd, sinbad_cmd, accent_dir);

    Ok(sin)
}

pub fn invoke(sin: &SinBAD, duration: usize, backend: &str, depth: usize, gp: &str, lp: &str)
              -> Result<bool, SinBADError> {
    let res = sin.invoke(duration, backend, depth, gp, lp)?;
    Ok(res.is_amb())
}


