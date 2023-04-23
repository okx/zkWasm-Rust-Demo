use wasm_bindgen::prelude::*;
use num_bigint::BigInt;
use rlp::{Encodable, Decodable, RlpStream, DecoderError};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PositionAsset {
    pub balance: BigInt,
    pub asset_id: u64,
    // A snapshot of the funding index at the last time that funding was applied (fxp 32.32).
    pub cached_funding_index: u128,
}

impl Encodable for PositionAsset {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(3);
        stream.append(&self.balance.to_signed_bytes_le());
        stream.append(&self.asset_id);
        stream.append(&self.cached_funding_index);
    }
}

impl Decodable for PositionAsset {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, DecoderError> {
        Ok(PositionAsset {
            balance: BigInt::from_signed_bytes_le(rlp.at(0)?.as_raw()),
            asset_id: rlp.val_at(1)?,
            cached_funding_index: rlp.val_at(2)?,
        })
    }
}


#[wasm_bindgen]
pub fn zkmain() -> i64 {
    // let data = vec![0x83, b'c', b'a', b't'];
    // let _animal: String = rlp::decode(&data).unwrap();
    // // assert_eq!(animal, "cat".to_owned());
    //
    // 0
    let asset = PositionAsset {
        balance: BigInt::from(1),
        asset_id: 2,
        cached_funding_index: 3,
    };
    let data = rlp::encode(&asset).to_vec();
    let asset2: PositionAsset = rlp::decode(&data).unwrap();
    asset2.asset_id as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rlp() {
        let asset = PositionAsset {
            balance: BigInt::from(1),
            asset_id: 2,
            cached_funding_index: 3,
        };
        println!("{:?}", asset);
        let data = rlp::encode(&asset).to_vec();
        println!("{:?}", data);

        let asset2: PositionAsset = rlp::decode(&data).unwrap();
        println!("{:?}", asset2);
    }

}
