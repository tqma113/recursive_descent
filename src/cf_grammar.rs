use super::symbol::Symbol;

pub use std::collections::{HashMap, HashSet};

#[macro_export]
macro_rules! cf_grammar {
    (
        Start($start:literal);
        NonTerminals[$($non_terminal:literal),+ $(,)?];
        Terminals[$($terminal:literal),+ $(,)?];
        Rules[$($left:literal => [$([$($right:literal),+ $(,)?]),+ $(,)?]),+ $(,)?]
    ) => {
        {
            let mut non_terminals: $crate::cf_grammar::HashSet<$crate::symbol::Symbol>
                = $crate::cf_grammar::HashSet::new();
            $(
                non_terminals.insert($crate::symbol::Symbol::intern($non_terminal));
            )*

            let start_terminal = $crate::symbol::Symbol::intern($start);
            assert!(
                non_terminals.contains(&start_terminal),
                format!("Start:{} is not exist in non-terminals set", start_terminal)
            );

            let mut terminals: $crate::cf_grammar::HashSet<$crate::symbol::Symbol>
                = $crate::cf_grammar::HashSet::new();
            $(
                let symbol = $crate::symbol::Symbol::intern($terminal);
                assert!(
                    !non_terminals.contains(&symbol),
                    format!("Non-terminal:{} has already exist in terminal set", symbol)
                );

                terminals.insert(symbol);
            )*

            let mut table: $crate::cf_grammar::HashMap<$crate::symbol::Symbol, Vec<Vec<$crate::symbol::Symbol>>> = $crate::cf_grammar::HashMap::new();
            $(
                let left = $crate::symbol::Symbol::intern($left);
                assert!(
                    non_terminals.contains(&left),
                    format!("The rule's left part: {} is not exist in non-terminals", left)
                );

                let mut right: Vec<Vec<$crate::symbol::Symbol>> = vec![];
                $(
                    let mut line: Vec<$crate::symbol::Symbol> = vec![];
                    $(
                        line.push($crate::symbol::Symbol::intern($right));
                    )*
                    right.push(line);
                )*

                table.insert(left, right);
            )*

            $crate::cf_grammar::ContextFreeGrammar::new(
                start_terminal,
                terminals,
                non_terminals,
                table,
            )
        }
    };
}

pub enum Category {
    Terminal,
    NonTerminal,
    Unknown,
}

pub trait Grammar {
    fn start(&self) -> Symbol;

    fn category(&self, input: &Symbol) -> Category;

    fn next(&self, input: &Symbol, index: u8) -> Option<&Vec<Symbol>>;

    fn len(&self, input: &Symbol, index: u8) -> usize;
}

pub mod sym {
    use super::Symbol;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref END: Symbol = Symbol::intern("#");
    }
}

#[derive(Debug, Clone)]
pub struct ContextFreeGrammar {
    start: Symbol,
    terminals: HashSet<Symbol>,
    non_terminals: HashSet<Symbol>,
    table: HashMap<Symbol, Vec<Vec<Symbol>>>,
}

impl ContextFreeGrammar {
    pub fn new(
        start: Symbol,
        terminals: HashSet<Symbol>,
        non_terminals: HashSet<Symbol>,
        table: HashMap<Symbol, Vec<Vec<Symbol>>>,
    ) -> Self {
        Self {
            start,
            terminals,
            non_terminals,
            table,
        }
    }
}

impl Grammar for ContextFreeGrammar {
    fn start(&self) -> Symbol {
        self.start
    }

    fn category(&self, input: &Symbol) -> Category {
        if self.terminals.contains(input) {
            Category::Terminal
        } else if self.non_terminals.contains(input) {
            Category::NonTerminal
        } else {
            Category::Unknown
        }
    }

    fn next(&self, input: &Symbol, index: u8) -> Option<&Vec<Symbol>> {
        match self.table.get(input) {
            Some(list) => list.get(index as usize),
            None => None,
        }
    }

    fn len(&self, input: &Symbol, index: u8) -> usize {
        match self.table.get(input) {
            Some(list) => match list.get((index - 1) as usize) {
                Some(list) => list.len(),
                None => 0,
            },
            None => 0,
        }
    }
}
