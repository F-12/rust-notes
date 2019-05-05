pub mod scalar {
    //! # Scalar Types
    //! - signed integers: i8, i16, i32, i64, isize
    //! - unsigend integers: u8, u16, u32, u64, usize
    //! - float: f32, f64
    //! - char: unicode harator
    //! - bool: true, false
    //! - unit type: ()
    pub fn scalar_literal_definition() {
        //! # Literal Definition
        let i: i8 = 1;
        let i: i8 = 1;
        let i: i16 = 1;
        let i: i32 = 1;
        let i: i64 = 1;
        let i: i128 = 1;

        let u: u8 = 1;
        let u: u16 = 1;
        let u: u32 = 1;
        let u: u64 = 1;
        let u: u128 = 1;

        let f: f32 = 1.0;
        let f: f64 = 1.0;

        let c: char = 'a';
        let c: char = '你';
        let c: char = 'α';

        let b_true: bool = true;
        let b_false: bool = false;

        let unit = ();
    }
    pub fn integer_range() {
        //! # Integer Range
        //! ## integer overflow
        //! 超出integer范围的字面量值
        //! ```
        //! let i8_max_overflow: i8 = 128;
        //! ```
        //! ## 默认整数字面量的数据类型
        //!
        let i8_max: i8 = i8::max_value();
        let i8_min: i8 = i8::min_value();

        let u8_max: u8 = u8::max_value();
        let u8_min: u8 = u8::min_value();
    }

    pub fn integer_arithmetic() {
        //! # 整形数运算
    }

    pub fn float_arithmetic() {
        //! # 浮点数运算
        //! - 同类型浮点数
        //! - f32和f64混合运算
        //! - 整型和浮点数混合运算
    }

}

pub mod tuple {
    //! # Tuple Type
    pub fn tuple_literal() {
        //! # Tuple Literal
        let tup = (1, 1.0, 'a', true);
        let tup = (1, 1, 1);
    }

    pub fn tuple_nested() {
        //! # Nested Tuple
        let tup = (1, (1, 1), (true, false));
    }

    pub fn tuple_members() {
        //! # Access Tuple Members
        let tup = (1, 1.0, 'a', true);
        let first = tup.0;
        let last = tup.3;
    }
}
pub mod array {
    //! # Array Type
    //! Array和Tuple区别在于元素类型是否可以不一样。
    
    pub fn array_literal_init() {
    //! # 使用字面量初始化
    let arr = [1,2,3,4,5];
    }

    pub fn array_fill_init() {
        //! # 使用一样元素填充
        let arr = [0;100];
    }

    pub fn array_index() {
        //! 使用下标访问元素
    }
}

pub mod struct_type {
    //! # Struct Type
    //! - struct定义
    //! - struct初始化
    //! - struct成员访问
    //! - struct解构
    //! - unit struct
    //! - tuple struct
    //! - tuple struct解构
}

pub mod enum_type {
    //! # Enum Type
}

pub mod casting {
    //! # 数据类型转换
    //! - no implicit type conversion
    //! - as 关键字
    //! - C语言转换惯例
    //! - C语言为定义的转换行为
}

pub mod conversion {
    //! # 类型转换
    //! - From trait
    //! - Into trait
}
