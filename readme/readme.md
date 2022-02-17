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

# 11、if let语法

```
let i = 0;  
match i {  
 0 => println!("zero"),  
 _ => {},  
}
```



放入主函数运行结果：

```
zero
```



这段程序的目的是判断 i 是否是数字 0，如果是就打印 zero。

现在用 if let 语法缩短这段代码：

```
let i = 0;
if let 0 = i {
    println!("zero");
}
```

if let 语法格式如下：

```
if let 匹配值 = 源变量 {
    语句块
}
```

可以在之后添加一个 else 块来处理例外情况。

if let 语法可以认为是只区分两种情况的 match 语句的"语法糖"（语法糖指的是某种语法的原理相同的便捷替代品）。

对于枚举类依然适用：

```
fn main() {
    let i = 0;
    //此处的=是
    if let 0 = i {
        println!("zero");
    }else{
        println!("not zero")
    }

    //区别对比
    enum Book {
        Papery(u32),
        Electronic(String)
    }
    enum Option<T>{
        Some(T)
    }


    let book = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}
```



```
zero
Not papery book
```



# 12、控制流

### 1、If

If是分支 (branch) 的一种特殊形式，也可以使用`else`和`else if`。  
与C语言不同的是，逻辑条件不需要用小括号括起来，但是条件后面必须跟一个代码块。  
Rust中的`if`是一个表达式 (expression)，可以赋给一个变量：

```
let x = 5;
let y = if x == 5 { 10 } else { 15 };
```



Rust是基于表达式的编程语言，有且仅有两种语句 (statement)：

1. <font color=red>**声明语句** (declaration statement)，比如进行变量绑定的`let`语句。</font>

2. <font color=red>**表达式语句** (expression statement)，它通过在末尾加上分号`;`来将表达式变成语句，  丢弃该表达式的值，一律返回unit`()`。</font>

表达式如果返回，总是返回一个值，但是语句不返回值或者返回`()`，所以以下代码会报错：

```
let y = (let x = 5);
let z: i32 = if x == 5 { 10; } else { 15; };
```



值得注意的是，在Rust中赋值 (如`x = 5`) 也是一个表达式，返回unit的值`()`。

### 2、for

Rust中的`for`循环与C语言的风格非常不同，抽象结构如下：

```
for var in expression {
code
}
```



```
fn main() {
    
    for i in 1..10{
        println!("{}",i)
    }
}

1
2
3
4
5
6
7
8
9


fn main() {
    
    for i in [1,2,3,4].iter(){
        println!("{}",i)
    }
}

1
2
3
4
```



其中`expression`是一个迭代器 (iterator)，具体的例子为`0..10` (不包含最后一个值)，  
或者`[0, 1, 2].iter()`。

### 3、while

Rust中的`while`循环与C语言中的类似。对于无限循环，Rust有一个专用的关键字`loop`。  
如果需要提前退出循环，可以使用关键字`break`或者`continue`，  
还允许在循环的开头设定标签 (同样适用于`for`循环)：

```
fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
                break 'outer;
            }
        println!("This point will never be reached");
        }
        println!("Exited the outer loop");
}
```

```
Entered the outer loop
Entered the inner loop
Exited the outer loop
```



### 4、match

其中`|`用于匹配多个值，`...`匹配一个范围 (包含最后一个值)，并且`_`在这里是必须的，  
因为`match`强制进行穷尽性检查 (exhaustiveness checking)，必须覆盖所有的可能值。  
如果需要得到`|`或者`...`(新版本..=)匹配到的值，可以使用`@`绑定变量：

```
fn main() {
    let day = 4;
    match day {
        0 | 6 => println!("weekend"),
        1 ..= 5 => println!("weekday"),
        _ => println!("invalid"),
    }
}
```

```
weekday
```

可以使用`@`绑定变量

```
fn main() {
    let day = 4;
    match day {
        0 | 6 => println!("weekend"),
        1 ..= 5 => println!("weekday"),
        _ => println!("invalid"),
    }

    let x = 1;
    match x {
        e @ 1 ..= 5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
}
```

```
got a range element 1
```

使用`ref`关键字来得到一个引用：

```
fn main() {
    let x = 5;
    let mut y = 5;
    match x {
        // the `r` inside the match has the type `&i32`
        ref r => println!("Got a reference to {}", r),
    }
    match y {
        // the `mr` inside the match has the type `&i32` and is mutable
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }
}
```



```
Got a reference to 5
Got a mutable reference to 5
```



再看一个使用`match`表达式来解构元组的例子：

