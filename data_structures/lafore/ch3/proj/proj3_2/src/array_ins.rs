pub struct ArrayIns {
    a: Box<[i64]>,
    n_elems: usize
}

impl ArrayIns {
    pub fn new(max: usize) -> ArrayIns {
        ArrayIns {
            a: vec![0; max].into_boxed_slice(),
            n_elems: 0
        }
    }

    pub fn insert(&mut self, value: i64) {
        self.a[self.n_elems] = value;
        self.n_elems += 1;
    }

    pub fn display(&self) {
        for i in 0..self.n_elems {
            print!("{} ", self.a[i]);
        }

        println!();
    }

    pub fn insertion_sort(&mut self) {
        for out in 1..self.n_elems {
            let temp = self.a[out];
            let mut inner = out;

            while inner > 0 && self.a[inner - 1] >= temp {
                self.a[inner] = self.a[inner -1];
                inner -= 1;
            }

            self.a[inner] = temp;
        }
    }

    pub fn median(&mut self) -> i64 {
        self.insertion_sort();

        self.a[self.n_elems / 2]
    }
}
