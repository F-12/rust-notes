//! # Rust Notes
//! 这个仓库用来记录rust学习过程中的代码笔记。
//! 
//! ## 组织结构
//! - 一个Topic一个module
//! 

pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
