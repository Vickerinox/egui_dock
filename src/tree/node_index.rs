﻿/// Wrapper around indices to the collection of nodes inside a [`Tree`](crate::Tree).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct NodeIndex(pub usize);

impl From<usize> for NodeIndex {
    #[inline(always)]
    fn from(index: usize) -> Self {
        NodeIndex(index)
    }
}

impl NodeIndex {
    /// Returns the index of the root node.
    #[inline(always)]
    pub const fn root() -> Self {
        Self(0)
    }

    /// Returns the index of the node to the left of the current one.
    #[inline(always)]
    pub const fn left(self) -> Self {
        Self(self.0 * 2 + 1)
    }

    /// Returns the index of the node to the right of the current one.
    #[inline(always)]
    pub const fn right(self) -> Self {
        Self(self.0 * 2 + 2)
    }

    /// Returns the index of the parent node or `None` if current node is the root.
    #[inline]
    pub const fn parent(self) -> Option<Self> {
        if self.0 > 0 {
            Some(Self((self.0 - 1) / 2))
        } else {
            None
        }
    }

    /// Returns the number of nodes leading from the root to the current node, including `self`.
    #[inline(always)]
    pub const fn level(self) -> usize {
        (usize::BITS - (self.0 + 1).leading_zeros()) as usize
    }

    /// Returns true if current node is the left node of its parent, false otherwise.
    #[inline(always)]
    pub const fn is_left(self) -> bool {
        self.0 % 2 != 0
    }

    /// Returns true if current node is the right node of its parent, false otherwise.
    #[inline(always)]
    pub const fn is_right(self) -> bool {
        self.0 % 2 == 0
    }

    #[inline]
    pub(super) const fn children_at(self, level: usize) -> std::ops::Range<usize> {
        let base = 1 << level;
        let s = (self.0 + 1) * base - 1;
        let e = (self.0 + 2) * base - 1;
        s..e
    }

    #[inline]
    pub(super) const fn children_left(self, level: usize) -> std::ops::Range<usize> {
        let base = 1 << level;
        let s = (self.0 + 1) * base - 1;
        let e = (self.0 + 1) * base + base / 2 - 1;
        s..e
    }

    #[inline]
    pub(super) const fn children_right(self, level: usize) -> std::ops::Range<usize> {
        let base = 1 << level;
        let s = (self.0 + 1) * base + base / 2 - 1;
        let e = (self.0 + 2) * base - 1;
        s..e
    }
}
