use std::path::PathBuf;

use crate::error::Error;

use self::{
    base64::Base64,
    hash::{deep_hash, sha256, DeepHashItem},
    sign::Signer,
};

pub mod base64;
pub mod hash;
pub mod merkle;
pub mod sign;
pub mod utils;

#[derive(Default)]

pub struct Provider {
    pub signer: Option<Box<Signer>>,
}

impl Provider {
    pub fn from_keypair_path(keypair_path: PathBuf) -> Result<Self, Error> {
        let signer = Signer::from_keypair_path(keypair_path)?;
        Ok(Provider::new(Some(Box::new(signer))))
    }

    pub fn new(signer: Option<Box<Signer>>) -> Self {
        Provider { signer }
    }
}

impl Provider {
    pub fn deep_hash(&self, deep_hash_item: DeepHashItem) -> [u8; 48] {
        deep_hash(deep_hash_item)
    }

    pub fn sign(&self, message: &[u8]) -> Result<Base64, Error> {
        if let Some(signer) = &self.signer {
            signer.sign(message)
        } else {
            Err(Error::NoneError("No private key present".to_owned()))
        }
    }

    pub fn hash_sha256(&self, message: &[u8]) -> [u8; 32] {
        sha256(message)
    }

    pub fn keypair_modulus(&self) -> Result<Base64, Error> {
        if let Some(signer) = &self.signer {
            signer.keypair_modulus()
        } else {
            Err(Error::NoneError("No private key present".to_owned()))
        }
    }

    pub fn wallet_address(&self) -> Result<Base64, Error> {
        if let Some(signer) = &self.signer {
            signer.wallet_address()
        } else {
            Err(Error::NoneError("No private key present".to_owned()))
        }
    }

    pub fn public_key(&self) -> Result<Base64, Error> {
        if let Some(signer) = &self.signer {
            signer.public_key()
        } else {
            Err(Error::NoneError("No private key present".to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::{error::Error, verify::verify};

    use super::{base64::Base64, Provider};

    #[test]
    fn test_sign_verify() -> Result<(), Error> {
        let message = Base64(
            [
                9, 214, 233, 210, 242, 45, 194, 247, 28, 234, 14, 86, 105, 40, 41, 251, 52, 39,
                236, 214, 54, 13, 53, 254, 179, 53, 220, 205, 129, 37, 244, 142, 230, 32, 209, 103,
                68, 75, 39, 178, 10, 186, 24, 160, 179, 143, 211, 151,
            ]
            .to_vec(),
        );
        let path = PathBuf::from_str("res/test_wallet.json").unwrap();
        let provider = Provider::from_keypair_path(path)?;
        let signature = provider.sign(&message.0)?;
        let pubk = provider.public_key()?;
        assert!(verify(&pubk.0, &message.0, &signature.0).is_ok());
        Ok(())
    }
}
