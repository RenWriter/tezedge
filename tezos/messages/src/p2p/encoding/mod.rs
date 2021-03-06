// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

//! Provides definitions of p2p messages.

pub mod advertise;
pub mod operation;
pub mod version;
pub mod connection;
pub mod metadata;
pub mod ack;
pub mod current_branch;
pub mod current_head;
pub mod mempool;
pub mod block_header;
pub mod protocol;
pub mod operations_for_blocks;
pub mod peer;
pub mod swap;
pub mod deactivate;
pub mod operation_hashes_for_blocks;

pub mod prelude {
    pub use super::ack::AckMessage;
    pub use super::advertise::AdvertiseMessage;
    pub use super::block_header::{BlockHeader, BlockHeaderBuilder, BlockHeaderMessage, GetBlockHeadersMessage};
    pub use super::connection::ConnectionMessage;
    pub use super::current_branch::{CurrentBranch, CurrentBranchMessage, GetCurrentBranchMessage};
    pub use super::current_head::{CurrentHeadMessage, GetCurrentHeadMessage};
    pub use super::mempool::Mempool;
    pub use super::metadata::MetadataMessage;
    pub use super::operation::{GetOperationsMessage, Operation, OperationMessage};
    pub use super::operations_for_blocks::{GetOperationsForBlocksMessage, OperationsForBlock, OperationsForBlocksMessage, Path, PathLeft, PathRight};
    pub use super::peer::{PeerMessage, PeerMessageResponse};
    pub use super::protocol::{Component, GetProtocolsMessage, Protocol, ProtocolMessage};
    pub use super::version::NetworkVersion;
    pub use super::swap::SwapMessage;
    pub use super::deactivate::DeactivateMessage;
    pub use super::operation_hashes_for_blocks::{GetOperationHashesForBlocksMessage, OperationHashesForBlocksMessage};
}