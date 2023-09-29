use std::path::Path;
use std::path::PathBuf;

fn cargo_manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn include_path() -> PathBuf {
    let include_path = cargo_manifest_dir().join("include");
    assert!(include_path.is_dir());
    include_path
}

/// Return a path to `protoc` binary for linux-riscv64.
pub fn protoc_bin_path() -> PathBuf {
    let protoc_bin_path = cargo_manifest_dir()
        .join("bin")
        .join("protoc");
    assert!(
        protoc_bin_path.exists(),
        "internal: protoc not found {}",
        protoc_bin_path.display(),
    );
    protoc_bin_path
}

#[cfg(test)]
mod test {
    use crate::include_path;
    use crate::protoc_bin_path;

    #[test]
    fn smoke() {
        assert!(include_path().exists());
        assert!(protoc_bin_path().exists());
    }
}