#[inline(always)]
pub fn find_index<T: PartialEq>(vec: &Vec<T>, value: &T) -> Option<usize> {
    vec.iter().position(|x| x == value)
}
