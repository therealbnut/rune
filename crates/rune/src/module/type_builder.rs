use crate::runtime::VmResult;

pub trait TypeWrapper: Sized {
    type Inner;
    fn with_wrapped<R>(self, f: impl FnOnce(Self::Inner) -> R) -> VmResult<R>;
}
// Same type.
impl<T> TypeWrapper for T {
    type Inner = T;
    #[inline(always)]
    fn with_wrapped<R>(self, f: impl FnOnce(Self::Inner) -> R) -> VmResult<R> {
        VmResult::Ok(f(self))
    }
}
