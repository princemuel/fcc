#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),*) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($x);
            )*
            temp
        }
    };
}
