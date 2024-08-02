

#[derive(Debug, thiserror::Error)]
pub enum HolodocErrors {
  #[error("Random number generator returned no result")]
  RNGFailure,
}