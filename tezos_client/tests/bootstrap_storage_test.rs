use std::stringify;
use networking::p2p::binary_message::{Hexable, MessageHash};
use networking::p2p::encoding::prelude::*;
use tezos_client::client;
use tezos_client::client::TezosStorageInitInfo;
use tezos_interop::ffi::ApplyBlockError;

mod common;

macro_rules! tezos_test {
    ($f:expr) => ( (stringify!($f), $f) )
}

#[test]
fn run_tests() {
    // We cannot run tests in parallel, because tezos does not handle situation when multiple storage
    // directories are initialized
    let tests: [(&str, fn() -> Result<(), failure::Error>); 4] = [
        tezos_test!(test_bootstrap_empty_storage_with_first_three_blocks),
        tezos_test!(test_bootstrap_empty_storage_with_second_block_should_fail_unknown_predecessor),
        tezos_test!(test_bootstrap_empty_storage_with_second_block_should_fail_incomplete_operations),
        tezos_test!(test_bootstrap_empty_storage_with_first_block_with_invalid_operations_should_fail_invalid_operations)
    ];

    for (name, f) in tests.iter() {
        let result = f();
        assert!(result.is_ok(), "Tezos test {:?} failed with error: {:?}", name, &result);
    }
}

fn test_bootstrap_empty_storage_with_first_three_blocks() -> Result<(), failure::Error> {
    // init empty storage for test
    let TezosStorageInitInfo { chain_id, genesis_block_header_hash, current_block_header_hash } = client::init_storage(
        common::prepare_empty_dir("bootstrap_test_storage_01")
    )?;

    // current head must be set (genesis)
    let current_header = client::get_current_block_header(&chain_id)?;
    assert_eq!(0, current_header.level, "Was expecting current header level to be 0 but instead it was {}", current_header.level);
    assert_eq!(current_block_header_hash, current_header.message_hash()?);

    let genesis_header = client::get_block_header(&genesis_block_header_hash)?;
    assert!(genesis_header.is_some());
    assert_eq!(genesis_header.unwrap().to_hex()?, current_header.to_hex()?);

    // apply first block - level 0
    let apply_block_result = client::apply_block(
        &hex::decode(test_data::BLOCK_HEADER_HASH_LEVEL_1)?,
        &BlockHeader::from_hex(test_data::BLOCK_HEADER_LEVEL_1.to_string())?,
        &test_data::block_operations_from_hex(
            test_data::BLOCK_HEADER_HASH_LEVEL_1,
            test_data::block_header_level1_operations(),
        ),
    );
    assert_eq!("activate PsddFKi32cMJ", &apply_block_result?.validation_result_message);

    // check current head changed to level 1
    let current_header = client::get_current_block_header(&chain_id)?;
    assert_eq!(1, current_header.level);

    // apply second block - level 2
    let apply_block_result = client::apply_block(
        &hex::decode(test_data::BLOCK_HEADER_HASH_LEVEL_2)?,
        &BlockHeader::from_hex(test_data::BLOCK_HEADER_LEVEL_2.to_string())?,
        &test_data::block_operations_from_hex(
            test_data::BLOCK_HEADER_HASH_LEVEL_2,
            test_data::block_header_level2_operations(),
        ),
    );
    assert_eq!("lvl 2, fit 2, prio 5, 0 ops", &apply_block_result?.validation_result_message);

    // check current head changed to level 2
    let current_header = client::get_current_block_header(&chain_id)?;
    assert_eq!(2, current_header.level);

    // apply third block - level 3
    let apply_block_result = client::apply_block(
        &hex::decode(test_data::BLOCK_HEADER_HASH_LEVEL_3)?,
        &BlockHeader::from_hex(test_data::BLOCK_HEADER_LEVEL_3.to_string())?,
        &test_data::block_operations_from_hex(
            test_data::BLOCK_HEADER_HASH_LEVEL_3,
            test_data::block_header_level3_operations(),
        ),
    );
    assert_eq!("lvl 3, fit 5, prio 12, 1 ops", &apply_block_result?.validation_result_message);

    // check current head changed to level 3
    let current_header = client::get_current_block_header(&chain_id)?;
    Ok(assert_eq!(3, current_header.level))
}

