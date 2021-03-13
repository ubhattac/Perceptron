mod perceptron;
mod read_csv;

fn main() {
    // modify 'read()' to construct the data into 3-tuples: (binary class, x, y)
    let file = read_csv::read();
    
    println!("{:?}", file);
    perceptron::train_model();
    perceptron::execute_test_suite();
}
