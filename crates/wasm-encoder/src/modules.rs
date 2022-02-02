use crate::{encoders, ComponentSection, ComponentSectionId, Module};

/// An encoder for the module section of WebAssembly components.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{Module, Component, ModuleSection};
///
/// let mut modules = ModuleSection::new();
/// modules.module(&Module::new());
///
/// let mut component = Component::new();
/// component.section(&modules);
///
/// let bytes = component.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct ModuleSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl ModuleSection {
    /// Create a new module section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of modules in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Writes a module into this module section.
    ///
    /// The given module must be fully formed.
    pub fn module(&mut self, module: &Module) -> &mut Self {
        self.bytes.extend(
            encoders::u32(u32::try_from(module.bytes.len()).unwrap())
                .chain(module.bytes.iter().copied()),
        );
        self.num_added += 1;
        self
    }
}

impl ComponentSection for ModuleSection {
    fn id(&self) -> u8 {
        ComponentSectionId::Module.into()
    }

    fn encode<S>(&self, sink: &mut S)
    where
        S: Extend<u8>,
    {
        let num_added = encoders::u32(self.num_added);
        let n = num_added.len();
        sink.extend(
            encoders::u32(u32::try_from(n + self.bytes.len()).unwrap())
                .chain(num_added)
                .chain(self.bytes.iter().copied()),
        );
    }
}
