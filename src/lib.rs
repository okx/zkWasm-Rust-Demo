use wasm_bindgen::prelude::*;
use num_bigint::BigInt;
use bincode::{config, Decode, Encode, enc::Encoder, de::Decoder, error::{EncodeError, DecodeError}};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PositionAsset {
    pub balance: BigInt,
    pub asset_id: u64,
    // A snapshot of the funding index at the last time that funding was applied (fxp 32.32).
    pub cached_funding_index: u128,
}

impl Encode for PositionAsset {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.balance.to_signed_bytes_be().encode(encoder)?;
        self.asset_id.encode(encoder)?;
        self.cached_funding_index.encode(encoder)
    }
}

impl Decode for PositionAsset {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let balance: Vec<u8> = bincode::Decode::decode(decoder)?;
        Ok(Self{
            balance: BigInt::from_signed_bytes_be(&balance),
            asset_id: bincode::Decode::decode(decoder)?,
            cached_funding_index: bincode::Decode::decode(decoder)?,
        })
    }
}


#[wasm_bindgen]
pub fn zkmain() -> i64 {
    let config = config::standard().with_fixed_int_encoding().with_big_endian();
    let asset = PositionAsset {
        balance: BigInt::from(1),
        asset_id: 2,
        cached_funding_index: 3,
    };
    let data = bincode::encode_to_vec(&asset, config).unwrap();
    let asset2: PositionAsset = bincode::decode_from_slice(&data, config).unwrap().0;
    asset2.asset_id as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bincode() {
        let config = config::standard().with_fixed_int_encoding().with_little_endian();
        let a: u128 = 1<<31;
        let asset = PositionAsset {
            balance: BigInt::from(a),
            asset_id: 2,
            cached_funding_index: 259,
        };
        println!("{:?}", asset);

        let data = bincode::encode_to_vec(&asset, config).unwrap();
        println!("{:?}, {}", data, data.len());

        //let (asset2, s):(PositionAsset, _) = bincode::decode_from_slice(&data, config).unwrap();


        let asset2: PositionAsset = bincode::decode_from_slice(&data, config).unwrap().0;
        println!("{:?}", asset2);
    }

}
