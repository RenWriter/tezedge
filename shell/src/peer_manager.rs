// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::net::SocketAddr;
use std::time::Duration;

use dns_lookup::LookupError;
use rand::seq::SliceRandom;
use riker::actors::*;
use slog::{info, Logger, warn};

use networking::p2p::network_channel::{NetworkChannelMsg, NetworkChannelRef};
use networking::p2p::network_manager::{ConnectToPeer, NetworkManagerRef};
use networking::p2p::peer::{PeerRef, SendMessage};
use tezos_messages::p2p::encoding::prelude::*;

use crate::{subscribe_to_actor_terminated, subscribe_to_network_events};

/// Check peer threshold
#[derive(Clone, Debug)]
pub struct CheckPeerCount;

#[derive(Copy, Clone, Debug)]
pub struct Threshold {
    low: usize,
    high: usize,
}

impl Threshold {
    pub fn new(low: usize, high: usize) -> Self {
        assert!(low <= high, "low must be less than or equal to high");
        Threshold { low, high }
    }
}

#[actor(CheckPeerCount, NetworkChannelMsg, SystemEvent)]
pub struct PeerManager {
    /// All events generated by the network layer will end up in this channel
    network_channel: NetworkChannelRef,
    network: NetworkManagerRef,
    threshold: Threshold,
    peers: HashMap<ActorUri, PeerRef>,
    bootstrap_addresses: Vec<String>,
    potential_peers: HashSet<SocketAddr>,
    log: Logger,
}

pub type PeerManagerRef = ActorRef<PeerManagerMsg>;

impl PeerManager {
    pub fn actor(sys: &impl ActorRefFactory,
               event_channel: NetworkChannelRef,
               network: NetworkManagerRef,
               bootstrap_addresses: &[String],
               initial_peers: &[SocketAddr],
               threshold: Threshold,
               log: Logger) -> Result<PeerManagerRef, CreateError> {

        sys.actor_of(
            Props::new_args(PeerManager::new, (event_channel, bootstrap_addresses.to_vec(), HashSet::from_iter(initial_peers.to_vec()), network, threshold, log)),
            PeerManager::name())
    }

    /// The `PeerManager` is intended to serve as a singleton actor so that's why
    /// we won't support multiple names per instance.
    fn name() -> &'static str {
        "peer-manager"
    }

    fn new((event_channel, bootstrap_addresses, potential_peers, network, threshold, log): (NetworkChannelRef, Vec<String>, HashSet<SocketAddr>, NetworkManagerRef, Threshold, Logger)) -> Self {
        PeerManager { network_channel: event_channel, network, bootstrap_addresses, threshold, peers: HashMap::new(), potential_peers, log }
    }

    fn discover_peers(&mut self) {
        if self.peers.is_empty() {
            info!(self.log, "Doing peer DNS lookup"; "bootstrap_addresses" => format!("{:?}", &self.bootstrap_addresses));
            dns_lookup_peers(&self.bootstrap_addresses, self.log.clone()).iter()
                .for_each(|i| {
                    info!(self.log, "Found potential peer"; "ip" => i);
                    self.potential_peers.insert(*i);
                });
        } else {
            self.peers.values()
                .for_each(|peer| peer.tell(SendMessage::new(PeerMessage::Bootstrap.into()), None));
        }
    }
}

impl Actor for PeerManager {
    type Msg = PeerManagerMsg;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
        subscribe_to_actor_terminated(ctx.system.sys_events(), ctx.myself());
        subscribe_to_network_events(&self.network_channel, ctx.myself());

        ctx.schedule::<Self::Msg, _>(
            Duration::from_secs(3),
            Duration::from_secs(10),
            ctx.myself(),
            None,
            CheckPeerCount.into());
    }

    fn sys_recv(&mut self, ctx: &Context<Self::Msg>, msg: SystemMsg, sender: Option<BasicActorRef>) {
        if let SystemMsg::Event(evt) = msg {
            self.receive(ctx, evt, sender);
        }
    }

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
        self.receive(ctx, msg, sender);
    }
}

impl Receive<SystemEvent> for PeerManager {
    type Msg = PeerManagerMsg;

    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: SystemEvent, _sender: Option<BasicActorRef>) {
        if let SystemEvent::ActorTerminated(evt) = msg {
            if let Some(_) = self.peers.remove(evt.actor.uri()) {
                ctx.myself().tell(CheckPeerCount, None);
            }
        }
    }
}

impl Receive<CheckPeerCount> for PeerManager {
    type Msg = PeerManagerMsg;

    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: CheckPeerCount, _sender: Sender) {
        if self.peers.len() < self.threshold.low {
            warn!(self.log, "Peer count is too low"; "actual" => self.peers.len(), "required" => self.threshold.low);
            if self.potential_peers.len() < self.threshold.low {
                self.discover_peers();
            }

            let num_required_peers = self.threshold.low - self.peers.len();
            let mut addresses_to_connect = self.potential_peers.iter().cloned().collect::<Vec<SocketAddr>>();
            // randomize peers as a security measurement
            addresses_to_connect.shuffle(&mut rand::thread_rng());
            addresses_to_connect
                .drain(0..cmp::min(num_required_peers, addresses_to_connect.len()))
                .for_each(|address| {
                    self.potential_peers.remove(&address);
                    self.network.tell(ConnectToPeer { address }, ctx.myself().into())
                });
        } else if self.peers.len() > self.threshold.high {
            warn!(self.log, "Peer count is too high. Some peers will be stopped"; "actual" => self.peers.len(), "limit" => self.threshold.high);

            // stop some peers
            self.peers.values()
                .take(self.peers.len() - self.threshold.high)
                .for_each(|peer| ctx.system.stop(peer.clone()))
        }
    }
}

impl Receive<NetworkChannelMsg> for PeerManager {
    type Msg = PeerManagerMsg;

    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: NetworkChannelMsg, _sender: Sender) {
        match msg {
            NetworkChannelMsg::PeerCreated(msg) => {
                self.peers.insert(msg.peer.uri().clone(), msg.peer);
            }
            NetworkChannelMsg::PeerMessageReceived(received) => {
                let messages = received.message.messages();
                messages.iter()
                    .for_each(|message| if let PeerMessage::Advertise(message) = message {
                        info!(self.log, "Received advertise message from peer"; "peer" => received.peer.name());
                        let sock_addresses = message.id().iter()
                            .filter_map(|str_ip_port| str_ip_port.parse().ok())
                            .collect::<Vec<SocketAddr>>();
                        self.potential_peers.extend(sock_addresses);
                        ctx.myself().tell(CheckPeerCount, None);
                    })
            }
            _ => ()
        }
    }
}

fn dns_lookup_peers(bootstrap_addresses: &[String], log: Logger) -> HashSet<SocketAddr> {
    let mut resolved_peers = HashSet::new();
    for address in bootstrap_addresses {
        match resolve_dns_name_to_peer_address(&address) {
            Ok(peers) => {
                resolved_peers.extend(&peers)
            },
            Err(e) => {
                warn!(log, "DNS lookup failed"; "ip" => address, "reason" => format!("{:?}", e))
            }
        }
    }
    resolved_peers
}

fn resolve_dns_name_to_peer_address(address: &str) -> Result<Vec<SocketAddr>, LookupError> {
    let addrs = dns_lookup::getaddrinfo(Some(address), None, None)?
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .map(|mut info| {
            info.sockaddr.set_port(9732);
            info.sockaddr
        })
        .collect();
    Ok(addrs)
}

