pub mod mat_gen;
pub mod solver;

use mat_gen::generate_mat;
use solver::solve;

fn main() {
    // A→B方向に並ぶ未知数の数をwidthで与える．
    // width = 6だと，未知数の数はitems = 21となる．
    let width = 6usize;
    let items = width * (width + 1) / 2;

    let (a, b) = generate_mat(width);

    // 解を格納する配列
    let mut ans = Vec::with_capacity(items);
    ans.resize(items, 0f64);

    // 行列式ソルバーに投げる
    solve(&a, &b, &mut ans).expect("Can't solve");
    println!("{:#?}", ans);
}
