//!
//! The `solc --standard-json` expected output selection.
//!

pub mod selector;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use self::selector::Selector;

///
/// The `solc --standard-json` expected output selection.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Selection {
    /// The inner selection map.
    #[serde(flatten)]
    inner: BTreeMap<String, BTreeMap<String, BTreeSet<Selector>>>,
}

impl Selection {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(via_ir: bool) -> Self {
        let mut root = BTreeMap::new();
        let mut inner = BTreeMap::new();
        let mut set = BTreeSet::new();
        set.insert(via_ir.into());
        inner.insert("*".to_owned(), set);
        root.insert("*".to_owned(), inner);
        Self { inner: root }
    }

    ///
    /// Extends the output selection with the IR required for compilation.
    ///
    pub fn extend(&mut self, via_ir: bool) {
        for file in self.inner.values_mut() {
            for contract in file.values_mut() {
                contract.insert(via_ir.into());
            }
        }
    }

    ///
    /// Retains only the selectors that request data from `solc`.
    ///
    pub fn retain_solc(&mut self) {
        for file in self.inner.values_mut() {
            for contract in file.values_mut() {
                contract.retain(Selector::is_received_from_solc);
            }
        }
    }

    ///
    /// Returns flags that are going to be automatically added by the compiler,
    /// but were not explicitly requested by the user.
    ///
    /// Afterwards, the flags are used to prune JSON output before returning it.
    ///
    pub fn to_prune(&self, via_ir: bool) -> BTreeSet<Selector> {
        let mut selection = BTreeSet::new();
        selection.insert(via_ir.into());
        selection
    }

    ///
    /// Whether the selection is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
