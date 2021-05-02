
#[derive(Debug, PartialEq)]
enum State {
    Off,
    Operational,
    Error,
}

#[derive(Debug)]
enum Signal {
    Init,
    Op,
    RaiseError,
    Repair,
    ToOff,
}

fn step (state: State, signal: Signal) -> Option<State> {
    match state {
        State::Off => match signal {
            Signal::Init => Option::Some(State::Operational),
            _ => Option::None,
        },
        State::Operational => match signal {
            Signal::Op => Option::Some(State::Operational),
            Signal::RaiseError => Option::Some(State::Error),
            Signal::ToOff => Option::Some(State::Off),
            _ => Option::None,
        },
        State::Error => match signal {
            Signal::ToOff => Option::Some(State::Off),
            Signal::Repair => Option::Some(State::Operational),
            _ => Option::None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_init_operational() {
        assert_eq!(Option::Some(State::Operational), step(State::Off, Signal::Init));
    }

    #[test]
    fn operational_op_operational() {
        assert_eq!(Option::Some(State::Operational), step(State::Operational, Signal::Op));
    }

    #[test]
    fn operational_raise_err_error() {
        assert_eq!(Option::Some(State::Error), step(State::Operational, Signal::RaiseError));
    }

    #[test]
    fn operational_to_off_off() {
        assert_eq!(Option::Some(State::Off), step(State::Operational, Signal::ToOff));
    }

    #[test]
    fn error_repair_operational() {
        assert_eq!(Option::Some(State::Operational), step(State::Error, Signal::Repair));
    }

    #[test]
    fn error_to_off_off() {
        assert_eq!(Option::Some(State::Off), step(State::Error, Signal::ToOff));
    }

    #[test]
    fn off_op_invalid() {
        assert_eq!(Option::None, step(State::Off, Signal::Op));
    }

    #[test]
    fn off_repair_invalid() {
        assert_eq!(Option::None, step(State::Off, Signal::Repair));
    }

    #[test]
    fn off_raise_error_invalid() {
        assert_eq!(Option::None, step(State::Off, Signal::RaiseError))
    }

    #[test]
    fn off_to_off_invalid() {
        assert_eq!(Option::None, step(State::Off, Signal::ToOff));
    }
}
