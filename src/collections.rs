//! # 标准库中的collections
//! - Sequences: String, Vector, VecDequeue, LinkedList
//! - Maps: HashMap, BTreeMap
//! - Sets: HashSet, BTreeSet
//! - Misc: BinaryHeap
//! 
pub mod string {
    //! # String type
    pub mod string_create {
        //! # 字符串创建
        pub fn string_literal() {
            //! # 字符串字面量
            //! 使用字面量创建的字符串类型为 string slice
            let a = "hello, world";
            let b : &str = "hello, world";
        }

        use std::string::String;
        pub fn string_constructor() {
            //! # 字符串创建函数
            let some_string = String::from("hello, world");
        }
    }

    pub mod string_access {
        //! # 字符串访问
        pub fn string_index() {
            //! # 下标访问
        }

        pub fn string_interate() {
            //! # 迭代访问
        }
    }

    pub mod string_concat {
        //! # 字符串连接
        pub fn string_push() {
            //! # 使用可变字符串连接
            let mut base = String::new();
            let a = "hello";
            let b = "rust";
            base.push_str(a);
            base.push_str(b);
        }

        pub fn string_format() {
            //! # 使用格式化宏
            let a = "hello";
            let comma = ',';
            let b = "rust";
            let c = 2333;
            let some_str = format!("{}{}{}{}",a,comma,b,c);
            println!("result: {}", some_str);
            // a and b is still available
            println!("a: {}",a);
            println!("b: {}",b);
        }


        pub fn string_add_not_available() {
            //! # 字符串不能使用加号连接
            //! ```
            //! let a = "hello";
            //! let b = "rust";
            //! let c = a+b;
            //! ```
        }
    }

}
pub mod vector {
    //! # Vector type
    pub mod vector_create {
        //! # vector创建
        pub fn vector_constructor() {
            //! # vector new函数
        }
        pub fn vector_macro() {
            //! # 使用vect！和array创建
            let a = vec![1,2,3,4];
        }
    }

    pub mod vector_reading {
        //! # vector访问
        pub fn vector_index() {
            //! # vector下标
        }

        pub fn vector_get() {
            //! # get函数
            let v  = vec![1,2,3,4];
            v.get(0);
        }

        pub fn vector_interate() {
            //! vector 迭代
        }
    }
}