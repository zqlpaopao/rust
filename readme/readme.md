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
