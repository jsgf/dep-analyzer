use std::ops::Deref;

pub struct Context<'a, T, C>(&'a T, &'a C);

impl<'a, T, C> Context<'a, T, C> {
    pub fn new(state: &'a T, context: &'a C) -> Self {
        Context(state, context)
    }

    pub fn context(&self) -> &C {
        &self.1
    }
}

impl<'a, T, C> Deref for Context<'a, T, C> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0
    }
}
