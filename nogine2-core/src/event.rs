/// Simple observer pattern.
pub struct Event<Params> {
    subscribers: Vec<fn(&Params)>,
}

impl<Params> Event<Params> {
    pub const fn new() -> Self {
        Self { subscribers: Vec::new() }
    }

    /// Adds the function to the execution stack.
    pub fn subscribe(&mut self, f: fn(&Params)) {
        self.subscribers.push(f);
    }

    /// Calls the whole function stack, in insertion order.
    pub fn call(&self, params: &Params) {
        for f in &self.subscribers {
            f(params)
        }
    }
}
