use std::{
    process::Command,
    fmt,
    string::FromUtf8Error,
};
use crate::SinBADInput;

#[derive(Debug)]
pub struct SinBADError {
    msg: String,
}

impl SinBADError {
    pub fn new(msg: String) -> Self {
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

pub struct SinBAD {
    timeout_cmd: String,
    sinbad_cmd: String,
    accent_dir: String,
}

impl SinBAD {
    pub fn new(timeout_cmd: String, sinbad_cmd: String, accent_dir: String) -> Self {
        Self {
            timeout_cmd,
            sinbad_cmd,
            accent_dir,
        }
    }

    pub fn invoke(&self, sin_input: &SinBADInput, gp: &str, lp: &str)
                  -> Result<SinBADOutput, SinBADError> {
        let mut cmd = Command::new(&self.timeout_cmd);
        cmd.env("ACCENT_DIR", &self.accent_dir);
        let args: &[&str] = &[
            &sin_input.duration.to_string(),
            &self.sinbad_cmd,
            "-b",
            &sin_input.backend,
            "-d",
            &sin_input.depth.to_string(),
            gp,
            lp
        ];
        cmd.args(args);
        let output = cmd.output()
            .map_err(|e| SinBADError::new(e.to_string()))?;
        let out = String::from_utf8(output.stdout)?;
        let err = String::from_utf8(output.stderr)?;

        // println!("r_code: {}", output.status.code().unwrap_or_default());
        // println!("err: *{}*", err);
        // SinBAD exits with code 1 when it finds an ambiguous string.
        // we return error for all other cases.
        if let Some(r) = output.status.code() {
            if r != 1 {
                return Err(SinBADError::new(err));
            }
        }
        Ok(SinBADOutput::new(output.status.code(), out, err))
    }
}
