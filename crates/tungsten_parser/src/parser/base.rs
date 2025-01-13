use tungsten_ast::*;
use tungsten_context::CompilerContext;
use tungsten_lexer::{Kind, Token, Value};

#[derive(Debug)]
pub struct Parser<'a> {
    pub(crate) context: &'a mut CompilerContext<'a>,
    pub(crate) source: &'a str,
    pub(crate) tokens: Vec<Token>,
    pub(crate) prev_token_end: usize,
}

macro_rules! chain {
    ($($kw:ident),*) => {
       &[$(Kind::$kw),*]
    };
}

impl<'a> Parser<'a> {
    pub fn new(context: &'a mut CompilerContext<'a>, source: &'a str) -> Self {
        Self {
            source,
            context,
            prev_token_end: 0,
            tokens: Vec::new(),
        }
    }

    pub fn parse(&mut self, tokens: &[Token]) -> SourceFile {
        self.tokens = tokens.to_vec();

        SourceFile {
            node: Node::new(0..self.source.len() + 1),
            body: self.parse_body(),
        }
    }
    pub(crate) fn parse_body(&mut self) -> Body {
        self.context.push_scope();

        let mut items = vec![];

        loop {
            if self.is_at_end() {
                break;
            }

            if let Some(node) = self.parse_body_item() {
                items.push(node);
            }
        }

        Body {
            node: Node::new(0..self.source.len()),
            items,
            symbol: self.context.pop_scope().unwrap(),
        }
    }

    pub(crate) fn parse_body_item(&mut self) -> Option<BodyItem> {
        match self.current_kind() {
            Kind::Comment | Kind::Semicolon | Kind::Illegal => {
                self.advance();
                None
            }
            Kind::ConstKw => {
                self.advance();

                match self.current_kind() {
                    Kind::FuncKw => Some(BodyItem::Func(Box::new(
                        self.parse_function_item(FunctionFlags::CONSTANT | FunctionFlags::GLOBAL),
                    ))),
                    // TODO: Constant Variable
                    _ => unreachable!(),
                }
            }
            Kind::PubKw => {
                self.advance();

                match self.current_kind() {
                    Kind::FuncKw => Some(BodyItem::Func(Box::new(
                        self.parse_function_item(FunctionFlags::PUBLIC | FunctionFlags::GLOBAL),
                    ))),
                    Kind::ConstKw => {
                        self.advance();

                        match self.current_kind() {
                            Kind::FuncKw => {
                                Some(BodyItem::Func(Box::new(self.parse_function_item(
                                    FunctionFlags::CONSTANT
                                        | FunctionFlags::GLOBAL
                                        | FunctionFlags::PUBLIC,
                                ))))
                            }
                            // TODO: Public Constant Variable
                            _ => unreachable!(),
                        }
                    }
                    // TODO: Enums, structs, etc
                    _ => unreachable!(),
                }
            }
            Kind::FuncKw => Some(BodyItem::Func(Box::new(
                self.parse_function_item(FunctionFlags::GLOBAL),
            ))),
            token => panic!("Invalid token inside of body. Expected BodyItem, got: {token:?}"),
        }
    }

    pub(crate) fn parse_function_item(&mut self, flags: FunctionFlags) -> FunctionItem {
        let node = self.start_node();

        // Consume the optional prefix const
        self.eat(Kind::ConstKw);
        // Consume the optional pub
        self.eat(Kind::PubKw);
        // Consume the optional postfix const
        self.eat(Kind::ConstKw);
        self.bump_seq(chain![FuncKw, Colon]);

        let return_type = self.parse_data_type();
        self.bump(Kind::FatArrow);

        let name = self.parse_identifier();

        self.bump(Kind::LParen);
        let parameters = self.parse_parameters();
        self.bump(Kind::RParen);

        let item = FunctionItem {
            node: self.finish_node(node),
            return_type,
            name,
            flags,
            parameters,
            block: self.parse_block(),
        };

        self.add_function_to_scope(&item);
        item
    }

    pub(crate) fn parse_block(&mut self) -> Block {
        let node = self.start_node();
        self.context.push_scope();

        self.bump(Kind::LBrace);

        let mut items = vec![];

        while !self.is_at_end() && !self.at(Kind::RBrace) {
            if let Some(item) = self.parse_block_item() {
                items.push(item);
            }
        }

        self.bump(Kind::RBrace);

        Block {
            node: self.finish_node(node),
            symbol: self.context.pop_scope().unwrap(),
            items,
        }
    }

