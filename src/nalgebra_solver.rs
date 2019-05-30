// nalgebra（LAPACKのラッパー）を用いて解く方法

use nalgebra as na;

pub fn solve_na<T: AsRef<[f64]>>(a: &[T], b: &[f64], ans: &mut [f64]) -> Option<()> {
    let a = na::DMatrix::from_fn(a.len(), a.len(), |r, c| a[r].as_ref()[c]);
    let b = na::DVector::from_row_slice(b);

    let lu = na::LU::new(a);
    let ansv = lu.solve(&b)?;

    ans.clone_from_slice(ansv.as_slice());

    Some(())
}
