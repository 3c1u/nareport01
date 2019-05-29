use std::f64;

macro_rules! assert_eq_opt {
  ($a: expr, $b: expr) => {
    if $a != $b {
      None?
    }
  };
}

macro_rules! assert_opt {
  ($a: expr) => {
    if !$a {
      None?
    }
  };
}

pub fn solve<T: AsRef<[f64]>>(a: &[T], b: &[f64], ans: &mut [f64]) -> Option<()> {
  solve_gauss(a, b, ans).or_else(|| {
    solve_cramer(a, b, ans)
  })
}

fn replace_column<T: AsMut<[f64]>>(a: &mut [T], b: &[f64], a_n: usize) -> Option<()> {
  // 行列のサイズが0なら中止．
  if a.is_empty() {
    return None;
  }

  assert_opt!(a_n < a.len());

  for i in 0..b.len() {
    let an: &mut [f64] = a[i].as_mut();
    an[a_n] = b[i];
  }

  Some(())
}

fn det<T: AsRef<[f64]>>(a: &[T]) -> Option<f64> {
  match a.len() {
    0 => Some(0f64),
    1 => Some(a[0].as_ref()[0]),
    2 => {
      let a = a.iter().map(|v| v.as_ref().to_vec()).collect::<Vec<_>>();
      // たすき掛け法
      Some(a[0][0] * a[1][1] - a[0][1] * a[1][0])
    }
    3 => {
      let a = a.iter().map(|v| v.as_ref().to_vec()).collect::<Vec<_>>();
      // Sarrusの方法で素早く求まる
      Some(
        a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
          - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
          + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]),
      )
    }
    _ => det_n(a), // 高々O(N^3)
  }
}

fn det_n<T: AsRef<[f64]>>(a: &[T]) -> Option<f64> {
  // 行列のサイズが0なら失敗
  if a.is_empty() {
    return None;
  }

  // 正方行列であることを確認．
  assert_eq_opt!(a.len(), a[0].as_ref().len());

  let mut a = a.iter().map(|v| v.as_ref().to_vec()).collect::<Vec<_>>();

  // 対角成分が!=0になるように操作
  for i in 0..a.len() {
    let a_i = &a[i];

    if a_i[i] != 0f64 {
      continue;
    }

    // 対角成分0となる行a[i]に対し，a[i][j] != 0となる行を足す
    let mut jv: Option<usize> = None;
    for (j, a_j) in a.iter().enumerate() {
      if i == j {
        continue;
      }
      if a_j[i] != 0f64 {
        jv = Some(j);
        break;
      }
    }

    if let Some(jv) = jv {
      let a_j = a[jv].clone();
      let a_i = &mut a[i];

      for k in 0..a_j.len() {
        a_i[k] += a_j[k];
      }
    } else {
      return Some(0f64); // すべての列が0となってしまうため，行列式は0となってしまう．
    }
  }
  // 三角行列に変換
  for i in 0..a.len() {
    for j in 0..a.len() {
      if i == j {
        continue;
      }

      let c = a[j][i] / a[i][i];

      for k in 0..a.len() {
        a[j][k] -= c * a[i][k];
      }
    }
  }
  // 三角行列の行列式は，対角成分の積に等しい．
  Some(a.iter().enumerate().fold(1f64, |prev, (i, v)| prev * v[i]))
}

fn solve_cramer<T: AsRef<[f64]>>(a: &[T], b: &[f64], ans: &mut [f64]) -> Option<()> {
  // クラメールの式による解法もあるが，あまり速くなはい．
  
  // 正方行列であることを確認．
  assert_eq_opt!(a.len(), b.len());
  assert_eq_opt!(a.len(), a[0].as_ref().len());

  let det_a = det(a)?;

  if det_a == 0f64 {
    // |a| = 0のとき，正則行列ではないため失敗
    return None;
  }

  // 扱いやすいため，いちどVec<Vec<f64>>に変換する．
  let a = a.iter().map(|v| v.as_ref().to_vec()).collect::<Vec<_>>();

  // クラーメルの式を用いて解く．
  for (i, ans_i) in ans.iter_mut().enumerate() {
    // aを複製しておく．
    let mut a_i = a.clone();

    // 列を入れ替える
    replace_column(&mut a_i, b, i)?;

    // 解は u_i = det(a_i)/det(a)
    *ans_i = det(&a_i)? / det_a;
  }

  Some(())
}

fn solve_gauss<T: AsRef<[f64]>>(a: &[T], b: &[f64], ans: &mut [f64]) -> Option<()> {
  // 解法の一つとして，Gaussの消去法を採用した．
  // 一意解が求まらなくても単純化するようになってはいるが，
  // その場合でもNoneを返す．また，非正方行列は想定していない．（インデックスエラーを吐く）

  // 行の入れ替えを行うため，mutableなVec<Vec<f64>>に変換する．
  let mut a = a.iter().map(|v| v.as_ref().to_vec()).collect::<Vec<_>>();
  let mut b = b.to_vec();

  // 前進消去
  for i in 0..(a.len() - 1) {
    // ピボット選択
    let (p, pmax) = a
      .iter()
      .enumerate()
      .fold((i, a[i][i].abs()), |(p, pmax), (j, a_j)| {
        if pmax < a_j[i].abs() {
          (j, a_j[i].abs())
        } else {
          (p, pmax)
        }
      });
    // p != iなら，行を交換する．
    if p != i {
      a.swap(p, i);
      b.swap(p, i);
    }

    // i行目の値を使ってi + 1, ..., n行目の行列からu_iを消去
    for j in (i + 1)..a.len() {
      let r = a[j][i] / pmax;
      for k in i..a[j].len() {
        a[j][k] -= a[i][k] * r;
      }
      b[j] -= b[i] * r;
    }
  }
  // 後退代入
  let mut solved = true;

  for i in (0..a.len()).rev() {
    for j in (i + 1)..a.len() {
      if a[j][j] != 0f64 {
        b[i] -= b[j] * a[i][j];
        a[i][j] = 0f64;
      } else {
        solved = false;
      }
    }
    if a[i][i] != 0f64 {
      b[i] /= a[i][i];
      a[i][i] = 1.0;
    } else {
      solved = false;
    }
  }

  if solved {
    // bに解が入っているので，ansにコピーする．
    let len = ans.len();
    ans[..len].clone_from_slice(&b);
    Some(())
  } else {
    None
  }
}