```
fn main() {
    let pair = (0, -2);
    match pair {
            (0, y) => println!("x is `0` and `y` is `{:?}`", y),
            (x, 0) => println!("`x` is `{:?}` and y is `0`", x),
            _ => println!("It doesn't matter what they are"),
    }
}
```



```
x is `0` and `y` is `-2`
```

`match`的这种解构同样适用于结构体或者枚举。如果有必要，还可以使用`..`来忽略域或者数据：

```
fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    enum OptionalInt {
        Value(i32),
        Missing,
    }
    let x = OptionalInt::Value(5);
    match x {
        // 这里是 match 的 if guard 表达式，我们将在以后的章节进行详细介绍
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }
}
```



```
x is 0
Got an int!
```

看下==

```
fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    enum OptionalInt {
        Value(i32),
        Missing,
    }
    let x = OptionalInt::Value(5);
    match x {
        // 这里是 match 的 if guard 表达式，我们将在以后的章节进行详细介绍
        OptionalInt::Value(i) if i == 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }
}
```

```
x is 0
Got an int bigger than five!
```

此外，Rust还引入了`if let`和`while let`进行模式匹配：

```
fn main() {
    let number = Some(7);
    let mut optional = Some(0);

    // If `let` destructures `number` into `Some(i)`, evaluate the block.
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number!");
    }
    // While `let` destructures `optional` into `Some(i)`, evaluate the block.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        println!("{}",i);
        println!("optional---{:#?}",optional)
    }
}
```

```
Matched 7!
`i` is `0`. Try again.
0
optional---Some(
    1,
)
`i` is `1`. Try again.
1
optional---Some(
    2,
)
`i` is `2`. Try again.
2
optional---Some(
    3,
)
`i` is `3`. Try again.
3
optional---Some(
    4,
)
`i` is `4`. Try again.
4
optional---Some(
    5,
)
`i` is `5`. Try again.
5
optional---Some(
    6,
)
`i` is `6`. Try again.
6
optional---Some(
    7,
)
`i` is `7`. Try again.
7
optional---Some(
    8,
)
`i` is `8`. Try again.
8
optional---Some(
    9,
)
`i` is `9`. Try again.
9
optional---Some(
    10,
)
Greater than 9, quit!
10
optional---None

```

# 12、option的一些方法

