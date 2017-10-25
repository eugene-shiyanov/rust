pub struct StackX {
    max_size : usize,
    stack_array : Box<[i64]>,
    top : isize
}

impl StackX {
    pub fn new(size : usize) -> StackX {
        StackX {
            max_size : size,
            stack_array : vec![0; size].into_boxed_slice(),
            top : -1
        }
    }

    pub fn push(&mut self, value: i64) {
        self.top += 1;
        self.stack_array[self.top as usize] = value;
    }

    pub fn pop(&mut self) -> i64 {
        let value = self.stack_array[self.top as usize];
        self.top -= 1;
        value
    }

    pub fn peek(&self) -> i64 {
        self.stack_array[self.top as usize]
    }

    pub fn is_empty(&self) -> bool {
        self.top == -1
    }

    pub fn is_full(&self) -> bool {
        self.top as usize == self.max_size - 1
    }
}