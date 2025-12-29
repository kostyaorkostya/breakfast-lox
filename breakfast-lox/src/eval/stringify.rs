use std::fmt;

pub trait Stringify {
    fn stringify(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn display(&self) -> impl fmt::Display + '_ {
        struct Adapter<'a, T: Stringify + ?Sized>(&'a T);
        impl<T: Stringify + ?Sized> fmt::Display for Adapter<'_, T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.stringify(f)
            }
        }
        Adapter(self)
    }
}
