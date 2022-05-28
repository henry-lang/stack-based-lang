use std::mem::MaybeUninit;

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

    pub fn pop_several<const N: usize>(&mut self) -> Option<[T; N]> {
        let mut top: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for element in top.iter_mut().rev() {
            *element = match self.items.pop() {
                Some(item) => MaybeUninit::new(item),
                None => return None,
            }
        }

        Some(top.map(|e| unsafe { e.assume_init() }))
    }
}
