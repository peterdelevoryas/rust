use crate::ffi::OsStr;
use crate::fmt;
use crate::io;
use crate::marker::PhantomData;
use crate::num::NonZeroI32;
use crate::path::Path;
use crate::sys::fs::File;
use crate::sys::pipe::AnonPipe;
use crate::sys::unsupported;
use crate::sys_common::process::{CommandEnv, CommandEnvs};

pub use crate::ffi::OsString as EnvKey;

////////////////////////////////////////////////////////////////////////////////
// Command
////////////////////////////////////////////////////////////////////////////////

pub struct Command {
    env: CommandEnv,
}

// passed back to std::process with the pipes connected to the child, if any
// were requested
pub struct StdioPipes {
    pub stdin: Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}

// FIXME: This should be a unit struct, so we can always construct it
// The value here should be never used, since we cannot spawn processes.
pub enum Stdio {
    Inherit,
    Null,
    MakePipe,
}

impl Command {
    pub fn new(_program: &OsStr) -> Command {
        Command { env: Default::default() }
    }

    pub fn arg(&mut self, _arg: &OsStr) {}

    pub fn env_mut(&mut self) -> &mut CommandEnv {
        &mut self.env
    }

    pub fn cwd(&mut self, _dir: &OsStr) {}

    pub fn stdin(&mut self, _stdin: Stdio) {}

    pub fn stdout(&mut self, _stdout: Stdio) {}

    pub fn stderr(&mut self, _stderr: Stdio) {}

    pub fn get_program(&self) -> &OsStr {
        panic!("unsupported")
    }

    pub fn get_args(&self) -> CommandArgs<'_> {
        CommandArgs { _p: PhantomData }
    }

    pub fn get_envs(&self) -> CommandEnvs<'_> {
        self.env.iter()
    }

    pub fn get_current_dir(&self) -> Option<&Path> {
        None
    }

    pub fn spawn(
        &mut self,
        _default: Stdio,
        _needs_stdin: bool,
    ) -> io::Result<(Process, StdioPipes)> {
        unsupported()
    }

    pub fn output(&mut self) -> io::Result<(ExitStatus, Vec<u8>, Vec<u8>)> {
        unsupported()
    }
}

impl From<AnonPipe> for Stdio {
    fn from(pipe: AnonPipe) -> Stdio {
        pipe.diverge()
    }
}

impl From<io::Stdout> for Stdio {
    fn from(_: io::Stdout) -> Stdio {
        // FIXME: This is wrong.
        // Instead, the Stdio we have here should be a unit struct.
        panic!("unsupported")
    }
}

impl From<io::Stderr> for Stdio {
    fn from(_: io::Stderr) -> Stdio {
        // FIXME: This is wrong.
        // Instead, the Stdio we have here should be a unit struct.
        panic!("unsupported")
    }
}

impl From<File> for Stdio {
    fn from(_file: File) -> Stdio {
        // FIXME: This is wrong.
        // Instead, the Stdio we have here should be a unit struct.
        panic!("unsupported")
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct ExitStatus();

impl ExitStatus {
    pub fn exit_ok(&self) -> Result<(), ExitStatusError> {
        Ok(())
    }

    pub fn code(&self) -> Option<i32> {
        Some(0)
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<dummy exit status>")
    }
}

pub struct ExitStatusError(!);

impl Clone for ExitStatusError {
    fn clone(&self) -> ExitStatusError {
        self.0
    }
}

impl Copy for ExitStatusError {}

impl PartialEq for ExitStatusError {
    fn eq(&self, _other: &ExitStatusError) -> bool {
        self.0
    }
}

impl Eq for ExitStatusError {}

impl fmt::Debug for ExitStatusError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl Into<ExitStatus> for ExitStatusError {
    fn into(self) -> ExitStatus {
        self.0
    }
}

impl ExitStatusError {
    pub fn code(self) -> Option<NonZeroI32> {
        self.0
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitCode(bool);

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(false);
    pub const FAILURE: ExitCode = ExitCode(true);

    pub fn as_i32(&self) -> i32 {
        self.0 as i32
    }
}

impl From<u8> for ExitCode {
    fn from(code: u8) -> Self {
        match code {
            0 => Self::SUCCESS,
            1..=255 => Self::FAILURE,
        }
    }
}

pub struct Process(!);

impl Process {
    pub fn id(&self) -> u32 {
        self.0
    }

    pub fn kill(&mut self) -> io::Result<()> {
        self.0
    }

    pub fn terminate(&mut self) -> io::Result<()> {
        self.0
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        self.0
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        self.0
    }
}

pub struct CommandArgs<'a> {
    _p: PhantomData<&'a ()>,
}

impl<'a> Iterator for CommandArgs<'a> {
    type Item = &'a OsStr;
    fn next(&mut self) -> Option<&'a OsStr> {
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

impl<'a> ExactSizeIterator for CommandArgs<'a> {}

impl<'a> fmt::Debug for CommandArgs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().finish()
    }
}
