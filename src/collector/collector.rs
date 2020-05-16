use crate::analyzer::analyzer::{Analyzer, DefaultAnalyzer};
use crate::Data;

use std::cell::RefCell;
use std::rc::Rc;
pub trait Collector {
    fn collect(&mut self, data: Box<dyn Data>);
    fn is_ready(&self) -> bool;
    fn collect_and_fire(&mut self, data: Box<dyn Data>);
    fn get_analyzer(&self) -> Rc<RefCell<dyn Analyzer>>;
}

pub struct DefaultCollector {
    analyzer: Rc<RefCell<dyn Analyzer>>,
    data: Vec<Box<dyn Data>>,
}

impl Default for DefaultCollector {
    fn default() -> Self {
        DefaultCollector {
            analyzer: Rc::new(RefCell::new(DefaultAnalyzer::default())),
            data: Vec::new(),
        }
    }
}

impl Collector for DefaultCollector {
    fn collect(&mut self, data: Box<dyn Data>) {
        if self.is_ready() {
            self.data.remove(0);
        }
        self.data.push(data);
    }

    fn is_ready(&self) -> bool {
        return self.data.len() == self.analyzer.borrow().min_capacity();
    }

    fn collect_and_fire(&mut self, data: Box<dyn Data>) {
        self.collect(data);
        if self.is_ready() {
            if self.analyzer.borrow().is_trained() {
                self.analyzer.borrow().analyze(&self.data);
            } else {
                let mut analyzer = self.analyzer.borrow_mut();
                analyzer.train(&self.data);
            }
        }
    }

    fn get_analyzer(&self) -> Rc<RefCell<dyn Analyzer>> {
        Rc::clone(&self.analyzer)
    }
}
