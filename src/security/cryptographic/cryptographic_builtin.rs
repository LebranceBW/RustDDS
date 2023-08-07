mod aes_gcm_gmac;
mod crypto_key_exchange;
mod crypto_key_factory;
mod crypto_transform;
mod decode;
mod encode;
mod types;

use std::collections::{HashMap, HashSet};

use crate::{
  security::{
    access_control::types::*,
    authentication::types::*,
    cryptographic::{cryptographic_builtin::types::*, cryptographic_plugin::*, types::*},
    types::*,
  },
  security_error,
};

// A struct implementing the built-in Cryptographic plugin
// See sections 8.5 and 9.5 of the Security specification (v. 1.1)
pub struct CryptographicBuiltIn {
  encode_keys_: HashMap<CryptoHandle, KeyMaterial_AES_GCM_GMAC_seq>,
  decode_keys_: HashMap<CryptoHandle, KeyMaterial_AES_GCM_GMAC_seq>,
  participant_encrypt_options_: HashMap<ParticipantCryptoHandle, ParticipantSecurityAttributes>,
  entity_encrypt_options_: HashMap<EntityCryptoHandle, EndpointSecurityAttributes>,
  participant_to_entity_info_: HashMap<ParticipantCryptoHandle, HashSet<EntityInfo>>,
  // For reverse lookups
  entity_to_participant_: HashMap<EntityCryptoHandle, ParticipantCryptoHandle>,

  // sessions_ ?
  /// For each (local datawriter (/datareader), remote participant) pair, stores
  /// the matched remote datareader (/datawriter)
  matched_remote_entity_:
    HashMap<EntityCryptoHandle, HashMap<ParticipantCryptoHandle, EntityCryptoHandle>>,
  ///For reverse lookups,  for each remote datawriter (/datareader), stores the
  /// matched local datareader (/datawriter)
  matched_local_entity_: HashMap<EntityCryptoHandle, EntityCryptoHandle>,

  handle_counter_: u32,
}

// Combine the trait implementations from the submodules
impl super::Cryptographic for CryptographicBuiltIn {}

impl CryptographicBuiltIn {
  pub fn new() -> Self {
    CryptographicBuiltIn {
      encode_keys_: HashMap::new(),
      decode_keys_: HashMap::new(),
      participant_encrypt_options_: HashMap::new(),
      entity_encrypt_options_: HashMap::new(),
      participant_to_entity_info_: HashMap::new(),
      entity_to_participant_: HashMap::new(),
      matched_remote_entity_: HashMap::new(),
      matched_local_entity_: HashMap::new(),
      handle_counter_: 0,
    }
  }

  fn insert_encode_keys_(
    &mut self,
    handle: CryptoHandle,
    keys: KeyMaterial_AES_GCM_GMAC_seq,
  ) -> SecurityResult<()> {
    match self.encode_keys_.insert(handle, keys) {
      None => SecurityResult::Ok(()),
      Some(old_key_materials) => {
        self.encode_keys_.insert(handle, old_key_materials);
        SecurityResult::Err(security_error!(
          "The handle {} was already associated with encode key material",
          handle
        ))
      }
    }
  }
  fn get_encode_keys_(
    &self,
    handle: &CryptoHandle,
  ) -> SecurityResult<&KeyMaterial_AES_GCM_GMAC_seq> {
    self.encode_keys_.get(handle).ok_or(security_error!(
      "Could not find encode keys for the handle {}",
      handle
    ))
  }

  fn insert_decode_keys_(
    &mut self,
    handle: CryptoHandle,
    keys: KeyMaterial_AES_GCM_GMAC_seq,
  ) -> SecurityResult<()> {
    match self.decode_keys_.insert(handle, keys) {
      None => SecurityResult::Ok(()),
      Some(old_key_materials) => {
        self.decode_keys_.insert(handle, old_key_materials);
        SecurityResult::Err(security_error!(
          "The handle {} was already associated with decode key material",
          handle
        ))
      }
    }
  }

  fn get_decode_keys_(
    &self,
    handle: &CryptoHandle,
  ) -> SecurityResult<&KeyMaterial_AES_GCM_GMAC_seq> {
    self.decode_keys_.get(handle).ok_or(security_error!(
      "Could not find decode keys for the handle {}",
      handle
    ))
  }

  fn insert_entity_info_(
    &mut self,
    participant_handle: ParticipantCryptoHandle,
    entity_info: EntityInfo,
  ) {
    match self
      .participant_to_entity_info_
      .get_mut(&participant_handle)
    {
      Some(entity_set) => {
        entity_set.insert(entity_info);
      }
      None => {
        self
          .participant_to_entity_info_
          .insert(participant_handle, HashSet::from([entity_info]));
      }
    };
  }

  fn insert_participant_attributes_(
    &mut self,
    handle: ParticipantCryptoHandle,
    attributes: ParticipantSecurityAttributes,
  ) -> SecurityResult<()> {
    match self.participant_encrypt_options_.insert(handle, attributes) {
      None => SecurityResult::Ok(()),
      Some(old_attributes) => {
        self
          .participant_encrypt_options_
          .insert(handle, old_attributes);
        SecurityResult::Err(security_error!(
          "The handle {} was already associated with security attributes",
          handle
        ))
      }
    }
  }

  fn insert_entity_attributes_(
    &mut self,
    handle: EntityCryptoHandle,
    attributes: EndpointSecurityAttributes,
  ) -> SecurityResult<()> {
    match self.entity_encrypt_options_.insert(handle, attributes) {
      None => SecurityResult::Ok(()),
      Some(old_attributes) => {
        self.entity_encrypt_options_.insert(handle, old_attributes);
        SecurityResult::Err(security_error!(
          "The handle {} was already associated with security attributes",
          handle
        ))
      }
    }
  }
}
