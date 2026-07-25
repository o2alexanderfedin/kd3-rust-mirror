use super::*;
use crate::kd3::kdtree_h::{DataPoint, Kdtree, KdtreeIterator, Space, TreeNode};
use crate::{
    __assert_rtn, __builtin___memcpy_chk, __builtin_object_size, free, malloc, qsort, realloc,
};

/// Deallocates a tree object referenced by tree_ptr and sets the ptr to NULL
pub(crate) extern "C" fn kdtree_delete(tree_ptr: &mut *mut Kdtree) -> () {
    let tree: *mut Kdtree = *tree_ptr;
    if tree as *mut () == 0 as *mut () {
        return;
    }
    unsafe { free(unsafe { (*tree).points } as *mut ()) };
    unsafe { free(unsafe { (*tree).node_data } as *mut ()) };
    unsafe { free(tree as *mut ()) };
    *tree_ptr = 0 as *mut () as *mut Kdtree;
}

/// dimensions hard coded to 3. Declare constants for convenience
pub(crate) const NDIMS: u32 = 3;

pub(crate) const DIM_X: u32 = 0;

pub(crate) const DIM_Y: u32 = 1;

pub(crate) const DIM_Z: u32 = 2;

#[inline]
/// declaration of internal functions
extern "C" fn _next_node(tree: &mut Kdtree) -> *mut TreeNode {
    if !((*tree).next_node < (*tree).max_nodes) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"_next_node".as_ptr() as *const i8,
                c"kdtree.c".as_ptr() as *const i8,
                280,
                c"tree->next_node < tree->max_nodes".as_ptr() as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    return unsafe {
        &mut *(*tree).node_data.add({
            let __p = &mut (*tree).next_node;
            let __t = *__p;
            *__p = (*__p).wrapping_add(1);
            __t
        } as usize)
    };
}

#[inline]
/// return a leaf node. Holds the index of the actual data point
extern "C" fn _get_leaf_node(tree: *mut Kdtree, offset: u64) -> *mut TreeNode {
    let node: *mut TreeNode = _next_node(unsafe { &mut *tree });
    unsafe { (*node).left = 0 as *mut () as *mut TreeNode };
    unsafe { (*node).right = 0 as *mut () as *mut TreeNode };
    unsafe { (*node).idx = offset };
    return node;
}

/// datatype for cmp function pointer
pub(crate) type CmpFunc = unsafe extern "C" fn(*const (), *const ()) -> i32;

extern "C" fn cmp_x(a1: *const (), a2: *const ()) -> i32 {
    let a1: *const DataPoint = a1 as *const DataPoint;
    let a2: *const DataPoint = a2 as *const DataPoint;
    return if unsafe { (*a1).x } as f64 > unsafe { (*a2).x } {
        1
    } else {
        if (unsafe { (*a1).x } as f64) < unsafe { (*a2).x } {
            -1
        } else {
            0
        }
    };
}

extern "C" fn cmp_y(a1: *const (), a2: *const ()) -> i32 {
    let a1: *const DataPoint = a1 as *const DataPoint;
    let a2: *const DataPoint = a2 as *const DataPoint;
    return if unsafe { (*a1).y } as f64 > unsafe { (*a2).y } {
        1
    } else {
        if (unsafe { (*a1).y } as f64) < unsafe { (*a2).y } {
            -1
        } else {
            0
        }
    };
}

extern "C" fn cmp_z(a1: *const (), a2: *const ()) -> i32 {
    let a1: *const DataPoint = a1 as *const DataPoint;
    let a2: *const DataPoint = a2 as *const DataPoint;
    return if unsafe { (*a1).z } as f64 > unsafe { (*a2).z } {
        1
    } else {
        if (unsafe { (*a1).z } as f64) < unsafe { (*a2).z } {
            -1
        } else {
            0
        }
    };
}

/// static array of cmp functions indexed by dim
static func_select: [unsafe extern "C" fn(*const (), *const ()) -> i32; 3] = [cmp_x, cmp_y, cmp_z];

#[inline]
/// return a branch node
extern "C" fn _get_branch_node(tree: *mut Kdtree, split: f64) -> *mut TreeNode {
    let node: *mut TreeNode = _next_node(unsafe { &mut *tree });
    unsafe { (*node).split = split };
    return node;
}

