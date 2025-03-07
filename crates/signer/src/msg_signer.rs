use alloy::{
    primitives::keccak256,
    signers::{local::PrivateKeySigner, Signature as AlloySignature, SignerSync},
};
use secp256k1::{ecdsa::Signature, Message, PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha3::{Digest, Keccak256};
use std::str::FromStr;
use thiserror::Error;
use reqwest;

use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(1);

#[async_trait::async_trait]
pub trait Signer {
    type PrivKey;
    type PubKey;

    fn sign_message<T: Serialize>(&self, privkey: &Self::PrivKey, message: &T) -> String;
    async fn sign_message_remote<T: Serialize + Sync>(&self, message: &T) -> String;
    fn verify_signature<T: Serialize>(
        &self,
        pubkey: &Self::PubKey,
        message: &T,
        signature: &str,
    ) -> bool;
}

#[derive(Error, Debug)]
pub enum SignerError {
    #[error("Error: serde_json error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Clone, Copy, Debug)]
pub struct Keccak256Secp256k1(pub SecretKey);

impl Keccak256Secp256k1 {
    fn generate_hash<T: Serialize>(message: &T) -> Result<Vec<u8>, SignerError> {
        let serialized_message = serde_json::to_vec(message).expect("Failed to serialize message");
        let mut hasher = Keccak256::new();
        hasher.update(serialized_message);
        Ok(hasher.finalize().to_vec())
    }

    pub fn generate_hash_str<T: Serialize>(message: &T) -> Result<String, SignerError> {
        let serialized_message = serde_json::to_vec(message).expect("Failed to serialize message");
        let mut hasher = Keccak256::new();
        hasher.update(serialized_message);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }
}

#[async_trait::async_trait]
impl Signer for Keccak256Secp256k1 {
    type PrivKey = SecretKey;
    type PubKey = PublicKey;
    fn sign_message<T: Serialize>(&self, privkey: &Self::PrivKey, message: &T) -> String {
        let secp = secp256k1::Secp256k1::new();
        let hash = Self::generate_hash(message).unwrap();
        let msg = Message::from_slice(&hash).expect("hash need 32 bytes");
        let signature = secp.sign_ecdsa(&msg, privkey);
        hex::encode(signature.serialize_compact())
    }

    async fn sign_message_remote<T: Serialize + Sync>(&self, _message: &T) -> String {
        "".to_string()
    }

    fn verify_signature<T: Serialize>(
        &self,
        pubkey: &Self::PubKey,
        message: &T,
        signature: &str,
    ) -> bool {
        let secp = secp256k1::Secp256k1::new();
        let hash = Self::generate_hash(message).unwrap();
        let msg = Message::from_slice(&hash).expect("hash need 32 bytes");
        let sig = Signature::from_compact(&hex::decode(signature).expect("Invalid signature"))
            .expect("Invalid format");
        secp.verify_ecdsa(&msg, &sig, pubkey).is_ok()
    }
}

#[derive(Clone, Debug)]
pub struct MessageVerify(pub PrivateKeySigner, pub String, pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMethodMsg {
    pub id: String,
    pub method: Option<String>,
    pub params: Option<Value>,
    pub result: Option<Value>,
    pub address: String,
    pub hash: String,
    pub signature: String,
}

impl MessageVerify {
    pub fn generate_hash_str<T: Serialize>(message: &T) -> Result<String, SignerError> {
        let serialized_message = serde_json::to_vec(message).expect("Failed to serialize message");
        let mut hasher = Keccak256::new();
        hasher.update(serialized_message);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    pub fn get_address(&self) -> String {
        self.1.clone()
    }
    pub fn verify_message(message: &WsMethodMsg) -> bool {
        let sig = message.signature.as_str();
        let signature = AlloySignature::from_str(sig).unwrap();

        let mut origin_message = message.clone();
        origin_message.signature = String::new();

        let msg = serde_json::to_vec(&origin_message).unwrap();

        let origin = signature.recover_address_from_msg(&msg).unwrap();

        let address = message.address.clone();
        let addr = origin.to_string();
        addr.to_lowercase().eq(&address.to_lowercase())
    }
}

//{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":\"0x2ba756ccee00fb7845bb0e83067f62d45dd47cbc8e68107f16a13dfc6219ecb068e4f166b1d159ffe40ee3706519599f472d2669b0bd06dfbf5213bd16576c111b\"}
#[derive(Serialize, Deserialize, Debug)]
pub struct WsResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: String,
}

#[async_trait::async_trait]
impl Signer for MessageVerify {
    type PrivKey = PrivateKeySigner;
    type PubKey = String; //address
    fn sign_message<T: Serialize>(&self, privkey: &Self::PrivKey, message: &T) -> String {
        let msg = serde_json::to_vec(message).unwrap();
        let signature = self.0.sign_message_sync(&msg).unwrap();
        hex::encode(signature.as_bytes())
    }

    async fn sign_message_remote<T: Serialize + Sync>(&self, message: &T) -> String {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let msg = serde_json::to_string(message).unwrap();

        let data = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_sign",
            "params": [self.1, msg],
            "id": id,
        });
        let signaure = reqwest::Client::new()
            .post(self.2.as_str())
            .json(&data)
            .send()
            .await
            .unwrap();

    ;   signaure.json::<WsResponse>().await.unwrap().result
    }

    fn verify_signature<T: Serialize>(
        &self,
        pubkey: &Self::PubKey,
        message: &T,
        signature: &str,
    ) -> bool {
        let sig = AlloySignature::from_str(signature).unwrap();
        let msg = serde_json::to_vec(message).unwrap();
        let recover = sig.recover_address_from_msg(&msg).unwrap();

        let addr = recover.to_string();
        //addr.eq(pubkey)
        addr.to_lowercase().eq(&pubkey.to_lowercase())
    }
}
