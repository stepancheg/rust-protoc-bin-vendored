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
//!
//! Crate also packs `.proto` files distributed with protobuf:
//!
//! ```no_run
//! # let _ =
//! protoc_bin_vendored::include_path().unwrap()
//! # ;
//! ```

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
    #[cfg(any(all(target_os = "linux", target_arch = "x86"), test))]
    Linux_X86_32,
    #[cfg(any(all(target_os = "linux", target_arch = "x86_64"), test))]
    Linux_X86_64,
    #[cfg(any(all(target_os = "linux", target_arch = "aarch64"), test))]
    Linux_Aarch_64,
    #[cfg(any(all(target_os = "linux", target_arch = "powerpc64"), test))]
    Linux_Ppcle_64,
    #[cfg(any(all(target_os = "linux", target_arch = "s390x"), test))]
    Linux_S390_64,
    #[cfg(any(all(target_os = "macos", target_arch = "aarch64"), test))]
    Macos_Aarch_64,
    #[cfg(any(all(target_os = "macos", target_arch = "x86_64"), test))]
    Macos_x86_64,
    #[cfg(any(target_os = "windows", test))]
    Win32,
}

impl ArchCrate {
    fn detect() -> Result<ArchCrate, Error> {
        Ok(match (env::consts::OS, env::consts::ARCH) {
            #[cfg(any(all(target_os = "linux", target_arch = "x86"), test))]
            ("linux", "x86") => ArchCrate::Linux_X86_32,
            #[cfg(any(all(target_os = "linux", target_arch = "x86_64"), test))]
            ("linux", "x86_64") => ArchCrate::Linux_X86_64,
            #[cfg(any(all(target_os = "linux", target_arch = "aarch64"), test))]
            ("linux", "aarch64") => ArchCrate::Linux_Aarch_64,
            #[cfg(any(all(target_os = "linux", target_arch = "powerpc64"), test))]
            ("linux", "powerpc64") => ArchCrate::Linux_Ppcle_64,
            #[cfg(any(all(target_os = "linux", target_arch = "s390x"), test))]
            ("linux", "s390x") => ArchCrate::Linux_S390_64,
            #[cfg(any(all(target_os = "macos", target_arch = "aarch64"), test))]
            ("macos", "aarch64") => ArchCrate::Macos_Aarch_64,
            #[cfg(any(all(target_os = "macos", target_arch = "x86_64"), test))]
            ("macos", "x86_64") => ArchCrate::Macos_x86_64,
            #[cfg(any(target_os = "windows", test))]
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
        #[cfg(any(all(target_os = "linux", target_arch = "x86"), test))]
        ArchCrate::Linux_X86_32 => protoc_bin_vendored_linux_x86_32::protoc_bin_path(),
        #[cfg(any(all(target_os = "linux", target_arch = "x86_64"), test))]
        ArchCrate::Linux_X86_64 => protoc_bin_vendored_linux_x86_64::protoc_bin_path(),
        #[cfg(any(all(target_os = "linux", target_arch = "aarch64"), test))]
        ArchCrate::Linux_Aarch_64 => protoc_bin_vendored_linux_aarch_64::protoc_bin_path(),
        #[cfg(any(all(target_os = "linux", target_arch = "powerpc64"), test))]
        ArchCrate::Linux_Ppcle_64 => protoc_bin_vendored_linux_ppcle_64::protoc_bin_path(),
        #[cfg(any(all(target_os = "linux", target_arch = "s390x"), test))]
        ArchCrate::Linux_S390_64 => protoc_bin_vendored_linux_s390_64::protoc_bin_path(),
        #[cfg(any(all(target_os = "macos", target_arch = "aarch64"), test))]
        ArchCrate::Macos_Aarch_64 => protoc_bin_vendored_macos_aarch_64::protoc_bin_path(),
        #[cfg(any(all(target_os = "macos", target_arch = "x86_64"), test))]
        ArchCrate::Macos_x86_64 => protoc_bin_vendored_macos_x86_64::protoc_bin_path(),
        #[cfg(any(target_os = "windows", test))]
        ArchCrate::Win32 => protoc_bin_vendored_win32::protoc_bin_path(),
    })
}

pub(crate) fn include_path_for_arch(arch_crate: &ArchCrate) -> PathBuf {
    match arch_crate {
        #[cfg(any(all(target_os = "linux", target_arch = "x86"), test))]
        ArchCrate::Linux_X86_32 => protoc_bin_vendored_linux_x86_32::include_path(),
        #[cfg(any(all(target_os = "linux", target_arch = "x86_64"), test))]
        ArchCrate::Linux_X86_64 => protoc_bin_vendored_linux_x86_64::include_path(),
        #[cfg(any(all(target_os = "linux", target_arch = "aarch64"), test))]
        ArchCrate::Linux_Aarch_64 => protoc_bin_vendored_linux_aarch_64::include_path(),
        #[cfg(any(all(target_os = "linux", target_arch = "powerpc64"), test))]
        ArchCrate::Linux_Ppcle_64 => protoc_bin_vendored_linux_ppcle_64::include_path(),
        #[cfg(any(all(target_os = "linux", target_arch = "s390x"), test))]
        ArchCrate::Linux_S390_64 => protoc_bin_vendored_linux_s390_64::include_path(),
        #[cfg(any(all(target_os = "macos", target_arch = "aarch64"), test))]
        ArchCrate::Macos_Aarch_64 => protoc_bin_vendored_macos_aarch_64::include_path(),
        #[cfg(any(all(target_os = "macos", target_arch = "x86_64"), test))]
        ArchCrate::Macos_x86_64 => protoc_bin_vendored_macos_x86_64::include_path(),
        #[cfg(any(target_os = "windows", test))]
        ArchCrate::Win32 => protoc_bin_vendored_win32::include_path(),
    }
}

/// Include path which contains protobuf bundled `.proto` (like `descriptor.proto`).
///
/// Include directory content is guaranteed to be identical regardless of the platform.
pub fn include_path() -> Result<PathBuf, Error> {
    Ok(include_path_for_arch(&ArchCrate::detect()?))
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::Read;
    use std::path::Path;
    use std::path::PathBuf;
    use std::process;

    use crate::include_path_for_arch;
    use crate::ArchCrate;

    #[test]
    fn include_path() {
        assert!(crate::include_path()
            .unwrap()
            .join("google/protobuf/descriptor.proto")
            .exists());
    }

    #[test]
    fn arch_crates_includes_identical() {
        fn compare_recursively(a: &Path, b: &Path) {
            assert_eq!(a.is_file(), b.is_file());
            if a.is_file() {
                let a_content = fs::read(a).unwrap();
                let b_content = fs::read(b).unwrap();
                assert_eq!(a_content, b_content);
            } else {
                let mut a_files: Vec<PathBuf> = fs::read_dir(a)
                    .unwrap()
                    .map(|e| e.unwrap().path())
                    .collect();
                let mut b_files: Vec<PathBuf> = fs::read_dir(b)
                    .unwrap()
                    .map(|e| e.unwrap().path())
                    .collect();
                a_files.sort();
                b_files.sort();
                let mut a_files = a_files.as_slice();
                let mut b_files = b_files.as_slice();
                while !a_files.is_empty() || !b_files.is_empty() {
                    let (a_next, a_rem) = a_files.split_first().unwrap();
                    let (b_next, b_rem) = b_files.split_first().unwrap();

                    compare_recursively(a_next, b_next);

                    a_files = a_rem;
                    b_files = b_rem;
                }
                assert!(a_files.is_empty());
                assert!(b_files.is_empty());
            }
        }

        compare_recursively(
            &include_path_for_arch(&ArchCrate::Linux_X86_64),
            &include_path_for_arch(&ArchCrate::Linux_X86_64),
        );
        compare_recursively(
            &include_path_for_arch(&ArchCrate::Linux_X86_64),
            &include_path_for_arch(&ArchCrate::Linux_X86_32),
        );
        compare_recursively(
            &include_path_for_arch(&ArchCrate::Linux_X86_64),
            &include_path_for_arch(&ArchCrate::Linux_Aarch_64),
        );
        compare_recursively(
            &include_path_for_arch(&ArchCrate::Linux_X86_64),
            &include_path_for_arch(&ArchCrate::Linux_Ppcle_64),
        );
        compare_recursively(
            &include_path_for_arch(&ArchCrate::Linux_X86_64),
            &include_path_for_arch(&ArchCrate::Linux_S390_64),
        );
        compare_recursively(
            &include_path_for_arch(&ArchCrate::Linux_X86_64),
            &include_path_for_arch(&ArchCrate::Macos_Aarch_64),
        );
        compare_recursively(
            &include_path_for_arch(&ArchCrate::Linux_X86_64),
            &include_path_for_arch(&ArchCrate::Macos_x86_64),
        );
        compare_recursively(
            &include_path_for_arch(&ArchCrate::Linux_X86_64),
            &include_path_for_arch(&ArchCrate::Win32),
        );
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
