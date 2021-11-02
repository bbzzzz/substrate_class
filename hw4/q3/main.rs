fn main() {
  let square = Square { side: 3 };
  calculate_area(&square);
}

pub trait Figure {
  fn area(&self) -> u32;
}

struct Square {
  side: u32
}

impl Figure for Square {
  fn area(&self) -> u32 {
      self.side * self.side
  }
}

pub fn calculate_area<T: Figure>(figure: &T) {
  println!("Area is {}", figure.area());
}
