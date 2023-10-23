//! Nouns Graffiti Oracle.
#![allow(clippy::needless_range_loop)]

use hints::NounsGraffitiProposersHint;
use itertools::Itertools;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2x::backend::circuit::{Circuit, PlonkParameters};
use plonky2x::backend::function::Plonky2xFunction;
use plonky2x::frontend::builder::permutation::RandomPermutationHint;
use plonky2x::frontend::eth::beacon::vars::BeaconHeaderVariable;
use plonky2x::frontend::extension::CubicExtensionVariable;
use plonky2x::frontend::mapreduce::generator::MapReduceGenerator;
use plonky2x::frontend::uint::uint64::U64Variable;
use plonky2x::frontend::vars::{SSZVariable, U32Variable, VariableStream};
use plonky2x::prelude::{ArrayVariable, Bytes32Variable, BytesVariable, CircuitBuilder};
use plonky2x::utils::{bytes, bytes32};

mod hints;

#[derive(Debug, Clone)]
struct NounsGraffitiOracle;

/// The noggles graffiti ("⌐◨-◨") encoded in bytes.
pub const NOGGLES_GRAFFITI: &str = "0xe28c90e297a82de297a8";

/// The proposer id used to represent a none vaue.
pub const DUMMY_PROPOSER_ID: u64 = 0;

pub const DUMMY_WITHDRAWAL_CREDENTIALS: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";

/// The maximum number of proposers that can be returned by the witness.
pub const NB_MAX_PROPOSERS: usize = 1024;

/// The number of blocks we iterate over in a single proof.
pub const NB_BLOCKS: usize = 131072;

/// The number of blocks we iterate over in a single map proof.
pub const BATCH_SIZE: usize = 64;

/// The number of winners we return.
pub const NB_WINNERS: usize = 10;

impl Circuit for NounsGraffitiOracle {
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        // Read start slot, end slot, target slot, and source block root from the EVM.
        let start_slot = builder.evm_read::<U64Variable>();
        let end_slot = builder.evm_read::<U64Variable>();
        let target_slot = builder.evm_read::<U64Variable>();
        let block_root = builder.evm_read::<Bytes32Variable>();

        // Read the pseudorandom challenges from the EVM. Assumes the U64 are in the range
        // [0, 2**64-2**32+1+1). Construct a cubic extension element with ~192 bits of security.
        let gamma_a = builder.evm_read::<U64Variable>().to_variable(builder);
        let gamma_b = builder.evm_read::<U64Variable>().to_variable(builder);
        let gamma_c = builder.evm_read::<U64Variable>().to_variable(builder);
        let gamma = CubicExtensionVariable::new(gamma_a, gamma_b, gamma_c);

        // Read a pseudorandom seed to use for choosing the winners.
        let seed = builder.evm_read::<U32Variable>();

        // Assert that the target_slot >= end_slot.
        let target_gte_end = builder.gte(target_slot, end_slot);
        let t = builder._true();
        builder.assert_is_equal(target_gte_end, t);

        // Get the target block root from the source block root.
        let target_block_root = builder.beacon_get_historical_block(block_root, target_slot);

