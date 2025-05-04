use crate::treepp::*;
use crate::utils::pseudo::OP_HINT;
use crate::CovenantProgram;
use anyhow::Result;
use bitcoin_scriptexec::utils::scriptint_vec;
use sha2::digest::Update;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// The covenant program of the counter example.
pub struct CounterProgram;

/// State of the counter example, which is a counter.
#[derive(Clone, Debug)]
pub struct CounterState {
    /// The counter.
    pub counter: usize,
}

impl Into<Script> for CounterState {
    fn into(self) -> Script {
        script! {
            { self.counter }
        }
    }
}

/// Input of the counter example.
#[derive(Clone)]
pub struct CounterInput(Option<usize>);

impl Into<Script> for CounterInput {
    fn into(self) -> Script {
        if let Some(v) = self.0 {
            script! {
                { v }
            }
        } else {
            script! {}
        }
    }
}

impl CovenantProgram for CounterProgram {
    type State = CounterState;

    type Input = CounterInput;

    const CACHE_NAME: &'static str = "COUNTER";

    fn new() -> Self::State {
        Self::State { counter: 0 }
    }

    fn get_hash(state: &Self::State) -> Vec<u8> {
        let mut sha256 = Sha256::new();
        Update::update(&mut sha256, &scriptint_vec(state.counter as i64));
        sha256.finalize().to_vec()
    }

    fn get_all_scripts() -> BTreeMap<usize, Script> {
        let mut map = BTreeMap::new();
        // increase by 1
        map.insert(
            123456,
            script! {
                // stack:
                // - old counter
                // - new counter
                OP_1SUB OP_EQUAL
            },
        );
        // increase by 2
        map.insert(
            123457,
            script! {
                // stack:
                // - old counter
                // - new counter
                OP_1SUB OP_1SUB OP_EQUAL
            },
        );
        // increase by a given number as long as it is smaller than 100
        map.insert(
            456789,
            script! {
                // stack:
                // - old counter
                // - new counter

                OP_HINT
                OP_DUP 0 OP_GREATERTHANOREQUAL OP_VERIFY
                OP_DUP 100 OP_LESSTHAN OP_VERIFY
                OP_SUB OP_EQUAL
            },
        );
        map
    }

    fn get_common_prefix() -> Script {
        script! {
            // stack:
            // - old state hash
            // - new state hash

            // get the old counter and the new counter
            OP_HINT OP_HINT
            // save a copy to the altstack
            // altstack: new counter, old counter
            OP_2DUP OP_TOALTSTACK OP_TOALTSTACK

            // stack:
            // - old state hash
            // - new state hash
            // - old counter
            // - new counter
            OP_SHA256 OP_ROT OP_EQUALVERIFY
            OP_SHA256 OP_EQUALVERIFY

            OP_FROMALTSTACK OP_FROMALTSTACK
        }
    }

    fn run(id: usize, old_state: &Self::State, input: &Self::Input) -> Result<Self::State> {
        if id == 123456 {
            Ok(CounterState {
                counter: old_state.counter + 1,
            })
        } else if id == 123457 {
            Ok(CounterState {
                counter: old_state.counter + 2,
            })
        } else if id == 456789 {
            assert!(input.0.is_some());

            let input = input.0.unwrap();
            assert!(input < 100);

            Ok(CounterState {
                counter: old_state.counter + input,
            })
        } else {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::examples::counter::{CounterInput, CounterProgram, CounterState};
    use crate::test::{simulation_test, SimulationInstruction};
    use rand::prelude::SliceRandom;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_simulation() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        let mut test_generator = |_: &CounterState| {
            let id = *[123456usize, 123457, 456789].choose(&mut prng).unwrap();
            let input = if id == 456789 {
                CounterInput(Some(prng.gen_range(0..100)))
            } else {
                CounterInput(None)
            };
            Some(SimulationInstruction::<CounterProgram> {
                program_index: id,
                program_input: input,
            })
        };

        simulation_test::<CounterProgram>(100, &mut test_generator);
    }
}
