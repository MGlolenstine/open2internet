use std::net::AddrParseError;

use igd::{AddPortError, RemovePortError, SearchError};
use netstat::Error as NetstatError;
use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Unable to find IPv4 address on this device"))]
    NoIPv4Found,
    #[snafu(display("Failed to parse the IPv4 address: {e}"))]
    AddrParseError { e: AddrParseError },
    #[snafu(display("Failed to parse the ports from the netstat utility: {e}"))]
    NetstatError { e: NetstatError },
    #[snafu(display("An IO error occured: {e}"))]
    IoError { e: std::io::Error },
    #[snafu(display("An issue with fetching public address has occured: {e}"))]
    PublicAddress { e: public_ip_address::error::Error },
    #[snafu(display("An issue with IGD search has occurred: {e}"))]
    Search { e: SearchError },
    #[snafu(display("An issue with IGD add port has occurred: {e}"))]
    AddPort { e: AddPortError },
    #[snafu(display("An issue with IGD remove port has occurred: {e}"))]
    RemovePort { e: RemovePortError },
}

impl From<AddrParseError> for Error {
    fn from(value: AddrParseError) -> Self {
        Self::AddrParseError { e: value }
    }
}

impl From<NetstatError> for Error {
    fn from(value: NetstatError) -> Self {
        Self::NetstatError { e: value }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError { e: value }
    }
}

impl From<public_ip_address::error::Error> for Error {
    fn from(value: public_ip_address::error::Error) -> Self {
        Self::PublicAddress { e: value }
    }
}

impl From<SearchError> for Error {
    fn from(value: SearchError) -> Self {
        Self::Search { e: value }
    }
}

impl From<AddPortError> for Error {
    fn from(value: AddPortError) -> Self {
        Self::AddPort { e: value }
    }
}

impl From<RemovePortError> for Error {
    fn from(value: RemovePortError) -> Self {
        Self::RemovePort { e: value }
    }
}
