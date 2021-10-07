macro_rules! lambda {
    ($($type:ty: $name:ident($($param_type:ty: $param:ident),*) => $body:block)*) => {$(fn $name($($param: $param_type),*) -> $type $body)*};
    ($($type:ty: $name:ident($($param_type:ty: $param:ident),*) => $body:expr)*) => {$(fn $name($($param: $param_type),*) -> $type {$body})*};
    ($($type:ty: $name:ident($($param_type:ty: $param:ident),*) => $body:stmt)*) => {$(fn $name($($param: $param_type),*) -> $type {$body})*};

    ($($name:ident($($param_type:ty: $param:ident),*) => $body:block)*) => {$(fn $name($($param: $param_type),*) $body)*};
    ($($name:ident($($param_type:ty: $param:ident),*) => $body:expr)*) => {$(fn $name($($param: $param_type),*) {$body})*};
    ($($name:ident($($param_type:ty: $param:ident),*) => $body:stmt)*) => {$(fn $name($($param: $param_type),*) {$body})*};
}

lambda! {
    i32: multiply(i32: n, i32: m) => n*m

    i32: square(i32: n) => multiply(n, n)
}

fn main() {
    println!("{}", square(2))
}
