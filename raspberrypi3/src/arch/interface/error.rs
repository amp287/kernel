pub enum InterfaceError {
    NotSupported,
}

impl fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterfaceError::NotSupported => write!(f, "Operation Not Supported")
        }
    }
}

impl std::error::Error for InterfaceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InterfaceError::NotSupported => None,
        }
    }
}