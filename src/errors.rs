use std::{fmt, process};

use crate::container::MINIMAL_KERNEL_VERSION;

#[derive(Debug)]
pub enum Errcode {
    ArgumentInvalid(&'static str),
    NotSupported(u8),
    SocketError(u8),
    ContainerError(u8),
}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        1 // Everything != 0 will be treated as an error
    }
}

#[allow(clippy::match_single_binding)]
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define what behaviour for each variant of the enum
        match &self {
            Errcode::NotSupported(errtype) => match errtype {
                0 => write!(
                    f,
                    "Minimal kernel version required: {}",
                    MINIMAL_KERNEL_VERSION
                ),
                1 => write!(f, "Only x86_64 architecture is supported"),
                _ => write!(f, "{:?} (unknown id)", self),
            },

            // Message to display when an argument is invalid
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),

            _ => write!(f, "{:?}", self), // For any variant not previously covered
        }
    }
}

// Get the result from a function, and exit the process with the correct error code
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        // If it's a success, return 0
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            process::exit(0);
        }

        // If there's an error, print an error message and return the retcode
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
            process::exit(retcode);
        }
    }
}
