use serde::{Deserialize, Serialize};

use super::constraint::Constraint;
use super::export_format::ExportFormat;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExportSetting {
    pub suffix: String,
    pub format: ExportFormat,
    pub constraint: Constraint,
}
