#[derive(Debug, thiserror::Error)]
pub enum HolodocErrors {
    #[error("Random number generator returned no result")]
    RNGFailure,
    #[error("Invalid roll expression: '{0}'")]
    RollExprFormatError(String),
    #[error("Invalid {0} value: {1}")]
    RollExprValueError(String, String),
}
