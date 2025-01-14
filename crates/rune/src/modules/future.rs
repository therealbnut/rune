//! The `std::future` module.

use crate::no_std::prelude::*;

use crate::runtime::{Future, SelectFuture, Shared, Stack, Value, VmErrorKind, VmResult};
use crate::{ContextError, Module};

/// Construct the `std::future` module.
pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::with_crate_item("std", ["future"]);
    module.ty::<Future>()?;
    module.raw_fn(["join"], raw_join)?;
    Ok(module)
}

async fn try_join_impl<'a, I, F>(values: I, len: usize, factory: F) -> VmResult<Value>
where
    I: IntoIterator<Item = &'a Value>,
    F: FnOnce(Vec<Value>) -> Value,
{
    use futures_util::stream::StreamExt as _;

    let mut futures = futures_util::stream::FuturesUnordered::new();
    let mut results = Vec::with_capacity(len);

    for (index, value) in values.into_iter().enumerate() {
        let future = match value {
            Value::Future(future) => vm_try!(future.clone().into_mut()),
            value => {
                return VmResult::err(vm_try!(VmErrorKind::bad_argument::<Future>(index, value)))
            }
        };

        futures.push(SelectFuture::new(index, future));
        results.push(Value::Unit);
    }

    while !futures.is_empty() {
        let (index, value) = vm_try!(futures.next().await.unwrap());
        *results.get_mut(index).unwrap() = value;
    }

    VmResult::Ok(factory(results))
}

async fn join(value: Value) -> VmResult<Value> {
    match value {
        Value::Tuple(tuple) => {
            let tuple = vm_try!(tuple.borrow_ref());
            VmResult::Ok(vm_try!(
                try_join_impl(tuple.iter(), tuple.len(), Value::tuple).await
            ))
        }
        Value::Vec(vec) => {
            let vec = vm_try!(vec.borrow_ref());
            VmResult::Ok(vm_try!(
                try_join_impl(vec.iter(), vec.len(), Value::vec).await
            ))
        }
        value => VmResult::err(vm_try!(VmErrorKind::bad_argument::<Vec<Value>>(0, &value))),
    }
}

/// The join implementation.
fn raw_join(stack: &mut Stack, args: usize) -> VmResult<()> {
    if args != 1 {
        return VmResult::err(VmErrorKind::BadArgumentCount {
            actual: args,
            expected: 1,
        });
    }

    let value = vm_try!(stack.pop());
    let value = Value::Future(Shared::new(Future::new(join(value))));
    stack.push(value);
    VmResult::Ok(())
}
