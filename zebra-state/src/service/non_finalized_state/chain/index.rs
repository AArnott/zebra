//! Transparent address indexes for non-finalized chains.

use std::collections::{BTreeMap, HashMap, HashSet};

use zebra_chain::{
    amount::{Amount, NegativeAllowed},
    transaction, transparent,
};

use crate::{OutputLocation, TransactionLocation};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransparentTransfers {
    /// The partial chain balance for a transparent address.
    ///
    /// TODO:
    /// To avoid [`ReadStateService`] response inconsistencies when a block has just been finalized,
    /// revert UTXO receives and spends that are at a height less than or equal to the finalized tip.
    balance: Amount<NegativeAllowed>,

    /// The partial list of transactions that spent or received UTXOs to a transparent address.
    ///
    /// Since transactions can only be added to this set, it does not need special handling
    /// for [`ReadStateService`] response inconsistencies.
    ///
    /// The `getaddresstxids` RPC needs these transaction IDs to be sorted in chain order.
    ///
    /// TODO: use Arc<Hash> to save 24 bytes per transaction
    tx_ids: BTreeMap<TransactionLocation, transaction::Hash>,

    /// The partial list of UTXOs received by a transparent address.
    ///
    /// The `getaddressutxos` RPC doesn't need these transaction IDs to be sorted in chain order,
    /// but it might in future. So Zebra does it anyway.
    ///
    /// TODO:
    /// To avoid [`ReadStateService`] response inconsistencies when a block has just been finalized,
    /// ignore UTXOs that are at a height less than or equal to the finalized tip.
    ///
    /// TODO: use Arc<Utxo> to save 2-100 bytes per output
    ///
    ///       if we add an OutputLocation to UTXO, remove this OutputLocation,
    ///       and use the inner OutputLocation to sort Utxos in chain order
    //
    // TODO: use BTreeMap as part of PR #3978
    created_utxos: HashMap<OutputLocation, transparent::Utxo>,

    /// The partial list of UTXOs spent by a transparent address.
    ///
    /// The `getaddressutxos` RPC doesn't need these transaction IDs to be sorted in chain order,
    /// but it might in future. So Zebra does it anyway.
    ///
    /// TODO:
    /// To avoid [`ReadStateService`] response inconsistencies when a block has just been finalized,
    /// ignore UTXOs that are at a height less than or equal to the finalized tip.
    ///
    /// TODO: use Arc<Utxo> to save 2-100 bytes per output
    ///
    ///       if we add an OutputLocation to UTXO, remove this OutputLocation,
    ///       and use the inner OutputLocation to sort Utxos in chain order
    //
    // TODO: use BTreeSet as part of PR #3978
    spent_utxos: HashSet<OutputLocation>,
}
