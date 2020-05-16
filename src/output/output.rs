use crate::analyzer::analyzer::{AnalyzerOutput};

pub trait Output {
    fn act(&mut self, data: AnalyzerOutput);
}

pub struct DefaultOutput {}

impl Default for DefaultOutput {
    fn default() -> Self {
        return DefaultOutput {};
    }
}

impl Output for DefaultOutput {
    fn act(&mut self, data: AnalyzerOutput) {
        println!("{:#?}", data);
    }
}
