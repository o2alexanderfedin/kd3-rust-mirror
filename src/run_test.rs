use super::*;
use crate::kd3::kdtree::{
    kdtree_build, kdtree_delete, kdtree_iterator_delete,
    kdtree_iterator_get_next, kdtree_search, kdtree_search_space,
};
use crate::kd3::kdtree_h::{Kdtree, KdtreeIterator};

pub(crate) static mut x: *mut f64 = unsafe { core::mem::zeroed() };

pub(crate) static mut y: *mut f64 = unsafe { core::mem::zeroed() };

pub(crate) static mut z: *mut f64 = unsafe { core::mem::zeroed() };

#[inline]
extern "C" fn set_point(idx: u64, x_1: f64, y_1: f64, z_1: f64) -> () {
    unsafe {
        unsafe { *x.add(idx as usize) = x_1 };
        unsafe { *y.add(idx as usize) = y_1 };
        unsafe { *z.add(idx as usize) = z_1 };
    }
}

extern "C" fn initialise_points() -> () {
    unsafe {
        x =
            unsafe {
                    malloc((core::mem::size_of::<f64>() as
                                u64).wrapping_mul(11 as u64))
                } as *mut f64;
        y =
            unsafe {
                    malloc((core::mem::size_of::<f64>() as
                                u64).wrapping_mul(11 as u64))
                } as *mut f64;
        z =
            unsafe {
                    malloc((core::mem::size_of::<f64>() as
                                u64).wrapping_mul(11 as u64))
                } as *mut f64;
        set_point(0 as u64, 0.5, 0.5, 0.5);
        set_point(1 as u64, 0.5, 0.5, 0.5);
        set_point(2 as u64, 0.5, 0.5, 0.5);
        set_point(3 as u64, 0.0, 0.0, 0.0);
        set_point(4 as u64, 1.0, 0.0, 0.0);
        set_point(5 as u64, 1.0, 1.0, 0.0);
        set_point(6 as u64, 0.0, 1.0, 0.0);
        set_point(7 as u64, 0.0, 0.0, 1.0);
        set_point(8 as u64, 1.0, 0.0, 1.0);
        set_point(9 as u64, 1.0, 1.0, 1.0);
        set_point(10 as u64, 0.0, 1.0, 1.0);
    }
}

extern "C" fn cmp(v1: *const (), v2: *const ()) -> i32 {
    let a: i32 = unsafe { *(v1 as *const i32) } as i32;
    let b: i32 = unsafe { *(v2 as *const i32) } as i32;
    return if a as i32 > b { 1 } else { if (a as i32) < b { -1 } else { 0 } };
}

