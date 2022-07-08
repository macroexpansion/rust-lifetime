struct MyIterator<'iter, T> {
    slice: &'iter [T],
}

impl<'iter, T> Iterator for MyIterator<'iter, T> {
    type Item = &'iter T;

    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some(first)
    }
}

struct MyMutIterator<'iter, T> {
    slice: &'iter mut [T],
}

impl<'iter, T> Iterator for MyMutIterator<'iter, T> {
    type Item = &'iter mut T;

    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let slice: &mut &mut [T] = &mut self.slice;
        let slice = std::mem::replace(slice, &mut []);
        let (first, rest) = slice.split_first_mut()?;
        self.slice = rest;
        Some(first)
    }
}

mod tests {
    use super::*;

    #[test]
    fn immutable_iterator() {
        let vec = vec![1, 2, 3];

        let iterator = MyIterator { slice: &vec };

        for it in iterator {
            println!("{it}");
        }

        assert_eq!(vec, [1, 2, 3]);
    }

    #[test]
    fn mutable_iterator() {
        let mut vec = vec![1, 2, 3];

        let mut_iterator = MyMutIterator { slice: &mut vec };

        for it in mut_iterator {
            *it += 1;
            println!("{it}");
        }

        assert_eq!(vec, [2, 3, 4]);
    }
}
