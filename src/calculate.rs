/// Perform arithmetic operation on two expression with operator.
/// Supported operator.
/// Add
/// Substract
/// Multiply
/// Divide
/// 
/// If unknown operation received nor devision by 0 an error will occur, compilation nor panic
macro_rules! calculate {
    (add, $var:expr, $var2:expr) => {{
        let var = $var;
        let var2 = $var2;
        var.checked_add(var2).expect("overflow in addition")
    }};
    (subtract, $var:expr, $var2:expr) => {{
        let var = $var;
        let var2 = $var2;
        var.checked_sub(var2).expect("overflow in subtraction")
    }};
    (multiply, $var:expr, $var2:expr) => {{
        let var = $var;
        let var2 = $var2;
        var.checked_mul(var2).expect("overflow in multiplication")
    }};
    (divide, $var:expr, $var2:expr) => {{
        let var = $var;
        let var2 = $var2;

        if var2 == 0 {
            panic!("Can't divide by 0");
        }

        var.checked_div(var2).expect("overflow in division")
    }};
    (divide, $var:expr, 0) => {
        complie_error("Invalid divide operation");
    };

    ($unknown:expr) => {
        complie_error("Received unknown operator");
    };
}
