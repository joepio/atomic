//! Logic for Agents
//! Agents are actors (such as users) that can edit content.
//! https://docs.atomicdata.dev/commits/concepts.html

use crate::{Resource, Storelike, datetime_helpers, errors::AtomicResult, urls};

#[derive(Clone, Debug)]
pub struct Agent {
  /// Private key for signing commits
  pub private_key: String,
  /// Private key for signing commits
  pub public_key: String,
  /// URL of the Agent
  pub subject: String,
  pub created_at: i64,
  pub name: String,
}

impl Agent {
  /// Converts Agent to Resource.
  /// Does not include private key, only public.
  pub fn to_resource(&self, store: &impl Storelike) -> AtomicResult<Resource> {
    let mut agent = Resource::new_instance(urls::AGENT, store)?;
    agent.set_subject(self.subject.clone());
    agent.set_propval_string(crate::urls::NAME.into(), &self.name, store)?;
    agent.set_propval_string(crate::urls::PUBLIC_KEY.into(), &self.public_key, store)?;
    agent.set_propval_string(crate::urls::CREATED_AT.into(), &self.created_at.to_string(), store)?;
    Ok(agent)
  }

  /// Creates a new Agent, generates a new Keypair.
  pub fn new(name: String, store: &impl Storelike) -> AtomicResult<Agent> {
    let keypair = generate_keypair()?;

    Ok(Agent::new_from_private_key(name, store, &keypair.private))
  }

  pub fn new_from_private_key(name: String, store: &impl Storelike, private_key: &str) -> Agent {
    let keypair = generate_public_key(private_key);

    Agent {
      private_key: keypair.private,
      public_key: keypair.public.clone(),
      subject: format!("{}/agents/{}", store.get_base_url(), keypair.public),
      name,
      created_at: datetime_helpers::now(),
    }
  }
}

/// keypair, serialized using base64
pub struct Pair {
  pub private: String,
  pub public: String,
}

/// Returns a new random keypair.
fn generate_keypair() -> AtomicResult<Pair> {
  use ring::signature::KeyPair;
  let rng = ring::rand::SystemRandom::new();
  const SEED_LEN: usize = 32;
  let seed: [u8; SEED_LEN] = ring::rand::generate(&rng).map_err(|_| "Error generating random seed: {}")?.expose();
  let key_pair = ring::signature::Ed25519KeyPair::from_seed_unchecked(&seed)
      .map_err(|e| format!("Error generating keypair {}", e)).unwrap();
  Ok(Pair {
    private: base64::encode(&seed),
    public: base64::encode(&key_pair.public_key()),
  })
}

/// Returns a Key Pair (including public key) from a private key, base64 encoded.
pub fn generate_public_key(private_key: &str) -> Pair {
  use ring::signature::KeyPair;
  let private_key_bytes = base64::decode(private_key).unwrap();
  let key_pair = ring::signature::Ed25519KeyPair::from_seed_unchecked(private_key_bytes.as_ref())
      .map_err(|_| "Error generating keypair").unwrap();
  Pair {
    private: base64::encode(private_key_bytes),
    public: base64::encode(key_pair.public_key().as_ref()),
  }
}

#[cfg(test)]
mod test {
#[cfg(test)]
    use super::*;

  #[test]
  fn keypair() {
    let pair = generate_keypair().unwrap();
    let regenerated_pair = generate_public_key(&pair.private);
    assert_eq!(pair.public, regenerated_pair.public);
  }

  #[test]
  fn generate_from_private_key() {
    let private_key = "CapMWIhFUT+w7ANv9oCPqrHrwZpkP2JhzF9JnyT6WcI=";
    let public_key = "7LsjMW5gOfDdJzK/atgjQ1t20J/rw8MjVg6xwqm+h8U=";
    let regenerated_pair = generate_public_key(private_key);
    assert_eq!(public_key, regenerated_pair.public);
  }
}
