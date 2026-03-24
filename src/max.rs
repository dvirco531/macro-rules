/// This macro used to return the maximum value from list of expressions.
/// Each expression evalutaed once.
/// # Examples
/// ```rust
/// let x = max!(1,4,2,3);
/// assert_eq!(x,4);
/// ```
#[macro_export]
macro_rules! max {
    ($first:expr $(,$rest:expr) *) => {
   {
     let mut max = $first;
     $(let val = $rest;
        if val > max{
            max = val
        }
     )   *
     max
    }
    };
}
