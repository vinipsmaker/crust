// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use endpoint::Endpoint;
use connection::Connection;
use std::net::UdpSocket;
use socket_addr::SocketAddr;
use sodiumoxide::crypto::sign::PublicKey;
use std::io;

/// The result of a `Service::prepare_contact_info` call.
#[derive(Debug)]
pub struct ContactInfoResult {
    /// The token that was passed to `prepare_contact_info`.
    pub result_token: u32,
    /// The new contact info, if successful.
    pub result: io::Result<OurContactInfo>,
}

/// Contact info generated by a call to `Service::prepare_contact_info`.
#[derive(Debug)]
pub struct OurContactInfo {
    /// The mapped UDP socket that may be used for this connection.
    pub socket: UdpSocket,
    /// Secret data used for rendezvous connect.
    pub secret: Option<[u8; 4]>,
    /// Our tcp listening addresses.
    pub static_addrs: Vec<Endpoint>,
    /// The mapped addresses of our UDP socket.
    pub rendezvous_addrs: Vec<SocketAddr>,
    /// TODO: documentation
    pub pub_key: PublicKey,
}

/// Contact info used to connect to another peer.
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct TheirContactInfo {
    /// Secret data used for rendezvous connect.
    pub secret: Option<[u8; 4]>,
    /// Their tcp listening addresses.
    pub static_addrs: Vec<Endpoint>,
    /// Their mapped addresses for UDP rendezvous connect.
    pub rendezvous_addrs: Vec<SocketAddr>,
    /// TODO: documentation
    pub pub_key: PublicKey,
}

impl OurContactInfo {
    /// Takes an `OurContactInfo` and turns it into a `TheirContactInfo`. This `TheirContactInfo`
    /// can then be sent out-of-band to another peer so that they can use it to connect to us.
    pub fn make_their_info(&self) -> TheirContactInfo {
        TheirContactInfo {
            secret: self.secret.clone(),
            static_addrs: self.static_addrs.clone(),
            rendezvous_addrs: self.rendezvous_addrs.clone(),
            pub_key: self.pub_key,
        }
    }
}

/// Enum representing different events that will be sent over the asynchronous channel to the user
/// of this module.
#[derive(Debug)]
pub enum Event {
    /// Invoked when a new message is received.  Passes the peer's public key and the message.
    NewMessage(PublicKey, Vec<u8>),
    /// Invoked when we get a new bootstrap connection.
    /// Passes the new connection and the peer's public key.
    NewBootstrapConnection {
        /// TODO Doc
        connection: Connection,
        /// TODO Doc
        their_pub_key: PublicKey,
    },
    /// Invoked when the new rendezvous connection request finishes.
    /// Passes the new connection and the peer's public key.
    NewConnection {
        /// TODO Doc
        connection: io::Result<Connection>,
        /// TODO Doc
        their_pub_key: PublicKey,
    },
    /// Invoked when a connection to a peer is lost.  Passes the peer's public key.
    LostConnection(PublicKey),
    /// TODO THIS IS WRONG - Invoked when a new bootstrap connection to a peer is established.
    /// Passes the peer's endpoint.
    BootstrapFinished,
    /// Invoked when a new bootstrap connection to a peer is established.
    /// Passes the peer's endpoint.
    ExternalEndpoints(Vec<Endpoint>),
    /// Invoked as a result to the call of `Service::prepare_contact_info`.
    ContactInfoPrepared(ContactInfoResult),
}
