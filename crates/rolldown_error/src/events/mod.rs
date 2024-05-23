use std::fmt::Debug;

use crate::{
  diagnostic::Diagnostic, event_kind::EventKind, types::diagnostic_options::DiagnosticOptions,
};

pub mod circular_dependency;
pub mod eval;
pub mod external_entry;
pub mod forbid_const_assign;
pub mod sourcemap_error;
pub mod unresolved_entry;
pub mod unresolved_import;
pub mod unresolved_import_treated_as_external;

pub trait BuildEvent: Debug + Sync + Send {
  fn kind(&self) -> EventKind;

  fn code(&self) -> &'static str;

  fn message(&self, opts: &DiagnosticOptions) -> String;

  fn on_diagnostic(&self, _diagnostic: &mut Diagnostic, _opts: &DiagnosticOptions) {}
}

impl<T: BuildEvent + 'static> From<T> for Box<dyn BuildEvent>
where
  Self: Sized,
{
  fn from(e: T) -> Self {
    Box::new(e)
  }
}

// --- TODO(hyf0): These errors are only for compatibility with legacy code. They should be replaced with more specific errors.

#[derive(Debug)]
pub struct NapiError {
  pub status: String,
  pub reason: String,
}

impl BuildEvent for NapiError {
  fn kind(&self) -> EventKind {
    EventKind::NapiError
  }
  fn code(&self) -> &'static str {
    "NAPI_ERROR"
  }

  fn message(&self, _opts: &DiagnosticOptions) -> String {
    format!("Napi error: {status}: {reason}", status = self.status, reason = self.reason)
  }
}

impl BuildEvent for std::io::Error {
  fn kind(&self) -> EventKind {
    EventKind::IoError
  }
  fn code(&self) -> &'static str {
    "IO_ERROR"
  }

  fn message(&self, _opts: &DiagnosticOptions) -> String {
    format!("IO error: {self}")
  }
}

// --- end
