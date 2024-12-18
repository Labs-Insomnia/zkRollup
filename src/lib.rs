use std::marker::PhantomData;

use sha2::Digest;
use sov_rollup_interface::da::{BlobReaderTrait, DaSpec};
use sov_rollup_interface::stf::{BatchReceipt, SlotResult, StateTransitionFunction};
use sov_rollup_interface::zk::{ValidityCondition, Zkvm};

/// An implementation of the [`StateTransitionFunction`]
/// that is specifically designed to check if someone knows a preimage of a specific hash.
#[derive(PartialEq, Debug, Clone, Eq, serde::Serialize, serde::Deserialize, Default)]
pub struct CheckHashPreimageStf<Cond> {
    phantom_data: PhantomData<Cond>,
}

/// Outcome of the apply_slot method.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ApplySlotResult {
    /// Incorrect hash preimage was posted on the DA.
    Failure,
    /// Correct hash preimage was posted on the DA.
    Success,
}

impl<Vm: Zkvm, Cond: ValidityCondition, Da: DaSpec> StateTransitionFunction<Vm, Da>
for CheckHashPreimageStf<Cond>
{
    // Since our rollup is stateless, we don't need to consider the StateRoot.
    type StateRoot = [u8; 32];
    type InitialState = ();

    // This represents the initial configuration of the rollup, but it is not supported in this tutorial.
    // type GenesisParams = ();
    // type PreState = ();
    // type ChangeSet = ();

    // We could incorporate the concept of a transaction into the rollup, but we leave it as an exercise for the reader.
    type TxReceiptContents = ();

    // This is the type that will be returned as a result of `apply_blob`.
    type BatchReceiptContents = ApplySlotResult;

    // This data is produced during actual batch execution or validated with proof during verification.
    // However, in this tutorial, we won't use it.
    type Witness = ();

    type Condition = Cond;

    // Perform one-time initialization for the genesis block.
    //
    // Fixes: linter says:
    // Method `init_chain` has 2 parameters, but the declaration in trait `StateTransitionFunction` has 1 [E0050]
    fn init_chain(
        &mut self,
        _initial_state: Self::InitialState,
    ) -> Self::StateRoot {
        [0u8; 32]
    }

    // Fixes: linter says:
    // Method `apply_slot` has 6 parameters, but the declaration in trait `StateTransitionFunction` has 5 [E0050]
    fn apply_slot<'a, I>(
        &mut self,
        _pre_state_root: &Self::StateRoot,
        _witness: Self::Witness,
        _slot_header: &Da::BlockHeader,
        _validity_condition: &Da::ValidityCondition,
        blobs: I,
    ) -> SlotResult<
        Self::StateRoot,
        Self::BatchReceiptContents,
        Self::TxReceiptContents,
        Self::Witness,
    >
    where
        I: IntoIterator<Item = &'a mut Da::BlobTransaction>,
    {
        let mut receipts = vec![];
        for blob in blobs {
            let data = blob.verified_data();

            // Check if the sender submitted the preimage of the hash.
            let hash = sha2::Sha256::digest(data).into();
            let desired_hash = [
                102, 104, 122, 173, 248, 98, 189, 119, 108, 143, 193, 139, 142, 159, 142, 32, 8,
                151, 20, 133, 110, 226, 51, 179, 144, 42, 89, 29, 13, 95, 41, 37,
            ];

            let result = if hash == desired_hash {
                ApplySlotResult::Success
            } else {
                ApplySlotResult::Failure
            };

            // Return the `BatchReceipt`
            receipts.push(BatchReceipt {
                batch_hash: hash,
                tx_receipts: vec![],
                inner: result,
            });
        }

        SlotResult {
            state_root: [0u8; 32], // We don't need to update the state root, let's return a dummy state root for now.
            batch_receipts: receipts,
            witness: (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::Sha256;
    use proptest::prelude::*;
    use sov_rollup_interface::da::{BlobReaderTrait, DaSpec, BlockHeaderTrait, ValidityCondition as DaValidityCondition, BasicAddress, Time};
    use sov_rollup_interface::stf::StateTransitionFunction;
    use serde::{Serialize, Deserialize};

    // Mock Address
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct MockAddress;

    impl BasicAddress for MockAddress {}

    // Mock structures to implement required traits
    #[derive(Clone, Serialize, Deserialize)]
    struct MockBlobTransaction {
        data: Vec<u8>,
    }

    impl BlobReaderTrait for MockBlobTransaction {
        type Address = MockAddress;

        fn sender(&self) -> Self::Address {
            MockAddress
        }

        fn hash(&self) -> [u8; 32] {
            Sha256::digest(&self.data).into()
        }

        fn verified_data(&self) -> &[u8] {
            &self.data
        }

        fn total_len(&self) -> usize {
            self.data.len()
        }
    }

    #[derive(Default, Clone, Serialize, Deserialize)]
    struct MockBlockHeader;

    impl BlockHeaderTrait for MockBlockHeader {
        type Hash = [u8; 32];

        fn prev_hash(&self) -> Self::Hash {
            [0u8; 32]
        }

        fn hash(&self) -> Self::Hash {
            [0u8; 32]
        }

        fn height(&self) -> u64 {
            0
        }

        fn time(&self) -> Time {
            Time::from_secs(0)
        }
    }

    #[derive(Default, Clone, Serialize, Deserialize)]
    struct MockValidityCondition;

    impl DaValidityCondition for MockValidityCondition {}

    #[derive(Default)]
    struct MockDaSpec;

    impl DaSpec for MockDaSpec {
        type SlotHash = [u8; 32];
        type BlockHeader = MockBlockHeader;
        type BlobTransaction = MockBlobTransaction;
        type Address = MockAddress;
        type ValidityCondition = MockValidityCondition;
        type InclusionMultiProof = ();
        type CompletenessProof = ();
        type ChainParams = ();
    }

    #[test]
    fn test_apply_slot_success() {
        let mut stf = CheckHashPreimageStf::<MockValidityCondition>::default();
        let pre_state_root = [0u8; 32];
        let witness = ();
        let slot_header = MockBlockHeader::default();
        let validity_condition = MockValidityCondition::default();

        // Data that hashes to the desired hash
        let data = [
            102, 104, 122, 173, 248, 98, 189, 119, 108, 143, 193, 139, 142, 159, 142, 32, 8,
            151, 20, 133, 110, 226, 51, 179, 144, 42, 89, 29, 13, 95, 41, 37,
        ];
        let mut blob = MockBlobTransaction { data: data.to_vec() };

        let result = stf.apply_slot(
            &pre_state_root,
            witness,
            &slot_header,
            &validity_condition,
            vec![&mut blob],
        );

        assert_eq!(result.batch_receipts.len(), 1);
        assert_eq!(result.batch_receipts[0].inner, ApplySlotResult::Success);
    }

    #[test]
    fn test_apply_slot_failure() {
        let mut stf = CheckHashPreimageStf::<MockValidityCondition>::default();
        let pre_state_root = [0u8; 32];
        let witness = ();
        let slot_header = MockBlockHeader::default();
        let validity_condition = MockValidityCondition::default();

        // Data that does not hash to the desired hash
        let data = b"incorrect preimage data";
        let mut blob = MockBlobTransaction { data: data.to_vec() };

        let result = stf.apply_slot(
            &pre_state_root,
            witness,
            &slot_header,
            &validity_condition,
            vec![&mut blob],
        );

        assert_eq!(result.batch_receipts.len(), 1);
        assert_eq!(result.batch_receipts[0].inner, ApplySlotResult::Failure);
    }

    proptest! {
        #[test]
        fn test_apply_slot_with_random_data(data in any::<Vec<u8>>()) {
            let mut stf = CheckHashPreimageStf::<MockValidityCondition>::default();
            let pre_state_root = [0u8; 32];
            let witness = ();
            let slot_header = MockBlockHeader::default();
            let validity_condition = MockValidityCondition::default();

            let mut blob = MockBlobTransaction { data };

            let result = stf.apply_slot(
                &pre_state_root,
                witness,
                &slot_header,
                &validity_condition,
                vec![&mut blob],
            );

            assert_eq!(result.batch_receipts.len(), 1);
        }
    }
}