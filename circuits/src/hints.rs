use std::env;
use std::time::Duration;

use log::debug;
use plonky2x::frontend::hint::simple::hint::Hint;
use plonky2x::frontend::uint::uint64::U64Variable;
use plonky2x::frontend::vars::{U32Variable, ValueStream};
use plonky2x::prelude::{ArrayVariable, BoolVariable, Bytes32Variable, PlonkParameters};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::NB_MAX_SLOTS;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NounsGraffiti {
    slot: u64,
    block: Option<u64>,
    epoch: Option<u64>,
    timestamp: Option<u64>,
    proposer_id: u64,
    graffiti: Option<String>,
    fee_recipient: Option<String>,
    withdrawal_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NounsGraffitiResetHint;

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for NounsGraffitiResetHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, _: &mut ValueStream<L, D>) {
        let _ = input_stream.read_value::<Bytes32Variable>();
        let rpc_url = env::var("CONSENSUS_RPC_1").unwrap();
        let endpoint = format!("{}/api/integrations/nouns/reset", rpc_url);
        debug!("resetting nouns graffiti at {}", endpoint);
        let client = Client::new();
        let response = client
            .post(endpoint)
            .timeout(Duration::new(60, 0))
            .send()
            .unwrap();
        if response.status() != 200 {
            panic!("failed to reset nouns graffiti");
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NounsGraffitiPushHint;

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for NounsGraffitiPushHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, _: &mut ValueStream<L, D>) {
        let slot = input_stream.read_value::<U64Variable>();
        let filter = input_stream.read_value::<BoolVariable>();
        if filter {
            let rpc_url = env::var("CONSENSUS_RPC_1").unwrap();
            let endpoint = format!("{}/api/integrations/nouns/push/{}", rpc_url, slot);
            let client = Client::new();
            let response = client
                .post(endpoint)
                .timeout(Duration::new(60, 0))
                .send()
                .unwrap();
            if response.status() != 200 {
                panic!("failed to push nouns graffiti");
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NounsGraffitiPullHint;

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for NounsGraffitiPullHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let _ = input_stream.read_value::<Bytes32Variable>();
        let rpc_url = env::var("CONSENSUS_RPC_1").unwrap();
        let endpoint = format!("{}/api/integrations/nouns/pull", rpc_url);
        let client = Client::new();
        let mut slots: Vec<u32> = client
            .post(endpoint)
            .timeout(Duration::new(60, 0))
            .send()
            .unwrap()
            .json()
            .unwrap();
        assert!(slots.len() < NB_MAX_SLOTS);
        debug!("nouns graffiti proposer slots: {:?}", slots);
        slots.resize(NB_MAX_SLOTS, 0);
        output_stream.write_value::<ArrayVariable<U32Variable, NB_MAX_SLOTS>>(slots);
    }
}
