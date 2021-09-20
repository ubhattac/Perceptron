mod perceptron;
fn main() {
    perceptron::train_model();
    let _file = perceptron::safe_read();
    perceptron::execute_test_suite();
}
