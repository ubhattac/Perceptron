use std::error::Error;
use std::io;
use std::process;
extern crate csv;
extern crate serde_derive;
// #[macro_use]

#[derive(serde_derive::Deserialize)]
struct StringInput {
  bc_str: String,
  x_str: String,
  y_str: String
}

struct Input {
  binary_class: i32,
  x: i32,
  y: i32
}

fn read() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut points: Vec<[i32; 3]>;
    for result in rdr.records() {
      let record = result?;
      // iterator yields Result<StringRecord, Error>, so we check the error here.
      assert_eq!(3, record.len()); // minimal input validation
      let str_input: StringInput = record.deserialize(None)?;

      let point_strs: [String; 3] = [str_input.bc_str, str_input.x_str, str_input.y_str];
      let mut point: [i32; 3] = [0,0,0];
      for i in 0..point_strs.len() {
        point[i] = point_strs[i].parse::<i32>().unwrap();
      }
      println!("{:?}",point); //debug trait
    }
    Ok(())
}

pub fn safe_read() {
    if let Err(err) = read() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

pub struct Perceptron {
  w1: i32,
  w2: i32,
  b: i32,
  points: Vec<[i32;3]>
}

impl Perceptron {

  /**
   * Construct a Perceptron model.
   * @param w1: x weight
   * @param w2: y weight
   * @param b: y intercept
   */
  fn new(w1: i32, w2: i32, b: i32, points: Vec<[i32; 3]>) -> Perceptron {
    Perceptron {
      w1: w1,
      w2: w2,
      b: b,
      points: points
    }
  }

  /**
   * Activation function for binary perceptron.
   * @param net: the "net value" a.k.a. the model's output.
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
   * Bypass mutable borrowing.
   * Allow [i32; 3] to be pushed onto 'points' vector
   * @param point: the point to be pushed onto points.
   */
  fn push_point(&mut self, point: [i32; 3]) {
    self.points.push(point);
  }
}

/**
 * Train the perceptron model on the data
 * according to perceptron learning algorithm.
 */
pub fn train_model() {
  let mut p: Perceptron = Perceptron::new(1,1,1,vec![]);
  p.push_point([1,2,3]);
  println!("x weight: {}, y weight: {}, y intercept: {}, points: {:?}", p.w1, p.w2, p.b, p.points);
  // /**
  //  * 1. read data from csv line by line
  //  *    (1, 2,1)
  //  *    (1, 5,3)
  //  *   (-1, 6,3)
  //  *   (-1, 7,2)
  //  *    (1, 4,2)
  //  *    (1, 4,5)
  //  *    ...
  //  *
  //  * 2. transform data: points = [ (1,2,1), (1,5,3), (-1,6,3), ... ]
  //  *
  //  * 3. for every point, adjust perceptron to create linear model if possible.
  //  *
  //  * 4.
  //  */
}

/**
 * Basic, Quick Test Suite.
 */
pub fn execute_test_suite() {
  // SUITE: Classify Points
  assert_eq!(1, Perceptron::classify_point([1,4], [-2,1,2]));
  assert_eq!(1, Perceptron::classify_point([3,4], [-2,1,2]));
  assert_eq!(1, Perceptron::classify_point([0,-2], [-2,1,2]));
  assert_eq!(1, Perceptron::classify_point([-1,23], [-2,1,2]));
  assert_eq!(-1,Perceptron::classify_point([5,0], [-2,1,2]));

  // SUITE: Actual Model
  // to be implemented...
}

