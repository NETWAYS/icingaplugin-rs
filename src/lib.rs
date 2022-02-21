pub mod check;
pub mod utils;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn evaluate_ok() {
        let cr = utils::evaluate(0, 1, 2);
        assert_eq!(cr.state(), check::State::OK);
    }

    #[test]
    fn evalue_warn() {
        let cr = utils::evaluate(1, 1, 2);
        assert_eq!(cr.state(), check::State::Warning);
    }

    #[test]
    fn evaluate_crit() {
        let cr = utils::evaluate(2, 1, 2);
        assert_eq!(cr.state(), check::State::Critical);
    }

    #[test]
    fn evaluate_floats() {
        let cr = utils::evaluate(1.0, 5.0, 7.3);
        assert_eq!(cr.state(), check::State::OK);
        let cr2 = utils::evaluate(2.0, 2.0, 8.1);
        assert_eq!(cr2.state(), check::State::Warning);
    }

    #[test]
    fn evaluate_misc() {
        let cr = utils::evaluate(1, 1.0, 2.43);
        assert_eq!(cr.state(), check::State::Warning);
        let cr2 = utils::evaluate(1.11, 1.12, 1.13);
        assert_eq!(cr2.state(), check::State::OK);
    }
}
