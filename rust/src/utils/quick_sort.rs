use std::fmt::Debug;

// move first element to the correct place
// everything lower should be on the left
// everything higher should be on the right
// return the pivot index
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            // move our pivot forward 1
            // put this element before the pivot
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);

    let (left, right) = v.split_at_mut(p);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

// incomplete using std threads
// struct RawSend<T>(*mut [T]);

// unsafe impl<T> Send for RawSend<T> {}

// pub fn threaded_quick_sort<
//     T: 'static +  PartialOrd + Debug +
// std::marker::Send, >(
//     v: &mut [T],
// ) {
//     if v.len() <= 1 {
//         return;
//     }

//     let p = pivot(v);
//     dbg!(&v);

//     let (left, right) = v.split_at_mut(p);

//     let raw_left: *mut [T] = left as *mut [T];
//     let raw_s = RawSend(raw_left);

//     unsafe {
//         let handle = std::thread::spawn(move ||
// {             threaded_quick_sort(&mut
// *raw_s.0);         });
//         threaded_quick_sort(&mut right[1..]);
//         handle.join().ok();
//     }
// }

pub fn quick_sort_rayon<T: PartialOrd + Debug + Send>(
    v: &mut [T],
) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);

    let (left, right) = v.split_at_mut(p);
    
    // put f2 on the queue and then start f1
    // if another thread is ready it will steal f2
    // this works recursively down the stack
    rayon::join(
        || quick_sort_rayon(left),
        || quick_sort_rayon(&mut right[1..]),
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
        let p = pivot(&mut v);

        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p))
        }
        // assert_eq!(v, vec![1, 3, 4, 6, 19, 8,
        // 11, 13]);
    }
    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13, 19]);

        let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 6, 7, 9, 12, 13, 14]);
    }

    #[test]
    fn test_quick_sort_rayon() {
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
        quick_sort_rayon(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13, 19]);

        let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        quick_sort_rayon(&mut v);
        assert_eq!(v, vec![1, 2, 6, 7, 9, 12, 13, 14]);
    }
}