        // Compute the filtered accumulator by iterating over the previous `NB_BLOCKS` block roots.
        let offsets = (0..NB_BLOCKS).map(|i| i as u64).collect_vec();
        let result = builder.mapreduce::<(
            CubicExtensionVariable,
            U64Variable,
            U64Variable,
            Bytes32Variable,
        ), U64Variable, (
            CubicExtensionVariable,
            Bytes32Variable,
            BeaconHeaderVariable,
            Bytes32Variable,
            BeaconHeaderVariable,
        ), Self, BATCH_SIZE, _, _>(
            (gamma, start_slot, end_slot, target_block_root),
            offsets,
            |(gamma, start_slot, end_slot, target_block_root), offsets, builder| {
                // Witness block roots in the range [start_offset, end_offset).
                let start_offset = offsets[0];
                let end_offset = offsets[offsets.len() - 1];
                let block_roots = builder.beacon_witness_headers_from_offset_range::<BATCH_SIZE>(
                    target_block_root,
                    start_offset,
                    end_offset,
                );

                // Prove that this is a valid chain of headers going from the newest header
                // to the oldest header.
                let end_header = builder.beacon_get_block_header(block_roots[0]);
                let mut prev_header = end_header;
                let mut headers = vec![prev_header];
                for i in 1..block_roots.len() {
                    let header = builder.beacon_get_block_header(block_roots[i]);
                    let header_root = header.hash_tree_root(builder);
                    builder.assert_is_equal(prev_header.parent_root, header_root);
                    prev_header = header;
                    headers.push(header);
                }

                // Compute the filtered accumulator of proposers with noggles graffiti.
                let mut filtered_acc = builder.one::<CubicExtensionVariable>();
                for i in 0..headers.len() {
                    let header = headers[i];
                    let proposer_index = header.proposer_index.limbs[0];

                    // Get the graffiti and check if it contains ⌐◨-◨.
                    let goggles = builder.constant::<BytesVariable<10>>(bytes!(NOGGLES_GRAFFITI));
                    let graffiti = builder.beacon_get_graffiti(block_roots[i]);
                    let mut goggles_found = builder._false();
                    for j in 0..22 {
                        let mut found = builder._true();
                        for k in 0..10 {
                            let graffiti_byte = graffiti.0[j + k];
                            let goggles_byte = goggles[k];
                            let eq = builder.is_equal(graffiti_byte, goggles_byte);
                            found = builder.and(found, eq);
                        }
                        goggles_found = builder.or(found, goggles_found);
                    }

                    // Accumulate the proposer index if the goggles exist is in the range of
                    // `start_slot` and `end_slot`.
                    let one = builder.one::<CubicExtensionVariable>();
                    let proposer_index_extension = proposer_index.0.as_cubic_extension(builder);
                    let term = builder.sub(gamma, proposer_index_extension);
                    let within_range = builder.within_range(header.slot, start_slot, end_slot);
                    let filter = builder.and(within_range, goggles_found);
                    let filtered_term = builder.select(filter, term, one);
                    filtered_acc = builder.mul(filtered_acc, filtered_term);
                }

                // Return the auxiliary information needed during the reduce step.
                (
                    filtered_acc,                       // acc
                    block_roots[0],                     // end block root
                    headers[0],                         // end header
                    block_roots[block_roots.len() - 1], // start block root
                    headers[block_roots.len() - 1],     // start header
                )
            },
            |_, a, b, builder| {
                // Chain: Head -> ... -> a_end -> a_start -> b_end -> b_end -> ... -> Genesis

                // Transition: b_end <-> a_start
                builder.assert_is_equal(b.1, a.4.parent_root);

                // Merge: (a_acc * b_acc, a_end, a_end, b_start, b_start)
                (builder.mul(a.0, b.0), a.1, a.2, b.3, b.4)
            },
        );

        // Witness the set of proposers.
        let mut input_stream = VariableStream::new();
        input_stream.write(&start_slot);
        input_stream.write(&end_slot);
        input_stream.write(&target_slot);
        let output = builder.hint(input_stream, NounsGraffitiProposersHint {});
        let proposer_ids = output.read::<ArrayVariable<U32Variable, NB_MAX_PROPOSERS>>(builder);

        // Recompute the filtered accumulator and assert that it equals the expected accumulator.
        let dummy = builder.constant::<U32Variable>(DUMMY_PROPOSER_ID as u32);
        let mut filtered_acc = builder.one::<CubicExtensionVariable>();
        for i in 0..proposer_ids.len() {
            let is_dummy = builder.is_equal(proposer_ids[i], dummy);
            let proposer_index_extension = proposer_ids[i].0.as_cubic_extension(builder);
            let term = builder.sub(gamma, proposer_index_extension);
            let acc = builder.mul(filtered_acc, term);
            filtered_acc = builder.select(is_dummy, filtered_acc, acc);
        }

