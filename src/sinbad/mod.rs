use std::{
    env,
    process::Command,
    fmt,
    string::FromUtf8Error,
};

#[derive(Debug)]
pub struct SinBADError {
    msg: String,
}

impl SinBADError {
    pub(crate) fn new(msg: String) -> Self {
        Self {
            msg
        }
    }
}

impl fmt::Display for SinBADError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<FromUtf8Error> for SinBADError {
    fn from(e: FromUtf8Error) -> Self {
        SinBADError::new(e.to_string())
    }
}

impl From<env::VarError> for SinBADError {
    fn from(e: env::VarError) -> Self {
        SinBADError::new(e.to_string())
    }
}

pub struct SinBADOutput {
    pub r_code: Option<i32>,
    pub out: String,
    pub err: String,
}

impl SinBADOutput {
    fn new(r_code: Option<i32>, out: String, err: String) -> Self {
        Self {
            r_code,
            out,
            err,
        }
    }

    pub fn is_amb(&self) -> bool {
        for l in self.err.split("\n") {
            if l.contains("Grammar ambiguity detected") {
                return true;
            }
        }
        false
    }
}

pub(crate) struct SinBAD {
    timeout_cmd: String,
    duration: usize,
    sinbad_cmd: String,
    accent_dir: String,
    backend: String,
    depth: usize,
    weight: Option<f32>,
}

impl SinBAD {
    pub fn new(
        timeout_cmd: String,
        duration: usize,
        sinbad_cmd: String,
        accent_dir: String,
        backend: String,
        depth: usize,
        weight: Option<f32>) -> Self {
        Self {
            timeout_cmd,
            duration,
            sinbad_cmd,
            accent_dir,
            backend,
            depth,
            weight,
        }
    }

    pub(crate) fn invoke(&self, gp: &str, lp: &str) -> Result<SinBADOutput, SinBADError> {
        let mut cmd = Command::new(&self.timeout_cmd);
        cmd.env("ACCENT_DIR", &self.accent_dir);
        let args: &[&str] = &[
            &self.duration.to_string(),
            &self.sinbad_cmd,
            "-b",
            &self.backend,
            "-d",
            &self.depth.to_string(),
            gp,
            lp
        ];
        cmd.args(args);
        let output = cmd.output()
            .map_err(|e| SinBADError::new(e.to_string()))?;
        let out = String::from_utf8(output.stdout)?;
        let err = String::from_utf8(output.stderr)?;

        Ok(SinBADOutput::new(output.status.code(), out, err))
    }
}
