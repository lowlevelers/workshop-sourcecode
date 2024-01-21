use std::future::Future;

trait Handler<Args>: Clone + 'static {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, args: Args) -> Self::Future;
}

impl<Func, Arg1, Arg2, Fut> Handler<(Arg1, Arg2)> for Func
where
    Func: Fn(Arg1, Arg2) -> Fut + Clone + 'static,
    Fut: Future,
{
    type Output = Fut::Output;
    type Future = Fut;

    fn call(&self, (arg1, arg2): (Arg1, Arg2)) -> Self::Future {
        (self)(arg1, arg2)
    }
}
