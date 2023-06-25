use super::cf_grammar::{sym, Category, Grammar};
use super::error::*;
use super::symbol::Symbol;

use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Parser<'a, G: Grammar + Debug + Clone> {
    grammar: &'a G,
    src: &'a str,

    symbols: Vec<Symbol>,
    index: usize,
    stack: Vec<(Symbol, u8)>,

    pub analysis_stack: Vec<(Symbol, u8)>,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'a, G: Grammar + Debug + Clone> Parser<'a, G> {
    pub fn new(grammar: &'a G) -> Self {
        Parser {
            grammar,
            src: "",
            symbols: vec![],
            index: 0,
            analysis_stack: vec![],
            stack: vec![],
            diagnostics: vec![],
        }
    }

    pub fn parse(&mut self, string: &'a str) -> bool {
        self.src = string;
        self.symbols = self
            .src
            .chars()
            .map(|c| Symbol::intern(c.to_string().as_str()))
            .collect();
        self.stack.push((*sym::END, 0));
        self.stack.push((self.grammar.start(), 0));

        self.walk()
    }

    fn walk(&mut self) -> bool {
        loop {
            match self.stack.pop() {
                Some((left, index)) => {
                    if left.eq(&sym::END) {
                        return if self.is_eos() {
                            true
                        } else {
                            self.diagnostics.push(Diagnostic::End(EndDiagnostic::new(
                                self.analysis_stack.clone(),
                                self.index,
                            )));
                            false
                        };
                    } else {
                        match self.grammar.category(&left) {
                            Category::Terminal => {
                                if left.eq(self.current()) {
                                    self.push();
                                    self.analysis_stack.push((left, index));
                                } else {
                                    self.stack.push((left, index));
                                    self.backtrack();
                                }
                            }
                            Category::NonTerminal => match self.grammar.next(&left, index) {
                                Some(symbols) => {
                                    self.refresh();

                                    self.analysis_stack.push((left, index + 1));

                                    for &symbol in symbols.clone().iter().rev() {
                                        self.stack.push((symbol, 0))
                                    }
                                }
                                None => match self.analysis_stack.pop() {
                                    Some((symbol, index)) => match self.grammar.category(&symbol) {
                                        Category::Terminal => {
                                            unreachable!("Backtracking should stop before the analysis stack became empty.")
                                        }
                                        Category::NonTerminal => {
                                            for _ in 0..(self.grammar.len(&symbol, index) - 1) {
                                                self.stack.pop();
                                            }
                                            self.stack.push((symbol, index));
                                        }
                                        Category::Unknown => {
                                            self.diagnostics.push(Diagnostic::Input(
                                                InputDiagnostic::new(left, self.index),
                                            ));
                                            return false;
                                        }
                                    },
                                    None => {
                                        unreachable!("Backtracking should stop before the analysis stack became empty.")
                                    }
                                },
                            },
                            Category::Unknown => {
                                self.diagnostics
                                    .push(Diagnostic::Input(InputDiagnostic::new(
                                        left, self.index,
                                    )));
                                return false;
                            }
                        }
                    }
                }
                None => {
                    unreachable!("Parsing should stop before the stack became empty.")
                }
            }
        }
    }

    fn backtrack(&mut self) {
        loop {
            match self.analysis_stack.pop() {
                Some((symbol, index)) => match self.grammar.category(&symbol) {
                    Category::Terminal => {
                        self.stack.push((symbol, index));
                        self.pop();
                    }
                    Category::NonTerminal => {
                        for _ in 0..self.grammar.len(&symbol, index) {
                            self.stack.pop();
                        }
                        self.stack.push((symbol, index));
                        match self.grammar.next(&symbol, index) {
                            Some(_) => break,
                            None => {}
                        }
                    }
                    Category::Unknown => {
                        unreachable!("Unknown symbol should report before.")
                    }
                },
                None => {
                    unreachable!("Backtracking should stop before the analysis stack became empty.")
                }
            }
        }
    }

    fn refresh(&mut self) {
        self.stack.iter_mut().for_each(|pair| {
            pair.1 = 0;
        });
    }

    fn is_eos(&self) -> bool {
        self.index >= self.symbols.len()
    }

    fn push(&mut self) {
        self.index += 1;
    }

    fn pop(&mut self) {
        self.index -= 1;
    }

    fn current(&self) -> &Symbol {
        self.symbols.get(self.index).unwrap()
    }
}
