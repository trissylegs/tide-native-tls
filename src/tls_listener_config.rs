use async_std::path::PathBuf;
use native_tls::Identity;
use crate::tls_acceptor::TlsAcceptor;

#[derive(Debug)]
pub(crate) enum TlsListenerConfig {
    Unconfigured,
    Path {
        path: PathBuf,
        password: String,
    },
    Identity(Identity),
    Acceptor(TlsAcceptor),
}

impl Default for TlsListenerConfig {
    fn default() -> Self {
        Self::Unconfigured
    }
}