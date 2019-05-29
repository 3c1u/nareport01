pub mod mat_gen;
pub mod solver;

use mat_gen::generate_mat;
use solver::solve;

fn main() {
    // 入力する行列式
    /*let a = [[4.0, -1.0, 0.0], [-1.0, 4.0, -1.0], [0.0, -1.0, 4.0]];
    let b = [
        30.0 * f64::sin(f64::consts::PI / 4.0) + 30.0 * f64::sin(f64::consts::PI / 8.0) + 5.0,
        22.5,
        30.0 * f64::sin(3.0 * f64::consts::PI / 8.0)
            + 30.0 * f64::sin(f64::consts::PI / 4.0)
            + 20.0,
    ];*/

    let width = 70usize;
    let items = width * (width + 1) / 2;

    let (a, b) = generate_mat(width);

    // 解を格納する配列
    let mut ans = Vec::with_capacity(items);
    ans.resize(items, 0f64);

    // 行列式ソルバーに投げる
    solve(&a, &b, &mut ans).expect("Can't solve");
    println!("{:#?}", ans);
}