/// internal routine to recursively build the kdtree
#[allow(unused_doc_comments)]
extern "C" fn _build_kdtree(
    idx_from: u64,
    idx_to: u64,
    depth: u64,
    tree: *mut Kdtree,
) -> *mut TreeNode {
    let mut split: f64 = 0.0;
    let mut node: *mut TreeNode = core::ptr::null_mut();
    let mut point: *const DataPoint = core::ptr::null();
    let count: u64 = idx_to.wrapping_sub(idx_from).wrapping_add(1 as u64) as u64;
    let mid: u64 = idx_from.wrapping_add(idx_to.wrapping_sub(idx_from) / 2 as u64) as u64;
    let axis: u64 = (depth % NDIMS as u64) as u64;
    if count as u64 == 1 as u64 {
        return _get_leaf_node(tree, idx_from);
    }

    /// sort the points within this group to determine the median point
    ///- This can be a potential performance bottleneck. There are methods
    ///  to determine median in linear time, but that can get rather
    ///  complicated. Will consider if this proves to be an issue.
    unsafe {
        qsort(
            unsafe { unsafe { (*tree).points.add(idx_from as usize) } } as *mut (),
            count,
            core::mem::size_of::<DataPoint>() as u64,
            func_select[axis as usize],
        )
    };

    /// determine point where axis will be split
    (point = unsafe { unsafe { (*tree).points.add(mid as usize) } });
    split = if axis as u64 == 0 as u64 {
        unsafe { (*point).x }
    } else {
        if axis as u64 == 1 as u64 {
            unsafe { (*point).y }
        } else {
            unsafe { (*point).z }
        }
    };

    /// recursively build a tree for the left and right planes
    (node = _get_branch_node(tree, split));
    unsafe { (*node).left = _build_kdtree(idx_from, mid, depth.wrapping_add(1 as u64), tree) };
    unsafe {
        (*node).right = _build_kdtree(
            mid.wrapping_add(1 as u64),
            idx_to,
            depth.wrapping_add(1 as u64),
            tree,
        )
    };
    return node;
}

/// Build a 3D k-d tree based on the points stored in x, y, z arrays (with count
///specifying the number of points).
///
///To optimise for cases where the data points may move and we need to rebuild
///the tree for said points, we take in a reference to the kdtree object pointer
///instead of simply returning the address of a new object.
///
///This allows the user to specify a NULL pointer when creating an new tree object,
///or reuse the memory of the previously created object when rebuilding the tree
///during the next iteration.
///
///  kdtree *tree = NULL;
///  for (...) {
///      kdtree_build(x, y, z, count, &tree);
///  }
///  kdtree_delete(&tree);
///
///Note that tree object can only be reused if the count is equal. Mismatching
///counts will cause the previous object to be deleted and a new one built in
///its place.
///
///To reduce the amount of checks, we do not handle cases where count < 0.
///Do ensure that we're dealing with at lease two points
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn kdtree_build(
    x: *mut f64,
    y: *mut f64,
    z: *mut f64,
    count: u64,
    tree_ptr: &mut *mut Kdtree,
) -> () {
    unsafe {
        let mut i: u64 = 0 as u64;
        let mut tree: *mut Kdtree = *tree_ptr;

        /// sanity check
        if !(count > 1 as u64) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(
                    c"kdtree_build".as_ptr() as *const i8,
                    c"kdtree.c".as_ptr() as *const i8,
                    142,
                    c"count > 1".as_ptr() as *const i8,
                )
            }
        } else {
            {
                let _ = 0;
            }
        };
        if (tree).is_null() as i32 != 0 || unsafe { (*tree).count } != count {
            if !(tree).is_null() {
                kdtree_delete(&mut tree);
            }

            /// delete prev obj
            /// allocate new object and update user's reference
            (tree = unsafe { malloc(core::mem::size_of::<Kdtree>() as u64) } as *mut Kdtree);
            if !(tree as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(
                        c"kdtree_build".as_ptr() as *const i8,
                        c"kdtree.c".as_ptr() as *const i8,
                        150,
                        c"tree != NULL".as_ptr() as *const i8,
                    )
                }
            } else {
                {
                    let _ = 0;
                }
            };
            *tree_ptr = tree;

            /// initialise values and memory
            unsafe {
                (*tree).count = count
            };
            unsafe {
                (*tree).max_nodes = count
                    .wrapping_sub(1 as u64)
                    .wrapping_mul(2 as u64)
                    .wrapping_add(1 as u64)
            };
            unsafe {
                (*tree).points = unsafe {
                    malloc((core::mem::size_of::<DataPoint>() as u64).wrapping_mul(count))
                } as *mut DataPoint
            };
            unsafe {
                (*tree).node_data = unsafe {
                    malloc(
                        (core::mem::size_of::<TreeNode>() as u64)
                            .wrapping_mul(unsafe { (*tree).max_nodes }),
                    )
                } as *mut TreeNode
            };
            if !(unsafe { (*tree).points } as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(
                        c"kdtree_build".as_ptr() as *const i8,
                        c"kdtree.c".as_ptr() as *const i8,
                        158,
                        c"tree->points != NULL".as_ptr() as *const i8,
                    )
                }
            } else {
                {
                    let _ = 0;
                }
            };
            if !(unsafe { (*tree).node_data } as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(
                        c"kdtree_build".as_ptr() as *const i8,
                        c"kdtree.c".as_ptr() as *const i8,
                        159,
                        c"tree->node_data != NULL".as_ptr() as *const i8,
                    )
                }
            } else {
                {
                    let _ = 0;
                }
            };
        }

        /// reset control values
        unsafe {
            (*tree).next_node = 0 as u64
        };
        {
            i = 0 as u64;
            '__b0: loop {
                if !(i < count) {
                    break '__b0;
                }
                '__c0: loop {
                    unsafe { (*unsafe { (*tree).points.add(i as usize) }).idx = i };
                    unsafe {
                        (*unsafe { (*tree).points.add(i as usize) }).x =
                            unsafe { *x.add(i as usize) }
                    };
                    unsafe {
                        (*unsafe { (*tree).points.add(i as usize) }).y =
                            unsafe { *y.add(i as usize) }
                    };
                    unsafe {
                        (*unsafe { (*tree).points.add(i as usize) }).z =
                            unsafe { *z.add(i as usize) }
                    };
                    break '__c0;
                }
                {
                    let __p = &mut i;
                    let __t = *__p;
                    *__p = (*__p).wrapping_add(1);
                    __t
                };
            }
        }

        /// build tree and store ptr to root node
        unsafe {
            (*tree).root = _build_kdtree(0 as u64, count.wrapping_sub(1 as u64), 0 as u64, tree)
        };
    }
}

