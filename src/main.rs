mod mat_gen;

mod nalgebra_solver;
mod solver;
use mat_gen::generate_mat;

use nalgebra_solver::solve_na;
// use solver::solve;

fn main() {
    // A→B方向に並ぶ未知数の数をwidthで与える．
    // width = 3だと，未知数の数はitems = 6（問題1），
    // width = 6だと，未知数の数はitems = 21（問題2）となる．
    let width = 6usize;
    let items = width * (width + 1) / 2;

    let (a, b) = generate_mat(width);

    // 解を格納する配列
    let mut ans = Vec::with_capacity(items);
    ans.resize(items, 0f64);

    // 行列式ソルバーに投げる
    /* nalgebra (LU, width = 70):
     * 7.61user 0.05system 0:07.75elapsed 98%CPU (0avgtext+0avgdata 99172maxresident)k
     * 0inputs+0outputs (0major+26907minor)pagefaults 0swaps
     */
    /* solve (Gaussの消去法, width = 70):
     * 8.47user 0.04system 0:08.61elapsed 98%CPU (0avgtext+0avgdata 99384maxresident)k
     * 0inputs+0outputs (0major+26908minor)pagefaults 0swaps
     */
    solve_na(&a, &b, &mut ans).expect("Can't solve");
    println!("{:#?}", ans);
}
