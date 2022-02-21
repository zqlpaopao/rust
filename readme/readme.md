# 1、安装rust

```
curl https://sh.rustup.rs -sSf | sh

输入1

Current installation options:


   default host triple: aarch64-apple-darwin
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1
```

[其他方法](https://forge.rust-lang.org/infra/other-installation-methods.html)

验证

```
Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source $HOME/.cargo/env
```

```
执行
source $HOME/.cargo/env

rustc -version

rustc --version        
rustc 1.58.1 (db9d1b20b 2022-01-20)
```

卸载

```
rustup self uninstall
```

# 2、 rustup安装

rustup 是rust官方的版本管理工具。应当作为安装 Rust 的首选。

项目主页是: https://github.com/rust-lang-nursery/rustup.rs

卸载

```
rustup self uninstall
```

# 3、安装vscode编译器

下载 VScode
请打开官网 https://code.visualstudio.com/ 下载编辑器。

依赖
如本章第一节所述，准备好 racer，rust 源代码，rustfmt，rls 这四样东西，并且配置好相应的环境变量，此不赘述。

安装 Rust 扩展 Rust
打开 VScode 编辑器；
按 Ctrl + p 打开命令面板；
在编辑器中上部浮现出的输入框中，输入 ext install vscode-rust，会自动搜索可用的插件，搜索出来后，点击进行安装；使用VScode打开任意一个.rs文件，插件首次启动会自动引导用户完成配置。
注:推荐使用RLS模式，即使用Rust Langular Server提供各项功能支持

![image-20220214112618639](/Users/zhangqiuli24/Desktop/rust/readme/readme.assets/image-20220214112618639.png)

![image-20220214112931446](/Users/zhangqiuli24/Desktop/rust/readme/readme.assets/image-20220214112931446.png)

![image-20220214113058105](/Users/zhangqiuli24/Library/Application Support/typora-user-images/image-20220214113058105.png)

安装完毕

```
$ cargo new code
     Created binary (application) `code` package
$ cargo build   
   Compiling code v0.1.0 (/Users/xxx/Desktop/rust/code)
    Finished dev [unoptimized + debuginfo] target(s) in 1.15s
$ cargo run  
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/code`
Hello, world!
```

# 4、Cargo 教程

### Cargo 是什么

Cargo 是 Rust 的构建系统和包管理器。

Rust 开发者常用 Cargo 来管理 Rust 工程和获取工程所依赖的库。在上个教程中我们曾使用 cargo new greeting 命令创建了一个名为 greeting 的工程，Cargo 新建了一个名为 greeting 的文件夹并在里面部署了一个 Rust 工程最典型的文件结构。这个 greeting 文件夹就是工程本身。

### Cargo 功能

Cargo 除了创建工程以外还具备构建（build）工程、运行（run）工程等一系列功能，构建和运行分别对应以下命令：

```
cargo build 
cargo run 
```

Cargo 还具有获取包、打包、高级构建等功能，详细使用方法参见 Cargo 命令。



# 5、运行代码方式

## cargo

不用生成二进制文件的

```不用生成二进制文件的
cargo build
cargo build —release # 这个属于优化编译
cargo run 可直接运行
cargo run # 编译和运行合在一起
```

## rustc

生成二进制文件的

```
rustc helloworld.rs
rustc helloworld.rs -O # 也可以选择优化编译
```

# 使用cargo创建项目

* 创建项目 hellorust

```
ps: cargo new hellorust —bin
```



* 查看目录结构

```
ps: tree # win10 powershell 自带有 tree 查看文件目录结构的功能  
└─hellorust  
——└─src
```



这里显示的目录结构，在hellorust目录下有 src 文件夹和 Cargo.toml 文件，同时这个目录会初始化为 git 项目

* 查看Cargo.toml文件

```
ps: cat Cargo.toml  
[package]  
name = “hellorust”  
version = “0.1.”  
authors = [“YourName “]  
[dependencies]
```



* 编辑src目录下的main.rs文件

```
ps: subl ./src/main.rs
```



cargo 创建的项目，在src目录下会有一个初始化的main.rs文件，内容为：

```
fn main() {
println!("Hello, world!");
}
```



现在我们编辑这个文件，改为：

```
fn main() {
let rust = "Rust";
println!("Hello, {}!", rust);
}
```

这里的 `let rust = "Rust"` 是把 rust 变量绑定为 “Rust” ，  
`println!("Hello, {}!", rust);`里把 rust 变量的值代入到`"Hello, {}!"`中的`{}`。

* 编译和运行

```
ps: cargo build  
ps: cargo build —release # 这个属于优化编译  
ps: ./target/debug/hellorust.exe  
ps: ./target/release/hellorust.exe # 如果前面是优化编译，则这样运行  
ps: cargo run # 编译和运行合在一起  
ps: cargo run —release # 同上，区别是是优化编译的
```



# 6、数据类型

### 整数型（Integer）

整数型简称整型，按照比特位长度和有无符号分为一下种类：

| 位长度     | 有符号   | 无符号   |
| ------- | ----- | ----- |
| 8-bit   | i8    | u8    |
| 16-bit  | i16   | u16   |
| 32-bit  | i32   | u32   |
| 64-bit  | i64   | u64   |
| 128-bit | i128  | u128  |
| arch    | isize | usize |

isize 和 usize 两种整数类型是用来衡量数据大小的，它们的位长度取决于所运行的目标台，如果是 32 位架构的处理器将使用 32 位位长度整型。

整数的表述方法有以下几种：

| 进制            | 例           |
| ------------- | ----------- |
| 十进制           | 98_222      |
| 十六进制          | 0xff        |
| 八进制           | 0o77        |
| 二进制           | 0b1111_0000 |
| 字节(只能表示 u8 型) | b'A'        |

很显然，有的整数中间存在一个下划线，这种设计可以让人们在输入一个很大的数字时更容易判断数字的值大概是多少。

### 浮点数型（Floating-Point）

Rust 与其它语言一样支持 32 位浮点数（f32）和 64 位浮点数（f64）。默认情况下，64.0 将表示 64 位浮点数，因为现代计算机处理器对两种浮点数计算的速度几乎相同，但 64 位浮点数精度更高。

#### 实例

```
fn main() {  
 let x = 2.0; // f64  
 let y: f32 = 3.0; // f32  
}
```

### 布尔型

布尔型用 bool 表示，值只能为 true 或 false。

### 字符型

字符型用 char 表示。

Rust的 char 类型大小为 4 个字节，代表 Unicode标量值，这意味着它可以支持中文，日文和韩文字符等非英文字符甚至表情符号和零宽度空格在 Rust 中都是有效的 char 值。

<mark>**注意** 由于中文文字编码有两种（GBK 和 UTF-8），所以编程中使用中文字符串有可能导致乱码的出现，这是因为源程序与命令行的文字编码不一致，所以在 Rust 中字符串和字符都必须使用 UTF-8 编码，否则编译器会报错。</mark>

```
fn main() {

// string types
let str = "Hello, world!";
let  strings  = str.to_string();//to_string转换为队字符串
// let  mut strings  = str.to_string();//加了mut 后 warning: variable does not need to be mutable 因为String就是可变的，不需要加mut
println!("{}",str);
println!("{}",strings);
}
```



### 字符串类型

最底层的是不定长类型`str`，更常用的是字符串切片`&str`和堆分配字符串`String`

<mark>其中字符串切片是静态分配的，有固定的大小，并且不可变，而堆分配字符串是可变的。</mark>

```
fn main() {



// tuples
let tuple: (i32, &str) = (50, "hello");
println!("{}",tuple.0);
println!("{}",tuple.1);
let (fifty, _) = tuple;
println!("{}",fifty);
let hello = tuple.1;
println!("{}",hello);

let mut to_str = hello.to_string();

to_str.pop();//尾部推出一个元素
println!("{}",to_str);
to_str.clear();//清空元素
to_str.push_str("string");

println!("{}",to_str);
}
```



### 元组

具有固定大小的有序列表，每个元素都有自己的类型，通过解构或者索引来获得每个元素的值。

```

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
// tup.0 等于 500
// tup.1 等于 6.4
// tup.2 等于 1
println!("{},{},{}",tup.0,tup.1,tup.2);
let (x,y,z) = tup;
// y 等于 6.4
println!("{},{},{}",x,y,z)
}
```



```
500,6.4,1
500,6.4,1
```

### 数组

具有固定大小，并且元素都是同种类型，可表示为`[T; N]`。

```
fn main() {
   let a = [1,2,3,4,5];//a 是一个长度为 5 的整型数组
   println!("{}",a[0]);//注意此处和元组的访问方式不一样，元组是.

   let b = ["aaa","bbb","ccc"];//b 是一个长度为 3 的字符串数组
   println!("{}",b[1]);

   let c:[i32; 5] = [1,2,3,4,5];//c 是一个长度为 5 的 i32 数组
   println!("{}",c[2]);

   let d:[i32;5] = [3;5];//等同于 let d = [3, 3, 3, 3, 3];
   println!("{}",d[3]);

   let first = a[0];
   let second = a[1];

   println!("{}",first);
   println!("{}",second);

//    a[1]= 123;//错误，不可变元素
   let mut  a = [1,2,3];
   a[0]= 4;//正确
   println!("{}",a[0])

}
```



```
1
bbb
3
3
1
2
4
```

### 切片

引用一个数组的部分数据并且不需要拷贝，可表示为`&[T]`。

```
fn main() {


let a = [0, 1, 2, 3, 4];
let mut middle = &a[1..4];
// middle[2] = 10; middle` is a `&` reference, so the data it refers to cannot be written 切片是引用类型，不能重写原来的
println!("{}",middle[2]);

// let mut ten_zeros: [i64; 10] = [0; 10];//数组是不可变的，如果后边没有改变行为最好不要用mut，warning: variable does not need to be mutable
let  mut ten_zeros: [i64; 10] = [0; 10];//数组是不可变的，不能用mut

println!("{}",ten_zeros[0]);
ten_zeros[0] = 9;
println!("{}",ten_zeros[0]);
}
```

```
3
0
9
```

### 指针和引用

* 解除引用使用  `*`
* 构析使用 `&`, `ref`, 和 `ref mut`

```
fn main() {


// raw pointers
let x = 5;
let raw = &x as *const i32;

let x1 = "hello";
let raw1 = &x1;
println!("{}",x1);
println!("{}",*raw1);

let points_at = unsafe { *raw };
let points_at1 =  *raw1 ;
println!("{}",points_at);
println!("{}",points_at1)
// // functions
// fn foo(x: i32) -> i32 { x }
// let bar: fn(i32) -> i32 = foo;

}
```



```
hello
hello
5
hello
```



### 函数

：具有函数类型的变量实质上是一个函数指针

```
// functions
fn foo(x: i32) -> i32 { x }
let bar: fn(i32) -> i32 = foo;
println!("{}",bar(5));
```

```
5
```



### 元类型

：即`()`，其唯一的值也是`()`

### <mark>总结</mark>

有几点是需要特别注意的：

* 数值类型可以使用`_`分隔符来增加可读性。
* Rust还支持单字节字符`b'H'`以及单字节字符串`b"Hello"`，仅限制于ASCII字符。  
  此外，还可以使用`r#"..."#`标记来表示原始字符串，不需要对特殊字符进行转义。
