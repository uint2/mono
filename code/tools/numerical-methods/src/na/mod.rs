// Numerical Analysis functions

pub mod qr_decomp;

use crate::prelude::*;

/// Clones the matrix, and writes the Cholesky factor into the
/// lower-triangular half of the matrix.
///
/// Input matrix MUST be symmetric positive definite.
pub fn cholesky<const N: usize>(A: &mut Mat<N, N>) {
    for k in 1..N {
        A[(k, k)] = A[(k, k)].sqrt();
        for j in k + 1..=N {
            A[(j, k)] = A[(j, k)] / A[(k, k)];
        }
        for j in k + 1..=N {
            for i in j..=N {
                A[(i, j)] -= A[(i, k)] * A[(j, k)];
            }
        }
    }
    A[(N, N)] = A[(N, N)].sqrt();
}

#[test]
fn cholesky_test() {
    let mut i = 0;
    while i < REPS {
        let A = Mat::<5, 5>::symmetric_positive_definite();
        let mut L = A.clone();
        cholesky(&mut L);
        L.to_lower_triangular();
        if L.contains_nan() {
            continue;
        }
        let LT = L.transpose();
        assert_eq_mat!(A, L * LT);
        i += 1;
    }
}

/// Evaluate a polynomial in linear time, using Horner's Method.
/// `p` is read highest-degree first.
pub fn horners(p: &Vec<R>, x: R) -> R {
    if p.is_empty() {
        return 0.;
    }
    let mut v = p[0];
    for k in 1..p.len() {
        v = p[k] + (x * v)
    }
    v
}

#[test]
fn horners_test() {
    fn polyval(p: &Vec<R>, x: R) -> R {
        p.iter()
            .rev()
            .enumerate()
            .fold(0., |v, (i, p)| v + x.powi(i as i32) * p)
    }
    macro_rules! test {
        ($p:expr, $x:expr) => {
            let p: Vec<R> = $p.to_vec();
            let received = horners(&p, $x);
            let expected = polyval(&p, $x);
            assert!(received.abs_diff(expected) < 1e-10);
        };
    }
    test!([1., 2., 3., 4.], 10.);
    test!([0.2, 2.2, 1.9, 4.1], 2.1);
    test!([0.2, 2.2], 0.1);
    test!([1.2], 91.1);
}

pub fn backward_sub<const N: usize>(A: &Mat<N, N>, b: &Mat<N, 1>) -> Mat<N, 1> {
    assert!(A.is_upper_triangular(), "A needs to be upper-triangular:\n{A:?}");
    let mut x = b.clone();
    for k in (1..=N).rev() {
        let mut s = b[k];
        for j in k + 1..=N {
            s -= A[(k, j)] * x[j];
        }
        x[k] = s / A[(k, k)];
    }
    x
}

#[test]
fn backward_sub_test() {
    const N: usize = 6;
    for _ in 0..HIGH_REPS {
        let A = Mat::<N, N>::rand().upper_triangular();
        let b = Mat::<N, 1>::rand();
        let x = backward_sub(&A, &b);
        assert_eq_mat!(A * x, b, 1e-3);
    }
}

/// Determine the dominant eigenvector of a matrix, and its
/// corresponding eigenvalue.
pub fn power_iteration<const N: usize>(A: &Mat<N, N>) -> (R, Mat<N, 1>) {
    let mut v = A.col(1).clone();
    loop {
        let mut v2 = A * &v;
        v2.l2_normalize();
        if (&v2 - &v).l2_norm() < 1e-15 {
            v = v2;
            break (v.dot(A * &v), v);
        }
        v = v2;
    }
}

#[test]
fn power_iteration_test() {
    const N: usize = 6;
    for _ in 0..REPS {
        let A = Mat::<N, N>::rand();
        let (lambda, v) = power_iteration(&A);
        assert_eq_mat!(&A * &v, lambda * v);
    }
}

/// Rayleigh Quotient.
/// Useful for calculating the eigenvalue of `v` when it is known that
/// it is an eigenvector of `A`.
pub fn rayleigh_quotient<const N: usize>(v: &Mat<N, 1>, A: &Mat<N, N>) -> R {
    v.dot(A * v) / v.dot(v) // = vᵀAv/vᵀv
}

/// Inverse iteration.
///
/// `a` is the shift.
pub fn inverse_iteration<const N: usize>(
    A: &Mat<N, N>,
    a: R,
) -> (R, Mat<N, 1>) {
    let mut v = A.col(1).clone();
    let mut B = A.clone();
    B.add_identity(-a);

    loop {
        v = B.solve_lls(&v);
        v.l2_normalize();

        if v.is_eigenvector_of(&B, 1e-8) {
            break (v.dot(A * &v), v);
        }
    }
}

#[test]
fn inverse_iteration_test() {
    const N: usize = 6;
    for _ in 0..SMALL_REPS {
        let A = Mat::<N, N>::rand();
        let (lambda, v) = inverse_iteration(&A, A.l1_norm());
        assert_eq_mat!(A * &v, lambda * &v);
    }
}

#[test]
fn inverse_iteration_against_power_iteration() {
    const N: usize = 6;
    for _ in 0..SMALL_REPS {
        let A = Mat::<N, N>::rand();
        let (_, v) = inverse_iteration(&A, A.l1_norm());
        let (_, pv) = power_iteration(&A);
        assert!(pv.eq(&v, 1e-5) || pv.eq(-v, 1e-5));
    }
}

/// Rayleigh Quotient Iteration.
///
/// Currently runs forever on matrices that have no eigenvalues.
pub fn rayleigh_quotient_iteration<const N: usize>(
    A: &Mat<N, N>,
) -> Result<(R, Mat<N, 1>)> {
    let B = |mu: R| {
        let mut B = A.clone();
        B.add_identity(-mu);
        B
    };

    let mut x = Mat::<N, 1>::rand();
    let mut mu = x.dot(A * &x);
    let mut y = B(mu).solve_lls(&x);
    mu += y.dot(&x).recip();

    for _ in 0..100 {
        x.l2_normalize();
        y = B(mu).solve_lls(&x);
        let lambda = y.dot(&x);
        mu += lambda.recip();
        if (&y - lambda * &x).l2_norm() / y.l2_norm() < 1e-9 {
            return Ok((mu, x));
        }
        x = y;
    }
    Err(Error::NoEigenvalues)
}

#[test]
fn rayleigh_quotient_iteration_test() {
    const N: usize = 6;
    let mut k = 0;
    let zero = Mat::zero();
    while k < REPS {
        let A = Mat::<N, N>::rand();
        if let Ok((lambda, v)) = rayleigh_quotient_iteration(&A) {
            let Av = &A * &v;
            let lv = lambda * &v;
            assert_ne_mat!(lv, zero);
            assert_eq_mat!(Av, lv, 5e-4);
            k += 1;
        }
    }
}
