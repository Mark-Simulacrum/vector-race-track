use std::rc::Rc;
use std::fmt;

#[derive(Clone)]
pub struct List<T> {
    head: Option<Rc<List<T>>>,
    tail: Option<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
            tail: None,
        }
    }

    #[must_use]
    pub fn push(&self, element: T) -> Self
        where T: Clone,
    {
        List {
            head: Some(Rc::new(self.clone())),
            tail: Some(element),
        }
    }

    // Note that this iterates in reverse of the expected order
    pub fn iter(&self) -> ListIter<'_, T> {
        ListIter {
            list: Some(self),
        }
    }
}

impl<T: Copy + fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list()
            .entries(self.iter())
            .finish()
    }
}

pub struct ListIter<'a, T> {
    list: Option<&'a List<T>>,
}

impl<'a, T: Copy> Iterator for ListIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.list.is_none() { return None; }

        let list = self.list.unwrap();

        match list.head {
           None => {
               self.list = None;
               return list.tail;
           },

           Some(_) => {
               self.list = self.list.unwrap().head.as_ref().map(|t| &**t);
               return list.tail;
           }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn list_1() {
        let mut l = List::new();
        l = l.push(1).push(2);
        assert_eq!(l.iter().collect::<Vec<_>>(), &[2, 1]);
        let l2 = l.push(3);
        assert_eq!(l.iter().collect::<Vec<_>>(), &[2, 1]);
        assert_eq!(l2.iter().collect::<Vec<_>>(), &[3, 2, 1]);
    }
}
