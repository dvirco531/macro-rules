macro_rules! calculate {
    (add, $var:expr, $var2:expr) => {
        $var + $var2
    };
    (subtract, $var:expr, $var2:expr) => {
        $var - $var2
    };
    (multiply, $var:expr, $var2:expr) => {
        $var * $var2
    };

    (divide, $var:expr, $var2:expr) => {
        if $var2 == 0 {
            panic!("Can't divide by 0");
        } else {
            $var / $var2
        }
    };

    (divide, $var:expr, 0) => {
        complie_error("Invalid divide operation");
    };

    ($unknown:expr) => {
        complie_error("Received unknown operator");
    };
}
