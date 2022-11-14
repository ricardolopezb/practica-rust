pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let mid = array.len()/2;
    let item = array[mid];
    if key < item {
        return find(&array[..mid], key);
    }
    else if key > item {
        return find(&array[mid+1..], key).map(|x| x + mid + 1);
    }
    else {
        return Some(mid as usize);
    }
}


// [a,  b,  c,  d,  e,  f] -> mid = 3  -> 0 + 3 + 1 = 4
//  0   1   2   3   4   5

//                 [e,  f]
//                  0   1