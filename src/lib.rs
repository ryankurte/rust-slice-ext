
/// SplitBefore trait returns an iterator splitting a slice before a predicate
/// and including the matched item at the start of the next set (if found).
/// For example:
/// 
/// ```
/// use slice_ext::*;
/// 
/// let a: &[u8] = &[0, 1, 2]; 
/// let mut s = (&a[..]).split_before(|v| *v == 1 );
/// 
/// assert_eq!(s.next().unwrap(), &[0]);
/// assert_eq!(s.next().unwrap(), &[1, 2]);
/// assert_eq!(s.next().is_none(), true);
/// 
/// ```
pub trait SplitBefore<'a, T: 'a, P> {
    fn split_before(&self, predicate: P) -> SplitInc<'a, T, P>;
}

impl <'a, T: 'a, P> SplitBefore<'a, T, P> for &'a [T] 
where
    P: FnMut(&T) -> bool,
    T: core::fmt::Debug,
{
    fn split_before(&self, predicate: P) -> SplitInc<'a, T, P> {
        SplitInc::split_before(&self, predicate)
    }
}

/// SplitAfter trait returns an iterator splitting a slice after a predicate
/// and including the matched item at the end of each set (if existing).
/// For example: 
/// 
/// ```
/// use slice_ext::*;
/// 
/// let a: &[u8] = &[0, 1, 2]; 
/// let mut s = (&a[..]).split_after(|v| *v == 1 );
/// 
/// assert_eq!(s.next().unwrap(), &[0, 1]);
/// assert_eq!(s.next().unwrap(), &[2]);
/// assert_eq!(s.next().is_none(), true);
/// 
/// ```
pub trait SplitAfter<'a, T: 'a, P> {
    fn split_after(&self, predicate: P) -> SplitInc<'a, T, P>;
}

impl <'a, T: 'a, P> SplitAfter<'a, T, P> for &'a [T] 
where
    P: FnMut(&T) -> bool,
    T: core::fmt::Debug,
{
    fn split_after(&self, predicate: P) -> SplitInc<'a, T, P> {
        SplitInc::split_after(&self, predicate)
    }
}

pub struct SplitInc<'a, T: 'a, F> {
    index: usize,
    data: &'a [T],
    matcher: F,
    mode: Mode,
}

enum Mode {
    Before,
    After,
}

impl <'a, T, F> SplitInc<'a, T, F> 
where 
    F: FnMut(&T) -> bool,
    T: core::fmt::Debug,
{
    pub fn split_before(data: &'a [T], matcher: F) -> Self {
        SplitInc{ index: 0, data, matcher, mode: Mode::Before }
    }

    pub fn split_after(data: &'a [T], matcher: F) -> Self {
        SplitInc{ index: 0, data, matcher, mode: Mode::After }
    }

    fn iter_before(&mut self) -> Option<&'a [T]> {
        // Short circuit on completion
        if self.index == self.data.len() {
            return None
        }

        // Select search range
        let index = self.index;

        for i in index..self.data.len() {

            if (self.matcher)(&self.data[i]) {
                // If our match is in the first position, and we're not at the end,
                // continue searching
                if i == index && i < self.data.len() - 1 {
                    continue
                // If our match is in the first position, and we are at the end,
                // return the last entry
                } else if i == index {
                    self.index = self.data.len();
                    return Some(&self.data[index..])
                }

                // When a match is found, update the count and return preceding data
                self.index = i;
                return Some(&self.data[index..i])
            }

             // When we're out of data, return anything left
            if i == (self.data.len() - 1) {
                self.index = self.data.len();
                return Some(&self.data[index..])
            }
        }

        None
    }

    fn iter_after(&mut self) -> Option<&'a [T]> {
        // Short circuit on completion
        if self.index == self.data.len() {
            return None
        }

        // Select search range
        let index = self.index;

        for i in index..self.data.len() {

            // When a match is found, update the count and return preceding data
            if (self.matcher)(&self.data[i]) {
                self.index = i+1;
                return Some(&self.data[index..i+1])
            }

            // When we're out of data, return anything left
            if i == (self.data.len() - 1) {
                self.index = self.data.len();
                return Some(&self.data[index..])
            }
        }

        None
    }
}

impl <'a, T, F> Iterator for SplitInc<'a, T, F> 
where 
    F: FnMut(&T) -> bool,
    T: core::fmt::Debug,
{
    type Item = &'a [T];
    
    fn next(&mut self) -> Option<Self::Item> {
    
        match self.mode {
            Mode::Before => self.iter_before(),
            Mode::After => self.iter_after(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_before() {
        let a: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
        
        let mut s = (&a[..]).split_before(|v| *v == 2 || *v == 5);
        
        assert_eq!(s.next().unwrap(), &[0, 1]);
        assert_eq!(s.next().unwrap(), &[2, 3, 4]);
        assert_eq!(s.next().unwrap(), &[5, 6, 7, 8]);
        assert_eq!(s.next().is_none(), true);
    }
    
    #[test]
    fn test_split_before_no_match() {
        let a: &[u8] = &[0, 1, 2];
        
        let mut s = SplitInc::split_before(&a, |v| *v == 12);
        
        assert_eq!(s.next().unwrap(), &[0, 1, 2]);
        assert_eq!(s.next().is_none(), true);
    }
    
    #[test]
    fn test_split_before_start() {
        let a: &[u8] = &[0, 1, 2];
        
        let mut s = SplitInc::split_before(&a, |v| *v == 0 );
        
        assert_eq!(s.next().unwrap(), &[0, 1, 2]);
        assert_eq!(s.next().is_none(), true);
    }
    
    #[test]
    fn test_split_before_end() {
        let a: &[u8] = &[0, 1, 2];
        
        let mut s = SplitInc::split_before(&a, |v| *v == 2 );
        
        assert_eq!(s.next().unwrap(), &[0, 1]);
        assert_eq!(s.next().unwrap(), &[2]);
        assert_eq!(s.next().is_none(), true);
    }

    #[test]
    fn test_split_after() {
        let a: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
        
        let mut s = SplitInc::split_after(&a, |v| *v == 2 || *v == 5);
        
        assert_eq!(s.next().unwrap(), &[0, 1, 2]);
        assert_eq!(s.next().unwrap(), &[3, 4, 5]);
        assert_eq!(s.next().unwrap(), &[6, 7, 8]);
        assert_eq!(s.next().is_none(), true);
    }
    
    #[test]
    fn test_split_after_no_match() {
        let a: &[u8] = &[0, 1, 2];
        
        let mut s = SplitInc::split_after(&a, |v| *v == 12);
        
        assert_eq!(s.next().unwrap(), &[0, 1, 2]);
        assert_eq!(s.next().is_none(), true);
    }
    
    #[test]
    fn test_split_after_start() {
        let a: &[u8] = &[0, 1, 2];
        
        let mut s = SplitInc::split_after(&a, |v| *v == 0 );
        
        assert_eq!(s.next().unwrap(), &[0]);
        assert_eq!(s.next().unwrap(), &[1, 2]);
        assert_eq!(s.next().is_none(), true);
    }
    
    #[test]
    fn test_split_after_end() {
        let a: &[u8] = &[0, 1, 2];
        
        let mut s = SplitInc::split_after(&a, |v| *v == 2 );
        
        assert_eq!(s.next().unwrap(), &[0, 1, 2]);
        assert_eq!(s.next().is_none(), true);
    }
}

