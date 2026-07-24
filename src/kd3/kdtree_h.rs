use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct DataPoint {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    pub(crate) idx: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TreeNode {
    pub(crate) left: *mut TreeNode,
    pub(crate) right: *mut TreeNode,
    pub(crate) split: f64,
    pub(crate) idx: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Boundaries {
    pub(crate) min: f64,
    pub(crate) max: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Space {
    pub(crate) dim: [Boundaries; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Kdtree {
    pub(crate) count: u64,
    pub(crate) max_nodes: u64,
    pub(crate) next_node: u64,
    pub(crate) points: *mut DataPoint,
    pub(crate) node_data: *mut TreeNode,
    pub(crate) root: *mut TreeNode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct KdtreeIterator {
    pub(crate) data: *mut u64,
    pub(crate) capacity: u64,
    pub(crate) size: u64,
    pub(crate) current: u64,
}
