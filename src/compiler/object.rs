use crate::compiler::chunk::Chunk;
use std::any;
use std::any::Any;
use std::cell::Cell;
use std::fmt::{Display, Formatter, Pointer, Result};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct Closure {
    pub fun: Gc<Function>,
}

impl Closure {
    pub fn new(fun: Gc<Function>) -> Closure {
        Closure { fun }
    }
}

#[derive(Clone, PartialEq)]
pub enum FunctionType {
    Function,
    Script,
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    chunk: Chunk,
    arity: u8,
}

impl Function {
    pub fn new() -> Self {
        Function {
            name: String::new(),
            chunk: Chunk::new(),
            arity: 0,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_arity(&mut self, arity: u8) {
        self.arity = arity;
    }

    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }

    pub fn arity(&self) -> &u8 {
        &self.arity
    }

    pub fn chunk_mut(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    pub fn arity_mut(&mut self) -> &mut u8 {
        &mut self.arity
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // TODO
        // if let Some(name) = self.chunk. {
        write!(f, "<fn {}>", self.name)
        // } else {
        //     write!(f, "<script>")
        // }
    }
}

#[derive(Debug)]
struct ObjBox<T: ?Sized> {
    mark: Cell<bool>,
    value: T,
}

#[derive(Debug)]
pub struct Gc<T: ?Sized>(*mut ObjBox<T>);

impl<T> Gc<T> {
    pub fn new(value: T) -> Self {
        Self(Box::into_raw(Box::new(ObjBox {
            mark: Cell::new(false),
            value,
        })))
    }
}

impl<T: ?Sized> Gc<T> {
    fn report_null(&self) -> ! {
        panic!(
            "Holding null reference to type {} at address {:p}.",
            any::type_name::<T>(),
            self.0
        );
    }

    fn deref_non_null(&self) -> &ObjBox<T> {
        if self.0.is_null() {
            self.report_null();
        } else {
            unsafe { &*self.0 }
        }
    }

    pub fn is_marked(&self) -> bool {
        self.deref_non_null().mark.get()
    }

    pub fn clear_mark(&self) {
        self.deref_non_null().mark.set(false);
    }

    pub fn mark(&self) {
        #[cfg(feature = "trace-gc")]
        log::debug!("{:p} mark", self.0);

        self.deref_non_null().mark.set(true);
    }

    pub fn free(self) {
        #[cfg(feature = "trace-gc")]
        log::debug!("{:p} free", self.0);

        unsafe {
            // drop inner wrapper, and thus the value it owns
            Box::from_raw(self.0);
        }
    }
}

impl<T: Any> Gc<T> {
    pub fn as_any(self) -> Gc<dyn Any> {
        Gc(self.0 as *mut ObjBox<dyn Any>)
    }
}

impl<T: ?Sized> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: ?Sized> Copy for Gc<T> {}

impl<T: ?Sized> Deref for Gc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.deref_non_null().value
    }
}

impl<T: ?Sized> DerefMut for Gc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.0.is_null() {
            self.report_null();
        } else {
            &mut unsafe { &mut (*self.0) }.value
        }
    }
}

impl<T: ?Sized> Pointer for Gc<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.0.fmt(f)
    }
}
