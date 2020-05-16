use crate::collector::collector::{Collector, DefaultCollector};
use crate::Data;
use std::cell::RefCell;
use std::rc::Rc;
pub trait Input {
    fn push(&mut self, data: Box<dyn Data>);
    fn get_collector(&self) -> Rc<RefCell<dyn Collector>>;
}

pub struct DefaultInput {
    collector: Rc<RefCell<dyn Collector>>,
}

impl Default for DefaultInput {
    fn default() -> Self {
        return DefaultInput {
            collector: Rc::new(RefCell::new(DefaultCollector::default())),
        };
    }
}

impl Input for DefaultInput {
    fn push(&mut self, data: Box<dyn Data>) {
        self.collector.borrow_mut().collect_and_fire(data);
    }

    fn get_collector(&self) -> Rc<RefCell<dyn Collector>> {
        self.collector.clone()
    }
}
