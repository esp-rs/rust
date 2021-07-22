use crate::fmt;
use crate::io;
use crate::sys::pipe::AnonPipe;

use crate::sys;
use crate::sys::cvt;
use crate::sys::process::process_common::*;

use crate::io as std_io;

use libc::{c_int, pid_t};

#[path = "../../unsupported/common.rs"]
#[deny(unsafe_op_in_unsafe_fn)]
mod unsupported;

////////////////////////////////////////////////////////////////////////////////
// Command
////////////////////////////////////////////////////////////////////////////////

impl Command {
    pub fn spawn(
        &mut self,
        default: Stdio,
        needs_stdin: bool,
    ) -> io::Result<(Process, StdioPipes)> {
        unsupported::unsupported()
    }

    pub fn exec(&mut self, default: Stdio) -> io::Error {
        unsupported::unsupported_err()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Processes
////////////////////////////////////////////////////////////////////////////////

pub struct Process {
    handle: pid_t,
}

impl Process {
    pub fn id(&self) -> u32 {
        0
    }

    pub fn kill(&mut self) -> io::Result<()> {
        unsupported::unsupported()
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        unsupported::unsupported()
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        unsupported::unsupported()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitStatus(i32);

impl ExitStatus {
    pub fn success(&self) -> bool {
        self.code() == Some(0)
    }

    pub fn code(&self) -> Option<i32> {
        None
    }

    pub fn signal(&self) -> Option<i32> {
        None
    }

    pub fn core_dumped(&self) -> bool {
        false
    }

    pub fn stopped_signal(&self) -> Option<i32> {
        None
    }

    pub fn continued(&self) -> bool {
        false
    }

    pub fn into_raw(&self) -> c_int {
        0
    }
}

/// Converts a raw `c_int` to a type-safe `ExitStatus` by wrapping it without copying.
impl From<c_int> for ExitStatus {
    fn from(a: c_int) -> ExitStatus {
        ExitStatus(a as i32)
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "exit code: {}", self.0)
    }
}
