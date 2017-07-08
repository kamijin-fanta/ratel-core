use ast::{Loc, List, Value, OperatorKind, Function, Class, OptionalName};
use ast::{PropertyPtr, IdentifierPtr, IdentifierList};
use ast::{ExpressionPtr, ExpressionList, StatementPtr, StatementList};

#[derive(Debug, PartialEq, Clone)]
pub enum Property<'ast> {
    Computed(ExpressionPtr<'ast>),
    Literal(&'ast str),
    Binary(u64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectMember<'ast> {
    Shorthand(&'ast str),
    Value {
        property: PropertyPtr<'ast>,
        value: ExpressionPtr<'ast>,
    },
    Method {
        property: PropertyPtr<'ast>,
        params: IdentifierList<'ast>,
        body: StatementList<'ast>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression<'ast> {
    Error,
    Void,
    This,
    Identifier(&'ast str),
    Value(Value<'ast>),
    Sequence {
        body: ExpressionList<'ast>
    },
    Array {
        body: ExpressionList<'ast>
    },
    Member {
        object: ExpressionPtr<'ast>,
        property: IdentifierPtr<'ast>,
    },
    ComputedMember {
        object: ExpressionPtr<'ast>,
        property: ExpressionPtr<'ast>,
    },
    Call {
        callee: ExpressionPtr<'ast>,
        arguments: ExpressionList<'ast>,
    },
    Binary {
        operator: OperatorKind,
        left: ExpressionPtr<'ast>,
        right: ExpressionPtr<'ast>,
    },
    Prefix {
        operator: OperatorKind,
        operand: ExpressionPtr<'ast>,
    },
    Postfix {
        operator: OperatorKind,
        operand: ExpressionPtr<'ast>,
    },
    Conditional {
        test: ExpressionPtr<'ast>,
        consequent: ExpressionPtr<'ast>,
        alternate: ExpressionPtr<'ast>,
    },
    Arrow {
        params: IdentifierList<'ast>,
        body: StatementPtr<'ast>,
    },
    Object {
        body: List<'ast, Loc<ObjectMember<'ast>>>,
    },
    Function {
        function: Function<'ast, OptionalName<'ast>>,
    },
    Class {
        class: Class<'ast, OptionalName<'ast>>,
    },
}

impl<'ast> Expression<'ast> {
    #[inline]
    pub fn at(self, start: u32, end: u32) -> Loc<Expression<'ast>> {
        Loc::new(start, end, self)
    }

    #[inline]
    pub fn binding_power(&self) -> u8 {
        use self::Expression::*;

        match *self {
            Member { .. } | Arrow { .. } => 18,

            Call { .. } => 17,

            Prefix { .. } => 15,

            Binary { ref operator, .. }  |
            Postfix { ref operator, .. } => operator.binding_power(),

            Conditional { .. } => 4,

            Sequence { .. } => 0,

            _  => 100,
        }
    }

    #[inline]
    pub fn is_allowed_as_bare_statement(&self) -> bool {
        use self::Expression::*;

        match *self {
            Object { .. }   |
            Function { .. } |
            Class { .. }    => false,
            _               => true,
        }
    }
}