* 使用`&`符号将`String`类型转换成`&str`类型很廉价，  
  但是使用`to_string()`方法将`&str`转换到`String`类型涉及到分配内存，  
  除非很有必要否则不要这么做。
* 数组的长度是不可变的，动态的数组称为Vec (vector)，可以使用宏`vec!`创建。
* 元组可以使用`==`和`!=`运算符来判断是否相同。
* 不多于32个元素的数组和不多于12个元素的元组在值传递时是自动复制的。
* Rust不提供原生类型之间的隐式转换，只能使用`as`关键字显式转换。
* 可以使用`type`关键字定义某个类型的别名，并且应该采用驼峰命名法。

```
// explicit conversion
let decimal = 65.4321_f32;
let integer = decimal as u8;
let character = integer as char;
// type aliases
type NanoSecond = u64;
type Point = (u8, u8);
```



### 数组、动态数组、字符串

#### 数组 array

Rust 使用数组存储相同类型的数据集。  
`[T; N]`表示一个拥有 T 类型，N 个元素的数组。数组的大小是固定。

**例子：**

```
fn main() {
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;
   assert_eq!([1, 2], &array[1..]);
    // This loop prints: 0 1 2
    for x in &array {
    println!("{} ", x);
    }
}

0
1
2
```



### 动态数组 Vec

动态数组是一种基于堆内存申请的连续动态数据类型，拥有 O(1) 时间复杂度的索引、压入（push）、弹出（pop)。

**例子：**

```
fn main() {
    //创建空Vec
    let v: Vec<i32> = Vec::new();
    println!("{:#?}",v);

    //使用宏创建空Vec
    let v: Vec<i32> = vec![];
    println!("{:#?}",v);

    //创建包含5个元素的Vec
    let v = vec![1, 2, 3, 4, 5];
    println!("{}",v[1]);

    //创建十个零
    let v = vec![0; 10];
    println!("{:#?}",v[2]);

    //创建可变的Vec，并压入元素3
    let mut v = vec![1, 2];
    v.push(3);
    println!("{:#?}",v);

    //创建拥有两个元素的Vec，并弹出一个元素
    let mut v = vec![1, 2];
    let two = v.pop();
    println!("{:#?}",two);

    //创建包含三个元素的可变Vec，并索引一个值和修改一个值
    let mut v = vec![1, 2, 3];
    let three = v[1];
    println!("{}",three);
    v[1] = v[1] + 5;
    println!("{:#?}",v);
    println!("{}",three);
}
```



```
     Running `target/debug/code`
[]
[]
2
0
[
    1,
    2,
    3,
]
Some(
    2,
)
2
[
    1,
    7,
    3,
]
2
```

### 字符串

Rust 里面有两种字符串类型。`String` 和 `str`。

#### &str

`str` 类型基本上不怎么使用，通常使用 `&str` 类型，它其实是 `[u8]` 类型的切片形式 `&[u8]`。这是一种固定大小的字符串类型。  
常见的的字符串字面值就是 `&'static str` 类型。这是一种带有 `'static` 生命周期的 &str 类型。

**例子：**

```
fn main() {
// 字符串字面值
let hello = "Hello, world!";
println!("{}",hello);

// 附带显式类型标识
let hello1: &'static str = "Hello, world1!";
println!("{}",hello1);
}
```



#### String

`String` 是一个带有的 `vec:Vec<u8>` 成员的结构体，你可以理解为 `str` 类型的动态形式。  
它们的关系相当于 `[T]` 和 `Vec<T>` 的关系。  
显然 `String` 类型也有压入和弹出。

**例子：**

```
fn main() {
    // 创建一个空的字符串
    let mut s = String::new();
    s.push('c');
    println!("{:#?}",s);

    // 从 `&str` 类型转化成 `String` 类型
    let mut hello = String::from("Hello, ");
    // 压入字符和压入字符串切片
    hello.push('w');
    hello.push_str("orld!");
    // 弹出字符。
    let mut s = String::from("foo");
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('f'));
    assert_eq!(s.pop(), None);
}

```





### static

<font color=red>`'static` 生命周期是可能的生命周期中最长的，它会在整个程序运行的时期中 存在。`<mark>'static` 生命周期也可被强制转换成一个更短的生命周期。有两种方式使变量 拥有 `'static` 生命</mark>周期，它们都把数据保存在可执行文件的只读内存区：</font>

* 使用 `static` 声明来产生常量（constant）。
* 产生一个拥有 `&'static str` 类型的 `string` 字面量。

看下面的例子，了解列举到的各个方法：

```
// 产生一个拥有 `'static` 生命周期的常量。
static NUM: i32 = 18;

// 返回一个指向 `NUM` 的引用，该引用不取 `NUM` 的 `'static` 生命周期，
// 而是被强制转换成和输入参数的一样。
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // 产生一个 `string` 字面量并打印它：
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // 当 `static_string` 离开作用域时，该引用不能再使用，不过
        // 数据仍然存在于二进制文件里面。
    }
    // println!("static_string: {}", static_string);//报错 static_string 已经离开作用域了
    
    {
        // 产生一个整型给 `coerce_static` 使用：
        let lifetime_num = 9;

        // 将对 `NUM` 的引用强制转换成 `lifetime_num` 的生命周期：
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }
    
    println!("NUM: {} stays accessible!", NUM);
}

```

# 7、结构体

- 结构体 (struct) 是一种记录类型，所包含的每个域 (field) 都有一个名称。  

- 每个结构体也都有一个名称，通常以大写字母开头，使用驼峰命名法。  

- 元组结构体 (tuple struct) 是由元组和结构体混合构成，元组结构体有名称，  

- 但是它的域没有。当元组结构体只有一个域时，称为新类型 (newtype)。  

- 没有任何域的结构体，称为类单元结构体 (unit-like struct)。  

- 结构体中的值默认是不可变的，需要给结构体加上`mut`使其可变。

```
use std::ops::Shr;

 // structs
 struct Point {
    x: i32,
    y: i32,
}

fn main() {
  
    let point = Point { x: 0, y: 10 };
    println!("{:#?},{}",point.x,point.y);

    // tuple structs 元组结构体
    struct Color(u8, u8, u8);
    let android_green = Color(0xa4, 0xc6, 0x39);
    let Color(red, green, blue) = android_green;
    println!("{}",red);
    println!("{}",green);
    println!("{}",blue);

    // A tuple struct’s constructors can be used as functions.
    struct Digit(i32);
    let v = vec![0, 1, 2];

    let d: Vec<Digit> = v.into_iter().map(Digit).collect();

    // newtype: a tuple struct with only one element
    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;
   

    // unit-like structs
    struct EmptyStruct;
    let empty = EmptyStruct;
}
```

# 8、Rust 结构体

Rust 中的结构体（Struct）与元组（Tuple）都可以将若干个类型不一定相同的数据捆绑在一起形成整体，但结构体的每个成员和其本身都有一个名字，这样访问它成员的时候就不用记住下标了。元组常用于非定义的多值传递，而结构体用于规范常用的数据结构。结构体的每个成员叫做"字段"。

## 结构体定义

这是一个结构体定义：

```
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32
}
```



注意：如果你常用 C/C++，请记住在 Rust 里 struct 语句仅用来定义，不能声明实例，结尾不需要 ; 符号，而且每个字段定义之后用 , 分隔。

## 结构体实例

Rust 很多地方受 JavaScript 影响，在实例化结构体的时候用 JSON 对象的 key: value 语法来实现定义：

## 实例

```
let runoob = Site {
    domain: String::from("www.runoob.com"),
    name: String::from("RUNOOB"),
    nation: String::from("China"),
    found: 2013
};
```



如果你不了解 JSON 对象，你可以不用管它，记住格式就可以了：

```
结构体类名 {
    字段名 : 字段值,
    ...
}
```



这样的好处是不仅使程序更加直观，还不需要按照定义的顺序来输入成员的值。

如果正在实例化的结构体有字段名称和现存变量名称一样的，可以简化书写：

## 实例

```
let domain = String::from("www.runoob.com");
let name = String::from("RUNOOB");
let runoob = Site {
    domain,  // 等同于 domain : domain,
    name,    // 等同于 name : name,
    nation: String::from("China"),
    traffic: 2013
};
```



有这样一种情况：你想要新建一个结构体的实例，其中大部分属性需要被设置成与现存的一个结构体属性一样，仅需更改其中的一两个字段的值，可以使用结构体更新语法：

```
let site = Site {
    domain: String::from("www.runoob.com"),
    name: String::from("RUNOOB"),
    ..runoob
};
```



注意：..runoob 后面不可以有逗号。这种语法不允许一成不变的复制另一个结构体实例，意思就是说至少重新设定一个字段的值才能引用其他实例的值。

### 元组结构体

有一种更简单的定义和使用结构体的方式：**元组结构体**。

元组结构体是一种形式是元组的结构体。

与元组的区别是它有名字和固定的类型格式。它存在的意义是为了处理那些需要定义类型（经常使用）又不想太复杂的简单数据：

```
struct Color(u8, u8, u8);
struct Point(f64, f64);

