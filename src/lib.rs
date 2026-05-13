//! wicket-guard — admissibility preflight for AI-agent-authored diffs.
//!
//! Wraps [`wicket`]. Cooks unified diffs into Intents focused on
//! authority-bearing surfaces (LICENSE, NOTICE, CODEOWNERS, SECURITY.md).

pub mod cook;
pub mod diff;
pub mod surfaces;

pub use cook::{cook_diff, CookOpts, CookedIntent};
pub use diff::{parse, FileChanges, ParsedDiff};
pub use surfaces::{classify_path, AuthoritySurface};