fn test_bootstrap_empty_storage_with_second_block_should_fail_unknown_predecessor() -> Result<(), failure::Error> {
    // init empty storage for test
    let TezosStorageInitInfo { chain_id, genesis_block_header_hash, current_block_header_hash } = client::init_storage(
        common::prepare_empty_dir("bootstrap_test_storage_02")
    )?;

    // current head must be set (genesis)
    let current_header = client::get_current_block_header(&chain_id)?;
    assert_eq!(0, current_header.level);
    assert_eq!(current_block_header_hash, current_header.message_hash()?);

    let genesis_header = client::get_block_header(&genesis_block_header_hash)?;
    assert!(genesis_header.is_some());
    assert_eq!(genesis_header.unwrap().to_hex()?, current_header.to_hex()?);

    // apply second block - level 2
    let apply_block_result = client::apply_block(
        &hex::decode(test_data::BLOCK_HEADER_HASH_LEVEL_2)?,
        &BlockHeader::from_hex(test_data::BLOCK_HEADER_LEVEL_2.to_string())?,
        &test_data::block_operations_from_hex(
            test_data::BLOCK_HEADER_HASH_LEVEL_2,
            test_data::block_header_level2_operations(),
        ),
    );
    assert!(apply_block_result.is_err());
    Ok(assert_eq!(ApplyBlockError::UnknownPredecessor, apply_block_result.unwrap_err()))
}

fn test_bootstrap_empty_storage_with_second_block_should_fail_incomplete_operations() -> Result<(), failure::Error> {
    // init empty storage for test
    let TezosStorageInitInfo { chain_id, genesis_block_header_hash, current_block_header_hash } = client::init_storage(
        common::prepare_empty_dir("bootstrap_test_storage_03")
    )?;

    // current head must be set (genesis)
    let current_header = client::get_current_block_header(&chain_id)?;
    assert_eq!(0, current_header.level);
    assert_eq!(current_block_header_hash, current_header.message_hash()?);

    let genesis_header = client::get_block_header(&genesis_block_header_hash)?;
    assert!(genesis_header.is_some());
    assert_eq!(genesis_header.unwrap().to_hex()?, current_header.to_hex()?);

    // apply second block - level 3 has validation_pass = 4
    let apply_block_result = client::apply_block(
        &hex::decode(test_data::BLOCK_HEADER_HASH_LEVEL_3)?,
        &BlockHeader::from_hex(test_data::BLOCK_HEADER_LEVEL_3.to_string())?,
        vec![None].as_ref(),
    );
    assert!(apply_block_result.is_err());
    Ok(assert_eq!(ApplyBlockError::IncompleteOperations { expected: 4, actual: 1 }, apply_block_result.unwrap_err()))
}

fn test_bootstrap_empty_storage_with_first_block_with_invalid_operations_should_fail_invalid_operations() -> Result<(), failure::Error> {
    // init empty storage for test
    let TezosStorageInitInfo { chain_id, genesis_block_header_hash, current_block_header_hash } = client::init_storage(
        common::prepare_empty_dir("bootstrap_test_storage_04")
    )?;

    // current head must be set (genesis)
    let current_header = client::get_current_block_header(&chain_id)?;
    assert_eq!(0, current_header.level);
    assert_eq!(current_block_header_hash, current_header.message_hash()?);

    let genesis_header = client::get_block_header(&genesis_block_header_hash)?;
    assert!(genesis_header.is_some());
    assert_eq!(genesis_header.unwrap().to_hex()?, current_header.to_hex()?);

    // apply second block - level 1 ok
    let apply_block_result = client::apply_block(
        &hex::decode(test_data::BLOCK_HEADER_HASH_LEVEL_1)?,
        &BlockHeader::from_hex(test_data::BLOCK_HEADER_LEVEL_1.to_string())?,
        &test_data::block_operations_from_hex(
            test_data::BLOCK_HEADER_HASH_LEVEL_1,
            test_data::block_header_level1_operations(),
        ),
    );
    assert!(apply_block_result.is_ok());

    // apply second block - level 2 with operations for level 3
    let apply_block_result = client::apply_block(
        &hex::decode(test_data::BLOCK_HEADER_HASH_LEVEL_2)?,
        &BlockHeader::from_hex(test_data::BLOCK_HEADER_LEVEL_2.to_string())?,
        &test_data::block_operations_from_hex(
            test_data::BLOCK_HEADER_HASH_LEVEL_3,
            test_data::block_header_level3_operations(),
        ),
    );
    Ok(assert!(apply_block_result.is_err()))
}

