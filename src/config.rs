use std::{ffi::CString, os::unix::prelude::RawFd, path::PathBuf};

use crate::{errors::Errcode, ipc::generate_socketpair};

#[derive(Clone)]
pub struct ContainerOpts {
    // The path of the binary / executable / script to execute inside the container.
    pub path: CString,
    // The full arguments passed (including the `path` option) into the commandline.
    pub argv: Vec<CString>,
    //
    pub fd: RawFd,
    // The ID of the user inside the container. An ID of `0` means it's root (administrator).
    pub uid: u32,
    // The path of the directory we want to use as a `/` root inside our container.
    pub mount_dir: PathBuf,
}

impl ContainerOpts {
    pub fn new(
        command: String,
        uid: u32,
        mount_dir: PathBuf,
    ) -> Result<(ContainerOpts, (RawFd, RawFd)), Errcode> {
        let sockets = generate_socketpair()?;

        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read arg"))
            .collect();
        let path = argv[0].to_owned();

        Ok((
            ContainerOpts {
                path,
                argv,
                uid,
                mount_dir,
                fd: sockets.1.clone(),
            },
            sockets,
        ))
    }
}