let black = Color(0, 0, 0);
let origin = Point(0.0, 0.0);
```



"颜色"和"点坐标"是常用的两种数据类型，但如果实例化时写个大括号再写上两个名字就为了可读性牺牲了便捷性，Rust 不会遗留这个问题。元组结构体对象的使用方式和元组一样，通过 . 和下标来进行访问：

## 实例

```
fn main() {
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);
}
```

运行结果：

black = (0, 0, 0) origin = (0, 0)

## 结构体所有权

结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。

这就是为什么本章的案例中使用了 String 类型而不使用 &str 的原因。

但这不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现。

但现在还难以说明"生命周期"概念，所以只能在后面章节说明。

### 输出结构体

调试中，完整地显示出一个结构体实例是非常有用的。但如果我们手动的书写一个格式会非常的不方便。所以 Rust 提供了一个方便地输出一整个结构体的方法：

## 实例

```
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
```



如第一行所示：一定要导入调试库 **#[derive(Debug)]** ，之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体：

```
rect1 is Rectangle { width: 30, height: 50 }
```

如果属性较多的话可以使用另一个占位符 {:#?} 。

输出结果：

```
rect1 is Rectangle {
    width: 30,
    height: 50
}
```



### 结构体方法

方法（Method）和函数（Function）类似，只不过它是用来操作结构体实例的。

如果你学习过一些面向对象的语言，那你一定很清楚函数一般放在类定义里并在函数中用 this 表示所操作的实例。

Rust 语言不是面向对象的，从它所有权机制的创新可以看出这一点。但是面向对象的珍贵思想可以在 Rust 实现。

结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字。

计算一个矩形的面积：

## 实例

```
struct Rectangle {
    width: u32,
    height: u32,
}
   
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1's area is {}", rect1.area());
}
```



输出结果：

```
rect1's area is 1500
```

请注意，在调用结构体方法的时候不需要填写 self ，这是出于对使用方便性的考虑。

一个多参数的例子：

## 实例

```
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 40, height: 20 };

    println!("{}", rect1.wider(&rect2));
}
```

运行结果：

```
false
```

这个程序计算 rect1 是否比 rect2 更宽。

---

## 结构体关联函数

之所以"结构体方法"不叫"结构体函数"是因为"函数"这个名字留给了这种函数：它在 impl 块中却没有 &self 参数。

这种函数不依赖实例，但是使用它需要声明是在哪个 impl 块中的。

一直使用的 String::from 函数就是一个"关联函数"。

## 实例

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect = Rectangle::create(30, 50);
    println!("{:?}", rect);
}
```



运行结果：

```
Rectangle { width: 30, height: 50 }
```



<mark>贴士：</mark>结构体 impl 块可以写几次，效果相当于它们内容的拼接！

### 单元结构体

结构体可以只作为一种象征而无需任何成员：

```
struct UnitStruct;
```

我们称这种没有身体的结构体为单元结构体（Unit Struct）。



# 9、enum枚举类

```
#[derive(Debug)]

enum Book {
    Papery, Electronic
}

fn main() {
    let book = Book::Papery;
    println!("{:?}", book);
    println!("{:?}",Book::Electronic)
}
```



```
Electronic
```

书分为纸质书（Papery book）和电子书（Electronic book）。

如果你现在正在开发一个图书管理系统，你需要描述两种书的不同属性（纸质书有索书号，电子书只有 URL），你可以为枚举类成员添加元组属性描述：

```

#[derive(Debug)]

enum Book {
    Papery(u32),
    Electronic(String),
}

fn main() {
    let book = Book::Papery(100);
    println!("{:?}", book);
    println!("{:?}",Book::Electronic(String::from("url://....")))
}


Papery(100)
Electronic("url://....")
```



如果你想为属性命名，可以用结构体语法：

```

#[derive(Debug)]

enum Book {
    Papery{
        index:  u32,
        index1:isize,
    },
    Electronic{
        url:String,
        url1: &'static str,
    },
}

fn main() {

    let index :u32 = 100;
    let index1 :isize = 101;

    let url :String = String::from("utl://...");
    let url1 :&'static str = "url://....1";

    //虽然可以如此命名，但请注意，并不能像访问结构体字段一样访问枚举类绑定的属性。访问的方法在 match 语法中。

    let book = Book::Papery{
        index:index,
        index1:index1,
    };

    match book {
        Book::Papery{index,index1} =>{
            println!("{}-1",index);
            println!("{}-1",index1);
        },
        Book::Electronic { url, url1 } => {
            println!("{}",url);
            println!("{}",url1);
        },
    }

    let elaec = Book::Electronic{
        url,
        url1,
    };

    match elaec {
        Book::Papery{index,index1}=>{
            println!("{}",index);
            println!("{}",index1);
        },

        Book::Electronic{url,url1}=>{
            println!("{}-2",url1);
            println!("{}-2",url);
        }
        
    }
    

    // println!("{:?}", book);
    // println!("{:?}",elaec)
}



```



```
100-1
101-1
url://....1-2
utl://...-2
```



match 块也可以当作函数表达式来对待，它也是可以有返回值的：

```
match 枚举类实例 {
    分类1 => 返回值表达式,
    分类2 => 返回值表达式,
    ...
}
```

但是所有返回值表达式的类型必须一样！

如果把枚举类附加属性定义成元组，在 match 块中需要临时指定一个名字：

```

#[derive(Debug)]

enum Book {
    Papery(u32),
    Electronic {url: String},
}

fn main() {

    let book = Book::Papery(1001);
    let elac = Book::Electronic{url:String::from("url://....")};

    match book {
        Book::Papery(i) => {
            println!("{}", i+5);
        },
        Book::Electronic { url } => {
            println!("{}", url);
        }
    }

    match elac {
        Book::Papery(i) => {
            println!("{}", i);
        },
        Book::Electronic { url } => {
            println!("{}", url);
        }
    }
}


1006
url://....

```



<font color=red>match 除了能够对枚举类进行分支选择以外，还可以对整数、浮点数、字符和字符串切片引用（&str）类型的数据进行分支选择。其中，浮点数类型被分支选择虽然合法，但不推荐这样使用，因为精度问题可能会导致分支错误。</font>

对非枚举类进行分支选择时必须注意处理例外情况，即使在例外情况下没有任何要做的事 . 例外情况用下划线 _ 表示：

```

// #[derive(Debug)]

fn main() {
    let t = "abc";
    match t {
        // "abc" => println!("Yes"),
        _ => {},
    }
}


```

# 10、Option 枚举类

Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白。

许多语言支持 null 的存在（C/C++、Java），这样很方便，但也制造了极大的问题，null 的发明者也承认这一点，"一个方便的想法造成累计 10 亿美元的损失"。

null 经常在开发者把一切都当作不是 null 的时候给予程序致命一击：毕竟只要出现一个这样的错误，程序的运行就要彻底终止。

为了解决这个问题，很多语言默认不允许 null，但在语言层面支持 null 的出现（常在类型前面用 ? 符号修饰）。

Java 默认支持 null，但可以通过 @NotNull 注解限制出现 null，这是一种应付的办法。

Rust 在语言层面彻底不允许空值 null 的存在，但无奈null 可以高效地解决少量的问题，所以 Rust 引入了 Option 枚举类：

```
enum Option<T> {
    Some(T),
    None,
}
```

如果你想定义一个可以为空值的类，你可以这样：

```
let opt = Option::Some("Hello");
```



如果你想针对 opt 执行某些操作，你必须先判断它是否是 **Option::None**：

相当于是应用了范形

```

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}
fn main() {
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}

Hello
```

如果你的变量刚开始是空值，你体谅一下编译器，它怎么知道值不为空的时候变量是什么类型的呢？

所以初始值为空的 Option 必须明确类型：

```
fn main() {
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}
opt is nothing
```



```

#[derive(Debug)]
enum Option<T> {
    // Some(T),
    None(T),
}
fn main() {
    let opt: Option<&str> = Option::None("something");
    match opt {
        Option::None(something) => {
            println!("opt is nothing{}",something);
        }
    }
}

opt is nothingsomething
```

这种设计会让空值编程变得不容易，但这正是构建一个稳定高效的系统所需要的。由于 Option 是 Rust 编译器默认引入的，在使用时可以省略 Option:: 直接写 None 或者 Some()。

Option 是一种特殊的枚举类，它可以含值分支选择：

```
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}
fn main() {
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    // None,
}
fn main() {
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => println!("{}", something),
        // Option::None => println!("opt is nothing"),
        
    }
}

//可以不写{}



#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}
fn main() {
    let t = Some(64);
    match t {
            Some(a) => println!("Yes{}-->",&a),
            _ => println!("No"),
    }
}


```

# 常量

Rust 有两种常量，可以在任意作用域声明，包括全局作用域。这两种常量都要显式地标注：

* `const`： 不可改变的值（常用类型）。
* `static`： 在 [`'static`](https://llever.com/rust-by-example-cn/custom_types/scope/lifetime/static_lifetime.html) 生命周期内可能发生改变的变量。

有个特例就是 `"string"` 原始类型。可以给它直接赋一个不可改变的 `static` 变量，是因为它的 类型标记：`&'static str` 包含了生命周期 `'static`。其他的引用类型都必须特别注明从而拥有 `'static` 生命周期。这似乎是无关紧要的，因为所需的显式标记会隐藏差异（This may seem minor though because the required explicit annotation hides the distinction.）。

## const

对于const，常量贯穿于整个程序的生命周期。更具体的，Rust 中的常量并没有固定的内存地址。这是因为实际上它们会被内联到用到它们的地方。为此对同一常量的引用并不能保证引用到相同的内存地址。

```
const N: i32 = 5;    //定义一个整型常量
```


一个常量可以理解为是一个C语言中的#define，它有元数据开销但无运行时开销。

## static

对于static，Rust以静态量的方式提供了类似“全局变量”的功能。它们与常量类似，不过静态量在使用时并不内联。这意味着对每一个值只有一个实例，并且位于内存中的固定位置。

```
static N: i32 = 5;
static NAME: &'static str = "Steve";     //静态量贯穿于整个程序的生命周期，因此任何存储在常量中的引用有一个'static生命周期

//因为这是可变的，一个线程可能在更新N同时另一个在读取它，导致内存不安全。
//因此访问和改变一个static mut是不安全（unsafe）的，因此必须在unsafe块中操作
static mut NUM: i32 = 10;
unsafe {
	NUM=NUM+1;
}

```



如果想赋予一个非常量表达式，可以参考用第三方库lazy_static（A macro for declaring lazily evaluated statics in Rust.）解决。


## 区别

* <font color=red>定义方式类型，一个使用 static，一个使用 const；</font>

* <font color=red>名称都要求使用大写，否则会报 Warning；</font>

* <font color=red>都必须明确指定类型；</font>

* <font color=red>两者都要求必须使用常量表达式进行赋值，即必须是编译期能计算出的值；</font>

但和常量也有一些重要的区别：

* <font color=green>常量在编译时被内联，但静态变量不会。在整个程序中静态变量只有一个实例，也就是说所有引用都指向同一个地址。</font>

* <font color=green>常量不可变，而静态变量和普通变量一样，默认不可变，但可以通过 mut 关键字定义为可变。</font>

也正是因为全局变量有可变特性，导致多个线程同时访问时，可能引发数据竞争，导致内存安全问题。因此，对于全局可变变量的访问和修改必须放在 unsafe 块中进行。以下代码编译不通过：

```
static mut NUM: i32 = 10;

fn main() {
    NUM += 1;
    println!("{}", NUM);
}

```

编译器提示：

> error[E0133]: use of mutable static is unsafe and requires unsafe function or block

改为这样就可以了：

```
static mut NUM: i32 = 10;

fn main() {
    unsafe {
        NUM += 1;
        println!("{}", NUM);
    }
}
```

unsafe，Go 程序员应该很熟悉。在 Go 中一般也建议别用它。

如果不用 unsafe，也就是静态变量别定义为可变，那这和常量似乎没啥区别，直接使用 const 更好。



## 实例

```
// 在所有的作用域外声明全局变量。
static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > THRESHOLD
}


//因此访问和改变一个static mut是不安全（unsafe）的，因此必须在unsafe块中操作
//可变静态变量
static mut N: i32 = 5;



fn main() {
    let n = 16;

    // 在 main 函数(主函数)中访问常量
    println!("This is {}", LANGUAGE);

    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错！不能修改一个 `const` 常量。
    // THRESHOLD = 5;
    // 改正 ^ 注释掉此行

    //全局可变变量的访问和修改必须放在 unsafe 块中进行
    unsafe {
        N += 1;
        println!("N: {}", N);
    }
}



```



```
This is Rust
The threshold is 10
16 is big
N: 6
```

# 变量绑定

Rust 通过静态类型确保类型安全。变量绑定可以在声明变量时标注类型。不过在多数情况下，编译器能够 从字面内容推导出变量的类型，大大减少了标注类型的负担。

使用 `let` 绑定操作可以将值（像具体数据）绑定到变量中。

```
fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将 `an_integer` 复制到 `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用变量绑定产生警告；可在变量名加上下划线的前缀来消除这些警告内容。
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // 改正 ^ 在变量名加上下划线前缀消除警告
}