        // Assert that the filtered accumulator equals the expected accumulator.
        builder.assert_is_equal(result.0, filtered_acc);

        // Permute the values with a random ordering based on `gamma`.
        let permuted_proposers = builder.permute_with_dummy(proposer_ids, dummy, gamma, seed);

        // Return the first N validators.
        let validators = builder.beacon_get_validators(block_root);
        let dummy_withdrawal_credentials =
            builder.constant::<Bytes32Variable>(bytes32!(DUMMY_WITHDRAWAL_CREDENTIALS));
        for i in 0..NB_WINNERS {
            let proposer_id_u64 = permuted_proposers[i].to_u64(builder);
            let validator = builder.beacon_get_validator(validators, proposer_id_u64);
            let is_dummy_proposer = builder.is_equal(permuted_proposers[i], dummy);
            let withdrawal_credentials = builder.select(
                is_dummy_proposer,
                dummy_withdrawal_credentials,
                validator.withdrawal_credentials,
            );
            builder.evm_write(withdrawal_credentials);
        }
    }

    fn register_generators<L: PlonkParameters<D>, const D: usize>(
        registry: &mut plonky2x::prelude::HintRegistry<L, D>,
    ) where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        registry.register_hint::<RandomPermutationHint<NB_MAX_PROPOSERS>>();
        registry.register_hint::<NounsGraffitiProposersHint>();
        let id = MapReduceGenerator::<
            L,
            (
                CubicExtensionVariable,
                U64Variable,
                U64Variable,
                Bytes32Variable,
            ),
            U64Variable,
            (
                CubicExtensionVariable,
                Bytes32Variable,
                BeaconHeaderVariable,
                Bytes32Variable,
                BeaconHeaderVariable,
            ),
            Self,
            BATCH_SIZE,
            D,
        >::id();
        registry.register_simple::<MapReduceGenerator<
            L,
            (
                CubicExtensionVariable,
                U64Variable,
                U64Variable,
                Bytes32Variable,
            ),
            U64Variable,
            (
                CubicExtensionVariable,
                Bytes32Variable,
                BeaconHeaderVariable,
                Bytes32Variable,
                BeaconHeaderVariable,
            ),
            Self,
            BATCH_SIZE,
            D,
        >>(id);
    }
}

fn main() {
    NounsGraffitiOracle::entrypoint();
}

#[cfg(test)]
mod tests {
    use std::env;

    use plonky2x::prelude::DefaultParameters;
    use plonky2x::utils::bytes32;

    use super::*;

    type L = DefaultParameters;
    const D: usize = 2;

    /// An example source block root (slot 7453813).
    const BLOCK_ROOT: &str = "0x7a75d5502991b79c5ab31780e575d257335549a0a8fc02eb036ecd67f490be66";

    #[test]
    fn test_circuit() {
        env::set_var("RUST_LOG", "debug");
        env::set_var("CONSENSUS_RPC_1", "http://localhost:3000");
        env_logger::try_init().unwrap_or_default();

        let mut builder = CircuitBuilder::<L, D>::new();
        NounsGraffitiOracle::define(&mut builder);
        let circuit = builder.build();

        // Generate input.
        let mut input = circuit.input();
        input.evm_write::<U64Variable>(6123599);
        input.evm_write::<U64Variable>(6339599);
        input.evm_write::<U64Variable>(6339600);
        input.evm_write::<Bytes32Variable>(bytes32!(BLOCK_ROOT));
        input.evm_write::<U64Variable>(0);
        input.evm_write::<U64Variable>(1);
        input.evm_write::<U64Variable>(2);
        input.evm_write::<U32Variable>(3);

        // Generate the proof and verify.
        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        for _ in 0..NB_WINNERS {
            let winner = output.evm_read::<Bytes32Variable>();
            println!("winner: {:?}", winner);
        }
    }
}
