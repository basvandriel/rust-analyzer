type A = for<'a> fn() -> ();
fn foo<T>(_t: &T) where for<'a> &'a T: Iterator {}
fn bar<T>(_t: &T) where for<'a> &'a mut T: Iterator {}
