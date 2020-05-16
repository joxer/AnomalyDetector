use crate::output::output::{DefaultOutput, Output};
use crate::Data;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
#[derive(Debug)]
pub struct AnalyzerOutput {
    output: Option<bool>,
}

pub trait Analyzer {
    fn train(&mut self, data: &Vec<Box<dyn Data>>);
    fn is_trained(&self) -> bool;
    fn analyze(&self, data: &Vec<Box<dyn Data>>);
    fn min_capacity(&self) -> usize;
    fn get_output(&self) -> Rc<RefCell<dyn Output>>;
}

pub struct DefaultAnalyzer {
    output: Rc<RefCell<dyn Output>>,
    gaussian_distribution: Option<(f64, f64)>,
}

impl Analyzer for DefaultAnalyzer {
    fn is_trained(&self) -> bool {
        return self.is_trained();
    }
    fn train(&mut self, data: &Vec<Box<dyn Data>>) {
        if !self.is_trained() {
            self.train_distribution(data);
        }
    }
    fn analyze(&self, data: &Vec<Box<dyn Data>>) {
        let mut result = None;
        if self.is_trained() {
            result = Some(self.match_distribution(data.last().unwrap().get_numeric_value()));
        }

        let output = AnalyzerOutput { output: result };
        self.output.borrow_mut().act(output);
    }

    fn min_capacity(&self) -> usize {
        5000
    }

    fn get_output(&self) -> Rc<RefCell<dyn Output>> {
        Rc::clone(&self.output)
    }
}

impl Default for DefaultAnalyzer {
    fn default() -> Self {
        return DefaultAnalyzer {
            output: Rc::new(RefCell::new(DefaultOutput::default())),
            gaussian_distribution: None,
        };
    }
}

impl DefaultAnalyzer {
    fn is_trained(&self) -> bool {
        if self.gaussian_distribution.is_none() {
            return false;
        }
        return true;
    }

    fn train_distribution(&mut self, data: &Vec<Box<dyn Data>>) {
        if data.len() == self.min_capacity() {
            let avg: f64 =
                data.iter().map(|i| i.get_numeric_value()).sum::<f64>() / (data.len() as f64);
            let sum: f64 = data
                .iter()
                .map(|i| {
                    let r = i.get_numeric_value() - avg;
                    r * r
                })
                .sum::<f64>()
                / ((data.len()) as f64);
            let sqrt = sum.sqrt();
            self.gaussian_distribution = Some((avg, sqrt));
        }
    }

    fn match_distribution(&self, data: f64) -> bool {
        let (avg, sqrt) = self.gaussian_distribution.unwrap();

        if data > (avg - 2.4 * sqrt) || data < (avg + 2.4 * sqrt) {
            return true;
        }
        return false;
    }
}