```



```
An integer: 1
A boolean: true
Meet the unit value: ()
```



## 可变变量

变量绑定默认是不可变的，但加上 `mut` 修饰语后变量就可以改变。

```
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // 正确代码
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 错误！
    //cannot assign twice to immutable variable
    // _immutable_binding += 1;
    // 改正 ^ 将此行注释掉
}

```



```
Before mutation: 1
After mutation: 2
```



## 作用域和隐藏

变量绑定有一个作用域，并且限定在一个**代码块**（block）中存活（live）。代码块是一个被 `{}` 包围的 语句集合。另外也允许[变量隐藏](https://en.wikipedia.org/wiki/Variable_shadowing)。

```
fn main() {
    // 此绑定存在于 main 函数中
    let long_lived_binding = 1;

    // 这是一个代码块，比 main 函数拥有一个更小的作用域
    {
        // 此绑定只存在于本代码块
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // 此绑定*隐藏*了外面的绑定
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // 代码块结束

    // 报错！`short_lived_binding` 在此作用域上不存在
    //println!("outer short: {}", short_lived_binding);
//    |                                 ^^^^^^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `long_lived_binding
    // println!("outer short: {}", short_lived_binding);
    // 改正 ^ 注释掉这行

    println!("outer long: {}", long_lived_binding);

    // 此绑定同样*隐藏*了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}

```



```
inner short: 2
inner long: 5
outer long: 1
outer long: a
```



## 变量先声明

Rust 语言可以先声明变量绑定，后面才将它们初始化。但是这种情况用得很少，因为这样很可能导致使用未 初始的变量。

```
fn main() {
    // 声明一个变量绑定
    let a_binding;
    // a_binding = 5;

    {
        let x = 2;

        // 初始化一个绑定
        a_binding = x * x;
    }

    //初始化，未赋值的变量 作用域不同也可以
    println!("a binding: {}", a_binding);

    let another_binding;

    // 报错！使用了未初始化的绑定
    //编译器禁止使用未经初始化的变量，因为这会产生未定义行为（undefined behavior）。
    //use of possibly-uninitialized `another_binding` 未初始化
    // println!("another binding: {}", another_binding);
    // 改正 ^ 注释掉此行

    another_binding = 1;

    println!("another binding: {}", another_binding);
}

```



```
a binding: 4
another binding: 1
```

# 类型转换

Rust 在基本类型之间没有提供隐式类型转换（强制类型转换）（implicit type conversion，coercion ）。不过使用 `as` 关键字进行显式类型转换（explict type conversion，casting）。

一般来说，Rust 的整型类型的转换规则遵循 C 语言的惯例，除了那些在 C 语言是未定义行为的情况。在 Rust 中，所有的整型类型转换的行为都得到了很好的定义。

```
// 消除会溢出的类型转换的所有警告。
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // 报错！不能隐式转换类型
    // error ^^^^^^^ expected `u8`, found `f32`
    // let integer: u8 = decimal;
    // 改正 ^ 注释掉此行

    // 显式转换类型
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当将任意整数值转成无符号类型（unsigned 类型） T 时，
    // 将会加上或减去 std::T::MAX + 1，直到值符合新的类型

    // 1000 原本就符合 u16 类型
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 在计算机底层会截取数字的低8位（the least significant bit，LSB），
    // 而高位（the most significant bit，MSB）数字会被抛掉。
    // （译注：此操作是按二进数存储的数字位进行）
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 对正数来说，上面的类型转换操作和取模效果一样
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当将整数值转成有符号类型（signed 类型）时，同样要先将（二进制）数值
    // 转成相应的无符号类型（译注：如 i32 和 u32 对应，i16 和 u16对应），
    // 然后再求此值的补码（two's complement）。如果数值的最高位是 1，则数值
    // 是负数。

    // 除非值本来就已经符合所要转的类型。
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128，再求数字128的8位二进制补码得到：
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 重复上面的例子
    // 1000 as u8 -> 232
    println!("1000 as a i8 is : {}", 1000 as i8);
    // 232 的补码是 -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}

```



```
Casting: 65.4321 -> 65 -> A
1000 as a u16 is: 1000
1000 as a u8 is : 232
  -1 as a u8 is : 255
1000 mod 256 is : 232
 128 as a i16 is: 128
 128 as a i8 is : -128
1000 as a i8 is : -24
 232 as a i8 is : -24
```



## 字面量

数字字面量可以加上类型标记作为后缀来标注类型。举个例子，要指定字面量 `42` 为 `i32` 类型，可以写成 `42i32`。

未加上后缀的数字字面量的类型视使用它们的情况而定。如果没有限定，编译器会将整型定为 `i32` 类型，将浮点数定为 `f64` 类型。

```
fn main() {
    // 有后缀的字面量，它们的类型在初始化时就确定
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量，它们的类型视使用情况而定
    let i = 1;
    let f = 1.0;

    // `size_of_val` 返回变量的大小，以字节（byte）为单位
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));


    //后缀可以用_表明类型
    let x_ = 1_u8;
    let y_ = 2_u32;
    let z_ = 3_f32;

    // 无后缀的字面量，它们的类型视使用情况而定
    let i_ = 1_;
    let f_ = 1.6_;

    // `size_of_val` 返回变量的大小，以字节（byte）为单位
    println!("size of `x_` in bytes: {},val of  {}", std::mem::size_of_val(&x_),x_);
    println!("size of `y_` in bytes: {},val of  {}", std::mem::size_of_val(&y_),y_);
    println!("size of `z_` in bytes: {},val of  {}", std::mem::size_of_val(&z_),&z_);
    println!("size of `i_` in bytes: {},val of  {}", std::mem::size_of_val(&i_),i_);
    println!("size of `f_` in bytes: {},val of  {}", std::mem::size_of_val(&f_),f_);



}

```



```
size of `x` in bytes: 1
size of `y` in bytes: 4
size of `z` in bytes: 4
size of `i` in bytes: 4
size of `f` in bytes: 8
size of `x_` in bytes: 1,val of  1
size of `y_` in bytes: 4,val of  2
size of `z_` in bytes: 4,val of  3
size of `i_` in bytes: 4,val of  1
size of `f_` in bytes: 8,val of  1.6
```

前面代码中用了一些尚未解释过的概念，这里列出一些简短的说明：

* `fun(&foo)` 是**通过引用**传参给一个函数，而不是通过值来传参（`fun(foo)`）。更多内容参见 [借用](https://llever.com/rust-by-example-cn/cast/scope/borrow.html)（borrowing）。
* `std::mem::size_of_val` 是一个函数，不过是通过**完整的路径**调用的。代码可以划分到称为 **模块**（module）的逻辑单元中。在这个例子中，`size_of_val` 函数是定义在 `mem` 模块的， `mem` 模块是定义在 `std` 包（crate）中。更多内容参考[模块](https://llever.com/rust-by-example-cn/cast/mod.html) 和 [crate](https://llever.com/rust-by-example-cn/cast/crates.html)。



## 类型推导

类型推导引擎是相当智能的。它不仅仅在初始化期间分析[右值](https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue)的类型，还会通过分析变量在后面是 怎么使用的来推导该变量的类型。这里给出一个类型推导的高级例子：



```
fn main() {
    // 借助类型标注，编译器知道 `elem` 具有 u8 类型。
    let elem = 5u8;
    let elem1 = 6_u16;

    // 创建一个空 vector（可增长数组）。
    let mut vec = Vec::new();
    // 此时编译器并未知道 `vec` 的确切类型，它只知道 `vec` 是一个含有某种类型
    // 的 vector（`Vec<_>`）。

    // 将 `elem` 插入 vector。
    vec.push(elem);
    //expected `u8`, found `u16`
    // vec.push(elem1);
    // Aha！现在编译器就知道了 `vec` 是一个含有 `u8` 类型的 vector(`Vec<u8>`)
    // 试一试 ^ 尝试将 `vec.push(elem)` 那行注释掉
    //error cannot infer type for type parameter `T`

    //先推入什么，数据就是什么类型

    println!("{:?}", vec);
}

```



```
[5]
```

## 别名

<font color=red>`type` 语句可以给一个已存在类型起一个新的名字。类型必须要有 `CamelCase`（驼峰方式）的名称，否则 编译器会产生一个警告。~~对规则为例外的是基本类型： `usize`，`f32`等等。</font>~~

实验证明，usize f32也是可以的

```
// `NanoSecond` 是 `u64` 的新名字。
type NanoSecond = u64;
type Inch = u64;

// 使用一个属性来忽略警告。
#[allow(non_camel_case_types)]
type u64_t = u64;
// 试一试 ^ 试着删掉属性。

type Us = usize;
type F32 = f32;


fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let nanoseconds1: NanoSecond = 5 ;
    let inches: Inch = 2 as u64_t;

    // 注意类型的别名*没有*提供任何额外的类型安全，因为别名*不是*新的类型
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);

    println!("{} nanoseconds1 + {} inches = {} unit?",
             nanoseconds1,
             inches,
             nanoseconds + inches);

    println!();

    let us :Us = 45;
    let f32 :F32 = 56.67;

    println!("{},{}",us,f32)


}

```



```
5 nanoseconds + 2 inches = 7 unit?
5 nanoseconds1 + 2 inches = 7 unit?

