use std::cmp::Ordering;

pub struct Levenshtein {
  one: String,
  two: String,
  matrix: Vec<Vec<u8>>,
  distance: Option<u8>,
}

impl Levenshtein {
  pub fn new(mut one: String, mut two: String) -> Self {
    match one.chars().count().cmp(&two.chars().count()) {
      Ordering::Equal => {}
      Ordering::Greater => {
        let diff = one.len() - two.len();
        for _ in 0..diff {
          two.push(' ');
        }
      }
      _ => {
        let diff = two.len() - one.len();
        for _ in 0..diff {
          one.push(' ');
        }
      }
    };
    let mut matrix = Vec::new();
    for i in 0..two.len() + 1 {
      let mut arr: Vec<u8> = vec![0; one.len() + 1];
      arr.fill(0);
      if i == 0 {
        for j in 0..one.len() + 1 {
          arr[j] = j as u8;
        }
      } else {
        arr[0] = i as u8;
      }
      matrix.push(arr);
    }
    Self {
      one,
      two,
      matrix,
      distance: None,
    }
  }
  fn calculate(&mut self) {
    for i in 1..self.two.len() + 1 {
      for j in 1..self.one.len() + 1 {
        let min = vec![
          self.matrix[i - 1][j - 1],
          self.matrix[i - 1][j],
          self.matrix[i][j - 1],
        ]
        .iter()
        .min()
        .unwrap()
        .clone();
        if self.one.chars().nth(j - 1).unwrap() != self.two.chars().nth(i - 1).unwrap() {
          self.matrix[i][j] = min + 1 as u8;
        } else {
          self.matrix[i][j] = min as u8;
        }
      }
    }
    self.distance = Some(self.matrix[self.two.len()][self.one.len()]);
  }

  fn print_matrix(&self) {
    print!("      ");
    for i in 0..self.one.len() {
      print!("{0:<3}", self.one.chars().nth(i).unwrap());
    }
    println!("");

    for idx in 0..self.matrix.len() {
      if idx > 0 {
        print!("{0:<3}", self.two.chars().nth(idx - 1 as usize).unwrap());
      } else {
        print!("   ");
      }
      for j in 0..self.one.len() + 1 {
        print!("{0:<3}", self.matrix[idx][j]);
      }
      println!("");
    }
  }

  pub fn distance(&mut self) -> u8 {
    match self.distance {
      Some(distance) => distance,
      None => {
        self.calculate();
        self.distance()
      }
    }
  }
}
