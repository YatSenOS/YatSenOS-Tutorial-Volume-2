use alloc::string::String;

#[derive(Debug, Clone)]
pub enum StdIO {
    Stdin,
    Stdout,
    Stderr,
}

#[derive(Debug, Clone)]
pub enum Resource {
    Console(StdIO),
    Null,
}

impl Resource {
    pub fn read(&self, buf: &mut [u8]) -> Option<usize> {
        match self {
            Resource::Console(stdio) => match stdio {
                &StdIO::Stdin => {
                    // FIXME: read from input buffer
                    Some(0)
                }
                _ => None,
            },
            Resource::Null => Some(0),
        }
    }

    pub fn write(&self, buf: &[u8]) -> Option<usize> {
        match self {
            Resource::Console(stdio) => match *stdio {
                StdIO::Stdin => None,
                StdIO::Stdout => {
                    print!("{}", String::from_utf8_lossy(buf));
                    Some(buf.len())
                }
                StdIO::Stderr => {
                    warn!("{}", String::from_utf8_lossy(buf));
                    Some(buf.len())
                }
            },
            Resource::Null => Some(buf.len()),
        }
    }
}
