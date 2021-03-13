/**
 * Coefficients should follow format: [x_weight, y_weight, y_intercept]
 */
fn classify_point(coord: [i32; 2], coeff: [i32; 3]) -> i32 {
  // output = w_1 * x + w_2 * y + y_intercept
  let model_out_val = coord[0]*coeff[0] + coord[1]*coeff[1] + coeff[2];
  return if model_out_val >= 0 {1} else {-1};
}

pub fn execute_test_suite() {

  assert_eq!(1, classify_point([1,4], [-2,1,2]));
  
  assert_eq!(1, classify_point([3,4], [-2,1,2]));
  
  assert_eq!(1, classify_point([0,-2], [-2,1,2]));
  
  assert_eq!(1, classify_point([-1,23], [-2,1,2]));
  
  assert_eq!(-1, classify_point([5,0], [-2,1,2]));

}
