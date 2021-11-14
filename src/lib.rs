pub mod sinbad;

use sinbad::SinBAD;
use std::env;
use crate::sinbad::SinBADError;
use crate::sinbad::SinBADOutput;

/// Defines SinBAD input
pub struct SinBADInput {
    /// SinBAD backend to apply
    pub backend: String,
    /// SinBAD threshold depth
    pub depth: usize,
    /// SinBAD running time for each CFG
    pub duration: usize,
}

impl SinBADInput {
    pub fn new(backend: String, depth: usize, duration: usize) -> Self {
        Self {
            backend,
            depth,
            duration
        }
    }
}

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

pub fn invoke(sin: &SinBAD, sin_input: &SinBADInput, gp: &str, lp: &str)
              -> Result<SinBADOutput, SinBADError> {
    sin.invoke(sin_input, gp, lp)
}

#[cfg(test)]
mod tests {
    use crate::{sinbad, invoke, SinBADInput};

    #[test]
    /// requires the following environment variables:
    /// SINBAD_CMD - path to sinbad python script
    /// TIMEOUT_CMD - path to `timeout` command (mostly `/usr/bin/timeout`)
    /// ACCENT_DIR - path to the accent accent compiler directory
    fn test_sinbad() {
        let sin_input = SinBADInput::new("dynamic1".to_owned(), 10, 5);
        let gp = "./grammars/amb.acc";
        let lp = "./grammars/general.lex";
        let sin = sinbad()
            .expect("Unable to create a SinBAD instance!");
        let res = invoke(&sin, &sin_input, gp, lp);
        match res {
            Ok(sin_out) => { println!("{} => {}", gp, sin_out.is_amb()) }
            Err(e) => {
                eprintln!("Error:\n{}", e.to_string());
            }
        }
    }
}