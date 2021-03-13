struct Perceptron {
  w1: i32,
  w2: i32,
  b: i32
}

impl Perceptron {

  /**
   * Construct a Perceptron model.
   * @param w1: x weight
   * @param w2: y weight
   * @param b: y intercept
   */
  fn new(w1: i32, w2: i32, b: i32) -> Perceptron {
    Perceptron {
      w1: w1,
      w2: w2,
      b: b
    }
  }

  /**
   * Activation function for binary perceptron.
   */
  fn activate(net: i32) -> i32 {
    return if net >= 0 {1} else {-1};
  }

  /**
   * Classify the perceptron's output as adhering to, 
   * or straying from the actual category (i.e. 1 or -1).
   * @param coord: array of x and y coordinate
   * @param coeff: array of x weight, y weight, and y intercept.
   * @return 1 if output is as expected, -1 otherwise.
   */
  fn classify_point(coord: [i32; 2], coeff: [i32; 3]) -> i32 {
    // output = w_1 * x + w_2 * y + y_intercept
    let net = (coord[0] * coeff[0]) + (coord[1] * coeff[1]) + coeff[2]; 
    return Perceptron::activate(net);
  }

  /**
   * @param
   */
  fn update_model() {

  }
}

/**
 * Basic, Quick Test Suite.
 */
pub fn execute_test_suite() {

  assert_eq!(1, Perceptron::classify_point([1,4], [-2,1,2]));
  
  assert_eq!(1, Perceptron::classify_point([3,4], [-2,1,2]));
  
  assert_eq!(1, Perceptron::classify_point([0,-2], [-2,1,2]));
  
  assert_eq!(1, Perceptron::classify_point([-1,23], [-2,1,2]));
  
  assert_eq!(-1,Perceptron::classify_point([5,0], [-2,1,2]));

}
