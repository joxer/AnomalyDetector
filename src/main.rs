use anomaly;

fn main() {
    let mut detector = anomaly::detector::DummyDetector::new();
    detector.run();
}
