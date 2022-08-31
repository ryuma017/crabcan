use std::{fmt, process};

#[derive(Debug)]
pub enum Errcode {}

impl Errcode {
    fn get_retcode(&self) -> i32 {
        1 // Everything != 0 will be treated as an error
    }
}

#[allow(clippy::match_single_binding)]
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "{:?}", self)
        }
    }
}

pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            process::exit(0);
        },
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
            std::process::exit(retcode);
        }
    }
}