mod test_data {
    use networking::p2p::binary_message::Hexable;
    use networking::p2p::encoding::prelude::*;
    use tezos_encoding::hash::BlockHash;

    // BMPtRJqFGQJRTfn8bXQR2grLE1M97XnUmG5vgjHMW7St1Wub7Cd
    pub const BLOCK_HEADER_HASH_LEVEL_1: &str = "dd9fb5edc4f29e7d28f41fe56d57ad172b7686ed140ad50294488b68de29474d";
    pub const BLOCK_HEADER_LEVEL_1: &str = include_str!("resources/block_header_level1.bytes");

    pub fn block_header_level1_operations() -> Vec<Vec<String>> {
        vec![]
    }

    // BLwKksYwrxt39exDei7yi47h7aMcVY2kZMZhTwEEoSUwToQUiDV
    pub const BLOCK_HEADER_HASH_LEVEL_2: &str = "60ab6d8d2a6b1c7a391f00aa6c1fc887eb53797214616fd2ce1b9342ad4965a4";
    pub const BLOCK_HEADER_LEVEL_2: &str = "0000000201dd9fb5edc4f29e7d28f41fe56d57ad172b7686ed140ad50294488b68de29474d000000005c017cd804683625c2445a4e9564bf710c5528fd99a7d150d2a2a323bc22ff9e2710da4f6d0000001100000001000000000800000000000000029bd8c75dec93c276d2d8e8febc3aa6c9471cb2cb42236b3ab4ca5f1f2a0892f6000500000003ba671eef00d6a8bea20a4677fae51268ab6be7bd8cfc373cd6ac9e0a00064efcc404e1fb39409c5df255f7651e3d1bb5d91cb2172b687e5d56ebde58cfd92e1855aaafbf05";

    pub fn block_header_level2_operations() -> Vec<Vec<String>> {
        vec![
            vec![],
            vec![],
            vec![],
            vec![]
        ]
    }

    // BLTQ5B4T4Tyzqfm3Yfwi26WmdQScr6UXVSE9du6N71LYjgSwbtc
    pub const BLOCK_HEADER_HASH_LEVEL_3: &str = "a14f19e0df37d7b71312523305d71ac79e3d989c1c1d4e8e884b6857e4ec1627";
    pub const BLOCK_HEADER_LEVEL_3: &str = "0000000301a14f19e0df37d7b71312523305d71ac79e3d989c1c1d4e8e884b6857e4ec1627000000005c017ed604dfcb6b41e91650bb908618b2740a6167d9072c3230e388b24feeef04c98dc27f000000110000000100000000080000000000000005f06879947f3d9959090f27054062ed23dbf9f7bd4b3c8a6e86008daabb07913e000c00000003e5445371002b9745d767d7f164a39e7f373a0f25166794cba491010ab92b0e281b570057efc78120758ff26a33301870f361d780594911549bcb7debbacd8a142e0b76a605";

    pub fn block_header_level3_operations() -> Vec<Vec<String>> {
        vec![
            vec!["a14f19e0df37d7b71312523305d71ac79e3d989c1c1d4e8e884b6857e4ec1627000000000236663bacdca76094fdb73150092659d463fec94eda44ba4db10973a1ad057ef53a5b3239a1b9c383af803fc275465bd28057d68f3cab46adfd5b2452e863ff0a".to_string()],
            vec![],
            vec![],
            vec![]
        ]
    }

    pub fn block_operations_from_hex(block_hash: &str, hex_operations: Vec<Vec<String>>) -> Vec<Option<OperationsForBlocksMessage>> {
        hex_operations
            .into_iter()
            .map(|bo| {
                let ops = bo
                    .into_iter()
                    .map(|op| Operation::from_hex(op).unwrap())
                    .collect();
                Some(
                    OperationsForBlocksMessage {
                        operation_hashes_path: Path::Op,
                        operations_for_block: OperationsForBlock {
                            validation_pass: 4,
                            hash: hex::decode(block_hash.clone()).unwrap() as BlockHash,
                        },
                        operations: ops,
                    }
                )
            })
            .collect()
    }
}