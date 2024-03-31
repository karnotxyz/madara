use mp_felt::Felt252Wrapper;
use starknet_api::transaction::{
    DeclareTransaction, DeclareTransactionV0V1, DeclareTransactionV2, DeclareTransactionV3, DeployAccountTransaction,
    InvokeTransaction, InvokeTransactionV0, InvokeTransactionV1, InvokeTransactionV3, L1HandlerTransaction,
};

use crate::UserTransaction;

impl UserTransaction {
    pub fn sender_address(&self) -> Felt252Wrapper {
        match self {
            UserTransaction::Declare(tx) => tx.tx.sender_address().into(),
            UserTransaction::DeployAccount(tx) => tx.tx.contract_address_salt().into(),
            UserTransaction::Invoke(tx) => tx.tx.sender_address().into(),
        }
    }

    pub fn signature(&self) -> Vec<Felt252Wrapper> {
        match self {
            UserTransaction::Declare(tx) => {
                tx.tx.signature().0.iter().map(|x| Felt252Wrapper::from(*x).into()).collect()
            }
            UserTransaction::DeployAccount(tx) => {
                tx.tx.signature().0.iter().map(|x| Felt252Wrapper::from(*x).into()).collect()
            }
            UserTransaction::Invoke(tx) => {
                tx.tx.signature().0.iter().map(|x| Felt252Wrapper::from(*x).into()).collect()
            }
        }
    }

    pub fn calldata(&self) -> Option<Vec<Felt252Wrapper>> {
        match self {
            UserTransaction::Declare(..) => None,
            UserTransaction::DeployAccount(tx) => {
                Some(tx.tx.constructor_calldata().0.iter().map(|x| Felt252Wrapper::from(*x).into()).collect())
            }
            UserTransaction::Invoke(tx) => {
                Some(tx.tx.calldata().0.iter().map(|x| Felt252Wrapper::from(*x).into()).collect())
            }
        }
    }

    pub fn nonce(&self) -> Option<Felt252Wrapper> {
        match self {
            UserTransaction::Declare(tx) => Some(tx.tx.nonce().0.into()),
            UserTransaction::DeployAccount(tx) => Some(tx.tx.nonce().0.into()),
            UserTransaction::Invoke(tx) => Some(tx.tx.nonce().0.into()),
        }
    }
}

pub trait TransactionVersion {
    fn version(&self) -> u8;
}

impl TransactionVersion for InvokeTransaction {
    #[inline(always)]
    fn version(&self) -> u8 {
        match self {
            InvokeTransaction::V0(tx) => tx.version(),
            InvokeTransaction::V1(tx) => tx.version(),
            InvokeTransaction::V3(tx) => tx.version(),
        }
    }
}

impl TransactionVersion for InvokeTransactionV0 {
    #[inline(always)]
    fn version(&self) -> u8 {
        0
    }
}

impl TransactionVersion for InvokeTransactionV1 {
    #[inline(always)]
    fn version(&self) -> u8 {
        1
    }
}

impl TransactionVersion for InvokeTransactionV3 {
    #[inline(always)]
    fn version(&self) -> u8 {
        3
    }
}

impl TransactionVersion for DeclareTransaction {
    #[inline(always)]
    fn version(&self) -> u8 {
        match self {
            DeclareTransaction::V0(tx) => tx.version(),
            DeclareTransaction::V1(tx) => tx.version(),
            DeclareTransaction::V2(tx) => tx.version(),
            DeclareTransaction::V3(tx) => tx.version(),
        }
    }
}

impl TransactionVersion for DeclareTransactionV0V1 {
    #[inline(always)]
    fn version(&self) -> u8 {
        0
    }
}

// TODO: what should we do here?
// impl TransactionVersion for DeclareTransactionV1 {
//     #[inline(always)]
//     fn version(&self) -> u8 {
//         1
//     }
// }

impl TransactionVersion for DeclareTransactionV2 {
    #[inline(always)]
    fn version(&self) -> u8 {
        2
    }
}

impl TransactionVersion for DeclareTransactionV3 {
    #[inline(always)]
    fn version(&self) -> u8 {
        3
    }
}

impl TransactionVersion for DeployAccountTransaction {
    #[inline(always)]
    fn version(&self) -> u8 {
        1
    }
}

impl TransactionVersion for L1HandlerTransaction {
    #[inline(always)]
    fn version(&self) -> u8 {
        0
    }
}
