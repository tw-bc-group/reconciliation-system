use std::{
    convert::AsRef,
    fs::read_dir,
    path::{Path, PathBuf},
};

use anyhow::Result;

#[macro_export]
macro_rules! plugin_load {
    ($dir:expr, $name:expr, $probe:path) => {{
        use libloading::{Library, Symbol};

        type ProbePlugin = unsafe extern "C" fn() -> Box<dyn $probe>;

        crate::plugin::dylib_list($dir).map(|dylib_list| {
            dylib_list.into_iter().fold(Vec::new(), |mut acc, dylib| {
                #[cfg(target_os = "linux")]
                let load_res = libloading::os::unix::Library::open(Some(&dylib), 0x2 | 0x1000);
                #[cfg(not(target_os = "linux"))]
                let load_res = Library::new(&dylib);

                let library = match load_res {
                    Ok(library) => library,
                    Err(err) => panic!("failed to load library, {:?}", err),
                };

                unsafe {
                    let probe = match library.get::<Symbol<ProbePlugin>>($name) {
                        Ok(probe) => probe,
                        Err(err) => {
                            panic!("failed to probe plugin for library {:?}, {:?}", dylib, err);
                        }
                    };
                    acc.push(probe());
                }
                acc
            })
        })
    }};
}

#[macro_use]
mod flush;

pub mod prelude {
    pub use super::flush::*;
    pub use anyhow::Result;
    pub use serde::Deserialize;
    pub use serde_json::{json, Value};
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

fn dylib_list<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
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
