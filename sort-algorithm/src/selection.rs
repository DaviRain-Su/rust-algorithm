use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct SelectionSort<T: PartialOrd + Debug + Display>(Vec<T>);

impl<T> SelectionSort<T>
    where
        T: PartialOrd + Debug + Display,
{
    pub fn new(vec: Vec<T>) -> Self {
        Self(vec)
    }

    pub fn get_vec(self) -> Vec<T> {
        self.0
    }

    // 按升序排序
    pub fn sort(&mut self)
    {
        let length = self.0.len(); // 数组长度

        for i in 0..length {
            // 将array[i]和array[i+1..length]中最小的元素交换
            let mut min = i; // 最小元素的索引
            for j in (i + 1)..length {
                if self.0[j] < self.0[min] {
                    min = j;
                }
            }
            let left = self.0[i..i+1].as_mut_ptr() as *mut [T; 1];
            let right = self.0[min..min+1].as_mut_ptr() as *mut [T; 1];
            unsafe {
                use std::ptr;
                ptr::swap(left, right);
            }
        }
    }
}



#[test]
fn test_selection_sort() {
    let mut  v = vec![5, 4, 3, 2, 1];
    assert_eq!(v, vec![5, 4, 3, 2, 1]);
    sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
}