mod perceptron;

fn main() {
    // modify 'read()' to construct the data into 3-tuples: (binary class, x, y)
    let _file = perceptron::safe_read();
    perceptron::train_model();
    perceptron::execute_test_suite();
}
