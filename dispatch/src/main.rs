
#[derive(Debug)]
struct ErrorWrapper;
struct DynError;

trait MapError {
    fn func<T>(self);
}

/* impl MapError for ErrorWrapper {
    fn func<Self>(self: &T) where Self: MapError {
        todo!()
    }
}

impl MapError for DynError {
    fn func<T>(&self) {
        todo!()
    }
}  */

/* pub trait MapError: Sized {
/*     fn static_dispatch(&self) {
        println!("{:?}", self)
    } */
    fn dynamic_dispatch<T>(t: &dyn MapError) {
        t.dynamic_dispatch()
    }
} */

/* impl MapError for ErrorWrapper {
    fn func<Self>(self: &T) where Self: MapError {
        todo!()
    }
}

impl MapError for DynError {
    fn func<T>(&self) {
        todo!()
    }
} */

pub trait VisitorDriver<'m>: Sized {
    fn VisitorDriver<F>(VisitorDriver: F) -> Result<Self, ErrorWrapper>
    where
        F: VisitorDriver<'m>;
}

impl<'m> VisitorDriver<'m> for i32 {
    fn VisitorDriver<F>(VisitorDriver: F) -> Result<i32, ErrorWrapper>
    where
        F: VisitorDriver<'m>,
    {
        VisitorDriver.VisitorDriver_i32(I32Visitor)
    }
}

impl<'m> VisitorDriver<'m> for i8 {
    fn VisitorDriver<D>(VisitorDriver: D) -> Result<i8, ErrorWrapper>
    where
        D: VisitorDriver<'m>,
    {
        VisitorDriver.VisitorDriver_i8(I8Visitor)
    }
}
