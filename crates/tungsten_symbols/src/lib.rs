use std::collections::HashMap;

use bitflags::bitflags;
use indextree::{Arena, NodeId};
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
pub enum SymbolAttributeValue {}

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    symbols: HashMap<Atom, Symbol>,
    parent: Option<NodeId>,
}

impl SymbolTable {
    pub fn new(parent: Option<NodeId>) -> Self {
        Self {
            symbols: HashMap::new(),
            parent,
        }
    }

    pub fn add_symbol(&mut self, name: Atom, flags: SymbolFlags) {
        let symbol = Symbol {
            name: name.clone(),
            flags,
            attributes: HashMap::new(),
        };

        self.symbols.insert(name, symbol);
    }

    pub fn set_attribute<'a>(
        &'a mut self,
        name: Atom,
        attribute: Atom,
        value: SymbolAttributeValue,
        arena: Option<&'a mut Arena<SymbolTable>>,
    ) {
        if let Some(symbol) = self.get_symbol_mut(name, arena) {
            symbol.attributes.insert(attribute, value);
        }
    }

    pub fn get_attribute<'a>(
        &'a self,
        name: Atom,
        attribute: Atom,
        arena: Option<&'a Arena<SymbolTable>>,
    ) -> Option<&'a SymbolAttributeValue> {
        self.get_symbol(name, arena)?.attributes.get(&attribute)
    }

    pub fn get_symbol_mut<'a>(
        &'a mut self,
        name: Atom,
        arena: Option<&'a mut Arena<SymbolTable>>,
    ) -> Option<&'a mut Symbol> {
        if let Some(symbol) = self.symbols.get_mut(&name) {
            return Some(symbol);
        }

        if let Some(arena) = arena {
            if let Some(parent_id) = self.parent {
                if let Some(parent_table) = arena.get_mut(parent_id) {
                    return parent_table.get_mut().get_symbol_mut(name, None);
                }
            }
        }

        None
    }

    pub fn get_symbol<'a>(
        &'a self,
        name: Atom,
        arena: Option<&'a Arena<SymbolTable>>,
    ) -> Option<&'a Symbol> {
        if let Some(symbol) = self.symbols.get(&name) {
            return Some(symbol);
        }

        if let Some(arena) = arena {
            if let Some(parent_id) = self.parent {
                if let Some(parent_table) = arena.get(parent_id) {
                    return parent_table.get().get_symbol(name, Some(arena));
                }
            }
        }

        None
    }

    pub fn contains(&self, name: Atom) -> bool {
        self.symbols.contains_key(&name)
    }
}
