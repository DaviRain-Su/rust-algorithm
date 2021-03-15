//! Queue 队列的实现
//!

#[cfg(test)]
mod tests;

pub struct Queue {
    older: Vec<char>,   // 旧元素，最老的在最后
    younger: Vec<char>, // 新元素，最新的在最后
}


impl Queue {
    pub fn new() -> Self {
        Self {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    /// 把一个字符推导队列的后端
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    /// 从队列前端取出一个字符，如果可以，取出字符返回Some(c),
    /// 否则，如果队列为空， 返回None
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            // 把younger中的元素转移到older中，
            // 并保持对外承诺的顺序
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        // 到这里older肯定有元素， vec中的pop方法，已经返回Option, 这就可以了
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}


