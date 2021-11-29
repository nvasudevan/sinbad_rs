use std::{
    process::Command,
    fmt,
    string::FromUtf8Error,
};

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
    pub fn new(backend: &str, depth: usize, duration: usize) -> Self {
        Self {
            backend: backend.to_owned(),
            depth,
            duration
        }
    }
}

#[derive(Debug)]
pub struct SinBADError {
    pub r_code: Option<i32>,
    pub msg: String,
}

impl SinBADError {
    pub fn new(r_code: Option<i32>, msg: String) -> Self {
        Self {
            r_code,
            msg
        }
    }
}

impl fmt::Display for SinBADError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.r_code.unwrap_or_else(|| -1), self.msg)
    }
}

impl From<FromUtf8Error> for SinBADError {
    fn from(e: FromUtf8Error) -> Self {
        SinBADError::new(None, e.to_string())
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
            .map_err(|e| SinBADError::new(e.raw_os_error(), e.to_string()))?;
        let out = String::from_utf8(output.stdout)?;
        let err = String::from_utf8(output.stderr)?;

        // SinBAD exits with code 1 (on finding ambiguity) or 124 (on reaching timeout).
        // we return error for all other cases.
        let r_code = output.status.code();
        if let Some(r) = r_code {
            if (r == 1) || (r == 124) {
                return Ok(SinBADOutput::new(output.status.code(), out, err));
            }
        }
        let msg: String = format!("out: {}\nerr: {}", out, err);
        return Err(SinBADError::new(r_code, msg));
    }
}
