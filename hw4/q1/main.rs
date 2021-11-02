fn main() {
  let light = TrafficLight::Red;
  println!("light time is {} seconds", light.time());
 
}

enum TrafficLight {
  Red,
  Green,
  Yellow,
}

pub trait Time {
  fn time(&self) -> u8;
}

impl Time for TrafficLight {
  fn time(&self) -> u8 {
      match &self {
          TrafficLight::Red => 60,
          TrafficLight::Green => 120,
          TrafficLight::Yellow => 10,
      }
  }
}