[Option in std::option - Rust](https://doc.rust-lang.org/std/option/enum.Option.html#method.map)

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

# 13、关键字dyn

**dyn是trait对象类型的前缀**

dyn关键字用于强调相关trait的方法是动态分配的。要以这种方式使用trait，它必须是“对象安全”的。

与泛型参数或植入型特质不同，编译器不知道被传递的具体类型。也就是说，该类型已经被抹去。因此，一个dyn Trait引用包含两个指针。一个指针指向数据（例如，一个结构的实例）。另一个指针指向方法调用名称与函数指针的映射（被称为虚拟方法表各vtable）。

impl trait 和 dyn trait 在Rust分别被称为静态分发和动态分发，即当代码涉及多态时，需要某种机制决定实际调动类型。

# 14、函数

要声明一个函数，需要使用关键字`fn`，后面跟上函数名，比如

```
fn add_one(x: i32) -> i32 {
x + 1
}
```

- <font color=red size=3x>其中函数参数的类型不能省略，可以有多个参数，但是最多只能返回一个值</font>  

- <font color=red>提前返回使用`return`关键字。Rust编译器会对未使用的函数提出警告</font>

- <font color=red>可以使用属性`#[allow(dead_code)]`禁用无效代码检查</font>

Rust有一个特殊特性适用于发散函数 (diverging function)，它不返回：

```
fn diverges() -> ! {
    panic!("This function never returns!");
}
```

其中`panic!`是一个宏，使当前执行线程崩溃并打印给定信息。返回类型`!`可用作任何类型：

```
let x: i32 = diverges();
let y: String = diverges();
```

# 15、运行的详细堆栈信息

```
环境变量
RUST_BACKTRACE=full
RUST_BACKTRACE=1
```

## 匿名函数

闭包*在Rust 是一个稍微专业的语法，可以捕捉到封闭的环境函数。 这种语法和能力使它们在运行使用非常方便。一些特性包括：

* 使用 `||` 替代 `()` 围绕输入变量。
* 输入和返回类型可以推断出。
* 输入变量名称必须指定。
* 主体定界 (`{}`) 是可选的一个表达式。强制性其他。
* 外环境变量可能被捕获。
* 调用闭包和函数与 `call(var)`是完全一样的
* *.也被称为lambda表达式或匿名函数。

```
fn main() {
    // Increment via closures and functions.
    fn  function            (i: i32) -> i32 { i + 1 }

    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    //闭包最完整的方式
    let closure_annotated = |i: i32| -> i32 { i + 1 };

    //闭包可以不指定传入的值的类型和返回值的类型 编译器会自动推断
    //闭包可以没有 {} 有具体的执行体就行
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("annotated closure: {}", closure_annotated(i));
    println!("inferred closure: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    //闭包可以不穿入参数
    let one = || 1;
    println!("closure returning one: {}", one());

    // It is possible to capture variables from the enclosing
    // environment; something which is impossible with functions.
    let professor_x = "Charles Xavier";

    // A closure which takes no argument, returning nothing, prints
    // a variable from the enclosing scope.
    //闭包可以获取外部的变量
    let print = || println!("Professor X's name is: {}", professor_x);
    println!("{}",professor_x);

    // Call the closure.
    print();
}
```

```
function: 2
annotated closure: 2
inferred closure: 2
closure returning one: 1
Charles Xavier
Professor X's name is: Charles Xavier
```

## 闭包的捕获

闭包是内在的灵活性，并会做哪些需要的功能，使撤销工作无需注释。这允许捕捉灵活适应使用的情况，有时会移动，有时借用。闭包可以捕捉变量：

* 通过参考: `&T`
* 通过可变引用: `&mut T`
* 通过传值: `T`

它们优先通过引用捕获变量并仅在需要时使用。

```
fn main() {
    let color = "green";

    // A closure to print `color` which immediately borrows (`&`)
    // `color` and stores the borrow and closure in the `print`
    // variable. It will remain borrowed until `print` goes out of
    // scope. `println!` only requires `by reference` so it doesn't
    // impose anything more restrictive.
    //可以捕获外部的变量
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();
    print();

    let mut count = 0;

    // A closure to increment `count` could take either `&mut count`
    // or `count` but `&mut count` is less restrictive so it takes
    // that. Immediately borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside.
    // Thus, calling the closure mutates the closure which requires
    // a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure.
    inc();
    inc();
    println!("{}",count);

    //let reborrow = &mut count;
    // ^ TODO: try uncommenting this line.

    // A non-copy type.
    let movable = Box::new(3);//Rust中使用的是Box::new来对数据进行封箱

    // `drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        drop(movable);//释放内存
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // println!("{:#?}",movable);//value borrowed here after moverustc(E0382)
    //consume();
    // ^ TODO: Try uncommenting this line.
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

[智能指针](https://so.csdn.net/so/search?q=%E6%99%BA%E8%83%BD%E6%8C%87%E9%92%88&spm=1001.2101.3001.7020)是Rust中一种特殊的数据结构。它与普通指针的本质区别在于普通指针是对值的借用，而智能指针通常拥有对数据的所有权。在Rust中，如果你想要在堆内存中定义一个对象，并不是像Java中那样直接new一个，也不是像C语言中那样需要手动malloc函数来分配内存空间。Rust中使用的是`Box::new`来对数据进行封箱，而`Box<T>`就是我们今天要介绍的智能指针之一。除了`Box<T>`之外，Rust标准库中提供的智能指针还有`Rc<T>`、`Ref<T>`、`RefCell<T>`等等。在详细介绍之前，我们还是先了解一下智能指针的基本概念。

## 闭包获取所有权

其中闭包`plus_num`借用了它作用域中的`let`绑定`num`。如果要让闭包获得所有权，  
可以使用`move`关键字：

```
fn main() {
    let mut num = 5;
    {
    let mut add_num = move |x: i32| num += x; // 闭包通过move获取了num的所有权
    add_num(5);
    }
    println!("{}",num);
    // 下面的num在被move之后还能继续使用是因为其实现了Copy特性
    // 具体可见所有权(Owership)章节
    assert_eq!(5, num);
}
```

```
5
```



# 16、高阶函数

Rust 还支持高阶函数 (high order function)，允许把闭包作为参数来生成新的函数：

```
fn add_one(x: i32) -> i32 { x + 1 }

//where从句，限定泛型的
fn apply<F>(f: F, y: i32) -> i32
where F: Fn(i32) -> i32
{
    f(y) * y
}
fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}
fn main() {
    let transform: fn(i32) -> i32 = add_one;

    //注意2i32的值是2 类型是i32
    let f0 = add_one(2i32) * 2;
    println!("{}",f0);

    let f1 = apply(add_one, 2);//6

    let f2 = apply(transform, 2);
    println!("{}, {}, {}", f0, f1, f2);

    let closure = |x: i32| x + 1;
    let c0 = closure(2i32) * 2;
    let c1 = apply(closure, 2);
    let c2 = apply(|x| x + 1, 2);
    println!("{}, {}, {}", c0, c1, c2);
    let box_fn = factory(1i32);
    let b0 = box_fn(2i32) * 2;
    let b1 = (*box_fn)(2i32) * 2;
    let b2 = (&box_fn)(2i32) * 2;
    println!("{}, {}, {}", b0, b1, b2);
    let add_num = &(*box_fn);
    let translate: &dyn Fn(i32) -> i32 = add_num;
    let z0 = add_num(2i32) * 2;
    let z1 = apply(add_num, 2);
    let z2 = apply(translate, 2);
    println!("{}, {}, {}", z0, z1, z2);
}
```



```
6
6, 6, 6
6, 6, 6
6, 6, 6
6, 6, 6
```

## 方法

Rust通过`impl`关键字在`struct`、`enum`或者`trait`对象上实现方法调用语法 (method call syntax)。  
关联函数 (associated function) 的第一个参数通常为`self`参数，有3种变体：

* `self`，允许实现者移动和修改对象，对应的闭包特性为`FnOnce`。
* `&self`，既不允许实现者移动对象也不允许修改，对应的闭包特性为`Fn`。
* `&mut self`，允许实现者修改对象但不允许移动，对应的闭包特性为`FnMut`。

不含`self`参数的关联函数称为静态方法 (static method)。

```
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// self，允许实现者移动和修改对象，对应的闭包特性为`FnOnce`。
// &self，既不允许实现者移动对象也不允许修改，对应的闭包特性为`Fn`。
// &mut self，允许实现者修改对象但不允许移动，对应的闭包特性为`FnMut`。
impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
    fn area(&self) -> f64 {
        println!("{}",self.x);
        println!("{}",self.y);
        return std::f64::consts::PI * (self.radius * self.radius)
        
    }
}


fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());
    // use associated function and method chaining
    println!("{}", Circle::new(0.0, 0.0, 2.0).area());
}
```



```
0
0
12.566370614359172
0
0
12.566370614359172
```



# 

```
Compiling code v0.1.0 (/Users/xxxxx/Desktop/rust/code)
warning: function is never used: `noisy_unused_function`
 --> src/main.rs:7:4
  |
7 | fn noisy_unused_function() {}
  |    ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `code` (bin "code") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/code`
```

可以看到要在不用的方法上边写



注意在实际程序中，需要将死代码清除掉。在这些例子中，我们是出于知识点讲解的需要才特意加上了一些死代码。



# 16、可见性

项（item）默认情况下拥有私有的可见性（private visibility），不过可以加上 `pub` （public 的前 3 个字母）修饰语（modifier）来改变默认行为。一个模块之外的作用域只能访问该模块里面的公有项（public item）。



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
called `function()`
called `my::function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```

## 结构体的可见性

结构体对字段的可见性有额外的规定（Structs have an extra level of visibility with their fields）。其可见性默认为私有，也可以加上 `pub` 修饰语来改变默认属性。只有当从定义在外部的模块访问一个结构体时，这可见性才显得重要，并具有隐藏信息的目的（封装，encapsulatoin）（原文：This visibility only matters when a struct is accessed from outside the module where it is defined, and has the goal of hiding information (encapsulation)）。

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
        //获取
        // pub fn get(&self) -> T {
        //     self.contents
        // }
    }
}

fn main() {
    // 带有公有字段的公有的结构体，可以像平常一样构造
    let white_box = my::WhiteBox { contents: "public information" };

    // 并且它们的字段可以正常访问到。
    println!("The white box contains: {}", white_box.contents);

    // 带有私有字段的公有结构体不能使用字段名来构造。
    // 报错！`BlackBox` 含有私有字段。
    //field `contents` of struct `BlackBox` is private
    // let black_box = my::BlackBox { contents: "classified information" };
    // 试一试 ^ 将此行注释去掉


    // 不过带有私有字段的结构体可以使用公有的构造器来创建。
    let _black_box = my::BlackBox::new("classified information");

    // 并且一个结构体中的私有字段不能访问到。
    // 报错！`content` 字段是私有的。
    // ^ private field
    // println!("The black box contains: {}", _black_box.contents);
    // 试一试 ^ 将此行注释去掉    

}

```



```
 Running `target/debug/code`
The white box contains: public information
```





# `use` 声明

`use` 声明可以将一个完整的路径绑定到一个新的名字，从而更容易访问。



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
        //覆盖只是在这个作用域内
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

# `super` 和 `self`

在路径上使用 `super` （父级）和 `self`（自身）关键字，可以在访问项时消除歧义和防止不必要的路径的硬编码。



```
fn function() {
    println!("called `function()`");
}

mod cool {
    #[allow(dead_code)]
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



# 文件分层

模块可以分配到文件/目录的层次结构中。让我们将[可见性小节例子](https://llever.com/rust-by-example-cn/mod/mod/visibility.html) 的代码拆开分到多个文件中：



```
$ tree
.
├── main.rs
└── my
    ├── inaccessible.rs
    ├── mod1.rs
    └── nested.rs

1 directory, 4 files
```



在main.rs中

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



```
called `my::function()`
called `function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```

[

# 17、crate

crate（中文有“包，包装箱”之意）是 Rust 中的编译单元。不管什么时候调用 `rustc some_file.rs`，`some_file.rs` 都被当作 **crate 文件**。如果 `some_file.rs` 里面含有 `mod` 声明，那么模块文件的内容将在运行编译器之前与 crate 文件合并。换句话说， 模块**不会**单独进行编译，只有 crate 文件进行了编译（英文：modules do *not* get compiled individually, only crates get compiled）。

crate 可以编译成二进制可执行文件（binary）或库文件（library）。默认情况下，`rustc` 将从 crate 产生库文件。这种行为可以通过 `rustc` 的选项 `--crate-type` 覆盖。



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
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib

```

库的前缀为 “lib”，默认情况下它们跟随着 crate 文件命名（原文：by default they get named after their crate file），但此默认名称可以使用 [`crate_name` 属性](https://llever.com/rust-by-example-cn/crates/attribute/crate.html) 覆盖。



# `extern crate🍵`

链接一个 crate 到这个新库，必须使用 `extern crate` 声明。这不仅会链接库，还会导入与库名相同的模块里面的所有项。适用于模块的可见性规则也适用于库。

```
// 链接到 `library`（库），导入 `rary` 模块里面的项
extern crate rary;

fn main() {
    rary::public_function();

    // 报错！ `private_function` 是私有的
    //rary::private_function();

    rary::indirect_access();
}

```



```
# library.rlib 是已编译好的库的路径，假设在这里它在同一目录下：
# （原文：Where library.rlib is the path to to the compiled library, 
# assumed that it's in the same directory here:）
$ rustc executable.rs --extern rary=library.rlib && ./executable
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`

```



# 18、属性

属性是应用于某些模块、crate 或项的元数据（metadata）。这元数据可以用来：

* [代码的条件编译][cfg] * [设置 crate 名称、版本和类型（二进制文件或库）][crate] * 禁用 [lint][lint] （警告） * 启用编译器的特性（宏、全局导入（glob import））等] * 链接到一个非 Rust 语言的库 * 标记函数作为单元测试（unit test） * 标记作为基准某个部分的函数

当属性用于一个完整的 crate 时，它们的语法为 `#![crate_attribute]`，当它们用于模块或项时，语法为 `#[item_attribute]`（注意少了感叹号 `!`）。

属性可以接受参数，有不同的语法形式：

* `#[attribute = "value"]`
* `#[attribute(key = "value")]`
* `#[attribute(value)]`

## 死代码 `dead_code`

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

# `crate`

`crate_type` 属性可以告知编译器 crate 是一个二进制的可执行文件还是一个库（甚至是哪种类型的库），`crate_time` 属性可以设定 crate 的名称。

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



当用到 `crate_type` 属性时，就不再需要给 `rustc` 命令加上 `--crate-type` 标记。

```
$ rustc lib.rs
$ ls lib*
library.rlib
```



# `cfg`

条件编译可能通过两种不同的操作：

* `cfg` 属性：在属性位置中使用 `#[cfg(...)]`
* `cfg!` 宏：在布尔表达式中使用 `cfg!(...)`

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
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```



### 自定义条件

有部分条件如 `target_os` 在使用 `rustc` 时会隐式地提供，但是自定义条件必须使用 `--cfg` 标记来传给 `rustc`。

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
$ rustc custom.rs && ./custom
No such file or directory (os error 2)

```

使用自定义的 `cfg` 标记：

```
$ rustc --cfg some_condition custom.rs && ./custom
condition met!

```