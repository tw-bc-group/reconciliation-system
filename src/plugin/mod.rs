use std::{
    convert::AsRef,
    fs::read_dir,
    path::{Path, PathBuf},
};

use anyhow::Result;

mod flush;

pub mod prelude {
    pub use super::flush::{Flush as FlushPlugin, *};
    pub use serde::Deserialize;
    pub use serde_json::Value;
}

fn dylib_extension() -> &'static str {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            "dll"
        } else if #[cfg(target_os = "macos")] {
            "dylib"
        } else if #[cfg(target_os = "linux")] {
            "so"
        } else {
            panic!("Unsupported OS, {:?}", os_info::get())
        }
    }
}

fn plugin_list<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
    read_dir(dir.as_ref()).map_err(Into::into).map(|read_res| {
        debug!("plugin_list, dir: {:?}", dir.as_ref());
        read_res
            .flat_map(|res| -> Result<_, std::io::Error> { Ok(res?) })
            .map(|entry| entry.path())
            .filter(|path| {
                debug!("plugin_list, path: {:?}", path);
                path.is_file()
                    && path
                        .extension()
                        .map(|extension| extension == dylib_extension())
                        .unwrap_or_else(|| false)
            })
            .collect()
    })
}
