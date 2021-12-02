//! # params
//!
//! Ssh config params for host rule

/**
 * MIT License
 *
 * ssh2-config - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use super::{Duration, PathBuf};

/// Describes the ssh configuration.
/// Configuration is describes in this document: <http://man.openbsd.org/OpenBSD-current/man5/ssh_config.5>
/// Only arguments supported by libssh2 are implemented
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HostParams {
    /// Specifies to use the specified address on the local machine as the source address of the connection
    pub bind_address: Option<String>,
    /// Use the specified address on the local machine as the source address of the connection
    pub bind_interface: Option<String>,
    /// Specifies which algorithms are allowed for signing of certificates by certificate authorities
    pub ca_signature_algorithms: Option<Vec<String>>,
    /// Specifies a file from which the user's certificate is read
    pub certificate_file: Option<PathBuf>,
    /// Specifies the ciphers allowed for protocol version 2 in order of preference
    pub ciphers: Option<Vec<String>>,
    /// Specifies whether to use compression
    pub compression: Option<bool>,
    /// Specifies the number of attempts to make before exiting
    pub connection_attemps: Option<usize>,
    /// Specifies the timeout used when connecting to the SSH server
    pub connect_timeout: Option<Duration>,
    /// Specifies the real host name to log into
    pub host_name: Option<String>,
    /// Specifies the MAC (message authentication code) algorithms in order of preference
    pub mac: Option<Vec<String>>,
    /// Specifies the signature algorithms that will be used for public key authentication
    pub pubkey_accepted_algorithms: Option<Vec<String>>,
    /// Specifies whether to try public key authentication using SSH keys
    pub pubkey_authentication: Option<bool>,
    /// Specifies that a TCP port on the remote machine be forwarded over the secure channel
    pub remote_forward: Option<u16>,
    /// Specifies whether to send TCP keepalives to the other side
    pub tcp_keep_alive: Option<bool>,
}

impl HostParams {
    /// Override current params with params of `b`
    pub fn merge(&mut self, b: &Self) {
        if let Some(bind_address) = b.bind_address.clone() {
            self.bind_address = Some(bind_address);
        }
        if let Some(bind_interface) = b.bind_interface.clone() {
            self.bind_interface = Some(bind_interface);
        }
        if let Some(ca_signature_algorithms) = b.ca_signature_algorithms.clone() {
            self.ca_signature_algorithms = Some(ca_signature_algorithms);
        }
        if let Some(certificate_file) = b.certificate_file.clone() {
            self.certificate_file = Some(certificate_file);
        }
        if let Some(ciphers) = b.ciphers.clone() {
            self.ciphers = Some(ciphers);
        }
        if let Some(compression) = b.compression {
            self.compression = Some(compression);
        }
        if let Some(connection_attemps) = b.connection_attemps {
            self.connection_attemps = Some(connection_attemps);
        }
        if let Some(connect_timeout) = b.connect_timeout {
            self.connect_timeout = Some(connect_timeout);
        }
        if let Some(host_name) = b.host_name.clone() {
            self.host_name = Some(host_name);
        }
        if let Some(mac) = b.mac.clone() {
            self.mac = Some(mac);
        }
        if let Some(pubkey_accepted_algorithms) = b.pubkey_accepted_algorithms.clone() {
            self.pubkey_accepted_algorithms = Some(pubkey_accepted_algorithms);
        }
        if let Some(pubkey_authentication) = b.pubkey_authentication {
            self.pubkey_authentication = Some(pubkey_authentication);
        }
        if let Some(remote_forward) = b.remote_forward {
            self.remote_forward = Some(remote_forward);
        }
        if let Some(tcp_keep_alive) = b.tcp_keep_alive {
            self.tcp_keep_alive = Some(tcp_keep_alive);
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_initialize_params() {
        let params = HostParams::default();
        assert!(params.bind_address.is_none());
        assert!(params.bind_interface.is_none());
        assert!(params.ca_signature_algorithms.is_none());
        assert!(params.certificate_file.is_none());
        assert!(params.ciphers.is_none());
        assert!(params.compression.is_none());
        assert!(params.connection_attemps.is_none());
        assert!(params.connect_timeout.is_none());
        assert!(params.host_name.is_none());
        assert!(params.mac.is_none());
        assert!(params.pubkey_accepted_algorithms.is_none());
        assert!(params.pubkey_authentication.is_none());
        assert!(params.remote_forward.is_none());
        assert!(params.tcp_keep_alive.is_none());
    }

    #[test]
    fn should_merge_params() {
        let mut params = HostParams::default();
        let mut b = HostParams::default();
        b.bind_address = Some(String::from("pippo"));
        b.bind_interface = Some(String::from("tun0"));
        b.ca_signature_algorithms = Some(vec![]);
        b.certificate_file = Some(PathBuf::default());
        b.ciphers = Some(vec![]);
        b.compression = Some(true);
        b.connect_timeout = Some(Duration::from_secs(1));
        b.connection_attemps = Some(3);
        b.host_name = Some(String::from("192.168.1.2"));
        b.mac = Some(vec![]);
        b.pubkey_accepted_algorithms = Some(vec![]);
        b.pubkey_authentication = Some(true);
        b.remote_forward = Some(32);
        b.tcp_keep_alive = Some(true);
        params.merge(&b);
        assert!(params.bind_address.is_some());
        assert!(params.bind_interface.is_some());
        assert!(params.ca_signature_algorithms.is_some());
        assert!(params.certificate_file.is_some());
        assert!(params.ciphers.is_some());
        assert!(params.compression.is_some());
        assert!(params.connection_attemps.is_some());
        assert!(params.connect_timeout.is_some());
        assert!(params.host_name.is_some());
        assert!(params.mac.is_some());
        assert!(params.pubkey_accepted_algorithms.is_some());
        assert!(params.pubkey_authentication.is_some());
        assert!(params.remote_forward.is_some());
        assert!(params.tcp_keep_alive.is_some());
        // merge twices
        b.tcp_keep_alive = None;
        params.merge(&b);
        assert_eq!(params.tcp_keep_alive.unwrap(), true);
    }
}