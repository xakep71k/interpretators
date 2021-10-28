use crate::token;
use crate::var_type::VarType;

#[derive(Debug, Clone)]
pub struct Param {
    pub id: String,
    pub ttype: VarType,
}

impl std::fmt::Display for Param {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "name='{}', type='{}'", self.id, self.ttype)
    }
}

#[derive(Debug, Clone)]
pub enum AST {
    Program {
        name: String,
        block: Box<AST>,
    },
    Block {
        declaration_nodes: Vec<AST>,
        compound_nodes: Box<AST>,
    },
    VarDecl {
        id: String,
        var_type: VarType,
        token: token::Token,
    },
    ProcedureDecl {
        id: String,
        params: Vec<Param>,
        block_node: Box<AST>,
    },
    ProcedureCall {
        id: String,
        params: Vec<AST>,
        token: token::Token,
    },
    NumInteger {
        value: i32,
    },
    NumReal {
        value: f32,
    },
    BinOp {
        left: Box<AST>,
        right: Box<AST>,
        op: token::Kind,
    },
    UnaryOp {
        op: token::Kind,
        expr: Box<AST>,
    },
    Compound {
        children: Vec<AST>,
    },
    Assign {
        left_id: String,
        left: Box<AST>,
        right: Box<AST>,
    },
    Var {
        id: String,
        token: token::Token,
    },
    NoOp,
}
