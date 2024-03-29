#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Level {
    Ignore,
    Warning,
    Panic,
}

impl Default for Level {
    fn default() -> Self {
        Self::Ignore
    }
}

impl Level {
    pub fn assert<M>(&self, condition: bool, message: M)
    where
        M: Fn() -> String,
    {
        if !condition {
            self.error(message);
        }
    }

    pub fn error<M>(&self, message: M)
    where
        M: Fn() -> String,
    {
        match self {
            Self::Warning => warn!("{}", message()),
            Self::Panic => panic!("{}", message()),
            _ => {}
        }
    }
}