#[inline]
/// determine if a node is a leaf node
extern "C" fn _is_leaf_node(node: &TreeNode) -> i32 {
    return ((*node).left as *mut () == 0 as *mut () && (*node).right as *mut () == 0 as *mut ())
        as i32;
}

#[inline]
/// resets and iterator so its memory can be reused
extern "C" fn _iterator_reset(iter: *mut KdtreeIterator) -> () {
    if !(iter as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"_iterator_reset".as_ptr() as *const i8,
                c"kdtree.c".as_ptr() as *const i8,
                463,
                c"iter != NULL".as_ptr() as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    unsafe { (*iter).size = 0 as u64 };
    unsafe { (*iter).current = 0 as u64 };
}

#[inline]
/// allocate and initialise a new iterator object
extern "C" fn _iterator_new() -> *mut KdtreeIterator {
    let iter: *mut KdtreeIterator =
        unsafe { malloc(core::mem::size_of::<KdtreeIterator>() as u64) } as *mut KdtreeIterator;
    if !(iter as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"_iterator_new".as_ptr() as *const i8,
                c"kdtree.c".as_ptr() as *const i8,
                450,
                c"iter != NULL".as_ptr() as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    unsafe { (*iter).current = 0 as u64 };
    unsafe { (*iter).size = 0 as u64 };
    unsafe { (*iter).capacity = 50 as u64 };
    unsafe {
        (*iter).data = unsafe {
            malloc((core::mem::size_of::<u64>() as u64).wrapping_mul(unsafe { (*iter).capacity }))
        } as *mut u64
    };
    if !(unsafe { (*iter).data } as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"_iterator_new".as_ptr() as *const i8,
                c"kdtree.c".as_ptr() as *const i8,
                456,
                c"iter->data != NULL".as_ptr() as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    return iter;
}

#[inline]
/// returns true if point is within search space
extern "C" fn _point_in_search_space(point: &DataPoint, search_space: &Space) -> i32 {
    return ((*point).x as f64 <= (*search_space).dim[DIM_X as usize].max
        && (*point).x as f64 >= (*search_space).dim[DIM_X as usize].min
        && (*point).y as f64 <= (*search_space).dim[DIM_Y as usize].max
        && (*point).y as f64 >= (*search_space).dim[DIM_Y as usize].min
        && (*point).z as f64 <= (*search_space).dim[DIM_Z as usize].max
        && (*point).z as f64 >= (*search_space).dim[DIM_Z as usize].min) as i32;
}

#[inline]
/// add a new value into the iterator. Resize memory if full
#[allow(unused_doc_comments)]
extern "C" fn _iterator_push(iter: &mut KdtreeIterator, value: u64) -> () {
    if (*iter).size == (*iter).capacity {
        /// full. need to grow capacity
        if !(2 as f64 > 1.0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(
                    c"_iterator_push".as_ptr() as *const i8,
                    c"kdtree.c".as_ptr() as *const i8,
                    471,
                    c"KDTREE_ITERATOR_GROWTH_RATIO > 1.0".as_ptr() as *const i8,
                )
            }
        } else {
            {
                let _ = 0;
            }
        };
        (*iter).capacity = (*iter).capacity.wrapping_mul(2 as u64);
        (*iter).data = unsafe {
            realloc(
                (*iter).data as *mut (),
                (core::mem::size_of::<u64>() as u64).wrapping_mul((*iter).capacity),
            )
        } as *mut u64;
    }
    unsafe {
        *(*iter).data.add({
            let __p = &mut (*iter).size;
            let __t = *__p;
            *__p = (*__p).wrapping_add(1);
            __t
        } as usize) = value
    };
}

#[inline]
/// returns true if search space and domain
///
///It is easier to determine if two cubes are completely separate, so
///we do just that and negate the return value.
extern "C" fn _search_area_intersects(search_space: &Space, domain: &Space) -> i32 {
    return !((*search_space).dim[DIM_X as usize].min as f64 > (*domain).dim[DIM_X as usize].max
        || ((*search_space).dim[DIM_X as usize].max as f64) < (*domain).dim[DIM_X as usize].min
        || (*search_space).dim[DIM_Y as usize].min as f64 > (*domain).dim[DIM_Y as usize].max
        || ((*search_space).dim[DIM_Y as usize].max as f64) < (*domain).dim[DIM_Y as usize].min
        || (*search_space).dim[DIM_Z as usize].min as f64 > (*domain).dim[DIM_Z as usize].max
        || ((*search_space).dim[DIM_Z as usize].max as f64) < (*domain).dim[DIM_Z as usize].min)
        as i32 as i32;
}

#[inline]
/// returns true if domain is completely enclosed within search space
extern "C" fn _completely_enclosed(search_space: &Space, domain: &Space) -> i32 {
    return ((*domain).dim[DIM_X as usize].min as f64 <= (*search_space).dim[DIM_X as usize].max
        && (*domain).dim[DIM_X as usize].min as f64 >= (*search_space).dim[DIM_X as usize].min
        && (*domain).dim[DIM_X as usize].max as f64 <= (*search_space).dim[DIM_X as usize].max
        && (*domain).dim[DIM_X as usize].max as f64 >= (*search_space).dim[DIM_X as usize].min
        && (*domain).dim[DIM_Y as usize].min as f64 <= (*search_space).dim[DIM_Y as usize].max
        && (*domain).dim[DIM_Y as usize].min as f64 >= (*search_space).dim[DIM_Y as usize].min
        && (*domain).dim[DIM_Y as usize].max as f64 <= (*search_space).dim[DIM_Y as usize].max
        && (*domain).dim[DIM_Y as usize].max as f64 >= (*search_space).dim[DIM_Y as usize].min
        && (*domain).dim[DIM_Z as usize].min as f64 <= (*search_space).dim[DIM_Z as usize].max
        && (*domain).dim[DIM_Z as usize].min as f64 >= (*search_space).dim[DIM_Z as usize].min
        && (*domain).dim[DIM_Z as usize].max as f64 <= (*search_space).dim[DIM_Z as usize].max
        && (*domain).dim[DIM_Z as usize].max as f64 >= (*search_space).dim[DIM_Z as usize].min)
        as i32;
}

/// add all leaf nodes under a branch to the iterator
extern "C" fn _report_all_leaves(
    tree: *const Kdtree,
    node: *const TreeNode,
    iter: *mut KdtreeIterator,
) -> () {
    if _is_leaf_node(unsafe { &*node }) != 0 {
        _iterator_push(unsafe { &mut *iter }, unsafe {
            (*unsafe { (*tree).points.add(unsafe { (*node).idx } as usize) }).idx
        });
    } else {
        _report_all_leaves(tree, unsafe { (*node).left } as *const TreeNode, iter);
        _report_all_leaves(tree, unsafe { (*node).right } as *const TreeNode, iter);
    }
}

#[inline]
/// convenience function to explore a sub-domain
extern "C" fn _explore_branch(
    tree: *mut Kdtree,
    node: *mut TreeNode,
    depth: u64,
    search_space: &Space,
    domain: *const Space,
    iter: *mut KdtreeIterator,
) -> () {
    if _is_leaf_node(unsafe { &*node }) != 0 {
        if _point_in_search_space(
            unsafe { &*unsafe { unsafe { (*tree).points.add(unsafe { (*node).idx } as usize) } } },
            search_space,
        ) != 0
        {
            _iterator_push(unsafe { &mut *iter }, unsafe {
                (*unsafe { (*tree).points.add(unsafe { (*node).idx } as usize) }).idx
            });
        }
    } else if _search_area_intersects(search_space, unsafe { &*domain }) != 0 {
        if _completely_enclosed(search_space, unsafe { &*domain }) != 0 {
            _report_all_leaves(tree as *const Kdtree, node as *const TreeNode, iter);
        } else {
            _search_kdtree(
                tree,
                unsafe { &*node },
                depth.wrapping_add(1 as u64),
                search_space,
                domain,
                iter,
            );
        }
    }
}

/// Recursively search the tree for points within a search space.
///Results are appended to the iterator object.
#[allow(unused_doc_comments)]
extern "C" fn _search_kdtree(
    tree: *mut Kdtree,
    root: &TreeNode,
    depth: u64,
    search_space: &Space,
    domain: *const Space,
    iter: *mut KdtreeIterator,
) -> () {
    let axis: u64 = (depth % NDIMS as u64) as u64;
    let mut new_domain: Space = unsafe { core::mem::zeroed() };

    /// initialise boundaries for new domain
    unsafe {
        __builtin___memcpy_chk(
            &raw mut new_domain as *mut (),
            domain as *const (),
            core::mem::size_of::<Space>() as u64,
            unsafe { __builtin_object_size(&raw mut new_domain as *const (), 0) },
        )
    };

    /// explore left branch
    (new_domain.dim[axis as usize].max = (*root).split);
    _explore_branch(
        tree,
        (*root).left,
        depth,
        search_space,
        &raw mut new_domain as *const Space,
        iter,
    );

    /// explore right branch
    (new_domain.dim[axis as usize].max = unsafe { (*domain).dim[axis as usize].max } as f64);

    /// reset
    (new_domain.dim[axis as usize].min = (*root).split);
    _explore_branch(
        tree,
        (*root).right,
        depth,
        search_space,
        &raw mut new_domain as *const Space,
        iter,
    );
}

/// search tree for points that fall within the 3d box defined by
///x_min, x_max, y_min, y_max, z_min, z_max.
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn kdtree_search_space(
    tree: *mut Kdtree,
    iter_ptr: &mut *mut KdtreeIterator,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
) -> () {
    let mut iter: *mut KdtreeIterator = *iter_ptr;
    let mut search_space: Space = unsafe { core::mem::zeroed() };
    let mut domain: Space = unsafe { core::mem::zeroed() };

    /// sanity checks
    if !(tree as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"kdtree_search_space".as_ptr() as *const i8,
                c"kdtree.c".as_ptr() as *const i8,
                203,
                c"tree != NULL".as_ptr() as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };

    /// The tree should have at least one point
    if !(unsafe { (*tree).root } as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"kdtree_search_space".as_ptr() as *const i8,
                c"kdtree.c".as_ptr() as *const i8,
                206,
                c"tree->root != NULL".as_ptr() as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if ((_is_leaf_node(unsafe { &*unsafe { (*tree).root } }) == 0) as i32 == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"kdtree_search_space".as_ptr() as *const i8,
                c"kdtree.c".as_ptr() as *const i8,
                207,
                c"!_is_leaf_node(tree->root)".as_ptr() as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if iter as *mut () != 0 as *mut () {
        _iterator_reset(iter);
    } else {
        iter = _iterator_new();
        *iter_ptr = iter;
    }

    /// define the search space
    (search_space.dim[DIM_X as usize].min = x_min);
    search_space.dim[DIM_X as usize].max = x_max;
    search_space.dim[DIM_Y as usize].min = y_min;
    search_space.dim[DIM_Y as usize].max = y_max;
    search_space.dim[DIM_Z as usize].min = z_min;
    search_space.dim[DIM_Z as usize].max = z_max;

    /// set initial domain to infinite space
    (domain.dim[DIM_X as usize].min = -1.7976931348623157e308);
    domain.dim[DIM_X as usize].max = 1.7976931348623157e308;
    domain.dim[DIM_Y as usize].min = -1.7976931348623157e308;
    domain.dim[DIM_Y as usize].max = 1.7976931348623157e308;
    domain.dim[DIM_Z as usize].min = -1.7976931348623157e308;
    domain.dim[DIM_Z as usize].max = 1.7976931348623157e308;

    /// search tree
    _search_kdtree(
        tree,
        unsafe { &*unsafe { (*tree).root } },
        0 as u64,
        &search_space,
        &raw mut domain as *const Space,
        iter,
    );
}

/// search tree for points that fall within the 3d cube defined by
///x, y, z, apothem where apothem is the distance from the point
///to each side of the cube.
pub(crate) extern "C" fn kdtree_search(
    tree: *mut Kdtree,
    iter_ptr: *mut *mut KdtreeIterator,
    x: f64,
    y: f64,
    z: f64,
    apothem: f64,
) -> () {
    unsafe {
        if !(apothem >= 0.0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(
                    c"kdtree_search".as_ptr() as *const i8,
                    c"kdtree.c".as_ptr() as *const i8,
                    184,
                    c"apothem >= 0.0".as_ptr() as *const i8,
                )
            }
        } else {
            {
                let _ = 0;
            }
        };
        kdtree_search_space(
            tree,
            unsafe { &mut *iter_ptr },
            x - apothem,
            x + apothem,
            y - apothem,
            y + apothem,
            z - apothem,
            z + apothem,
        );
    }
}

/// returns the next entry in the iteration, or KDTREE_END if the
///end is reached
pub(crate) extern "C" fn kdtree_iterator_get_next(iter: &mut KdtreeIterator) -> u64 {
    if (*iter).current == (*iter).size {
        return 18446744073709551615u64;
    }
    return unsafe {
        *(*iter).data.add({
            let __p = &mut (*iter).current;
            let __t = *__p;
            *__p = (*__p).wrapping_add(1);
            __t
        } as usize)
    };
}

/// rewind the iterator
pub(crate) extern "C" fn kdtree_iterator_rewind(iter: *mut KdtreeIterator) -> () {
    if !(iter as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"kdtree_iterator_rewind".as_ptr() as *const i8,
                c"kdtree.c".as_ptr() as *const i8,
                256,
                c"iter != NULL".as_ptr() as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    unsafe { (*iter).current = 0 as u64 };
}

/// for sorting iterators
extern "C" fn cmp_size_t(a1: *const (), a2: *const ()) -> i32 {
    let a1: *const u64 = a1 as *const u64;
    let a2: *const u64 = a2 as *const u64;
    return if unsafe { *a1 } as u64 > unsafe { *a2 } {
        1
    } else {
        if (unsafe { *a1 } as u64) < unsafe { *a2 } {
            -1
        } else {
            0
        }
    };
}

/// sort entries within the iterator
pub(crate) extern "C" fn kdtree_iterator_sort(iter: &KdtreeIterator) -> () {
    unsafe {
        qsort(
            (*iter).data as *mut (),
            (*iter).size,
            core::mem::size_of::<u64>() as u64,
            cmp_size_t,
        )
    };
}

/// deallocate memory associated with an iterator
pub(crate) extern "C" fn kdtree_iterator_delete(iter_ptr: &mut *mut KdtreeIterator) -> () {
    let iter: *mut KdtreeIterator = *iter_ptr;
    if iter as *mut () == 0 as *mut () {
        return;
    }
    unsafe { free(unsafe { (*iter).data } as *mut ()) };
    unsafe { free(iter as *mut ()) };
    *iter_ptr = 0 as *mut () as *mut KdtreeIterator;
}
