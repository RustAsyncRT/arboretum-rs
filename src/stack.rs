pub struct Stack<T: Default + Clone, const N: usize> {
    buffer: [T; N],
    top: usize,
}

impl<T: Default + Clone, const N: usize> Stack<T, N> {
    pub fn push(&mut self, data: T) -> Result<(), ()> {
        if self.top < self.buffer.len() {
            self.buffer[self.top] = data;
            self.top += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn peek(&self) -> Result<&T, ()> {
        if self.top > 0 {
            Ok(&self.buffer[self.top - 1])
        } else {
            Err(())
        }
    }

    pub fn pop(&mut self) -> Result<T, ()> {
        if self.top > 0 {
            self.top -= 1;
            let data = self.buffer[self.top].clone();
            Ok(data)
        } else {
            Err(())
        }
    }
}

impl<T: Default + Clone + Copy, const N: usize> From<T> for Stack<T, N> {
    fn from(value: T) -> Self {
        Stack {
            buffer: [value; N],
            top: 0,
        }
    }
}

impl<T: Default + Clone + Copy, const N: usize> Default for Stack<T, N> {
    fn default() -> Self {
        Self {
            buffer: [T::default(); N],
            top: 0,
        }
    }
}
