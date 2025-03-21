//!
//! The `solc --standard-json` input settings.
//!

pub mod libraries;
pub mod metadata;
pub mod optimizer;
pub mod selection;

use std::collections::BTreeSet;

use self::libraries::Libraries;
use self::metadata::Metadata;
use self::optimizer::Optimizer;
use self::selection::Selection;

///
/// The `solc --standard-json` input settings.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// The optimizer settings.
    #[serde(default)]
    pub optimizer: Optimizer,

    /// The linker library addresses.
    #[serde(default, skip_serializing_if = "Libraries::is_empty")]
    pub libraries: Libraries,
    /// The sorted list of remappings.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub remappings: BTreeSet<String>,

    /// The target EVM version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_version: Option<era_compiler_common::EVMVersion>,
    /// Whether to compile Solidity via IR.
    #[serde(
        default,
        rename = "viaIR",
        skip_serializing_if = "Settings::is_via_ir_default"
    )]
    pub via_ir: bool,

    /// The output selection filters.
    #[serde(default)]
    pub output_selection: Selection,
    /// The metadata settings.
    #[serde(default)]
    pub metadata: Metadata,

    /// The extra LLVM options.
    #[serde(default, skip_serializing)]
    pub llvm_options: Vec<String>,
}

impl Settings {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        optimizer: Optimizer,

        libraries: Libraries,
        remappings: BTreeSet<String>,

        evm_version: Option<era_compiler_common::EVMVersion>,
        via_ir: bool,

        output_selection: Selection,
        metadata: Metadata,
        llvm_options: Vec<String>,
    ) -> Self {
        Self {
            optimizer,

            libraries,
            remappings,

            evm_version,
            via_ir,

            output_selection,
            metadata,
            llvm_options,
        }
    }

    ///
    /// Extends the output selection with another one.
    ///
    pub fn extend_selection(&mut self, selection: Selection) {
        self.output_selection.extend(selection);
    }

    ///
    /// Returns flags that are going to be automatically added by the compiler,
    /// but were not explicitly requested by the user.
    ///
    /// Afterwards, the flags are used to prune JSON output before returning it.
    ///
    pub fn selection_to_prune(&self) -> Selection {
        self.output_selection.selection_to_prune()
    }

    ///
    /// Whether the via IR flag is the default.
    ///
    fn is_via_ir_default(via_ir: &bool) -> bool {
        !via_ir
    }
}
