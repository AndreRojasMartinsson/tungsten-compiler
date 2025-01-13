use std::{collections::HashMap, fmt};

use bitflags::bitflags;
use tungsten_utils::Atom;

bitflags! {
    #[derive(Debug, Clone)]
    pub struct SymbolFlags: u8 {
        const NONE = 1 << 0;
        /// Public symbol
        const PUB = 1 << 1;
        /// Constant symbol
        const CONST = 1 << 2;
        /// Static symbol
        const STATIC = 1 << 3;
        /// Function
        const FUNC = 1 << 4;
        /// Variable
        const VARIABLE = 1 << 5;
        /// Global scope
        const GLOBAL = 1 << 6;
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: Atom,
    pub flags: SymbolFlags,
    pub attributes: HashMap<Atom, SymbolAttributeValue>,
}

#[derive(Debug, Clone)]
pub enum SymbolAttributeValue {
    String(Atom),
}

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    symbols: HashMap<Atom, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn add_symbol(
        &mut self,
        name: Atom,
        flags: SymbolFlags,
        attributes: Option<HashMap<Atom, SymbolAttributeValue>>,
    ) {
        let symbol = Symbol {
            name: name.clone(),
            flags,
            attributes: attributes.unwrap_or_default(),
        };

        self.symbols.insert(name, symbol);
    }

    pub fn set_attribute(&mut self, name: Atom, attribute: Atom, value: SymbolAttributeValue) {
        if let Some(symbol) = self.get_symbol_mut(name) {
            symbol.attributes.insert(attribute, value);
        }
    }

    pub fn get_attribute(&self, name: Atom, attribute: Atom) -> Option<&SymbolAttributeValue> {
        self.get_symbol(name)?.attributes.get(&attribute)
    }

    pub fn get_symbol_mut(&mut self, name: Atom) -> Option<&mut Symbol> {
        self.symbols.get_mut(&name)
    }

    pub fn get_symbol(&self, name: Atom) -> Option<&Symbol> {
        self.symbols.get(&name)
    }

    pub fn contains(&self, name: Atom) -> bool {
        self.symbols.contains_key(&name)
    }
}
