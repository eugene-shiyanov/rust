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
            let mut temp = self.a[out];
            let mut inner = out;


            while inner > 0 && self.a[inner - 1] >= temp {
                if temp == self.a[inner - 1] {
                    temp = -1;
                }

                self.a[inner] = self.a[inner -1];
                inner -= 1;
            }

            self.a[inner] = temp;
        }

        self.display();
        let mut shift_count = 0;

        while shift_count < self.n_elems && self.a[shift_count] == -1 {
            shift_count += 1;
        }

        for i in shift_count..self.n_elems {
            self.a[i - shift_count] = self.a[i];
        }

        self.n_elems -= shift_count;
    }
}
