use std::collections::HashMap;
use std::f64;

#[derive(Clone, Copy, PartialEq, Debug)]
enum PointInfo {
  Value(f64),
  Unknown(usize),
  Undefined,
}


#[derive(Clone, PartialEq, Debug)]
struct PointMap {
  width: usize,
  axes_to_point: HashMap<(usize, usize), PointInfo>,
  indices_to_axis: HashMap<usize, (usize, usize)>,
}

impl PointMap {
  pub fn new() -> Self {
    PointMap {
      width: 0,
      axes_to_point: HashMap::new(),
      indices_to_axis: HashMap::new(),
    }
  }

  pub fn with_width(width: usize) -> Self {
    let mut this = Self::new();
    this.width = width;
    this.generate_map();
    this
  }

  fn generate_map(&mut self) {
    let mut i = 0usize;

    for x in 0..self.width {
      for y in (0..(self.width - x)).rev() {
        let x = x + 1;
        let y = y + 1;

        self.axes_to_point.insert((x, y), PointInfo::Unknown(i));
        self.indices_to_axis.insert(i, (x, y));

        i += 1;
      }
    }
  }

  pub fn get_point(&self, x: usize, y: usize) -> PointInfo {
    if (self.width + 3) < x || (self.width + 3) < y {
      panic!("Out of range at get_point()");
    }

    if let Some(p) = self.axes_to_point.get(&(x, y)) {
      *p
    } else if let Some(v) = self.calc_point(x, y) {
      PointInfo::Value(v)
    } else {
      PointInfo::Undefined
    }
  }

  pub fn get_point_at(&self, idx: usize) -> Option<(usize, usize)> {
    self.indices_to_axis.get(&idx).cloned()
  }

  fn calc_point(&self, x: usize, y: usize) -> Option<f64> {
    let width = 2.0 + (self.width as f64);
    if x == 0 {
      let y = y as f64;
      Some(10.0 - 10.0 * y / width)
    } else if y == 0 {
      let x = x as f64;
      Some(10.0 + 20.0 * x / width)
    } else if y == (self.width + 2 - x) {
      let x = x as f64;
      Some(30.0 * (x * f64::consts::PI / (2.0 * width)).sin())
    } else {
      None
    }
  }
}

pub fn generate_mat(width: usize) -> (Vec<Vec<f64>>, Vec<f64>) {
  // よこ（または縦）の長さから，必要な要素数を求める．
  let items = width * (width + 1) / 2;
  let pmap = PointMap::with_width(width);

  let mut a = Vec::<Vec<f64>>::with_capacity(items);
  let mut b = Vec::<f64>::with_capacity(items);

  a.resize_with(items, || {
    let mut a_i = Vec::<f64>::with_capacity(items);
    a_i.resize(items, 0f64);
    a_i
  });
  b.resize(items, 0f64);

  // すべての要素に対して連立方程式をつくる
  for i in 0..items {
    let (x, y) = pmap.get_point_at(i).unwrap();
    a[i][i] = 4.0;

    for j in 0..2 {
      for k in 0..2 {
        let xd = if j == 0 { x + (k * 2) - 1 } else { x };
        let yd = if j == 1 { y + (k * 2) - 1 } else { y };

        match pmap.get_point(xd, yd) {
          PointInfo::Value(v) => {
            b[i] += v;
          }
          PointInfo::Unknown(idx) => {
            a[i][idx] = -1.0;
          }
          PointInfo::Undefined => {
            panic!("Out of range: ({}, {})", xd, yd);
          }
        }
      }
    }
  }

  (a, b)
}
