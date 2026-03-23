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