45,56.67
```

别名的主要作用是减少按键，举个例子，`IoResult<T>` 类型是 `Result<T, IoError>` 类型的别名。



# 表达式

Rust 程序（大部分）由一系列语句构成：

```
fn main() {
    // 语句
    // 语句
    // 语句
}
```



Rust 有多种语句。最普遍的语句类型有两种：一种是绑定变量，另一种是表达式带上分号：

```
fn main() {
    // 变量绑定
    let x = 5;

    // 表达式;
    x;
    x + 1;
    15;
}
```

代码块也是表达式，所以它们在赋值操作中可以充当[右值（r-values）](https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue)。代码块中的最后一条 表达式将赋给[左值（l-value）](https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue)。需要注意的是，如果代码块最后一条表达式结尾处有分号，那 么返回值将变成 `()`。（译注：代码块中的最后一条语句是代码块中**实际执行**的最后一条语句，而不一 定是代码块中最后一行的语句。）

```
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 将此表达式赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号结束了这个表达式，于是将 `()` 赋给 `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
```



# 流程控制

任何编程语言都包含的一个必要部分就是改变控制流程：`if`/`else`，`for`等。让我们讲述 Rust 语言中 的这部分内容。



## `if/else`

`if`-`else`分支判断和其他语言类似。与很多语言不同的是，Rust 语言中的布尔判断条件不用小括号包住， 每个判断条件后连着一个代码块。`if`-`else`条件选择是一个表达式，<font color=red>并且所有分支都必须返回相同的类型。</font>

```
fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // 这条表达式返回一个 `i32` 类型。
            10 * n
        } else {
            println!(", and is a big number, reduce by two");

            // 这条表达式也必须返回一个 `i32` 类型。
            n / 2
            // 试一试 ^ 试着加上一个分号来结束这条表达式。
            //`if` and `else` have incompatible types
        };
    //   ^ 不要忘记在这里加上一个分号！所有的 `let` 绑定都需要它。

    println!("{} -> {}", n, big_n);
}
```



```
5 is positive, and is a small number, increase ten-fold
5 -> 50
```



## loop 循环

Rust 提供了 `loop` 关键字来实现一个无限循环。

可以使用 `break` 语可以在任何时刻退出一个循环，另外可用 `continue` 跳过迭代的剩余部分并重新开始 一轮循环。

<font color=red>只能跳出所在层的循环</font>

```
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过这次迭代的剩下内容
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // 退出循环
            break;
        }
    }

    println!("----------------");


    // // 无限循环
    // loop {
    //     loop {
    //         count += 1;
    
    //         if count == 3 {
    //             println!("three");
    
    //             // 跳过这次迭代的剩下内容
    //             continue;
    //         }
    
    //         println!("{}", count);
    
    //         if count == 5 {
    //             println!("OK, that's enough");
    
    //             // 退出循环
    //             break;
    //         }
    //     }
    //     println!("================")
    // }
    
}
```



```
Let's count until infinity!
1
2
three
4
5
OK, that's enough
----------------
```

### label嵌套循环和标签

在处理嵌套循环的时候可以`中断（break）`或`继续（continue）`外层循环。在这类情形中，循环必须用一 些`'label`（标签）来注明，并且标签传递给 `break`/`continue` 语句。

- break是跳出继续执行

- continue是跳出后从label处再次执行，无限循环

- rust可以使用标签 `'label_name:` (`单引号+标签名+冒号`) 来指定你的 `break` 或 `continue` 语句作用的循环。



```
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    'outer: loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过这次迭代的剩下内容
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // 退出循环
            break 'outer;
        }
    }

    println!("----------------") 
}


Let's count until infinity!
1
2
three
4
5
OK, that's enough
----------------
```



```
#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            // break 'outer;
            break 'inner;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}



```





# 从 loop 循环中返回

`loop` 有个用途是尝试一个操作直到成功为止。若操作返回一个值，则可能需要将其传递给代码的其余部分：将该值放在 `break` 之后，并由 `loop` 表达式返回。



# 流程

## while 循环

`while` 关键字可以用作当型循环（当条件满足时循环）。

让我们用 `while` 循环写一个不怎么出名的 [FizzBuzz](http://en.wikipedia.org/wiki/Fizz_buzz) 程序。

```
fn main() {
    // 计数器变量
    let mut n = 1;

    // 当 `n` 小于 101 时进入循环操作
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器值加1
        n += 1;
    }
}
```



```
1
2
fizz
4
buzz
fizz
7
8
fizz
buzz
11
fizz
13
14
fizzbuzz
16
17
fizz
19
buzz
fizz
22
23
fizz
buzz
26
fizz
28
29
fizzbuzz
31
32
...
```



## for 循环和区间

`for in` 结构可以通过一个计数器来迭代。创建计算器的一个最简便的方法就是使用区间标记 `a..b`。这 会生成从 `a`（包含此值） 到 `b` （不含此值）增幅为 1 的一系列值。

让我们使用 `for` 代替 `while` 来写 FizzBuzz 程序。

```

fn main() {
    // `n` 将从 1, 2, ..., 100 这些值依次获取进行每次循环
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    for v in [1,2,3,4,5,6]{
        println!("{}",v)
    }
}
```



```
1
2
fizz
4
buzz
fizz
7
8
fizz
buzz
11
fizz
13
14

...
1
2
3
4
5
6
```

## match 匹配

Rust 通过 `match` 关键字来提供模式匹配，用法和 C 语言的的 `switch` 类似。

```
fn main() {
    let number = 13;
    // 试一试 ^ 将不同的值赋给 `number`

    println!("Tell me about {}", number);
    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 匹配一个闭区间范围
        //... 变为..=了 版本 rustc 1.58.1 (db9d1b20b 2022-01-20)
        13..=19 => println!("A teen"),
        // 处理其他情况
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
        // 试一试 ^ 试着将其中一条分支注释掉
    };

    println!("{} -> {}", boolean, binary);
}
```



```
Tell me about 13
A teen
true -> 1
```

### 解构

`match` 代码块可以以多种方式解构内容。

#### 元组

元组可以在 `match` 中解构，如下所示：

```
fn main() {
    let pair = (0, -2);
    // 试一试 ^ 将不同的值赋给 `pair`

    println!("Tell me about {:?}", pair);
    // match 可以解构一个元组
    match pair {
        // 绑定到第二个元素
        (0, y) => {
            println!("First is `0` and `y` is `{:?}`", y)
        },
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // `_` 表示不将值绑定到变量
    }
}
```



```
Tell me about (0, -2)
First is `0` and `y` is `-2`
```

- 注意只走了一个分支，选择了一个分支就不会再走了

- 可以加代码块`{}`也可以不加

#### 枚举

和前面相似，解构 `enum` 方式如下：

```
// 需要 `allow` 来消除警告，因为只使用了一个变量。
#[allow(dead_code)]
enum Color {
    // 这三者仅由它们的名字来表示。
    Red,
    Blue,
    Green,
    // 这些元组含有类似的 `u32` 元素，分别对应不同的名字：颜色模型（color models）。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);
    // 试一试 ^ 将不同的值赋给 `color`

    println!("What color is it?");
    // 可以使用 `match` 来解构 `enum`。
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // 不需要其它分支，因为所有的情形都已覆盖
        _ => println!("is empty")
    }
}
```



```
What color is it?
Red: 122, green: 17, and blue: 40!
```

- <font color=red>如果所有的分支都覆盖了，就不需要其他分支的情况了</font>

#### 指针和引用

对指针来说，解构（destructuring）和解引用（dereferencing）要区分开， 因为这两者的概念是不同的，和 `C` 那样的语言用法不一样。

* 解引用使用 `*`
* 解构使用 `&`，`ref`， 和 `ref mut`

```
fn main() {
    // 获得一个 `i32` 类型的引用。`&` 表示获取一个引用。
    let reference = &4;

    match reference {
        // 如果 `reference` 是对 `&val` 进行模式匹配，则会产生如下比较行为：
        // `&i32`
        // `&val`
        // ^ 我们看到，如果匹配的 `&` 都去掉了，那么就是 `i32` 赋给 `val`。
        &val1 => println!("Got a value via destructuring: {:?}", val1),
    }

    // 为了避免 `&` 的使用，需要在匹配前解引用。
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // 如果没有一个引用头部（以 & 开头）会是怎样？ `reference` 是一个 `&`，
    // 因为右边已经是一个引用。
    // 下面这个不是引用，因为右边不是。
    let number = 13;

    match number {
        1 => println!("values"),
        _ => println!("other")
    }
    // Rust 对这种情况提供了 `ref`。它更改了赋值行为，使得可以对具体值
    // 创建引用。这将得到一个引用。
    let ref _is_a_reference = 3;

    match _is_a_reference {
        3 => println!("{}",_is_a_reference),
        _ => println!("other")
    }

    //match 不适用ref的
    // match  _is_a_reference {
        
    // }

    // 相应地，定义两个非引用的值，通过 `ref` 和 `mut` 可以取得引用。
    let value = 5;
    let mut mut_value = 6;

    // 使用 `ref` 关键字来创建引用。
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // 类似地使用 `ref mut`。
    match mut_value {
        ref mut m => {
            // 获得一个引用。在增加内容之前，要先得到解引用（Gotta
            // dereference it before we can add anything to it）。
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
```



```
Got a value via destructuring: 4
Got a value via dereferencing: 4
other
3
Got a reference to a value: 5
We added 10. `mut_value`: 16
```

#### 结构体

类似地，解构 `struct` 如下所示：

```
// #[derive(Debug)]
// #[derive(Debug)]

fn main() {
    struct Foo { x: (u32, u32), y: u32 }



    // 解构结构体的成员
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        ref _val => println!("print"),
    }
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // 可以解构结构体并重命名变量，成员顺序是不重要的

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // 也可以忽略某些变量
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // 这将得到一个错误：模式中没有提及 `x` 字段
    // let Foo { y } = foo;
}
```



```
print
a = 1, b = 2,  y = 3 
i = 3, j = (1, 2)
y = 3
```

- 可以做结构的转换操作等



### 守卫

可以加上 `match` **守卫**（guard） 来过滤分支。

```
fn main() {
    let pair = (2, -2);
    // 试一试 ^ 将不同的值赋给 `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // ^ `if condition`(if 条件)部分是一个守卫
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
```

```
Tell me about (2, -2)
Antimatter, kaboom!
```

### 变量绑定

间接地访问一个变量不可能在分支中使用这个没有重新绑定的变量。 `match` 提供了 `@` 符号来绑定变量到名称：

```
// `age` 函数，返回一个 `u32` 值。
fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me type of person you are");

    match age() {
        0             => println!("I'm not born yet I guess"),
        // 不能直接 `匹配（match）` 1 ... 12，但是孩子是几岁呢？
        // 相反，将 1 ... 12 序列绑定到 `n` 。现在年龄就可以读取了。
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // 没有绑定。返回结果。
        n             => println!("I'm an old person of age {:?}", n),
    }
}
```

```
Tell me type of person you are
I'm a teen of age 15
```

# if let

在一些例子中，`match` 使用起来并不优雅。比如：

```
#![allow(unused_variables)]
fn main() {
// 将 `optional` 定为 `Option<i32>` 类型
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ 行首需要2个缩进，就这样可以从 option 类型中对 `i`
        // 进行解构
    },
    _ => {},
    // ^ 必需内容，因为 `match` 需要覆盖全部情况。难道不觉得冗余吗？
};

}
```



```
This is a really long string and `7`
```

`if let` 对这样的用法要简洁得多，并且允许指明特定的各种不同的失败可选项 内容（options）：

```
fn main() {
    // 全部都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构解读：若 `let` 将 `number` 解构成 `Some(i)`，则执行
    // 语句块（`{}`）
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // 如果要指明失败情形，就使用 else：
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // 解构失败。换到失败情形（Change to the failure case）。
        println!("Didn't match a number. Let's go with a letter!");
    };

    // 提供一个改变的失败条件（Provide an altered failing condition）。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // 解构失败。执行 `else if` 条件来判断轮到的失败分支是否需要执行
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 条件执行错误。这是默认的分支：
        println!("I don't like letters. Let's go with an emoticon :)!");
    };
}
```

```
Matched 7!
Didn't match a number. Let's go with a letter!
I don't like letters. Let's go with an emoticon :)!
```



# while let

和 `if let` 类似，`while let` 会产生更加难看的 `match` 的一连串内容。 考虑下面的有关增量 `i` 的一连串内容：

```

