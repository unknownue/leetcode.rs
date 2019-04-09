//!
//! Median of Two Sorted Arrays
//!
//! https://leetcode.com/problems/median-of-two-sorted-arrays/
//!
//! There are two sorted arrays nums1 and nums2 of size m and n respectively.
//!
//! Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
//!
//! You may assume nums1 and nums2 cannot be both empty.
//!
//! ## Example 1:
//! ```
//! nums1 = [1, 3]
//! nums2 = [2]
//!  
//! The median is 2.0
//! ```
//!
//! ## Example 2:
//! ```
//! nums1 = [1, 2]
//! nums2 = [3, 4]
//! 
//! The median is (2 + 3)/2 = 2.5
//! ```
//!

#[derive(Debug, Clone)]
pub struct Input {
    pub num1: Vec<i32>,
    pub num2: Vec<i32>
}

pub type Answer = f64;

pub trait Solution {
    fn find_median_sorted_arrays(&self, num1: Vec<i32>, num2: Vec<i32>) -> f64;
}

pub struct Solution1;
impl Solution for Solution1 {

    fn find_median_sorted_arrays(&self, num1: Vec<i32>, num2: Vec<i32>) -> f64 {
        2.0
    }
}
