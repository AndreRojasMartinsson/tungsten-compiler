use core::panic;
use std::collections::HashMap;

use tungsten_ast::{CommaSeperatedList, DataType, FunctionFlags, FunctionItem, Node, Parameter};
use tungsten_context::error_builders;
use tungsten_symbols::{SymbolAttributeValue, SymbolFlags};
use tungsten_utils::{atom, Atom};

use crate::errors::ParserError;

use super::Parser;

impl Parser<'_> {
    pub(crate) fn add_function_to_scope(&mut self, item: &FunctionItem) {
        let name = item.name.image.clone();
        let func_span = &item.node.span;
        let flags = &item.flags;
        let parameters = &item.parameters;
        let return_type = &item.return_type;

        let scope = self.context.current_scope().unwrap();
        if scope.contains(atom!(name.clone())) {
            self.context
                .add_error(error_builders::build_duplicate_function_error(
                    name.to_string(),
                    func_span.clone(),
                    func_span.clone(),
                ));
        }

        let mut symbol_flags = SymbolFlags::FUNC;

        if flags.contains(FunctionFlags::PUBLIC) {
            symbol_flags |= SymbolFlags::PUB;
        }

        if flags.contains(FunctionFlags::CONSTANT) {
            symbol_flags |= SymbolFlags::CONST;
        }

        if flags.contains(FunctionFlags::GLOBAL) {
            symbol_flags |= SymbolFlags::GLOBAL;
        }

        scope.add_symbol(
            atom!(name.to_string()),
            symbol_flags,
            Some(HashMap::from([
                (
                    atom!("return_type"),
                    SymbolAttributeValue::String(atom!(return_type.to_string())),
                ),
                (
                    atom!("return_type"),
                    SymbolAttributeValue::String(atom!(return_type.to_string())),
                ),
            ])),
        );

        scope.set_attribute(
            atom!(name.to_string()),
            atom!("return_type"),
            SymbolAttributeValue::String(atom!(return_type.to_string())),
        );

        parameters.items.iter().for_each(|param| {
            scope.set_attribute(
                atom!(name.to_string()),
                atom!(format!("param:{}", param.name.image)),
                SymbolAttributeValue::String(atom!(param.r#type.to_string())),
            );
        });
    }
}
