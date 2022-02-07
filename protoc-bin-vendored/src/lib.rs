//! `protoc` binary downloaded and stored inside the crate.
//!
//! Can be used to avoid downloading and installing `protoc` binary.
//!
//! # Example
//!
//! ```no_run
//! # let _ =
//! protoc_bin_vendored::protoc_bin_path().unwrap()
//! # ;
//! ```
//!
//! returns a path to a `protoc` binary packaged into the crate.

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

use std::env;
use std::fmt;
use std::path::PathBuf;

/// Error returned when a binary is not available.
#[derive(Debug)]
pub struct Error {
    os: &'static str,
    arch: &'static str,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "protoc binary cannot be found for platform {}-{}",
            self.os, self.arch
        )
    }
}

impl std::error::Error for Error {}

#[allow(non_camel_case_types)]
enum ArchCrate {
    Linux_X86_32,
    Linux_X86_64,
    Linux_Aarch_64,
    Linux_Ppcle_64,
    Macos_x86_64,
    Win32,
}

impl ArchCrate {
    fn detect() -> Result<ArchCrate, Error> {
        Ok(match (env::consts::OS, env::consts::ARCH) {
            ("linux", "x86") => ArchCrate::Linux_X86_32,
            ("linux", "x86_64") => ArchCrate::Linux_X86_64,
            ("linux", "aarch64") => ArchCrate::Linux_Aarch_64,
            ("linux", "powerpc64") => ArchCrate::Linux_Ppcle_64,
            ("macos", "x86_64") => ArchCrate::Macos_x86_64,
            // Stopgap support for Apple M1.
            // Since M1 macs can run the x86_64 binary using Rosetta emulation,
            // this updates protoc_bin_path to reuse the protoc-osx-x86_64 binary for macos aarch64.
            // Once Google provides precompiled binaries for Apple ARM,
            // this should be updated to use that instead.
            ("macos", "aarch64") => ArchCrate::Macos_x86_64,
            ("windows", _) => ArchCrate::Win32,
            (os, arch) => return Err(Error { os, arch }),
        })
    }
}

/// Return a path to `protoc` binary.
///
/// This function returns an error when binary is not available for
/// the current operating system and architecture.
pub fn protoc_bin_path() -> Result<PathBuf, Error> {
    Ok(match ArchCrate::detect()? {
        ArchCrate::Linux_X86_32 => protoc_bin_vendored_linux_x86_32::protoc_bin_path(),
        ArchCrate::Linux_X86_64 => protoc_bin_vendored_linux_x86_64::protoc_bin_path(),
        ArchCrate::Linux_Aarch_64 => protoc_bin_vendored_linux_aarch_64::protoc_bin_path(),
        ArchCrate::Linux_Ppcle_64 => protoc_bin_vendored_linux_ppcle_64::protoc_bin_path(),
        ArchCrate::Macos_x86_64 => protoc_bin_vendored_macos_x86_64::protoc_bin_path(),
        ArchCrate::Win32 => protoc_bin_vendored_win32::protoc_bin_path(),
    })
}

pub fn include_path() -> Result<PathBuf, Error> {
    Ok(match ArchCrate::detect()? {
        ArchCrate::Linux_X86_32 => protoc_bin_vendored_linux_x86_32::include_path(),
        ArchCrate::Linux_X86_64 => protoc_bin_vendored_linux_x86_64::include_path(),
        ArchCrate::Linux_Aarch_64 => protoc_bin_vendored_linux_aarch_64::include_path(),
        ArchCrate::Linux_Ppcle_64 => protoc_bin_vendored_linux_ppcle_64::include_path(),
        ArchCrate::Macos_x86_64 => protoc_bin_vendored_macos_x86_64::include_path(),
        ArchCrate::Win32 => protoc_bin_vendored_win32::include_path(),
    })
}

#[cfg(test)]
mod test {
    use std::io::Read;
    use std::process;

    #[test]
    fn include_path() {
        assert!(crate::include_path().unwrap().join("google/protobuf/descriptor.proto").exists());
    }

    #[test]
    fn smoke() {
        let process = process::Command::new(crate::protoc_bin_path().unwrap())
            .arg("--version")
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::piped())
            .spawn()
            .unwrap();
        let mut stdout = String::new();
        process.stdout.unwrap().read_to_string(&mut stdout).unwrap();
        assert!(stdout.contains("libprotoc"), "stdout is: {:?}", stdout)
    }
}
