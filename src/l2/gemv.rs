// gemv.rs 

use std::ops::{AddAssign, Mul};

use crate::{assert_length_eq_n_cols, assert_length_eq_n_rows}; 
use crate::types::{MatRef, VecMut, VecRef};
use crate::l1::axpy::axpy; 
use crate::l1::scal::scal;

const L1_BUDGET: usize = 30000; 

/// calculates optimal NC column block width based on a given 
/// bytes budget. (e.g. 30KB for L1) 
pub(crate) fn choose_nc<T>(dimension: (usize, usize), bytes_budget: usize) -> usize { 
    let elem_bytes = std::mem::size_of::<T>(); 
    let budget_elems = bytes_budget / elem_bytes; 

    let n_rows = dimension.0; 
    let n_cols = dimension.1;

    // one column already exceeds bytes_budget 
    // so default to one column 
    if budget_elems <= n_rows { 
        return 1;
    }

    let nc = (budget_elems - n_rows) / (n_rows + 1); 
    nc.clamp(1, n_cols)
}

/// no-transpose gemv on an individual column panel of a 
pub(crate) fn gemv_n_panel<T>( 
    alpha: T, 
    a: MatRef<'_, T>, 
    x: VecRef<'_, T>, 
    mut y: VecMut<'_, T>, 
) 
where 
    T: Mul<Output=T>
        + AddAssign
        + Copy 
{ 
    for (j, &xj) in x.as_slice().iter().enumerate() { 
        axpy(alpha * xj, a.col(j), y.reborrow()); 
    }
}


pub fn gemv_n2<T>( 
    alpha: T, 
    beta: T, 
    a: MatRef<'_, T>, 
    x: VecRef<'_, T>, 
    mut y: VecMut<'_, T>, 
) 
where  
    T: Copy 
        + Clone 
        + AddAssign 
        + Mul<Output=T> 
{ 
    assert_length_eq_n_cols!(a, x); 
    assert_length_eq_n_rows!(a, y);

    scal(beta, y.reborrow()); 

    let (n_rows, n_cols) = a.dimension(); 
    let nc = choose_nc::<T>((n_rows, n_cols), L1_BUDGET); 

    for (cols, a_panel) in a.col_panels(nc) {
        let x_panel = VecRef::new(x.slice(cols)); 
        gemv_n_panel(alpha, a_panel, x_panel, y.reborrow());
    }
}

pub fn gemv_n<T>( 
    alpha: T, 
    beta: T, 
    a: MatRef<'_, T>, 
    x: VecRef<'_, T>, 
    mut y: VecMut<'_, T>, 
) 
where 
    T: Clone 
        + Copy 
        + AddAssign
        + Mul<Output = T>,
{
    assert_length_eq_n_cols!(a, x); 
    assert_length_eq_n_rows!(a, y); 

    scal(beta, y.reborrow()); 

    for (j, &x_j) in x.as_slice().iter().enumerate() { 
        let a_j = a.col(j); 
        axpy(alpha * x_j, a_j, y.reborrow()); 
    }
}
