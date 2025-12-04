use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct Ctx<E> {
    pub msg: &'static str,
    pub source: E,
}

impl<E: Display> Display for Ctx<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.msg, self.source)
    }
}

impl<E: Debug + Display> Debug for Ctx<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.msg, self.source)
    }
}

impl<E: Display + Error + 'static> Error for Ctx<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl<E> Ctx<E> {
    pub fn new(msg: &'static str, source: E) -> Self {
        Self { msg, source }
    }
}