#![allow(unused_variables)]
fn main() {
// 将 `optional` 设为 `Option<i32>` 类型
let mut optional = Some(0);

// Repeatedly try this test.
// 重复运行这个测试。
loop {
    match optional {
        // 如果 `optional` 解构成功，就执行下面语句块。
        Some(i) => {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
            // ^ 需要三个缩进！
        },
        // 当解构失败时退出循环：
        _ => { break; }
        // ^ 为什么要这样的语句呢？肯定有更优雅的处理方式！
    }
}
}
```

```
`i` is `0`. Try again.
`i` is `1`. Try again.
`i` is `2`. Try again.
`i` is `3`. Try again.
`i` is `4`. Try again.
`i` is `5`. Try again.
`i` is `6`. Try again.
`i` is `7`. Try again.
`i` is `8`. Try again.
`i` is `9`. Try again.
Greater than 9, quit!
```

使用 `while let` 可以使这一连串内容变得更加优雅：

```
fn main() {
    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

    // 分析：当 `let` 将 `optional` 解构成 `Some(i)` 时，就
    // 执行语句块（`{}`）。否则中断退出（`break`）。
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ 使用的缩进更少，并且不用显式地处理失败情况。
    }
    // ^ `if let` 有额外可选的 `else`/`else if` 分句，
    // 而 `while let` 没有。
}
```

```
`i` is `0`. Try again.
`i` is `1`. Try again.
`i` is `2`. Try again.
`i` is `3`. Try again.
`i` is `4`. Try again.
`i` is `5`. Try again.
`i` is `6`. Try again.
`i` is `7`. Try again.
`i` is `8`. Try again.
`i` is `9`. Try again.
Greater than 9, quit!
```

# 函数

- 函数使用 `fn` 关键字来声明。

- 函数的参数需要标注类型，就和变量一样

- 另外如果 函数返回一个值(只能返回一个值)，返回类型必须在箭头 `->` 之后特别指出来。

函数最后的表达式将作为返回值。或者在函数内使用 `return` 语句来提前返回值， 甚至在循环或 `if` 内部使用。

让我们使用函数来重写 FizzBuzz 函数吧！

```
// 和 C/C++ 不一样，Rust 的函数定义位置是没有限制的
fn main() {
    // 我们在这里使用函数，并在后面的其他位置定义它
    fizzbuzz_to(100);
}

// 函数返回一个布尔（boolean）值
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // 极端情况，提前返回（Corner case, early return）
    if rhs == 0 {
        return false;
    }

    // 这是一个表达式，这里可以不用 `return` 关键字
    lhs % rhs == 0
}

// 函数不返回值，而实际上是返回一个单元类型 `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// 当函数返回 `()` 时，可以从标记中删除返回类型
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}
```



```
1
2
fizz
4
buzz
fizz
7
8
fizz
buzz
...
```



## 方法

方法是从属于对象的函数(Mathods are functions attached to objects)。这些方法通过 关键字 `self` 来访问对象中的数据和其他方法。方法在 `impl` 代码块下定义。

```
struct Point {
    x: f64,
    y: f64,
}

// 实现的代码块，所有的 `Point` 方法都在这里给出
impl Point {
    // 这是一个静态方法（static method）
    // 静态方法不需要通过实例来调用
    // 这类方法一般用作构造器（constructor）
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // 另外一个静态方法，带有两个参数：
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 这是实例方法（instance method）
    // `&self` 是 `self: &Self` 的语法糖（sugar），其中 `Self` 是所调用对象
    // 的类型。在这个例子中 `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` 通过点运算符来访问结构体字段
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` 是一个 `f64` 类型的方法，返回调用者的绝对值
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 这个方法要求调用者对象是可变的
    // `&mut self` 为 `self: &mut Self` 的语法糖
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` 含有的资源：两个堆分配的整型
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 这个方法“消费”调用者对象的资源
    // `self` 为 `self: Self` 的语法糖
    fn destroy(self) {
        // 解构 `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` 和 `second` 离开作用域后释放
    }
}

