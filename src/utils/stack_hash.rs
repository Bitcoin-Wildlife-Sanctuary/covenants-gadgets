use crate::treepp::*;
use crate::utils::pseudo::OP_HINT;

/// Gadget for hashing the stack.
///
/// This trick was from QED (https://x.com/qedprotocol).
pub struct StackHash;

impl StackHash {
    /// Hashing the stack elements and drop them
    pub fn hash_drop(num: usize) -> Script {
        assert!(num > 0);
        script! {
            { num } OP_SHA256
            for _ in 0..num {
                OP_CAT OP_SHA256
            }
        }
    }

    /// Hashing the stack elements, without dropping.
    pub fn hash_nodrop(num: usize) -> Script {
        assert!(num > 0);
        script! {
            { num } OP_SHA256
            for _ in 0..num {
                OP_OVER OP_TOALTSTACK
                OP_CAT OP_SHA256
            }
            for _ in 0..num {
                OP_FROMALTSTACK
            }
            { num } OP_ROLL
        }
    }

    /// Hashing the stack elements retrieved from hints, without dropping.
    pub fn hash_from_hint(num: usize) -> Script {
        assert!(num > 0);
        script! {
            for _ in 0..num {
                OP_HINT
            }
            { Self::hash_nodrop(num) }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::treepp::*;
    use crate::utils::stack_hash::StackHash;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_hash() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        for i in 1..=20 {
            let mut v = vec![];
            for _ in 0..i {
                v.push(prng.gen::<u16>() as u32);
            }

            let script = script! {
                for elem in v.iter() {
                    { *elem }
                }
                { StackHash::hash_nodrop(i) }
                OP_TOALTSTACK
                { StackHash::hash_drop(i) }
                OP_FROMALTSTACK
                OP_EQUAL
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_hash_from_hint() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        for i in 2..=20 {
            let mut v = vec![];
            for _ in 0..i {
                v.push(prng.gen::<u16>() as u32);
            }

            let mut r = vec![]; // rubbish data
            for _ in 0..i {
                r.push(prng.gen::<u16>() as u32);
            }

            let script = script! {
                for elem in v.iter() {
                    { *elem }
                }
                for elem in r.iter() {
                    { *elem }
                }

                { StackHash::hash_from_hint(i) }
                OP_TOALTSTACK

                { StackHash::hash_drop(i) }
                OP_FROMALTSTACK
                OP_EQUALVERIFY

                for _ in 0..r.len() {
                    OP_DROP
                }

                OP_TRUE
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }
}