#[allow(unused_doc_comments)]
extern "C" fn validate(iter: *mut KdtreeIterator, v: &[u64]) -> () {
    let mut i: u64 = 0 as u64;
    let mut content: *mut u64 = core::ptr::null_mut();
    if !(iter as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"validate".as_ptr() as *const i8,
                c"run_test.c".as_ptr() as *const i8, 40,
                c"iter != NULL".as_ptr() as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*iter).size } == v.len() as u64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"validate".as_ptr() as *const i8,
                c"run_test.c".as_ptr() as *const i8, 41,
                c"iter->size == count".as_ptr() as *const i8)
        }
    } else { { let _ = 0; } };

    /// DBL_MAX
    (content =
        unsafe {
                malloc((core::mem::size_of::<u64>() as
                            u64).wrapping_mul(v.len() as u64))
            } as *mut u64);
    {
        i = 0 as u64;
        '__b1: loop {
            if !(i < v.len() as u64) { break '__b1; }
            '__c1: loop {
                unsafe {
                    *content.add(i as usize) =
                        kdtree_iterator_get_next(unsafe { &mut *iter })
                };
                break '__c1;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p = (*__p).wrapping_add(1);
                __t
            };
        }
    }
    if !(kdtree_iterator_get_next(unsafe { &mut *iter }) ==
                            18446744073709551615u64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"validate".as_ptr() as *const i8,
                c"run_test.c".as_ptr() as *const i8, 45,
                c"kdtree_iterator_get_next(iter) == KDTREE_END".as_ptr() as
                    *const i8)
        }
    } else { { let _ = 0; } };

    /// Routines used for sorting points along different axes
    ///
    ///We've tried more complicate cmp routines which proceed
    ///to compare the other axes in the event that two points
    ///have the same value, however tests show that a basic
    ///compare results in better overall performance.
    unsafe {
        qsort(content as *mut (), v.len() as u64,
            core::mem::size_of::<u64>() as u64, cmp)
    };
    {
        i = 0 as u64;
        '__b2: loop {
            if !(i < v.len() as u64) { break '__b2; }
            '__c2: loop {
                if !(unsafe { *content.add(i as usize) } == v[i as usize]) as
                                i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"validate".as_ptr() as *const i8,
                            c"run_test.c".as_ptr() as *const i8, 48,
                            c"content[i] == v[i]".as_ptr() as *const i8)
                    }
                } else { { let _ = 0; } };
                break '__c2;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p = (*__p).wrapping_add(1);
                __t
            };
        }
    }
    unsafe { free(content as *mut ()) };
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn __main_inner() -> Result<(), i32> {
    unsafe {
        let mut tree: *mut Kdtree = 0 as *mut () as *mut Kdtree;
        let mut iter: *mut KdtreeIterator =
            0 as *mut () as *mut KdtreeIterator;
        initialise_points();
        kdtree_build(x, y, z, 11 as u64, &mut tree);
        kdtree_search(tree, &mut iter, -10 as f64, 0 as f64, 0 as f64, 9.999);
        let e0: [u64; 1] = [0 as u64];
        validate(iter, &e0[..0 as usize]);
        kdtree_search(tree, &mut iter, 0 as f64, 0 as f64, 0 as f64, 0.499);
        let e1: [u64; 1] = [3 as u64];
        validate(iter, &e1[..1 as usize]);
        kdtree_search(tree, &mut iter, 0.5, 0.5, 0.5, 0.5);
        let e2: [u64; 11] =
            [0 as u64, 1 as u64, 2 as u64, 3 as u64, 4 as u64, 5 as u64,
                    6 as u64, 7 as u64, 8 as u64, 9 as u64, 10 as u64];
        validate(iter, &e2[..11 as usize]);
        kdtree_search(tree, &mut iter, 0.5, 0.5, 0.5, 100.0);
        validate(iter, &e2[..11 as usize]);
        kdtree_search(tree, &mut iter, 0.5, 0.5, 0.0, 0.5);
        let e3: [u64; 7] =
            [0 as u64, 1 as u64, 2 as u64, 3 as u64, 4 as u64, 5 as u64,
                    6 as u64];
        validate(iter, &e3[..7 as usize]);
        kdtree_search(tree, &mut iter, 0.5, 0.5, 1.0, 0.5);
        let e4: [u64; 7] =
            [0 as u64, 1 as u64, 2 as u64, 7 as u64, 8 as u64, 9 as u64,
                    10 as u64];
        validate(iter, &e4[..7 as usize]);

        /// datatype for cmp function pointer
        kdtree_search_space(tree, &mut iter, 0.0, 1.0, 0.5, 1.0, 0.0, 1.0);
        /// static array of cmp functions indexed by dim
        let e5: [u64; 7] =
            [0 as u64, 1 as u64, 2 as u64, 5 as u64, 6 as u64, 9 as u64,
                    10 as u64];
        validate(iter, &e5[..7 as usize]);
        unsafe {
            printf(c"\n ---- ALL TESTS PASSED ---- \n".as_ptr() as *const i8)
        };
        kdtree_iterator_delete(&mut iter);
        kdtree_delete(&mut tree);
        unsafe { free(x as *mut ()) };
        unsafe { free(y as *mut ()) };
        unsafe { free(z as *mut ()) };
        return Ok(());
    }
}