fn main() {
    let rectangle = Rectangle {
        // 静态方法使用双重冒号来调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 实例方法通过点运算符来调用
    // 注意第一个参数 `&self` 是隐式传递的，比如：
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 报错！ `rectangle` 是不可变的，但这方法需要一个可变对象
    //rectangle.translate(1.0, 0.0);
    // 试一试 ^ 将此行注释去掉

    // 正常运行！可变对象可以调用可变方法
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // 报错！前面的 `destroy` 调用“消费了” `pair`
    // pair.destroy();
    // 试一试 ^ 将此行注释去掉
    //use of moved value: `pair`
}
```



```
Rectangle perimeter: 14
Rectangle area: 12
Destroying Pair(1, 2)
```



## 闭包

闭包（closure）在 Rust 中也称为 lambda，是一类捕获封闭环境的函数。例如，一个可以捕获 x 变量的闭包如下：

`|val| val + x` 

它们的语法和能力使它们在临时（on the fly）使用相当方便。调用一个闭包和调用一个函数完全相同。然而，输入和返回类型两者都**可以**自动推导，<font color=red>且输入变量名**必须**指明。</font>

其他的特点包括：

* 使用 `||` 替代 `()` 将输入变量括起来。
* 区块定界符（`{}`）对于单条表达式是可选的，其他情况必须加上。
* 有能力捕获到外部环境变量。

```
fn main() {
    // 通过闭包和函数实现增量。
    fn  function            (i: i32) -> i32 { i + 1 }

    // 闭包是匿名的，这里我们将它们绑定到引用。
    // 类型标注和函数的一样，不过类型标注和使用 `{}` 来围住代码都是可选的。
    // 这些匿名函数（nameless function）赋值给合适命名的变量。
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // 调用函数和闭包。
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // 没有参数的闭包，返回一个 `i32` 类型。
    // 返回类型是自动推导的。
    let one = || 1;
    println!("closure returning one: {}", one());
}
```



### 捕获

闭包本身是相当灵活的，可以实现所需功能来让闭包运行而不用类型标注（原文：Closures are inherently flexible and will do what the functionality requires to make the closure work without annotation）。这允许变量捕获灵活地适应使用 情况，有时是移动（moving）有时是借用（borrowing）（原文：This allows capturing to flexibly adapt to the use case, sometimes moving and sometimes borrowing.）。闭包可以捕获变量：

* 通过引用：`&T`
* 通过可变引用：`&mut T`
* 通过值：`T`

它们更倾向于通过引用来捕获变量并且只在需要时才用后面用法（原文：They preferentially capture variables by reference and only go lower when required.）。

```
fn main() {
    use std::mem;
    
    let color = "green";

    // 闭包打印 `color`，它会马上借用（`&`）`color` 并将该借用和闭包存储
    // 到 `print` 变量中。它会一保持借用状态直到 `print` 离开作用域。
    // `println!` 只需要`通过引用`，所以它没有采用更多任何限制性的内容。
    // （原文：`println!` only requires `by reference` so it doesn't
    // impose anything more restrictive.）
    let print = || println!("`color`: {}", color);

    // 使用借用来调用闭包。
    print();
    print();

    let mut count = 0;

    // 闭包使 `count` 值增加，可以使用 `&mut count` 或者 `count`，
    // 但 `&mut count` 限制更少，所以采用它。立刻借用 `count`。
    // （原文： A closure to increment `count` could take either
    // `&mut count` or `count` but `&mut count` is less restrictive so
    // it takes that. Immediately borrows `count`.）
    //
    // `inc` 前面需要加上 `mut`，因为存储了一个'&mut'里面。因此，调用闭包会改变需要
    // 因此，调用该闭包转变成需要一个 `mut` 的闭包。
    // （原文：A `mut` is required on `inc` because a `&mut` is stored
    // inside. Thus, calling the closure mutates the closure which requires
    // a `mut`.）
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 调用闭包。
    inc();
    inc();

    let reborrow = &mut count;
    println!("{}",reborrow);
    // ^ 试一试： 将此行注释去掉。
    
    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // 
    // `mem::drop` 要求 `T`，所以这必须通过值来实现（原文：`mem::drop`
    // requires `T` so this must take by value.）。可复制类型将会复制
    // 值到闭包而不会用到原始值。不可复制类型必须移动（move），从而
    // `可移动`（movable） 立即移动到闭包中（原文：A non-copy must
    // move and so `movable` immediately moves into the closure）。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消费（consume）了该变量，所以这只能调用一次。
    consume();
    //value used here after moverustc(E0382)
    // consume();
    // ^ 试一试：将此行注释去掉。
}
```



```
`color`: green
`color`: green
`count`: 1
`count`: 2
2
`movable`: 3
```



### 作为输入参量

虽然 Rust 在捕获临时变量的方式大多选择不带标注，但在编写函数时，这种不确定性是不允许的。当以闭包作为输入参数时，闭包的完整类型必须使用以下的其中一种 `trait` 来标注。它们的受限程度依次递减，依次是（原文：In order of decreasing restriction, they are）：

* `Fn`：闭包需要通过引用（`&T`）捕获
* `FnMut`：闭包需要通过可变引用（`&mut T`）捕获
* `FnOnce`：闭包需要通过值（`T`）捕获

在值传值（variable-by-variable）的基础上，编译器将以限制最少的方式来捕获变量。

例如考虑一个标注为 `FnOnce` 的参量。这意味着闭包可能通过 `&T`，`&mut T` 或 `T` 来捕获，但是编译器将根据所捕获变量在闭包的使用情况做出最终选择。

这是因为若移动语义（move）可能的话，则任意借用类型也应该是可行的。注意反过来就不再成立：如果参量是 `Fn`，那么通过 `&mut T` 或 `T` 捕获的情况就不允许了。

在下面的例子中，试着换换 `Fn`、`FnMut` 和 `FnOnce` 的使用，看看会发生什么：

```
use std::fmt::{self, Display, Formatter};
// 将闭包作为参数并调用它的函数。
fn apply<F>(f: F) where
    // 闭包没有输入值和返回值。
    F: FnOnce() {
    // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。

    f();
}

// 使用闭包并返回一个 `i32` 整型的函数。
fn apply_to_3<F>(f: F) -> i32 where
// 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
    F: Fn(i32) -> i32 {

    f(3)
}
struct V(u8);

impl Display for V {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let v1 = V(0);
    let mut v2 = V(0);
    let v3 = V(0);
    let f = || {
        // `v1` 通过引用操作，闭包为 `Fn`
        println!("{}", v1);

        // `v2` 通过可变引用操作，闭包为 `FnMut`，所有权移动至闭包内
        v2.0 += 1;

        // `v3` 通过值操作，闭包为 `FnOnce`，所有权移动至闭包内
        drop(v3)
    };
    println!("{}", v1);
    println!("v2---------{}", v2); // 正常
    // println!("{}", v3); // 报错




    use std::mem;
    
    let greeting = "hello";
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建属于自己的数据。
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：通过引用方式的 `greeting` 和
    // 通过值方式的 `farewell`。
    let diary = || {
        // `greeting` 使用引用方式：需要 `Fn`。
        println!("I said {}.", greeting);

        // 改变迫使 `farewell` 变成了通过可变引用来捕获。
        // （原文：Mutation forces `farewell` to be
        // captured by mutable reference.）
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 将 `farewell` 强制转成通过值来捕获。
        // （原文：Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.）
        // 现在需要 `FnOnce`。
        mem::drop(farewell);
    };

    // 调用处理闭包的函数（原文：Call the function
    // which applies the closure）。
    apply(diary);

    // `double` 满足 `apply_to_3` 的 trait 限定。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
```



```
0
v2---------0
I said hello.
Then I screamed goodbye!!!.
Now I can sleep. zzzzz
3 doubled: 6
```

- `where`是限制闭包函数的类型的

- 注意Fn、MutFn、FnOnce等的特点



### 类型匿名

闭包从封闭的作用中域捕获变量简单明了。这样会有某些后果吗？当然会。观察一下使用闭包作为函数参量的方式是要求为[泛型](https://llever.com/rust-by-example-cn/fn/closures/generics.html)的，它们定义的方式决定了这是必要的（原文：Observe how using a closure as a function parameter requires [generics](https://llever.com/rust-by-example-cn/fn/closures/generics.html), which is necessary because of how they are defined）：

```

#![allow(unused_variables)]
fn main() {
    // `F` 必须是泛型。
    fn apply<F>(f: F) where
        F: FnOnce() {
        f();
    }
}



#![allow(unused_variables)]
fn main() {
    // `F` 必须是泛型。
    fn apply<F>(f: F) 
    where
        F: FnOnce() {
        f();
    }
}
```



当定义一个闭包时，编译器将隐式地创建一个新的匿名结构体来存储内部的捕获变量， 同时针对此未知类型通过其中的一种 `trait`：`Fn`，`FnMut`，或 `FnOnce` 来实现功能 （原文：implementing the functionality via one of the `traits`: `Fn`, `FnMut`, or `FnOnce` for this unknown type）。这个类型被赋给所存储的变量直到调用（原文： This type is assigned to the variable which is stored until calling）。

由于这个新类型是未知的类型，所以在函数中的任何用法都要求是泛型。然而， 未限定的类型参量 `<T>` 仍然是不明确的并且是不允许的。因此通过其中一种 `trait`：`Fn`，`FnMut`，或 `RnOnce`（已经实现）就足以指明它的类型。

```
// `F` 必须针对一个没有输入参数和返回值的闭包实现 `Fn`
// —— 确切地讲是 `print` 要求的类型。
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;


    // 捕获的 `x` 成为一个匿名类型并为它实现 `Fn`。
    // 将它存储到 `print` 中。
    let print = || println!("{}", x);

    apply(print);
}
```



```
7
```



### 输入函数

既然闭包可以作为参数，你很可能想知道函数是否也可以呢。确实可以！如果你声明一个接受闭包作为参数的函数，那么任何满足该闭包的 trait 约束的函数都可以作为参数传递。

```
// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
fn call_me<F: Fn()>(f: F)
where F:Fn()
{
    f()
}

// 定义一个满足 `Fn` 限定的装包函数（wrapper function）。
fn function() {
    println!("I'm a function!");
}

fn main() {
    // 定义一个满足 `Fn` 限定的闭包。
    let closure = || println!("I'm a closure!");
    
    call_me(closure);
    call_me(function);
}
```



```
I'm a closure!
I'm a function!
```



### 作为输出参量

闭包作为输入参量是可能的，所以返回闭包作为输出参量（output parameter）也应该是可能的。然而返回的闭包类型会有问题，因为目前的 Rust 只支持返回具体（非泛型）的类型。按照定义匿名的闭包类型是未知的，所以想要返回一个闭包只有使它变成具体的类型。通过 box 操作可以实现这点。

关于返回值的有效的 trait 和前面的略有不同：

* `Fn`：和前面的一样（normal）
* `FnMut`：和前面的一样
* `FnOnce`：这里运行会有些独特的地方（There are some unusual things at play here），所以目前需要 [`FnBox`](http://doc.rust-lang.org/std/boxed/trait.FnBox.html) 类型，这属于不稳定的内容。此处预计将来会发生改变。

除此之外，还必须使用 `move` 关键字，它表明了通过值来产生全部的捕获（which signals that all captures occur by value）。这是必需的，因为在函数退出的同时任何通过引用捕获的值将被丢弃（dropped），在闭包中留下无效的引用。

```
fn create_fn() -> Box<dyn Fn()> {
    let text = "Fn".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

fn create_fnmut() -> Box<dyn FnMut()> {
    let text = "FnMut".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}
```



```
This is a: Fn
This is a: FnMut
```

### Iterator::any 迭代器

`Iterator::any` 是一个函数，在处理一个迭代器（iterator）时，当其中任一元素符合条件（predicate）时将返回 `true`，否则返回 `false`。它的原型如下：

```
pub trait Iterator {
    // 迭代相关的类型（原文：The type being iterated over）。
    type Item;

    // `any` 接受 `&mut self` 作为调用者可能被借用和修改，但不会消费掉。
    // （原文： `any` takes `&mut self` meaning the caller may be
    // borrowed and modified, but not consumed.）
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` 表示任意捕获变量很可能都被修改，而非消费。
        // `Self::Item` 表明了通过值来接受闭包类型参数。
        // （原文：`FnMut` meaning any captured variable may at 
        // most be modified, not consumed. `Self::Item` states it
        // takes arguments to the closure by value.）
        F: FnMut(Self::Item) -> bool {}
}

```



```
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 的 `iter()` 产出 `&i32`（原文：`iter()` for vecs yields
    // `&i32`）。解构成 `i32` 类型。
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // 对 vec 的 `into_iter()` 产出 `i32` 类型。无需解构。
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组（array）的 `iter()` 产出 `&i32`。
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // 对数组的 `into_iter()` 通常产出 `&i32`。
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}
```



```
2 in vec1: true
2 in vec2: false
2 in array1: true
2 in array2: false
```

### Iterator::find  迭代器返回枚举option

`Iterator::find` 是一个函数，在处理一个迭代器（iterator）时，将返回第一个满足条件的元素作为一个 `Option` 类型。它的原型如下：

```
pub trait Iterator {
    // 迭代相关的类型。
    type Item;

    // `find` 接受 `&mut self` 作为调用者可能被借用和修改，但不会消费掉。
    // （原文：`find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.）
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` 表示任意捕获变量很可能都被修改，而非消费。
        // `&Self::Item` 表明了通过引用接受闭包类型的参数。
        // （原文：`FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.）
        P: FnMut(&Self::Item) -> bool {}
}
```



```
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 产出 `&i32` 类型。
    let mut iter = vec1.iter();
    // 对 vec 产出 `i32` 类型。
    let mut into_iter = vec2.into_iter();

    // 产出内容的引用是 `&&i32` 类型。解构成 `i32` 类型。
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // 产出内容的引用是 `&i32` 类型。解构成 `i32` 类型。
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组 `iter()`  产出 `&i32`。
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // 对数组的 `into_iter()` 通常产出 `&i32``。
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));
}
```



```
Find 2 in vec1: Some(2)
Find 2 in vec2: None
Find 2 in array1: Some(2)
Find 2 in array2: None
```

## 高阶函数



Rust 提供了高阶函数（Higher Order Function, HOF）。执行一个或多个函数来产生一个用处更大的函数。HOF 和惰性迭代器（lazy iterator）给 Rust 带来了函数式的风格（英文原文：HOFs and lazy iterators give Rust its functional flavor.）。

```
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 命令式方式（imperative approach）
    // 声明累加器变量
    let mut acc = 0;
    // 重复：0，1, 2, ... 到无穷大
    for n in 0.. {
        // 数字的平方
        let n_squared = n * n;

        if n_squared >= upper {
            // 若大于上限（upper limit）则退出循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数就累加值
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // 函数式方式（functional approach）
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)             // 所有自然数的平方
             .take_while(|&n| n < upper) // 小于上限
             .filter(|&n| is_odd(n))     // 为奇数
             .fold(0, |sum, i| sum + i); // 最后其后
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
```



```
Find the sum of all the squared odd numbers under 1000
imperative style: 5456
functional style: 5456
```

[Option](http://doc.rust-lang.org/core/option/enum.Option.html) 和 [迭代器](http://doc.rust-lang.org/core/iter/trait.Iterator.html) 实现了它们自己的高阶函数（英语原文：Option and Iterator implement their fair share of HOFs.）。



# 模块

Rust 提供了一套强大的模块系统，可以将代码按层次分成多个逻辑单元（模块），并在这些模块之间管理可见性（公开 public/私有 private）。

模块是一系列项的集合：函数，结构体，trait，`impl` 块，甚至其它模块。



## 可见性

项（item）默认情况下拥有私有的可见性（private visibility），不过可以加上 `pub` （public 的前 3 个字母）修饰语（modifier）来改变默认行为。一个模块之外的作用域只能访问该模块里面的公有项（public item）。



```
// 一个名为 `my` 的模块
mod my {
    // 在模块中的项默认带有私有可见性。
    fn private_function() {
        println!("called `my::private_function()`");
    }

    // 使用 `pub` 修饰语来改变默认可见性。
    pub fn function() {
        println!("called `my::function()`");
    }
    
    // 在同一模块中，项可以访问其它项，即使是私有属性。
    pub fn indirect_access() {
        print!("called `my::indirect_access()`, that\n> ");
        private_function();
    }

    // 项也可以嵌套。
    pub mod nested {
        pub fn function() {
            println!("called `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::nested::private_function()`");
        }
    }
    
    // 嵌套项的可见性遵循相同的规则。
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // 模块允许在拥有相同名字的项之间消除歧义。
    function();
    my::function();
    
    // 公有项，包括内部嵌套的公有项，可以在父级的模块中访问到。
    my::indirect_access();
    my::nested::function();

    // 一个模块中的私有项不能被直接访问，即使私有项嵌套在公有的模块中：

    // 报错！`private_function` 是私有的。
    //my::private_function();
    // 试一试 ^ 将此行注释去掉

    // 报错！ `private_function` 是私有的。
    //my::nested::private_function();
    // 试一试 ^ 将此行注释去掉    

    // 报错！ `private_nested` 是私有的模块。
    //my::private_nested::function();
    // 试一试 ^ 将此行注释去掉    

}
```



```
called `function()`
called `my::function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```



## 结构体的可见性

结构体对字段的可见性有额外的规定（Structs have an extra level of visibility with their fields）。其可见性默认为私有，也可以加上 `pub` 修饰语来改变默认属性。只有当从定义在外部的模块访问一个结构体时，这可见性才显得重要，并具有隐藏信息的目的（封装，encapsulatoin）（原文：This visibility only matters when a struct is accessed from outside the module where it is defined, and has the goal of hiding information (encapsulation)）。

```
mod my {
    // 一个公有的结构体，带有一个公有的泛型类型 `T` 的字段
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    // 一个公开的结构体，带有一个私有的泛型类型 `T` 的字段    
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        // 一个公有的构造器方法
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // 带有公有字段的公有的结构体，可以像平常一样构造
    let white_box = my::WhiteBox { contents: "public information" };

    // 并且它们的字段可以正常访问到。
    println!("The white box contains: {}", white_box.contents);

    // 带有私有字段的公有结构体不能使用字段名来构造。
    // 报错！`BlackBox` 含有私有字段。
    //let black_box = my::BlackBox { contents: "classified information" };
    // 试一试 ^ 将此行注释去掉


    // 不过带有私有字段的结构体可以使用公有的构造器来创建。
    let _black_box = my::BlackBox::new("classified information");

    // 并且一个结构体中的私有字段不能访问到。
    // 报错！`content` 字段是私有的。
    //println!("The black box contains: {}", _black_box.contents);
    // 试一试 ^ 将此行注释去掉    

}
```



```
The white box contains: public information
```



## `use` 声明

`use` 声明可以将一个完整的路径绑定到一个新的名字，从而更容易访问。

```
// 将 `deeply::nested::function` 路径绑定到 `other_function`。
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

fn main() {
    // 更容易访问 `deeply::nested::funcion`
    other_function();

    println!("Entering block");
    {
        // 这和 `use deeply::nested::function as function` 等价。
        // 此 `function()` 将覆盖掉的外部同名函数。
        use deeply::nested::function;
        function();

        // `use` 绑定拥有局部作用域。在这个例子中，`function()`
        // 的覆盖只限定在这个代码块中。
        println!("Leaving block");
    }

    function();
}
```



```
called `deeply::nested::function()`
Entering block
called `deeply::nested::function()`
Leaving block
called `function()`
```



## `super` 和 `self`

在路径上使用 `super` （父级）和 `self`（自身）关键字，可以在访问项时消除歧义和防止不必要的路径的硬编码。

```
fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }
    
    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
    
    pub fn indirect_call() {
        // 让我们从这个作用域中访问所有名为 `function` 的函数！
        print!("called `my::indirect_call()`, that\n> ");
        
        // `self` 关键字表示当前的模块作用域——在这个例子是 `my`。
        // 调用 `self::function()` 和直接访问 `function()` 两者都得到相同的结果，
        // 因为他们表示相同的函数。
        self::function();
        function();
        
        // 我们也可以使用 `self` 来访问 `my` 内部的另一个模块：
        self::cool::function();
        
        // `super` 关键字表示父级作用域（在 `my` 模块外面）。
        super::function();
        
        // 这将在 *crate* 作用域内绑定 `cool::function` 。
        // 在这个例子中，crate 作用域是最外面的作用域。
        {
            use cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
```



```
called `my::indirect_call()`, that
> called `my::function()`
called `my::function()`
called `my::cool::function()`
called `function()`
called `my::cool::function()`
```

## 文件分层

模块可以分配到文件/目录的层次结构中。让我们将[可见性小节例子](https://llever.com/rust-by-example-cn/mod/mod/visibility.html) 的代码拆开分到多个文件中：



```
tree
.
├── main.rs
└── my
    ├── inaccessible.rs
    ├── libnested.rlib
    ├── mod.rs
    └── nested.rs

1 directory, 5 files
```



main

```
// 此声明将会查找名为 `my.rs` 或 `my/mod.rs` 的文件，并将该文件的内容插入到
// 此作用域名为 `my` 的模块里面。
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}

```

在 `my/mod.rs` 文件：

```
// 类似地，`mod inaccessible` 和 `mod nested` 将找到 `nested.rs` 和
// `inaccessible.rs` 文件，并在它们各自的模块中插入它们的内容。
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}
```

在 `my/nested.rs` 文件：

```
pub fn function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}
```



在 `my/inaccessible.rs` 文件：

```
#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}
```



我们看到代码仍然正常运行，就和前面的一样：

```
called `my::function()`
called `function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```



# crate

crate（中文有“包，包装箱”之意）是 Rust 中的编译单元。不管什么时候调用 `rustc some_file.rs`，`some_file.rs` 都被当作 **crate 文件**。如果 `some_file.rs` 里面含有 `mod` 声明，那么模块文件的内容将在运行编译器之前与 crate 文件合并。换句话说， 模块**不会**单独进行编译，只有 crate 文件进行了编译（英文：modules do *not* get compiled individually, only crates get compiled）。

crate 可以编译成二进制可执行文件（binary）或库文件（library）。默认情况下，`rustc` 将从 crate 产生库文件。这种行为可以通过 `rustc` 的选项 `--crate-type` 覆盖。



## 库

让我们创建一个库，然后看看如何把它链接到另一个 crate。

```
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

```



```
rustc --crate-type=lib rary.rs
$ ll
total 40
-rw-r--r--  1 xxxxx  staff    14K  2 21 17:55 library.rlib
-rw-r--r--  1 xxxx  staff   273B  2 21 17:51 rary.rs
```

库的前缀为 “lib”，默认情况下它们跟随着 crate 文件命名（原文：by default they get named after their crate file），但此默认名称可以使用 [`crate_name` 属性](https://llever.com/rust-by-example-cn/crates/attribute/crate.html) 覆盖。



## `extern crate`

链接一个 crate 到这个新库，必须使用 `extern crate` 声明。这不仅会链接库，还会导入与库名相同的模块里面的所有项。适用于模块的可见性规则也适用于库。

```
$ tree
.
├── ku
│   ├── library.rlib
│   └── rary.rs
├── main
├── main.rs
└── my
    ├── inaccessible.rs
    ├── libnested.rlib
    ├── mod.rs
    └── nested.rs

2 directories, 8 files
```



```
rustc main.rs --extern rary=/Users/xxx/Desktop/rust/code/src/ku/library.rlib  && ./main

called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`
```



# 属性

属性是应用于某些模块、crate 或项的元数据（metadata）。这元数据可以用来：

* [代码的条件编译][cfg] 
* [设置 crate 名称、版本和类型（二进制文件或库）][crate] 
*  禁用 [lint][lint] （警告） 
* 启用编译器的特性（宏、全局导入（glob import））等] 
* 链接到一个非 Rust 语言的库 
* 标记函数作为单元测试（unit test） 
* 标记作为基准某个部分的函数

当属性用于一个完整的 crate 时，它们的语法为 `#![crate_attribute]`，当它们用于模块或项时，语法为 `#[item_attribute]`（注意少了感叹号 `!`）。

属性可以接受参数，有不同的语法形式：

* `#[attribute = "value"]`
* `#[attribute(key = "value")]`
* `#[attribute(value)]`



## 死代码 `dead_code`

编译器提供了 `dead_code`（死代码，无效代码） [*lint*](https://en.wikipedia.org/wiki/Lint_%28software%29)，这会对未使用的函数产生警告。可以加上**属性**来抑制这个 lint。

```
fn used_function() {}

// `#[allow(dead_code)]` 属性可以抑制 `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// 改正 ^ 增加一个属性来消除警告

fn main() {
    used_function();
}
```



```
function is never used: `noisy_unused_function`
```



## `crate`

`crate_type` 属性可以告知编译器 crate 是一个二进制的可执行文件还是一个库（甚至是哪种类型的库），`crate_time` 属性可以设定 crate 的名称。

```
// 这个 crate 是一个库文件
#![crate_type = "lib"]
// 库的名称为 “rary”
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

当用到 `crate_type` 属性时，就不再需要给 `rustc` 命令加上 `--crate-type` 标记。



```
$ rustc lib.rs
$ ls lib*
library.rlib
```



## `cfg`

条件编译可能通过两种不同的操作：

* `cfg` 属性：在属性位置中使用 `#[cfg(...)]`
* `cfg!` 宏：在布尔表达式中使用 `cfg!(...)`

两种形式使用参数的语法都相同。



```
// 这个函数仅当操作系统是 Linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// 而这个函数仅当操作系统**不是** Linux 时才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

fn main() {
    are_you_on_linux();
    
    println!("Are you sure?");
    println!("{:?}",cfg!(target_os = "linux"));
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```



```
You are *not* running linux!
Are you sure?
false
Yes. It's definitely *not* linux!
```



### 自定义条件

有部分条件如 `target_os` 在使用 `rustc` 时会隐式地提供，但是自定义条件必须使用 `--cfg` 标记来传给 `rustc`。



```
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    conditional_function();
}



```



不使用自定义的 `cfg` 标记：

```
rustc ./main.rs && ./main 
error[E0425]: cannot find function `conditional_function` in this scope
 --> ./main.rs:7:5
```



使用自定义的 `cfg` 标记：

```
$ rustc --cfg some_condition main.rs && ./main  
condition met!
```