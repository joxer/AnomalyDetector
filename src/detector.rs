use crate::analyzer::analyzer::{Analyzer};
use crate::collector::collector::{Collector};
use crate::input::input::{DefaultInput, Input};
use crate::output::output::{Output};
use rand::distributions::{Distribution, Normal};
use std::cell::RefCell;
use std::rc::Rc;
pub struct DummyDetector {
    input: Rc<RefCell<dyn Input>>,
    output: Rc<RefCell<dyn Output>>,
    analyzer: Rc<RefCell<dyn Analyzer>>,
    collector: Rc<RefCell<dyn Collector>>,
}
impl DummyDetector {
    pub fn new() -> DummyDetector {
        let input = Rc::new(RefCell::new(DefaultInput::default()));
        let collector = input.borrow().get_collector();
        let analyzer = collector.borrow().get_analyzer();
        let output = analyzer.borrow().get_output();

        DummyDetector {
            input: input,
            collector: collector,
            analyzer: analyzer,
            output: output,
        }
    }

    pub fn run(&mut self) {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(5.0, 1.0);
        loop {
            let d: f64 = normal.sample(&mut rng);
            self.input.borrow_mut().push(Box::new(d));
        }
    }
}
