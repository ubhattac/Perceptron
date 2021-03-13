mod perceptron;

fn main() {
    let _file = perceptron::safe_read();
    perceptron::train_model();
    perceptron::execute_test_suite();
}
