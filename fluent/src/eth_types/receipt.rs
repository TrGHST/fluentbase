use super::bytes::{de_hex_to_vec_u8, se_hex, Bytes};
use bytes::BytesMut;
use ethereum::{EnvelopedDecodable, EnvelopedDecoderError, EnvelopedEncodable};
use ethereum_types::{Address, Bloom, BloomInput, H256, U256, U64};
use rlp::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Type              uint8  `json:"type,omitempty"`
// PostState         []byte `json:"root"`
// Status            uint64 `json:"status"`
// CumulativeGasUsed uint64 `json:"cumulativeGasUsed" gencodec:"required"`
// Bloom             Bloom  `json:"logsBloom"         gencodec:"required"`
// Logs              []*Log `json:"logs"              gencodec:"required"`
// #[serde(rename_all = "camelCase")]
#[derive(
    Clone, Debug, PartialEq, Eq, rlp::RlpEncodable, rlp::RlpDecodable, Serialize, Deserialize,
)]
#[cfg_attr(
    feature = "with-codec",
    derive(codec::Encode, codec::Decode, scale_info::TypeInfo)
)]
// #[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Log {
    pub address: Address,
    pub topics: Vec<H256>,
    #[serde(serialize_with = "se_hex")]
    #[serde(deserialize_with = "de_hex_to_vec_u8")]
    data: Vec<u8>,
}

impl Log {
    /// Calculates the bloom of this log entry.
    pub fn bloom(&self) -> Bloom {
        self.topics.iter().fold(
            Bloom::from(BloomInput::Raw(self.address.as_bytes())),
            |mut b, t| {
                b.accrue(BloomInput::Raw(t.as_bytes()));
                b
            },
        )
    }
}

// #[derive(
//     Clone, Debug, PartialEq, Eq, rlp::RlpEncodable, rlp::RlpDecodable, Serialize, Deserialize,
// )]
// #[cfg_attr(
//     feature = "with-codec",
//     derive(codec::Encode, codec::Decode, scale_info::TypeInfo)
// )]
// pub struct Receipt {
//     pub status: U64,
//     #[serde(rename = "cumulativeGasUsed")]
//     pub cumulative_gas_used: U64,
//     #[serde(rename = "logsBloom")]
//     pub bloom: Bloom,
//     pub logs: Vec<Log>,
// }

// impl EnvelopedEncodable for Receipt {
//     fn type_id(&self) -> Option<u8> {
//         None
//     }
//     fn encode_payload(&self) -> BytesMut {
//         rlp::encode(self)
//     }
// }

// impl EnvelopedDecodable for Receipt {
//     type PayloadDecoderError = DecoderError;

//     fn decode(bytes: &[u8]) -> Result<Self, EnvelopedDecoderError<Self::PayloadDecoderError>> {
//         Ok(rlp::decode(bytes)?)
//     }
// }

