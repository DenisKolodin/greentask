extern crate coroutine;

use std::marker::PhantomData;
use coroutine::asymmetric::{Coroutine, Handle};

#[derive(Debug)]
pub struct Resumer<R, Y> {
    handle: Handle,
    _resume_marker: PhantomData<R>,
    _yield_marker: PhantomData<Y>,
}

impl<R, Y> Resumer<R, Y> {

    fn new(handle: Handle) -> Self {
        Resumer {
            handle: handle,
            _resume_marker: PhantomData,
            _yield_marker: PhantomData,
        }
    }

    pub fn resume_with(&mut self, r: R) -> Y {
        let boxed = Box::new(r);
        let pointer = self.handle.resume(Box::into_raw(boxed) as usize);
        let data = unsafe { Box::from_raw(pointer as *mut Y) };
        *data
    }

}

#[derive(Debug)]
pub struct Yielder<'a, R, Y> {
    coroutine: &'a mut Coroutine,
    _resume_marker: PhantomData<R>,
    _yield_marker: PhantomData<Y>,
}

impl<'a, R, Y> Yielder<'a, R, Y> {

    fn new(coroutine: &'a mut Coroutine) -> Self {
        Yielder {
            coroutine: coroutine,
            _resume_marker: PhantomData,
            _yield_marker: PhantomData,
        }
    }

    pub fn yield_with(&mut self, y: Y) -> R {
        let boxed = Box::new(y);
        let pointer = self.coroutine.yield_with(Box::into_raw(boxed) as usize);
        let data = unsafe { Box::from_raw(pointer as *mut R) };
        *data
    }
}

pub fn spawn<F, R, Y>(f: F) -> Resumer<R, Y>
    where F: FnOnce(Yielder<R, Y>, R) -> Y + 'static {

    let handle = Coroutine::spawn(|coroutine, pointer| {
        let data = unsafe { Box::from_raw(pointer as *mut R) };
        let result = f(Yielder::new(coroutine), *data);
        let boxed = Box::new(result);
        Box::into_raw(boxed) as usize
    });
    Resumer::new(handle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug,PartialEq)]
    enum Request {
        Init,
        Chars(&'static str),
    }

    #[derive(Debug,PartialEq)]
    enum Response {
        Ready,
        Integer(u64),
        Float(f64),
        Done,
    }

    #[test]
    fn test_coroutines() {
        let mut resumer = spawn(|mut yielder, init| {
            assert_eq!(init, Request::Init);
            assert_eq!(yielder.yield_with(Response::Ready), Request::Chars("integer"));
            assert_eq!(yielder.yield_with(Response::Integer(123)), Request::Chars("float"));
            assert_eq!(yielder.yield_with(Response::Float(1.23)), Request::Chars("done"));
            Response::Done
        });

        assert_eq!(resumer.resume_with(Request::Init), Response::Ready);
        assert_eq!(resumer.resume_with(Request::Chars("integer")), Response::Integer(123));
        assert_eq!(resumer.resume_with(Request::Chars("float")), Response::Float(1.23));
        assert_eq!(resumer.resume_with(Request::Chars("done")), Response::Done);
    }
}

