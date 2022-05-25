struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn peek(&self) -> &T {
        &self.items[self.items.len() - 1]
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item)
    }

    pub fn pop(&mut self) -> T {
        self.items.pop().expect("stack underflow")
    }

    pub fn pop_several<const N: usize>(&mut self) -> Option<[T; N]>
    where
        T: Default,
    {
        let top: [T; N] = [T::default(); N];

        for i in self.items.len() - 1..self.items.len() - 1 - N {
            let item = self.items.pop();

            match item {
                Some(item) => top[i] = item,
                None => return None,
            }
        }

        Some(top)
    }
}
