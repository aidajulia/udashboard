use std::collections::HashMap;
use std::rc::Rc;


// Abstract over various memory management strategies.
pub type Node<T> = Rc<T>;
pub type Seq<T> = Vec<Node<T>>;
pub type AList<T> = Vec<(String, Node<T>)>;
pub type Map<T> = HashMap<String, Node<T>>;


// Arithmetic and logic operations
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    And,
    Or,
    Xor,
    Lt,
    Gt,
    Lte,
    Gte,
    Eq,
    Shl,
    Shr,
    Min,
    Max
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UnOp {
    Not,
    Neg,
    Abs,
}


// ADT for types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeTag {
    Unit,
    Bool,
    Int,
    Float,
    Str,
    Point,
    List(Node<TypeTag>),
    Map(Map<TypeTag>),
    Lambda(Seq<TypeTag>, Node<TypeTag>),
    Union(Seq<TypeTag>),
}


// ADT for values
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Point(f64, f64),
    List(Seq<Expr>),
    Map(Map<Expr>),
    Id(String),
    Dot(Node<Expr>, String),
    Index(Node<Expr>, Node<Expr>),
    Cond(Seq<(Expr, Expr)>),
    Block(Seq<Statement>, Node<Expr>),
    BinOp(BinOp, Node<Expr>, Node<Expr>),
    UnOp(UnOp, Node<Expr>),
    Call(Node<Expr>, Seq<Expr>),
    Lambda(AList<TypeTag>, Node<Expr>)
}


// ADT for effects and structure
#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Emit(String, Seq<Expr>),
    Def(String, Node<Expr>),
    For(Node<Expr>, Node<Expr>),
    While(Node<Expr>, Node<Expr>),
}


// ADT for programs
#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub description: String,
    pub params: HashMap<String, (TypeTag, String)>,
    pub code: Seq<Statement>
}