    pub(crate) fn parse_parameters(&mut self) -> CommaSeperatedList<Parameter> {
        let node = self.start_node();

        if self.at(Kind::RParen) {
            return CommaSeperatedList {
                node: self.finish_node(node),
                items: vec![],
            };
        }

        let mut items = vec![self.parse_parameter()];

        while !self.is_at_end() && self.at(Kind::Comma) {
            self.bump(Kind::Comma);

            let parameter = self.parse_parameter();
            items.push(parameter);
        }

        CommaSeperatedList {
            node: self.finish_node(node),
            items,
        }
    }

    pub(crate) fn parse_parameter(&mut self) -> Parameter {
        let node = self.start_node();
        let name = self.parse_identifier();
        self.bump(Kind::Colon);
        let data_type = self.parse_data_type();

        Parameter {
            node: self.finish_node(node),
            r#type: data_type,
            name,
        }
    }

    pub(crate) fn parse_identifier(&mut self) -> Identifier {
        let node = self.start_node();
        let token = self.bump(Kind::Identifier);
        let value = match token.value.unwrap() {
            Value::String(value) => value,
            _ => unreachable!(),
        };

        Identifier {
            node: self.finish_node(node),
            image: value,
        }
    }

    pub(crate) fn parse_data_type(&mut self) -> DataType {
        let token = self.bump_any(chain!(PrimitiveType, Identifier, LBracket, LParArrow));
        let value = token.value.clone().unwrap();

        match &token.kind {
            Kind::LBracket => self.parse_array_type(),
            Kind::LParArrow => self.parse_tuple_type(),
            // TODO: Generic types
            Kind::Identifier => match value {
                Value::String(value) => DataType::Simple(SimpleType::Identifier(value)),
                _ => unreachable!(),
            },
            Kind::PrimitiveType => match value {
                Value::String(value) => match &*value {
                    "bool" => DataType::Simple(SimpleType::Boolean),
                    "str" => DataType::Simple(SimpleType::String),
                    "float" => DataType::Simple(SimpleType::Float),
                    "uint" => DataType::Simple(SimpleType::UnsignedInteger),
                    "int" => DataType::Simple(SimpleType::SignedInteger),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            c => unreachable!("{c:?}"),
        }
    }

    pub(crate) fn parse_tuple_type(&mut self) -> DataType {
        let mut types = vec![self.parse_data_type()];

        while !self.is_at_end() && self.at(Kind::Comma) {
            self.bump(Kind::Comma);

            types.push(self.parse_data_type());
        }

        DataType::Tuple(types)
    }

    pub(crate) fn parse_array_type(&mut self) -> DataType {
        let data_type = self.parse_data_type();
        let mut arr_len: Option<_> = None;

        let (ok, _) = self.eat(Kind::Colon);
        if ok {
            arr_len = Some(self.parse_literal());
        }

        self.bump(Kind::RBracket);

        DataType::Array(Box::new((data_type, arr_len)))
    }

    pub(crate) fn parse_literal(&mut self) -> Literal {
        let node = self.start_node();
        let token = self.bump_any(chain!(
            StringLiteral,
            FloatLiteral,
            BooleanLiteral,
            IntegerLiteral
        ));

        Literal {
            node: self.finish_node(node),
            value: token.value.unwrap(),
        }
    }

    pub(crate) fn parse_block_item(&mut self) -> Option<BlockItem> {
        match self.current_kind() {
            Kind::Comment | Kind::Semicolon | Kind::Illegal => {
                self.advance();
                None
            }
            Kind::VarKw => Some(BlockItem::Variable(Box::new(self.parse_variable_item()))),
            Kind::ReturnKw => Some(BlockItem::Return(Box::new(self.parse_return()))),
            _ => unimplemented!(),
        }
    }

    fn parse_return(&mut self) -> ReturnItem {
        let node = self.start_node();
        self.bump(Kind::ReturnKw);

        let mut expr: Option<Expression> = None;
        if !self.eat_ignore(Kind::Semicolon) {
            expr = Some(self.parse_expression(None, 0));
        }

        ReturnItem {
            node: self.finish_node(node),
            initializer: expr,
        }
    }

    fn parse_variable_item(&mut self) -> VariableItem {
        let mut flags = VariableFlags::empty();
        let node = self.start_node();
        self.bump(Kind::VarKw);

        let name = self.parse_identifier();
        self.bump(Kind::Colon);

        if self.eat_ignore(Kind::MutKw) {
            flags = VariableFlags::MUTABLE;
        }

        let r#type = self.parse_data_type();
        let mut initializer: Option<Expression> = None;

        if self.eat_ignore(Kind::Assign) {
            // Got a initializer
            initializer = Some(self.parse_expression(None, 0));
        }

        VariableItem {
            node: self.finish_node(node),
            name,
            r#type,
            flags,
            initializer,
        }
    }
}
