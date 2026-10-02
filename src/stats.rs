use crate::time_series::TimeSeries;

pub struct Stats {
    pub mean: f64,
    pub std: f64,
}

pub fn compute_stats(ts: &TimeSeries) -> Option<Stats> {
    let n = ts.values.len();
    if n < 2 {
        return None;
    }

    let mut sum = 0.0;
    for &v in &ts.values {
        sum += v;
    }
    let mean = sum / n as f64;

    let mut variance_sum = 0.0;
    for &v in &ts.values {
        let diff = v - mean;
        variance_sum += diff * diff;
    }
    let std = (variance_sum / (n - 1) as f64).sqrt();

    Some(Stats { mean, std })
}

/// Standard Normal Inverse Cumulative Distribution Function (probit)
/// Approximates z = Φ⁻¹(p) for 0 < p < 1 using Acklam's algorithm.
pub fn normal_inverse_cdf(p: f64) -> f64 {
    if p <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if p >= 1.0 {
        return f64::INFINITY;
    }

    // Coefficients for central region
    const A: [f64; 6] = [
        -3.969683028665376e+01,
         2.209460984245205e+02,
        -2.759285104469687e+02,
         1.383577518672690e+02,
        -3.066479806614716e+01,
         2.506628277459239e+00,
    ];
    const B: [f64; 5] = [
        -5.447609879822406e+01,
         1.615858368580409e+02,
        -1.556989798598866e+02,
         6.680131188771972e+01,
        -1.328068155288572e+01,
    ];

    // Coefficients for tail regions
    const C: [f64; 6] = [
        -7.784894002430293e-03,
        -3.223964580411365e-01,
        -2.400758277161838e+00,
        -2.549732539343734e+00,
         4.374664141464968e+00,
         2.938163982698783e+00,
    ];
    const D: [f64; 4] = [
         7.784695709041462e-03,
         3.224671290700398e-01,
         2.445134137142996e+00,
         3.754408661907416e+00,
    ];

    // Breakpoints
    const P_LOW: f64 = 0.02425;
    const P_HIGH: f64 = 1.0 - P_LOW;

    if p < P_LOW {
        // Lower tail region
        let q = (-2.0 * p.ln()).sqrt();
        ((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5]
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p <= P_HIGH {
        // Central region
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        // Upper tail region
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    }
}