// #[derive(Clone, Serialize, Deserialize)]
// pub struct ReceiptX {
//     pub status: U64,
//     #[serde(rename = "cumulativeGasUsed")]
//     pub cumulative_gas_used: U64,
//     #[serde(rename = "logsBloom")]
//     pub bloom: Bloom,
//     pub logs: Vec<Log>,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionOutcome {
    /// State root is known, before EIP-658 is enabled.
    StateRoot(H256),
    /// Status code is known. EIP-658 rules.
    StatusCode(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Receipt {
    Legacy(LegacyReceipt),
    AccessList(LegacyReceipt),
    EIP1559(LegacyReceipt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyReceipt {
    pub cumulative_gas_used: U256,
    pub log_bloom: Bloom,
    pub logs: Vec<Log>,
    pub outcome: TransactionOutcome,
}

impl LegacyReceipt {
    pub fn new(outcome: TransactionOutcome, cumulative_gas_used: U256, logs: Vec<Log>) -> Self {
        LegacyReceipt {
            cumulative_gas_used,
            log_bloom: logs.iter().fold(Bloom::default(), |mut b, l| {
                b.accrue_bloom(&l.bloom());
                b
            }),
            logs,
            outcome,
        }
    }
}

impl Decodable for LegacyReceipt {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        match rlp.item_count()? {
            4 => Ok(LegacyReceipt {
                cumulative_gas_used: rlp.val_at(1)?,
                log_bloom: rlp.val_at(2)?,
                logs: rlp.list_at(3)?,
                outcome: {
                    let first = rlp.at(0)?;
                    if first.is_data() && first.data()?.len() <= 1 {
                        TransactionOutcome::StatusCode(first.as_val()?)
                    } else {
                        TransactionOutcome::StateRoot(first.as_val()?)
                    }
                },
            }),
            _ => Err(DecoderError::RlpIncorrectListLen),
        }
    }
}

impl Encodable for LegacyReceipt {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(4);
        match self.outcome {
            TransactionOutcome::StateRoot(ref root) => {
                s.append(root);
            }
            TransactionOutcome::StatusCode(ref status_code) => {
                s.append(status_code);
            }
        }
        s.append(&self.cumulative_gas_used);
        s.append(&self.log_bloom);
        s.append_list(&self.logs);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LegacyReceiptHelper {
    pub cumulative_gas_used: U256,
    pub logs_bloom: Bloom,
    pub logs: Vec<Log>,
    #[serde(rename(deserialize = "status"))]
    pub status: U64,
}

impl Receipt {
    /// Create a new receipt.
    pub fn new(type_id: TransactionId, legacy_receipt: LegacyReceipt) -> Self {
        //curently we are using same receipt for both legacy and typed transaction
        match type_id {
            TransactionId::EIP1559 => Self::EIP1559(legacy_receipt),
            TransactionId::AccessList => Self::AccessList(legacy_receipt),
            TransactionId::Legacy => Self::Legacy(legacy_receipt),
        }
    }

    fn encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new();
        match self {
            Self::Legacy(receipt) => {
                receipt.rlp_append(&mut stream);
                stream.out().freeze().to_vec()
            }
            Self::AccessList(receipt) => {
                receipt.rlp_append(&mut stream);
                [&[TransactionId::AccessList as u8], stream.as_raw()].concat()
            }
            Self::EIP1559(receipt) => {
                receipt.rlp_append(&mut stream);
                [&[TransactionId::EIP1559 as u8], stream.as_raw()].concat()
            }
        }
    }

    fn decode(receipt: &[u8]) -> Result<Self, DecoderError> {
        // at least one byte needs to be present
        if receipt.is_empty() {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        let id = TransactionId::try_from(receipt[0])
            .map_err(|_| DecoderError::Custom("Unknown transaction id"))?;
        //other transaction types
        match id {
            TransactionId::EIP1559 => Ok(Self::EIP1559(rlp::decode(&receipt[1..])?)),
            TransactionId::AccessList => Ok(Self::AccessList(rlp::decode(&receipt[1..])?)),
            TransactionId::Legacy => Ok(Self::Legacy(rlp::decode(receipt)?)),
        }
    }
}

// impl Encodable for Receipt {
//     fn rlp_append(&self, stream: &mut RlpStream) {
//         stream.begin_list(4);
//         stream.append(&self.status);
//         stream.append(&self.cumulative_gas_used);
//         stream.append(&self.bloom);
//         stream.append_list(&self.logs);
//     }
// }

// impl Decodable for Receipt {
//     fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
//         let result = Receipt {
//             status: rlp.val_at(0)?,
//             cumulative_gas_used: rlp.val_at(1)?,
//             bloom: rlp.val_at(2)?,
//             logs: rlp.list_at(3)?,
//         };
//         Ok(result)
//     }
// }

#[derive(Eq, Hash, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
/// The typed transaction ID
pub enum TransactionId {
    EIP1559 = 0x02,
    AccessList = 0x01,
    Legacy = 0x00,
}

impl TryFrom<u8> for TransactionId {
    type Error = DecoderError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            id if id == TransactionId::EIP1559 as u8 => Ok(Self::EIP1559),
            id if id == TransactionId::AccessList as u8 => Ok(Self::AccessList),
            id if (id & 0x80) != 0x00 => Ok(Self::Legacy),
            id if id == TransactionId::Legacy as u8 => Ok(Self::Legacy),
            _ => Err(DecoderError::Custom(
                "Invalid byte selector for transaction type.",
            )),
        }
    }
}

impl TryFrom<Value> for TransactionId {
    type Error = DecoderError;

    fn try_from(val: Value) -> Result<Self, Self::Error> {
        let id = val.as_str().ok_or(DecoderError::Custom("Invalid tx id."))?;
        let id = id.trim_start_matches("0x");
        let id = id
            .parse::<u8>()
            .map_err(|_| DecoderError::Custom("Invalid tx id."))?;
        Self::try_from(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eth_types::bytes::hex_decode;
    use ethereum_types::H160;
    use serde_json::from_str;
    use std::{fs::File, io::Read, str::FromStr};
    //use crate::log::Log;

    #[test]
    fn basic_legacy() {
        let expected = hex_decode("0xf90162a02f697d671e9ae4ee24a43c4b0d7e15f1cb4ba6de1561120d43b9a4e8c4a8a6ee83040caeb9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000f838f794dcf421d093428b096ca501a7cd1a740855a7976fc0a00000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let receipt = Receipt::new(
            TransactionId::Legacy,
            LegacyReceipt::new(
                TransactionOutcome::StateRoot(
                    H256::from_str(
                        "2f697d671e9ae4ee24a43c4b0d7e15f1cb4ba6de1561120d43b9a4e8c4a8a6ee",
                    )
                    .unwrap(),
                ),
                U256::from_str_radix("40cae", 16).unwrap(),
                vec![Log {
                    address: H160::from_str("dcf421d093428b096ca501a7cd1a740855a7976f").unwrap(),
                    topics: vec![],
                    data: vec![0u8; 32],
                }],
            ),
        );
        let encoded = receipt.encode();
        assert_eq!(encoded, expected);
        let decoded = Receipt::decode(&encoded).expect("decoding receipt failed");
        assert_eq!(decoded, receipt);
    }

    // fn test_decode_external_rlp() {
    //     let mut encoded_receipts_json = String::new();
    //     File::open("src/test_data/receipts_encoded.json")
    //         .unwrap()
    //         .read_to_string(&mut encoded_receipts_json)
    //         .unwrap();

    //     let json_value: serde_json::Value =
    // serde_json::from_str(&encoded_receipts_json).unwrap();     let receipts =
    // json_value["receipts"].as_array().unwrap();

    //     for receipt_json in receipts.iter() {
    //         let receipt_bytes = serde_json::to_vec(&receipt_json).unwrap();
    //         // let receipt: receipt::Receipt =
    // from_str::<receipt::Receipt>(&receipt_str).unwrap();

    //         let decoded_receipt = rlp::decode::<Receipt>(&receipt_bytes).unwrap();
    //         let clone_rex = decoded_receipt.clone();
    //         // verify fields

    //         println!("{:?}", clone_rex.logs);

    //         // let receipt_bytes = rlp::encode(&receipt).to_vec();
    //     }
    // }

    // #[test]
    // fn test_no_state_root() {
    //     let r = Receipt::new(
    //         None,
    //         0x40cae.into(),
    //         vec![Log {
    //             address: "dcf421d093428b096ca501a7cd1a740855a7976f".into(),
    //             topics: vec![],
    //             data: //vec![0u8; 32],
    //             block_number: todo!(),
    //             transaction_hash: todo!(),
    //             transaction_index: todo!(),
    //             log_index: todo!(),
    //             transaction_log_index: todo!(),
    //             log_type: todo!(),
    //             removed: todo!(),
    //         }],
    //         None,
    //         1.into(),
    //         "2f697d671e9ae4ee24a43c4b0d7e15f1cb4ba6de1561120d43b9a4e8c4a8a6ee".into(),
    //     );
    //     let encoded = ::rlp::encode(&r);
    //     println!("encode ok");
    //     let decoded: Receipt = ::rlp::decode(&encoded);
    //     println!("decoded: {:?}", decoded);
    //     assert_eq!(decoded, r);
    // }
}
