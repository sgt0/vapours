//! Errors.

use std::string::String;

use miette::Diagnostic;
use thiserror::Error;

/// Errors from vapours.
#[derive(Debug, Diagnostic, Error)]
pub enum VapoursError {
  /// Missing dependency.
  #[error("Missing dependency '{0}'.")]
  DependencyNotFoundError(String),

  /// Frame property error.
  #[error("Error while trying to access frame property '{0}'.")]
  FramePropertyError(String),
}
