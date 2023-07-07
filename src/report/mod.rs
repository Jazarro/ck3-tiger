pub use builder::{err, fatal, tips, untidy, warn};
pub use error_key::ErrorKey;
pub use error_loc::ErrorLoc;
pub use errors::*;
pub use filter::FilterRule;
pub use output_style::OutputStyle;
pub use report_struct::{Confidence, LogReport, PointedMessage, Severity};

mod builder;
mod error_key;
mod error_loc;
mod errors;
mod filter;
mod output_style;
mod report_struct;
mod writer;
