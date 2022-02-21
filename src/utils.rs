use crate::check::CheckResult;
use std::convert::Into;

/// A function evaluating a given `value` with provided `warn` and `crit` thresholds to a
/// `CheckResult` with the corresponding `State`. Supports different value types per argument.
/// Works in 'both directions'.
///
/// # Arguments
///
/// * `value` - a value gathered by e.g. a check
/// * `warn` - a warning threshold
/// * `crit` - a critical threshold
///
/// # Examples
///
/// ```
/// use icingaplugin_rs::check::CheckResult;
/// use icingaplugin_rs::utils::evaluate;
/// let a = 0; let w = 3; let c = 4;
/// assert_eq!(evaluate(a, w, c), CheckResult::from(0));
/// assert_eq!(evaluate(5, 3.2, 7.00), CheckResult::from(1));
/// assert_eq!(evaluate(3.0, 2.99, 3.0), CheckResult::from(2));
/// assert_eq!(evaluate(2, 1, 0), CheckResult::from(0));
/// assert_eq!(evaluate(2, 3, 2.5), CheckResult::from(2));
/// ```
///
/// ```should_panic
/// use icingaplugin_rs::utils::evaluate;
/// let no_result = evaluate(1, 2, 2);
/// ```
pub fn evaluate<T, U, V>(value: T, warn: U, crit: V) -> CheckResult where 
T: Copy + Into<f64>,
U: Copy + Into<f64>,
V: Copy + Into<f64>
{
    let v_64: f64 = value.into();
    let w_64: f64 = warn.into();
    let c_64: f64 = crit.into();

    if w_64 == c_64 {
        panic!("warning and critical threshold must not be equal!");
    }


    if w_64 < c_64 {
        if v_64 >= c_64 {
            return CheckResult::from(2);
        } else if v_64 >= w_64 {
            return CheckResult::from(1);
        } else {
            return CheckResult::from(0);
        }
    }

    if v_64 <= c_64 {
        return CheckResult::from(2);
    } else if v_64 <= w_64 {
        return CheckResult::from(1);
    } else {
        return CheckResult::from(0);
    }
}
