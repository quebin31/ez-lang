use std::collections::HashMap;

type SymbolTable = HashMap<String, Ident>;

#[derive(Debug, Clone, Default)]
pub struct Env {
    stack: Vec<SymbolTable>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn push(&mut self) {
        self.stack.push(Default::default());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn put(&mut self, name: &str, id: &Expr) {
        if let Some(last) = self.stack.last_mut() {
            last.insert(name.to_owned(), id.clone());
        }
    }

    pub fn get(&self, name: &str) -> Option<&Expr> {
        for sym_table in self.stack.iter().rev() {
            if let Some(id) = sym_table.get(name) {
                return Some(id);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manage_env() {
        let mut env = Env::new();
        env.push();
        env.push();
        assert_eq!(env.len(), 2);
        env.push();
        assert_eq!(env.len(), 3);
        env.pop();
        env.pop();
        assert_eq!(env.len(), 1);
        env.pop();
        assert!(env.is_empty());
    }

    #[test]
    fn manage_with_ids() {
        use crate::lex::token::Token;
        use crate::sym::types::Type;

        let mut env = Env::new();
        env.push();
        env.push();
        assert_eq!(env.len(), 2);
        env.put(
            "a",
            &Expr::Identifier {
                id: Token::Identifier("a".to_owned()),
                tp: Type::Int32,
                offset: 0,
            },
        );
        env.put(
            "b",
            &Expr::Identifier {
                id: Token::Identifier("b".to_owned()),
                tp: Type::Int64,
                offset: 1,
            },
        );
        env.push();
        assert_eq!(env.len(), 3);
        env.pop();
        env.pop();
        assert_eq!(env.len(), 1);
        env.pop();
        assert!(env.is_empty());
    }
}
