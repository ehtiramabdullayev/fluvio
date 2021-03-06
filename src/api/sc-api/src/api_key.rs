//!
//! # SC Api Keys
//!
//! Stores Api Keys supported by the SC.
//!

use kf_protocol::derive::Encode;
use kf_protocol::derive::Decode;

#[derive(Encode, Decode, PartialEq, Debug, Clone, Copy)]
#[repr(u16)]
pub enum ScApiKey {
    // Mixed
    ApiVersion = 18,

    // Kafka
    KfMetadata = 3,

    // Topics
    FlvCreateTopics = 2001,
    FlvDeleteTopics = 2002,
    FlvFetchTopics = 2003,
    FlvTopicComposition = 2004,

    // Custom SPUs
    FlvCreateCustomSpus = 2005,
    FlvDeleteCustomSpus = 2006,
    FlvFetchSpus = 2007,

    // SPU Groups
    FlvCreateSpuGroups = 2008,
    FlvDeleteSpuGroups = 2009,
    FlvFetchSpuGroups = 2010,
}

impl Default for ScApiKey {
    fn default() -> ScApiKey {
        ScApiKey::ApiVersion
    }
}
