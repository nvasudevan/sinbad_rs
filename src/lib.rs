pub(crate) mod sinbad;
use sinbad::SinBAD;
use std::env;
use crate::sinbad::SinBADError;

pub fn sinbad(backend: String, depth: usize, gp: &str, lp: &str, duration: usize)
    -> Result<bool, SinBADError> {
    let timeout_cmd = env::var("TIMEOUT_CMD")?;
    let sinbad_cmd = env::var("SINBAD_CMD")?;
    let accent_dir = env::var("ACCENT_DIR")?;
    let sin = SinBAD::new(
        timeout_cmd,
        duration,
        sinbad_cmd,
        accent_dir,
        backend,
        depth,
        None
    );
    let res = sin.invoke(gp, lp)?;

    Ok(res.is_amb())
}
