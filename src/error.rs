use std::fmt;

use super::symbol::Symbol;

#[derive(Clone, Debug, PartialEq)]
pub enum Diagnostic {
    Input(InputDiagnostic),
    End(EndDiagnostic),
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(input) => fmt::Display::fmt(&input, f),
            Self::End(rule) => fmt::Display::fmt(&rule, f),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InputDiagnostic {
    symbol: Symbol,
    index: usize,
}

impl fmt::Display for InputDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown input:{} at {}", self.symbol, self.index)
    }
}

impl InputDiagnostic {
    pub fn new(symbol: Symbol, index: usize) -> Self {
        InputDiagnostic { symbol, index }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EndDiagnostic {
    stack: Vec<(Symbol, u8)>,
    index: usize,
}

impl fmt::Display for EndDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut comma_separated = String::new();

        for (symbol, index) in &self.stack[0..self.stack.len() - 1] {
            comma_separated.push_str(format!("({}, {}), ", &symbol.as_str(), index).as_str());
        }

        comma_separated.push_str(
            format!(
                "({}, {}), ",
                &self.stack[self.stack.len() - 1].0.as_str(),
                &self.stack[self.stack.len() - 1].1
            )
            .as_str(),
        );

        write!(
            f,
            "Unexpected ending. Index:{}, Stack: {}",
            self.index, comma_separated
        )
    }
}

impl EndDiagnostic {
    pub fn new(stack: Vec<(Symbol, u8)>, index: usize) -> Self {
        EndDiagnostic { stack, index }
    }
}
