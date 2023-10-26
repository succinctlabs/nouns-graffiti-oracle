use std::time::Duration;

use itertools::Itertools;
use log::debug;
use plonky2x::frontend::hint::simple::hint::Hint;
use plonky2x::frontend::uint::uint64::U64Variable;
use plonky2x::frontend::vars::{U32Variable, ValueStream};
use plonky2x::prelude::{ArrayVariable, BoolVariable, PlonkParameters};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::{NB_BLOCKS, NB_MAX_PROPOSERS};

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
pub struct NounsGraffitiProposerCheckHint;
impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for NounsGraffitiProposerCheckHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let slot = input_stream.read_value::<U64Variable>();
        let proposer_id = input_stream.read_value::<U32Variable>();
        let graffiti_found = input_stream.read_value::<BoolVariable>();
        let endpoint = "https://api.nogglesgraffiti.wtf/slots";
        debug!("fetching nouns graffiti from {}", endpoint);
        let client = Client::new();
        let response: Vec<NounsGraffiti> = client
            .get(endpoint)
            .timeout(Duration::new(60, 0))
            .send()
            .unwrap()
            .json()
            .unwrap();
        output_stream.write_value::<BoolVariable>(false);
        let mut sanity = false;
        for i in 0..response.len() {
            if response[i].slot == slot {
                assert_eq!(
                    response[i].proposer_id, proposer_id as u64,
                    "proposer id mismatch"
                );
                sanity = true;
            }
        }
        if graffiti_found != sanity {
            panic!("graffiti found mismatch for {} and {}", slot, proposer_id);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NounsGraffitiProposersHint;

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for NounsGraffitiProposersHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let start_slot = input_stream.read_value::<U64Variable>();
        let end_slot = input_stream.read_value::<U64Variable>();
        let target_slot = input_stream.read_value::<U64Variable>();
        let endpoint = "https://api.nogglesgraffiti.wtf/slots";
        debug!("fetching nouns graffiti from {}", endpoint);
        let client = Client::new();
        let response: Vec<NounsGraffiti> = client
            .get(endpoint)
            .timeout(Duration::new(60, 0))
            .send()
            .unwrap()
            .json()
            .unwrap();
        let mut nouns_graffitis = response;
        nouns_graffitis = nouns_graffitis
            .into_iter()
            .filter(|n| {
                start_slot <= n.slot
                    && n.slot < end_slot
                    && (target_slot - NB_BLOCKS as u64) < n.slot
                    && n.block.is_some()
            })
            .collect_vec();
        let mut proposer_ids = nouns_graffitis
            .iter()
            .map(|n| n.proposer_id as u32)
            .collect_vec();
        assert!(proposer_ids.len() < NB_MAX_PROPOSERS);
        debug!("nouns graffiti proposer ids: {:?}", proposer_ids);
        proposer_ids.resize(NB_MAX_PROPOSERS, 0);
        output_stream.write_value::<ArrayVariable<U32Variable, NB_MAX_PROPOSERS>>(proposer_ids);
    }
}
