# 1. 安装rust

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

# 2. rustup安装

rustup 是rust官方的版本管理工具。应当作为安装 Rust 的首选。

项目主页是: https://github.com/rust-lang-nursery/rustup.rs

卸载

```
rustup self uninstall
```



# 3. 安装vscode编译器

下载 VScode 请打开官网 https://code.visualstudio.com/ 下载编辑器。

依赖 如本章第一节所述，准备好 racer，rust 源代码，rustfmt，rls 这四样东西，并且配置好相应的环境变量，此不赘述。

安装 Rust 扩展 Rust 打开 VScode 编辑器； 按 Ctrl + p 打开命令面板； 在编辑器中上部浮现出的输入框中，输入 ext install vscode-rust，会自动搜索可用的插件，搜索出来后，点击进行安装；使用VScode打开任意一个.rs文件，插件首次启动会自动引导用户完成配置。 注:推荐使用RLS模式，即使用Rust Langular Server提供各项功能支持

[![image-20220214112618639](https://github.com/zqlpaopao/rust/raw/main/readme/readme.assets/image-20220214112618639.png)](https://github.com/zqlpaopao/rust/blob/main/readme/readme.assets/image-20220214112618639.png)

[![image-20220214112931446](https://github.com/zqlpaopao/rust/raw/main/readme/readme.assets/image-20220214112931446.png)](https://github.com/zqlpaopao/rust/blob/main/readme/readme.assets/image-20220214112931446.png)

[![image-20220214113058105](https://github.com/zqlpaopao/rust/raw/main/readme/readme.assets/image-20220214113058105.png)](https://github.com/zqlpaopao/rust/blob/main/readme/readme.assets/image-20220214113058105.png)

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



# 4. Cargo 教程

## 4.1 Cargo 是什么

Cargo 是 Rust 的构建系统和包管理器。

Rust 开发者常用 Cargo 来管理 Rust 工程和获取工程所依赖的库。在上个教程中我们曾使用 cargo new greeting 命令创建了一个名为 greeting 的工程，Cargo 新建了一个名为 greeting 的文件夹并在里面部署了一个 Rust 工程最典型的文件结构。这个 greeting 文件夹就是工程本身。

## 4.2 Cargo 功能

Cargo 除了创建工程以外还具备构建（build）工程、运行（run）工程等一系列功能，构建和运行分别对应以下命令：

```
cargo build 
cargo run 
```

Cargo 还具有获取包、打包、高级构建等功能，详细使用方法参见 Cargo 命令。

# 5. 运行代码方式

## 5.1 cargo

不用生成二进制文件的

```
cargo build
cargo build —release # 这个属于优化编译
cargo run 可直接运行
cargo run # 编译和运行合在一起
```

## 5.2 rustc

生成二进制文件的

```
rustc helloworld.rs
rustc helloworld.rs -O # 也可以选择优化编译
```

## 5.3 使用cargo创建项目

- 创建项目 hellorust

```
ps: cargo new hellorust —bin
```

- 查看目录结构

```
ps: tree # win10 powershell 自带有 tree 查看文件目录结构的功能  
└─hellorust  
——└─src
```

这里显示的目录结构，在hellorust目录下有 src 文件夹和 Cargo.toml 文件，同时这个目录会初始化为 git 项目

- 查看Cargo.toml文件

```
ps: cat Cargo.toml  
[package]  
name = “hellorust”  //包名
version = “0.1.”  //包的版本
authors = [“YourName “]  //作者
edition = "2021" //rust的发布版本
[dependencies]//依赖的包
```

- 编辑src目录下的main.rs文件

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

这里的 `let rust = "Rust"` 是把 rust 变量绑定为 “Rust” ，
`println!("Hello, {}!", rust);`里把 rust 变量的值代入到`"Hello, {}!"`中的`{}`。

- 编译和运行

```
ps: cargo build  
ps: cargo build —release # 这个属于优化编译  
ps: ./target/debug/hellorust.exe  
ps: ./target/release/hellorust.exe # 如果前面是优化编译，则这样运行  
ps: cargo run # 编译和运行合在一起  
ps: cargo run —release # 同上，区别是是优化编译的
```

## 5.4 cargo run

会直接运行

```
cargo run  
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/cargo_new`
```

如果没有改遍会直接运行 如果有改变会编译在运行

## 5.5 cargo check

此命令是检测程序是否有错误的，和buil的的区别在于，此命令不回进行编译，因此更快

```
$ cargo check
    Checking cargo_new v0.1.0 (/Users/zql/Desktop/rust/cargo_new)
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
```

## 5.6 为发布构建

```
cargo build --ralease
```

会进行代码优化，编译较慢，但是程序运行较快，进行代码优化 会在target/release下生成可执行文件，不是target/debug [![image](https://user-images.githubusercontent.com/43371021/230619487-a4f687d9-171e-48ee-ac80-8e7544266089.png)](https://user-images.githubusercontent.com/43371021/230619487-a4f687d9-171e-48ee-ac80-8e7544266089.png)

## 5.7 猜数游戏

```
use std::io;

//use 是引入库
//std 标准库 的io

fn main() {
    println!("猜数游戏!");

    //let 是定义变量
    //mut 是可引用的 
    //: String 是数据类型，自动推导的
    //任何变量是不可变的 immutable 加上mut是可变的
    // let mut foo = 1;
    // let bar = foo;
    // foo = 2;//cannot mutate immutable variable `foo`
    // foo= 2; 加上mut 是可变的

    //String utf8 可自懂扩展大小 标准库
    let mut guess = String::new();

    //io stdin 是控制台输入  read
    io::stdin().read_line(&mut guess).expect("无法读取行");//不使用expect 也可以，会提示为对异常进行处理
    //mut 是可变的 &引用默认也是不可变的，只能引用&mut 就是引用可变的
    //如果不用use 引入 可以这样写// &mut 引用可变对象，否则是错误的
    // std::io::stdin().read_line(guess).expect("错误信息并且推出")

    //read_line返回的是io::Result 枚举类型
    //io::Result Ok Err 也可以写成这昂
   match  std::io::stdin().read_line(&mut guess){
        Ok(n)=>{
            println!("{n} byte read" )
        }
        Err(error)=>{
            println!("read{error}")
        }
   }
    

    println!("你猜测的数是 {}",guess);
}
```

## 5.8 包的crate的管理

在cargo.toml中引入

```
rand = "0.3.14"
rand = "^0.3.14"
^代表和此版本兼容的所有版本都可以
```

然后会在cargo build的下载 https://crates.io/search?q=rand [![image](https://user-images.githubusercontent.com/43371021/230626714-86eaa893-eb1f-4b4f-8e27-af80afae5735.png)](https://user-images.githubusercontent.com/43371021/230626714-86eaa893-eb1f-4b4f-8e27-af80afae5735.png)

修改下载源

```
首先进入电脑的cargo目录，MAC OS 默认安装在 ~/.cargo下：

cd ~/.cargo

创建一个config文件，这里使用vim编辑器：

vim config

进到编辑界面后，键入i，切换到插入模式

将下面的代码贴进去：

[source.crates-io]

registry = "https://github.com/rust-lang/crates.io-index"

replace-with = 'ustc'

[source.ustc]

registry = "git://mirrors.ustc.edu.cn/crates.io-index"

此步骤相当于修改了crates的下载源，切换到国内镜像。

按esc退出编辑模式，键入:wq，表示保存并退出即可。

再次尝试，就可以愉快的下载安装依赖啦~！
```

## 5.9 cargo.lock

是符合本项目的包的版本 如果存在会优先使用 如果想升级的话在cargo.toml中国呢指定版本 运行

```
cargo update
```

或者直接执行cargo build 会自动进行升级 升级会覆盖cargo.lock

## 5.10 生成随机数

```
use std::io;

use rand::Rng;//trait 相当于interface
fn main(){
    println!("猜数游戏-rand");

    let mut guess = String::new();

    let rand_num = rand::thread_rng().gen_range(1, 101);
    println!("s随机数为{}",rand_num);
    
    match io::stdin().read_line(&mut guess) {
        Ok(n)=>{
            println!("read {n} byte" );
        }
        Err(error)=>{
            println!("read {error}")
        }
        
    }
    println!("猜的数字为{}",guess)
}
```

## 5.11 比较输入和随机数大小

```
use rand::Rng;
fn main(){
    println!("猜数游戏");

    let rand_num = rand::thread_rng().gen_range(1, 100);
    println!("生成的随机数是{}",rand_num);


//loop是循环
loop {
    let mut guess = String::new();

    match std::io::stdin().read_line(&mut guess){
        Ok(n)=>println!("read {n} byte"),
        Err(error)=>println!("read err:{error}")
    }

    let guess:u32 = match  guess.trim().parse() {
        Ok(n)=>n,
        Err(error)=>{
            println!("error - err {error}");
            //continue是跳出当前
           continue;
        },
        
    };

    println!("输入的参数是{} rand的📖是{}",guess,rand_num);

    match guess.cmp(&rand_num){
        //break是终止loop循环
        //返回的是Ordering类型
        std::cmp::Ordering::Equal => {println!("win");break},
        std::cmp::Ordering::Greater=> println!("is max"),
        std::cmp::Ordering::Less =>  println!("is less"),
        //都没匹配
        // Other => {println!("other")},
    }
}
   
}
```



# 6. 数据类型

### 整数型（Integer）

整数型简称整型，按照比特位长度和有无符号分为一下种类：

| 位长度  | 有符号 | 无符号 |
| ------- | ------ | ------ |
| 8-bit   | i8     | u8     |
| 16-bit  | i16    | u16    |
| 32-bit  | i32    | u32    |
| 64-bit  | i64    | u64    |
| 128-bit | i128   | u128   |
| arch    | isize  | usize  |

isize 和 usize 两种整数类型是用来衡量数据大小的，它们的位长度取决于所运行的目标台，如果是 32 位架构的处理器将使用 32 位位长度整型。

整数的表述方法有以下几种：

| 进制                 | 例          |
| -------------------- | ----------- |
| 十进制               | 98_222      |
| 十六进制             | 0xff        |
| 八进制               | 0o77        |
| 二进制               | 0b1111_0000 |
| 字节(只能表示 u8 型) | b'A'        |

很显然，有的整数中间存在一个下划线，这种设计可以让人们在输入一个很大的数字时更容易判断数字的值大概是多少。 [![image](https://user-images.githubusercontent.com/43371021/230719314-8bb2ca3e-cc22-44c9-b8f4-a1bdee7f05dd.png)](https://user-images.githubusercontent.com/43371021/230719314-8bb2ca3e-cc22-44c9-b8f4-a1bdee7f05dd.png)

整数溢出

- u8的范围是0-255，如果把一个u8变量值变为256，那么
- 调试模式下编译 rust会检测证书溢出，如果溢出就会报错
- 发布模式下 --release 编译不会检测panic发生 如果发生溢出 就会 发生环绕操作
- 256 变为 0 257 变为1 但是程序不回panic

## 6.1 浮点数型（Floating-Point）

Rust 与其它语言一样支持 32 位浮点数（f32）和 64 位浮点数（f64）。默认情况下，64.0 将表示 64 位浮点数，因为现代计算机处理器对两种浮点数计算的速度几乎相同，但 64 位浮点数精度更高。 float64精度更高，也是默认类型

#### 实例

```
fn main() {  
 let x = 2.0; // f64  
 let y: f32 = 3.0; // f32  
}
```

## 6.2 布尔型

布尔型用 bool 表示，值只能为 true 或 false。

## 6.3 字符型

字符型用 char 表示。

Rust的 char 类型大小为 4 个字节，代表 Unicode标量值，这意味着它可以支持中文，日文和韩文字符等非英文字符甚至表情符号和零宽度空格在 Rust 中都是有效的 char 值。

**注意** 由于中文文字编码有两种（GBK 和 UTF-8），所以编程中使用中文字符串有可能导致乱码的出现，这是因为源程序与命令行的文字编码不一致，所以在 Rust 中字符串和字符都必须使用 UTF-8 编码，否则编译器会报错。

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

## 6.4 字符串类型

最底层的是不定长类型`str`，更常用的是字符串切片`&str`和堆分配字符串`String`

其中字符串切片是静态分配的，有固定的大小，并且不可变，而堆分配字符串是可变的。

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

## 6.5 元组

具有固定大小的有序列表，每个元素都有自己的类型，通过解构或者索引来获得每个元素的值。

- 元组不可修改，长度不可修改
- 元组的数据访问使用tup.0 tup.1

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
500,6.4,1
500,6.4,1
```

## 6.6 数组

具有固定大小，并且元素都是同种类型，可表示为`[T; N]`。

- 数组的长度不可变
- 数组访问用下标
- 数据存在stack上 不是堆上
- vector 也是数组 长度可变
- 数组类型[类型;长度]
- 超过索引的范围编译不会报错，运行会报错

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
1
bbb
3
3
1
2
4
```

## 6.7 切片

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



    //注意要&用 用1..2  go的是1:2 但是都是前闭后开的
    let arr = &arr[1..2];

    println!("{}",arr[0]);
    // println!("{}",arr[1]);//thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/main.rs:13:19
   
}
3
0
9

2
```

## 6.8 指针和引用

- 解除引用使用 `*`
- 构析使用 `&`, `ref`, 和 `ref mut`

```
fn main() {


// raw pointers
let x = 5;
let raw = &x as *const i32;

let x1 = "hello";
let raw1 = &x1;
println!("{}",x1);//hello
println!("{}",*raw1);//hello 和go一样 可以识别第一层 但是第二层就是数据的地址了
println!("{}",raw1);//hello 和go一样 可以识别第一层 但是第二层就是数据的地址了

let points_at = unsafe { *raw };
let points_at1 =  *raw1 ;
println!("{}",points_at);
println!("{}",points_at1)
// // functions
// fn foo(x: i32) -> i32 { x }
// let bar: fn(i32) -> i32 = foo;

}
hello
hello
5
hello
```

## 6.9 函数

：具有函数类型的变量实质上是一个函数指针

- 声明使用fn
- 使用蛇形命令_ 分割
- 形参数必须指明类型
- 语句是执行动作的一些指令
- 表达式会计算产生一个值

```
fn main(){
    let x = 5;

    let y = {
        let x= 1;
        //如果没加;就是一个表达式
        //加了就是一个语句，是没有值的 是 `()`
        //语句没有返回值，是一些指令的操作
        //表达式才会产生结果
        x+3
    };
    println!("y is {}",y)
   
}
```

返回返回值

- 函数->符号声明返回值的类型，但是不恶意为返回值命名 ，和go是有区别的
- 在rust 返回值就是最后一个表达值的值
- 如果想提前返回 使用return

```
fn main(){
 let five = five();
 println!("return value is {}",five);

}



fn five()->i32{
    //此时是一个表达式
    5

    //return 也可以,有return的时候是可以加;的 
    // return 5;
    // return 5

    //此时是一个语句,返回时（）
    // 5;
}
```

# 7.注释

- 文档注释

# 8. if 表达式

```
if five ==3 {

    }else if five == 4 {
        
    }else{
        
    }
```

如果代码的if esle 比较多 使用match 会比较整洁

## 8.1 match 重构

```
fn main(){
    let five = five();

    match five {
        3 => println!("is 3"),
        4 => println!("is 4"),
	//都不满足走 相当于else
        _ => println!("is 5 good")
    }

    println!("return value is {}",five);

}



fn five()->i32{
    //此时是一个表达式
    5

    //return 也可以,有return的时候是可以加;的 
    // return 5;
    // return 5

    //此时是一个语句,返回时（）
    // 5;
}
```

## 8.2 在let 中使用if

因为if是一个表达式，所以可以将它放在let的右边

```
fn main(){
    let value = if true { 5} else{6};
    println!("{}",value);

}
```

# 9. 循环

rust 提供了三种循环 loop while for

loop 循环珂使用break 终止循环 return 也是可以的 ,可以使用

```
fn main(){
    
    let mut i = 0;
    let num = loop {
        i += 1;

        if i == 2{
            println!("is continue {}",i);
            continue;
        }else if i == 3{
        println!("is break {}",i);

            break i * 10;
            //return也可以
        }
        println!("is loop {}",i);
        
    };
    println!("is num {}",num);

}

s loop 1
is continue 2
is break 3
is num 30
```

while 每次循环之前判断一次条件，符合预期才会执行

```
fn main(){
    
    let mut i = 3;
    
    // while 是没有返回值的 因为条件先判断了
    // let num = while i != 3 {
    //     i -= 1;
    // };


     while i != 0 {
        i -= 1;
    };

}
```

for 遍历集合

range 指定开始和结束数字，但是不包含结束数字 rev 可以反转range

```
fn main(){
    
    //实现倒计时

    let num = [1,2,3];

    for v in num.iter(){
        println!("for in {}",v)
    }
    //for in 1
    // for in 2
    // for in 3

    //rev可以反转
    for num in (1..4).rev(){
        println!("for in  () {}",num)

    }
    //for in  () 3
    //for in  () 2
    //for in  () 1

}
```

# 10. 所有权

- 每个值都有一个变量，这个变量是这个值的所有者
- 每个值同时只有一个所有者
- 当超出所有者作用域（scope）时候，该值被删除

## 10.1 String

- 分配在heap上

- 是可变的

- from 创建

  ```
   fn main(){
  
      let mut s = String::from("Hello ");
      s.push_str("word");
  
      println!("{}",s)
  
   }
   
   Hello word
  ```

- 可需改原因

  - 字符串字面值在编译的时候就知道大小，被硬编码到可执行文件中，速度更快
  - String 为支持其可变性，需要在heap分配内存
  - Rust 的String 是所有者原则，当内存数据在不需要的时候就会立即释放，自动调用drop自动释放

## 10.2 数据move

- 多个数据可以使用同一种方式来move 例如 x的变量的所有权交给y

```
	let y = 5;
	x= 5
```

因为是基本类型 都会被压入stack

- String 是在堆上分配
- stack 上的数据是复制，基本数据类型

```
	fn main(){
    
		let y = 5;
		let x= 5;

		println!("y is {},x is {}",y,x);
		//y is 5,x is 5

		let s = String::from("string");
		let z1 = s.clone();
		println!("za is {}",z1);
		//za is string 如果同时使用两个 可以使用clone 在heap上重新clone一份

		let z = s;


		//println!("s is {}, z is {}",s,z);// ^ value borrowed here after move s把所有权给了z 所有s被清楚了，在打印就会报错

	}
```

[![image](https://user-images.githubusercontent.com/43371021/230769808-567a3695-fb52-4e7e-8057-7afbaafdc806.png)](https://user-images.githubusercontent.com/43371021/230769808-567a3695-fb52-4e7e-8057-7afbaafdc806.png) [![image](https://user-images.githubusercontent.com/43371021/230769864-dfb594c1-8e28-46e1-9210-948b878f7b67.png)](https://user-images.githubusercontent.com/43371021/230769864-dfb594c1-8e28-46e1-9210-948b878f7b67.png) [![image](https://user-images.githubusercontent.com/43371021/230770075-fcd04468-019c-4d10-b8eb-e500f867091a.png)](https://user-images.githubusercontent.com/43371021/230770075-fcd04468-019c-4d10-b8eb-e500f867091a.png) [![image](https://user-images.githubusercontent.com/43371021/230770100-9910b88d-8989-4445-9f49-c54cc9d05982.png)](https://user-images.githubusercontent.com/43371021/230770100-9910b88d-8989-4445-9f49-c54cc9d05982.png) [![image](https://user-images.githubusercontent.com/43371021/230770169-19e7b1d8-b4fe-4ceb-80de-b5996fe6bd54.png)](https://user-images.githubusercontent.com/43371021/230770169-19e7b1d8-b4fe-4ceb-80de-b5996fe6bd54.png)

- 基本数据类型都是可以copy的
- 整数类型、char、bool、元组tpule，也是不可变的，但是全部是才可以 [![image](https://user-images.githubusercontent.com/43371021/230770313-dac8b8ca-863b-4a3f-84ec-91803f9bd15a.png)](https://user-images.githubusercontent.com/43371021/230770313-dac8b8ca-863b-4a3f-84ec-91803f9bd15a.png)

## 10.3 所有权与函数

- 在语义上，将值传给函数或者赋值给变量是一样的
- 将值传递给函数也会发生**移动**或者**复制**
- 返回值也会发生所有权的move

```
fn main(){
    let s = String::from("Hello World");
    println!("s is {}",s);

    let s1 = take_ownership(s);

    /*
         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
...
5  |     let s1 = take_ownership(s);
   |                             - value moved here
6  |     println!("s is {}",s);
   |                        ^ value borrowed here after move
     */
    //println!("s is {}",s);//报错 因为 s在调用函数的时候已经发生移动，此处不能使用
    println!("s1 is {}",s1);//s1可以使用是在函数处理完毕后 将变量的所有权交给了s1


    let i1 : i32 = 5;//i32是基本类型 自动实现了copy trait的这个copy接口 所以要所有权还在 只是在stakcopy了一份
    makes_copy(i1);
    println!("i1 is {}",i1);
}

//返回值也会发生所有权的move
fn take_ownership(str:String)->String{
    println!("take_ownership str {}",str);
    str
}

fn makes_copy(i:i32){
    println!("makes_copy i is {}",i);
}

//s is Hello World
//take_ownership str Hello World
//s1 is Hello World
//makes_copy i is 5
//i1 is 5
```

[![image](https://user-images.githubusercontent.com/43371021/230770870-9bb86ecb-dff7-4d3c-a83c-db0d0ed7bd7f.png)](https://user-images.githubusercontent.com/43371021/230770870-9bb86ecb-dff7-4d3c-a83c-db0d0ed7bd7f.png)

## 10.4 引用和借用

### 10.4.1 引用

- 引用就是获取某些值但是不获取变量的所有权
- 引用是&,相当于是取址操作

```
fn main(){
    let s = String::from("Hello world");
    println!("引用之前的s-{}",s);
    //次数的函数调用传入的& 引用 就是取址操作 
    let l = calculate_length(&s);
    println!("引用之后s-{}，长度是-{}",s,l);

}

fn calculate_length(s :&String)->usize{
    s.len()
}

引用之前的s-Hello world
引用之后s-Hello world，长度是-11
```

### 10.4.2 借用

-把引用给函数参数的这个行为就是借用 &操作

- 是否可以修改 借用的值？

```
fn main(){
    let s = String::from("Hello world");
    println!("引用之前的s-{}",s);
    //次数的函数调用传入的& 引用 就是取址操作 
    let l = calculate_length(&s);
    println!("引用之后s-{}，长度是-{}",s,l);

}

fn calculate_length(s :&String)->usize{
    //s.push_str("!!");
    //     ^^^^^^^^^^^^^^^^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // s.push_str("!!");

    //借用是不可以修改这个值的 ,
    s.len()
}
```

## 10.5 可变引用

- 可变引用 就是引用的变量是可变的 就可以修改
- 变量为可变mut 其次是& 传参和入参 都是&mut
- 可变引用另一种 此值就是可变的，也是不可以的，这种属于借用，借用和引用都是不可以的，只有可变引用才可以 &mut
- 在一个作用域内只能有一个可变引用，好处是在编译就能保证不会产生data race
- 不同的作用域是可以同时有两个可变引用的
- 不可以同时拥有一个可变引用和一个不可变引用，保证不可变引用数据的一致性

```
fn main(){
    let mut s = String::from("Hello world");
    println!("引用之前的s-{}",s);
    //次数的函数调用传入的& 引用 就是取址操作 

    //传参，首先是可变变量 mut 其次是& 或者直接是可变变量传入String 本身就是可变的 所有不回加mut
    let l = calculate_length(&mut s);
    println!("引用之后s-{}，长度是-{}",s,l);

    //引用之前的s-Hello world
    //引用之后s-Hello world!!，长度是-13      

    let s1 = String::from("可变的变量");

}

//注意此处是&mut 传参的时候也是这样
fn calculate_length(s :&mut String)->usize{
    //s.push_str("!!");
    //     ^^^^^^^^^^^^^^^^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    s.push_str("!!");

    //借用是不可以修改这个值的 ,
    s.len()
}

fn ke_bian(s:String)->usize{

    //因为即使没加引用 这个是借用的变量 也是不能修改的
    //s.push_str("，可变的变量的后缀");//s.push_str("，可变的变量的后缀");
    // |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
    s.len()
}
```

同一作用域只能有一个可变引用 不同作用域的同时存在的可变引用

```
fn main(){
    let mut s = String::from("Hello world");
    println!("引用之前的s-{}",s);
    
    {
        let s1 =&mut s;
    }

    let s2 = &mut s;

}
```

{}是一个作用域

不可以同时拥有一个可变引用和一个不可变引用 保证不可变引用的数据一致性

```
fn main(){
    let mut s = String::from("Hello world");
    println!("引用之前的s-{}",s);

    //同一作用域
    let s1 =&s;
    let s3 =&s;
    //s2.push_str("string");
    // 10 |     println!("引用之后的s-{}",s);
    // |                               ^ immutable borrow occurs here
    // let s2 = &mut s;//前面有不变引用 ，此处如果有可变引用 保证不了引用的数据一致性
    s2.push_str("string");
    println!("引用之后的s-{}",s);
    
    println!("s-{},s1-{},s3-{},s2-{}",s,s1,s3,s2);

}
```

## 10.5 悬空引用 dangling references

- 一个指针引用了内存中的某个地址，但是这块地址已经释放给别人使用了
- 在rust 保证永远不会产生悬空引用

```
fn main(){
    let s = dangle();

}
fn dangle()->String{
    let mut s = String::from("Hello world");
    //^^ expected struct `String`, found `&String`
    &s //因为在返回返回的时候会把变量move到新的变量上 s是个空 在引用的话会出现悬空引用
}
```



# 11. 切片

rust的另一种不吃油所有权的数据类型：切片(slice)

- 字符串切片是只想字符串一部分内容的引用
- 形式：[开始索引...结束索引]
- 前闭后开
- 

```
fn main() {
    let str = String::from("Hello world");

    //如果从0开始，可以省略
    // let hello = &str[0..5];
    let hello = &str[..5];

    //如果末尾包含在内，可以不写末尾索引
    // let world = &str[6..11];
    // let world = &str[6..str.len()];
    let world = &str[6..];

    println!("{},{}",hello,world);
}

Hello,world
```



**返回字符串第一个出现空格的前面字符串**

```
fn main() {
    let str = String::from("Hello world");
    let word= first_world(&str);

    //cannot borrow `str` as mutable, as it is not declared as mutable
    //不能借用 可变，因为变量已经被借用为不可变的，所以不能清空
    // str.clear();

    println!("{}",word);

}

//&str 是字符的简写
fn first_world(s:&String)->&str{
    //as_bytes 转为 []byte类型
    //iter 返回迭代器
    //enumerate 返回tuple 类型
    for(i,&item) in s.as_bytes().iter().enumerate(){
        //b' '是字符类型
        if  item == b' '{
            return &s[..i]
        }
    }
    // return &s[..];
    &s[..]
}
Hello
```



- 字符串字面值是切片
- 字符串字面值被直接编译进二进制程序中，是不可变的 str
- let  str= "Hello World"

```

fn main() {
    // &str 是引用字符串切片值
    //同一作用域内不能同时存在可变引用和不可变引用 所以不能改变
    let str = "hello world";

    //mut variable does not need to be mutable
    // let mut str1 = "hello world";
    let str1 = "hello world";
    //warning: value assigned to `str1` is never read
    //str1 永远只是可读，不可写，但是结果是可以的
    // str1 = "hh";
    println!("str- {},str1- {}",str,str1);

}
变量str是&str ，是指向二进制程序特定的位置的切片
&str是不可变引用，所以字符串字面值是不可变的
```



**将字符串切片作为参数传递**

- fn first_world(s:&String)->&str
- 有经验的开发者回采用&str作为参数类型，因为这样接可以同时接受 == String和&str == 的参数
- fn first_world(s:&str)->&str
- 使用字符串切片，直接调用该函数
- String 要as_bytes然后嗲用该函数
- 定义字符切片来代替字符串回使我们的api更加通用

![image-20230410215843265](rust-new.assets/image-20230410215843265.png)

![image-20230410215911750](rust-new.assets/image-20230410215911750.png)



## 11.1 其它类型的切片

和字符串切片是适用的

```
fn main() {
    let a = [1,2,3,4,5];
    let slice = &a[1..5];
    
}
```



# 12. struct

- 使用struct关键字定义整个struct命名，和go的区别是有：指定类型

```
struct User{
	username :String,
	emial :String,
	sign_in_count : u64,
	active:bool,
}
```



- 想要使用struct，就必须创建struct的实例，为每个字段指定初始值,**必须全部制定初始值**
- 值的顺序不重要

```

fn main() {
   struct User {
        username:String,
        email:String,
        age:u8,
    }

    //必须全部都要指定初始值
    //missing structure fields:
    //- email
    //- age
    let user = User{
        username:String::from("zhangsan"),
        email:String::from("email"),
        age:28,
    };

    println!("user-username-{}",user.username);
     println!("user-email-{}",user.email);
     println!("user-age-{}",user.age);
}

user-username-zhangsan
user-email-email
user-age-28
```



- ==一但struct的实例是可变的，那么实例中的所有字段都是可变的==
- struct 可以作为函数返回值
- 字段初始化简写

```

fn main() {
   struct User {
        username:String,
        email:String,
        age:u8,
    }

    //必须全部都要指定初始值
    //missing structure fields:
    //- email
    //- age
    let username = String::from("zhangsan");
    let email = String::from("email");
    let user = User{
        username,
        email,
        age:28,
    };

    println!("user-username-{}",user.username);
    println!("user-email-{}",user.email);
    println!("user-age-{}",user.age);

}

user-username-zhangsan
user-email-email
user-age-28
```



- struct 更新语法

```

fn main() {
   struct User {
        username:String,
        email:String,
        age:u8,
    }

    //必须全部都要指定初始值
    //missing structure fields:
    //- email
    //- age

    let user2 = User{
        username:String::from("user2"),
        email:String::from("user2-email"),
        age:8,
    };
    let username = String::from("zhangsan");
    let user = User{
        username,
        ..user2
    };

    println!("user-username-{}",user.username);
    println!("user-email-{}",user.email);
    println!("user-age-{}",user.age);

}

user-username-zhangsan
user-email-user2-email
user-age-8
```



## 12.1 tuple struct

- 可以定义tuple struct

- tuple struct整体有名，但是字段没名

  适用：正给整个struct起名，但是不想给内部元素起名

```rust
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

let black = Color(0,0,0);
let origin = Point(0,0,0);

```

Black 和origin 是不同类型

访问 .下标

```

fn main() {
   struct Color (i32,i32,i32);
   let black = Color(9,9,9);
   println!("black-first-{}",black.0);

    let b1 = Color(3,4,5);
    //这样是不行的
    //expected struct `Color`, found tuple
//    let (b1,b2,b3) = bb;
    println!("black-b1-{}",b1.0);
     println!("black-b2-{}",b1.1);
      println!("black-b3-{}",b1.2);

}
```



## 12.1 unit-like-struct

struct () 没有任何字段的

- ==适用于某个类型实现trait，但是里面没有任何的存储数据==



## 12.3 struct数据的所有权

```
struct User{
	username:String,
	email:String,
	age:u8,
}
```

- 这里的字段使用了String而不是&str
- 该struct实例拥有其所有的数据
- 只要改struct实例是有效的，那么里面的字段也是有效的
- struct里面也可以存放引用，需要用到生命周期
- ==声明周期保证，只要struct是有效的，里面的引用也是有效的==
- 如果struct里面存储引用，但是没有使用生命周期，就会报错

![image-20230411205340675](rust-new.assets/image-20230411205340675.png)



## 12.4 struct例子

```

//#[derive(Debug)] derive是派生的意思
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}
fn main() {
   let rect = Rectangle{
    width:30,
    height:50,
   };

   //此处是借用，所有权还在
   println!("{}",area(&rect));

   //打印结构化数据
    //    println!("{:?}",rect)
    /*
    16 |    println!("{:?}",rect)
    |                    ^^^^ `Rectangle` cannot be formatted using `{:?}`
    |
    = help: the trait `Debug` is not implemented for `Rectangle`
    = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
    */

   println!("{:?}",rect);
   println!("{:#?}",rect)

}

fn area(rect :&Rectangle)->u32{
    rect.width*rect.height
}


1500
Rectangle { width: 30, height: 50 }
Rectangle {
    width: 30,
    height: 50,
}
```



## 12.5 struct 的方法

- 方法和实例类似：fn关键字、名称、参数、返回值
- 不同之处
- 方法在struct的上下文中定义
- 第一个参数是self，表示方法被调用的struct实例

```

//#[derive(Debug)] derive是派生的意思
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle {
    //此处是借用 可以是self 值得move ，也可以是&mut，需要struct变量也是mut的
    fn area(&self)->u32{
        self.width*self.height
    }
}

fn main() {
   let rect = Rectangle{
    width:30,
    height:50,
   };

   //此处是借用，所有权还在
   println!("{}",rect.area());


   println!("{:?}",rect);
   println!("{:#?}",rect)

}

1500
Rectangle { width: 30, height: 50 }
Rectangle {
    width: 30,
    height: 50,
}

```

- 定义方法在impl块里面定义
- 方法调用，如果调用的是方法是.，如果不是方法是::



**方法调用的运算符**

- rust会自动引用或者解引用
- 在调用方法的时候发生这种行为
- 在调用方法时候，会自动添加&、&mut或者*

下边的代码效果相同

P1.distance(&p2)

(&p1).disance(&p2)



## 12.6 关联函数

- 可以在impl块定义，但是不把self当作第一个参数
- 调用时::，而方法是.

```


//#[derive(Debug)] derive是派生的意思
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle {
    //此处是借用 可以是self 值得move ，也可以是&mut，需要struct变量也是mut的
    fn area(&self)->u32{
        self.width*self.height
    }
    fn square(size:u32)->Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
   let rect = Rectangle{
    width:30,
    height:50,
   };

   //此处是借用，所有权还在
   println!("{}",rect.area());


   println!("{:?}",rect);
   println!("{:#?}",rect);

   let square = Rectangle::square(40);
      println!("{:#?}",square)
}

1500
Rectangle { width: 30, height: 50 }
Rectangle {
    width: 30,
    height: 50,
}
Rectangle {
    width: 40,
    height: 40,
}
```

- 每个stauct 允许有多个impl块

![image-20230411212444882](rust-new.assets/image-20230411212444882.png)



# 13 枚举

- 关键字 enum 名字

```
#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6,
}

fn main(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V4)

}

fn route(ip_kind:IpAddrKind){
println!("{:#?}",ip_kind)
}

V4
V6
V4
```



## 13.1 添加数据到枚举变体中

![image-20230411213237288](rust-new.assets/image-20230411213237288.png)

```
#[derive(Debug)]
enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String),
}

fn main(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four(127,1,1,0));
    route(six(String::from("aaa:kkk:kkk")));

}

fn route(ip_kind:IpAddrKind){
println!("{:#?}",ip_kind)
}

V4(
    127,
    1,
    1,
    0,
)
V6(
    "aaa:kkk:kkk",
)

```



## 13.2 标准库中的struct



![image-20230411213507344](rust-new.assets/image-20230411213507344.png)



## 13.3 为枚举定义方法

```
#[derive(Debug)]
enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String),
}

impl IpAddrKind {
    fn ip_addr(&self){
        println!("{:#?}",self)
    }
}

fn main(){
    let four = IpAddrKind::V4(127,1,1,0);
    let six = IpAddrKind::V6(String::from("value"));

    four.ip_addr();
    six.ip_addr();

}



V4(
    127,
    1,
    1,
    0,
)
V6(
    "value",
)
```



# 14. Option<T>
- option 枚举有Some<T>和None
```
enum Option <T>{
	Some<T>,
	None,
}
```


```
	// #[derive(Debug)]
fn main(){
    let some = Option::Some(String::from("hello World"));
    is_some(some);



    // let no = None;
    // is_some(no)
}

//让T派生debug这个trait，就可以打印了
fn is_some<T:std::fmt::Debug>(op:Option<T>){
    match op {
        Some(op)=> println!("{:#?}",op),
        None=>println!("none"),
    }
}
"hello World"

```

# 15 match
- match 必须穷举所有情况
	
```

#![allow(unused)]
fn main() {
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
}
```

## 15.1 使用 match 表达式赋值
```
//同意所有没使用的代码
#[allow(dead_code)]
enum IpAddr{
    V4,
    V6,
}
fn main(){
    let addr = IpAddr::V4;
    let some = match addr {
        IpAddr::V4=>"127.0.0.1",
        _ => "::1",
    };
    println!("{}",some)
}
```

# 16.  if let

- 和match相比。match需要对所有的情况进行匹配，if let是可以对一种情况进行匹配，其余情况不需要处理的时候
```

//同意所有没使用的代码
#[allow(dead_code)]
enum IpAddr{
    V4,
    V6,
}
fn main(){
   let three = Some(3u8);

    match three {
        Some(3)=>println!("three"),
        _ => println!("none"),
    }

    //irrefutable `if let` pattern
    // this pattern will always match, so the `if let` is useless
    // if let three = Some(3){
    
    //如此写诗没有问题的，上边的有赋值的歧义
    if let Some(3) = three{
        println!("if let three");
    }else{
    
    }
}
```

# 17. matches!宏

matches!(expr,pattern)
- expr:是指条件判断的入参
- pattern 是期待为true的匹配模式。

	
```

//同意所有没使用的代码
#[allow(dead_code)]
#[derive(PartialEq)]
#[derive(Debug)]
enum IpAddr{
    V4,
    V6,
}
fn main(){
   let options = [IpAddr::V4,IpAddr::V6];

   //传统的match
   for (_,v) in options.iter().enumerate()  {
       match v {
        IpAddr::V4 =>{
            println!("v4")
        },
        _ => println!("other"),
       } 
   }

   //filter
   //IpAddr 必须实现 #[derive(PartialEq)] 否则x == IpAddr::V4 会报错
   let filter = options.iter().filter(|x| **x == IpAddr::V4);
   println!("{:?}",filter);

   //使用matches!
   let b = options.iter().filter(|x| matches!(x,IpAddr::V4));

   println!("b-{:?}",b.);

}
v4
other
Filter { iter: Iter([V4, V6]) }
b-Filter { iter: Iter([V4, V6]) }

```

```

//同意所有没使用的代码
#[allow(dead_code)]
#[derive(PartialEq)]
#[derive(Debug)]
enum IpAddr{
    V4,
    V6,
}
fn main(){
   let f = 'f';

   let bo = matches!(f,'A'..='Z'|'a'..='z');
    println!("bo-{}",bo);

    let some = Some(4);

    //匹配到Some(x) if x > 2的值代码如下，其中这里的匹配模式:匹配守卫 if x > 2
    //Some(x)是pattern匹配模式
    //if x > 2表示guard的匹配守卫(match guard)
    //这里的get_bar匹配守卫到其中一个模式后，返回true,未匹配到返回fasle
    assert!(matches!(some,Some(x) if x > 2 ));

}

//bo-true
```



# 18. Package 、crate、module

rust的代码组织

- 模块系统
- package(包)：cargo的特性，让你构建、测试、共享crate
- crate(单元包)：一个模块树，它可以产生一个libary或者可执行文件
- module(模块)：use：让你控制代码的组织、作用域、私有路径
- path(路径)：为struct、function或者module等项命名的方式



## 18.1 package 和crate

crate的类型

- binary
- libary

crate root

- 是源代码文件
- rust编译器从这里开始，组成你的的crate的根module

package

- 包含一个cargo.toml，它描述了如何构建这些crates
- 只能包含0-1个libary crate
- 可以包含多个binary crate
- 但是至少包含一个crate（library、binary）



![image-20230412213606710](rust-new.assets/image-20230412213606710.png)

![image-20230412213722095](rust-new.assets/image-20230412213722095.png)

## 18.2 module

![image-20230412213900490](rust-new.assets/image-20230412213900490.png)



![image-20230412214108372](rust-new.assets/image-20230412214108372.png)

![image-20230412214212531](rust-new.assets/image-20230412214212531.png)

## 18.3 path路径

- 绝对路径：从crate开始，使用crate或者字面值crate
- 相对路径：从当前模块开始，使用self，super或者当前模块的标识符

路径至少由一个标识符组成，标识符之前使用::

```
mod front_of_house{
    mod hosting{
        fn add_to_waitlist(){}
        fn seat_at_table(){}
    }

    mod serving{
        fn take_order(){}
        fn servr_order(){}
    }
}

pub fn eat_at_restaurant(){
    //绝对路径
    //绝对路径使用crate这个字面值，关键字
    crate::front_of_house::hosting::add_to_waitlist();

    //相对路径
    front_of_house::hosting::add_to_waitlist()
}
```

### 18.3.1  私有边界

![image-20230412215005564](rust-new.assets/image-20230412215005564.png)

![image-20230412215132850](rust-new.assets/image-20230412215132850.png)



### 18.3.2 super

- Super 来访问父级的内容

```
fn serve_order() {
    
}

mod back_of_house{
    fn six_int_order(){
        super::serve_order()
    }
}
```



![image-20230412215430251](rust-new.assets/image-20230412215430251.png)



# 19. Pub

## 19.1 pub struct

![image-20230412215540949](rust-new.assets/image-20230412215540949.png)

![image-20230412215600264](rust-new.assets/image-20230412215600264.png)

```
mod back_of_house{
    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String,
    }

    impl Breakfast {
        pub fn summer(toast:&str)->Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches") }
        }
    }
}

pub fn eat_at(){
    let mut meal = back_of_house::Breakfast::summer("tye");
    //共有的是可以访问的，meal String是可变的 ，因为加了mut
    meal.toast = String::from("wheat");
    println!("like {}",meal.toast);
    //私有的不可访问
    //field `seasonal_fruit` of `Breakfast` is private
    meal.seasonal_fruit = String::from("value")
}
```



![image-20230412220232198](rust-new.assets/image-20230412220232198.png)





## 19.2 pub enum

![image-20230412220432083](rust-new.assets/image-20230412220432083.png)



# 20. use

## 20.1 基本引入模式

相对路径和绝对路径

**绝对路径引入-相对路径引入**

```
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}

//绝对路径 引入，绝对路径的root crate
use  crate::front_of_house::hosting::add_to_waitlist;


//相对路径引入
use front_of_house::hosting::add_to_waitlist;
```

## 20.2 引入函数

```
//此时引入到了HashMap函数
use std::collections::HashMap;

fn main(){
    let mut mp = HashMap::new();
    mp.insert(1, 2);
    println!("{:?}",mp);
}
```

## 20.3 as别名避免重名引入

未避免引入的包的同名 可以使用as做别名使用

```
//此时引入到了HashMap函数
use std::io::Result as IoResult;
use std::fmt::Result;

fn main(){
    
}


fn function1()->Result{
    ...
}

fn function2()->IoResult>{
    ...
}
```

## 20.4 ==pub use引入项再导出==

当外部的模块项 `A` 被引入到当前模块中时，它的可见性自动被设置为私有的，如果你希望允许其它外部代码引用我们的模块项 `A`，那么可以对它进行再导出：

```
#![allow(unused)]
fn main() {
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
}
```

![image-20230413120347838](/Users/zhangqiuli24/Library/Application Support/typora-user-images/image-20230413120347838.png)

## 20.5 使用 `{}` 简化引入方式

对于以下一行一行的引入方式：

```
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;

use std::cmp::Ordering;
use std::io;
```

可以使用 `{}` 来一起引入进来，在大型项目中，使用这种方式来引入，可以减少大量 `use` 的使用：

```
use std::collections::{HashMap,BTreeMap,HashSet};
use std::{cmp::Ordering, io};
```

对于下面的同时引入模块和模块中的项：

```
use std::io;
use std::io::Write;
```

可以使用 `{}` 的方式进行简化:

```
use std::io::{self, Write};
```

## 20.6 self

上面使用到了模块章节提到的 `self` 关键字，用来替代模块自身，结合上一节中的 `self`，可以得出它在模块中的两个用途：

- `use self::xxx`，表示加载当前模块中的 `xxx`。此时 `self` 可省略
- `use xxx::{self, yyy}`，表示，加载当前路径下模块 `xxx` 本身，以及模块 `xxx` 下的 `yyy`

## 20.7 使用 `*` 引入模块下的所有项

对于之前一行一行引入 `std::collections` 的方式，我们还可以使用

```
use std::collections::*;
```

以上这种方式来引入 `std::collections` 模块下的所有公共项，这些公共项自然包含了 `HashMap`，`HashSet` 等想手动引入的集合类型。

当使用 `*` 来引入的时候要格外小心，因为你很难知道到底哪些被引入到了当前作用域中，有哪些会和你自己程序中的名称相冲突：

```
use std::collections::*;

struct HashMap;
fn main() {
   let mut v =  HashMap::new();
   v.insert("a", 1);
}
```

以上代码中，`std::collection::HashMap` 被 `*` 引入到当前作用域，但是由于存在另一个同名的结构体，因此 `HashMap::new` 根本不存在，因为对于编译器来说，本地同名类型的优先级更高。

在实际项目中，这种引用方式往往用于快速写测试代码，它可以把所有东西一次性引入到 `tests` 模块中。

# 21. 受限的可见性

在上一节中，我们学习了[可见性](https://course.rs/basic/crate-module/module.html#代码可见性)这个概念，这也是模块体系中最为核心的概念，控制了模块中哪些内容可以被外部看见，但是在实际使用时，光被外面看到还不行，我们还想控制哪些人能看，这就是 Rust 提供的受限可见性。

例如，在 Rust 中，包是一个模块树，我们可以通过 `pub(crate) item;` 这种方式来实现：`item` 虽然是对外可见的，但是只在当前包内可见，外部包无法引用到该 `item`。

所以，如果我们想要让某一项可以在整个包中都可以被使用，那么有两种办法：

- 在包根中定义一个非 `pub` 类型的 `X`(父模块的项对子模块都是可见的，因此包根中的项对模块树上的所有模块都可见)
- 在子模块中定义一个 `pub` 类型的 `Y`，同时通过 `use` 将其引入到包根

```
mod a {
    pub mod b {
        pub fn c() {
            println!("{:?}",crate::X);
        }

        #[derive(Debug)]
        pub struct Y;
    }
}

#[derive(Debug)]
struct X;
use a::b::Y;
fn d() {
    println!("{:?}",Y);
}
```

以上代码充分说明了之前两种办法的使用方式，但是有时我们会遇到这两种方法都不太好用的时候。例如希望对于某些特定的模块可见，但是对于其他模块又不可见：

```
// 目标：`a` 导出 `I`、`bar` and `foo`，其他的不导出
pub mod a {
    pub const I: i32 = 3;

    fn semisecret(x: i32) -> i32 {
        use self::b::c::J;
        x + J
    }

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        mod c {
            const J: i32 = 4;
        }
    }
}
```

这段代码会报错，因为与父模块中的项对子模块可见相反，子模块中的项对父模块是不可见的。这里 `semisecret` 方法中，`a` -> `b` -> `c` 形成了父子模块链，那 `c` 中的 `J` 自然对 `a` 模块不可见。

如果使用之前的可见性方式，那么想保持 `J` 私有，同时让 `a` 继续使用 `semisecret` 函数的办法是将该函数移动到 `c` 模块中，然后用 `pub use` 将 `semisecret` 函数进行再导出：

```
pub mod a {
    pub const I: i32 = 3;

    use self::b::semisecret;

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        pub use self::c::semisecret;
        mod c {
            const J: i32 = 4;
            pub fn semisecret(x: i32) -> i32 {
                x + J
            }
        }
    }
}
```

这段代码说实话问题不大，但是有些破坏了我们之前的逻辑，如果想保持代码逻辑，同时又只让 `J` 在 `a` 内可见该怎么办？

```
pub mod a {
    pub const I: i32 = 3;

    fn semisecret(x: i32) -> i32 {
        use self::b::c::J;
        x + J
    }

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        pub(in crate::a) mod c {
            pub(in crate::a) const J: i32 = 4;
        }
    }
}
```

通过 `pub(in crate::a)` 的方式，我们指定了模块 `c` 和常量 `J` 的可见范围都只是 `a` 模块中，`a` 之外的模块是完全访问不到它们的。

## 21.2 限制可见性语法

`pub(crate)` 或 `pub(in crate::a)` 就是限制可见性语法，前者是限制在整个包内可见，后者是通过绝对路径，限制在包内的某个模块内可见，总结一下：

- `pub` 意味着可见性无任何限制
- `pub(crate)` 表示在当前包可见
- `pub(self)` 在当前模块可见
- `pub(super)` 在父模块可见
- `pub(in <path>)` 表示在某个路径代表的模块中可见，其中 `path` 必须是父模块或者祖先模块

```
//一个名为my_mod的模块
mod my_mod{
    //在模块中的项默认具有私有可见性
    fn private_function(){
        println!("called `my_mod::private_function`");
    }

    //使用pub 修饰语来改变默认可见性
    pub fn function(){
        println!("called my_mod::function()");
    }

    //在同一模块中，项可以访问其他变量，即时他是私有的
    pub fn indirect_access(){
        println!("called my_mod::indirect_access() that");
        private_function()
    }


    pub mod nested{
        pub fn function(){
            println!("called my_mod::nested::function()");
        }
        #[allow(dead_code)]
        fn private_function(){
            println!("called my_mod::nested::public_function()");
        }
    

   

        //使用`pub(in path)`语法定义的函数只在给定的路径中可见
        pub(in crate::my_mod) fn public_function_in_my_mod(){
            println!("called my_mod::nested::public_function_in_mod()");
            public_function_nested()
        }

        //使用`pub(self)`语法定义的函数规则只在当前函数可见
        pub(self) fn public_function_nested(){
            println!("called `my_mod::nested::public_function_in_nested");
        }

        //使用`pub(super))`语法定义的函数则只在当前模块可见
        pub(super) fn public_function_in_super_mod(){
            println!("called `my_mod::nested::public_function_in_super_mod`")
        }

        
    }
    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    pub fn call_public_function_in_super_mod(){
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    //`pub(crate)`使得函数只在当前包可见
    pub(crate) fn public_function_in_crate(){
        println!("called `my_mod::public_function_in_crate()");
    }
    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}


fn function() {
    println!("called `function()`");
}
    
fn main() {
    // 模块机制消除了相同名字的项之间的歧义。
    function();
    my_mod::function();

    // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块中访问
    // 报错！函数 `public_function_in_my_mod` 是私有的
    //pub(in crate::my_mod) 只有在这个块里面才可以访问
    // my_mod::nested::public_function_in_my_mod();
    // 试一试 ^ 取消该行的注释

    // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

    // 报错！`private_function` 是私有的
    // my_mod::private_function();
    // 试一试 ^ 取消此行注释

    // 报错！`private_function` 是私有的
    // my_mod::nested::private_function();
    // 试一试 ^ 取消此行的注释

    // 报错！ `private_nested` 是私有的
    // my_mod::private_nested::function();
    // 试一试 ^ 取消此行的注释
}
```



# 22. 将模块拆分为不同的文件

- 模块定义的时候，如果模块名后边是";",而不是代码块
- rust会从与模块同名的文件中加载内容
- 模块树的结构不会发生变化



![image-20230413215832560](rust-new.assets/image-20230413215832560.png)

- 创建同名的front_os_house,rust 会自动查找这个文件

![image-20230413215946122](rust-new.assets/image-20230413215946122.png)

- 文件中添加模块的内容

![image-20230413220140569](rust-new.assets/image-20230413220140569.png)



**多级目录**

![image-20230413220349025](rust-new.assets/image-20230413220349025.png)



- 创建对应的包及文件

![image-20230413220926395](rust-new.assets/image-20230413220926395.png)



- Lib.rs 是library的根
- lib.rs里面的mod front_of-house;会查找==本路径的front_of_hous文件==
- 在本路径的front_of_house文件中加载了pub mod hosting;,==会加载本文件路径下的hosting.rs文件==



# 23. 动态数据Vector

动态数组允许你存储多个值，这些值在内存中一个紧挨着另一个排列，因此访问其中某个元素的成本非常低。动态数组只能==存储相同类型的元素==，如果你想存储不同类型的元素，可以使用之前讲过的枚举类型或者特征对象。

## 23.1 创建vector

**第一种初始化赋值**

```
fn main(){
    let mut er = vec![1,2,3];
    //cannot mutate immutable variable `er`
    er.push(4);
    println!("{:?}",er)
}
```



**初始化后fuzhi**

```
fn main(){
    let mut ve: Vec<u32> = Vec::new();
    ve.push(21);

    println!("{:?}",ve);
}
```

## 23.2 Vector 与其元素共存亡

跟结构体一样，`Vector` 类型在超出作用域范围后，会被自动删除：

```rust
{
    let v = vec![1, 2, 3];

    // ...
} // <- v超出作用域并在此处被删除
```

当 `Vector` 被删除后，它内部存储的所有内容也会随之被删除。目前来看，这种解决方案简单直白，但是当 `Vector` 中的元素被引用后，事情可能会没那么简单。

## 23.3 从Vevtor读取数据

**使用下标读取数据**

```
fn main(){
    let mut er = vec![1,2,3];
    //cannot mutate immutable variable `er`
    er.push(4);
    println!("{:?}",er);

    //使用下标读取数据
    let three = er[2];
    println!("three-{}",three);

    //下标去读数据超过范围
    //thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 4', src/main.rs:13:17
    //超过数据范围的会报错，阻止程序执行
    // let range = er[er.len()];
    // println!("读取超过vec的数据-{}",range);

    //使用.get读取数据
    let three_index_get  = er.get(2);
    println!("使用get读取的数据是-{:?}",three_index_get);

    //使用.get获取超过数据范围的数据
    let get_range = er.get(100);
    println!("get获取超过数据范围的数据-{:?}",get_range);
}

[1, 2, 3, 4]
three-3
使用get读取的数据是-Some(3)
get获取超过数据范围的数据-None
```

- 可以使用下标获取数据-[index]
- ==如果下标获取数据超过范围，会报错，阻止程序执行==
- 也可以使用.get获取数据
- 使用.get获取数据==返回的是Option==，超过范围会返回None

## 23.4 借用与可变借用不可同时存在

```
fn main(){
    let mut er = vec![1,2,3];
    let first = &er[0];
    println!("借用的第一个元素{}",first);

    //cannot borrow `er` as mutable because it is also borrowed as immutable
    //因为下边仍然存在不可变引用，新增数据导致Vec发生内存copy，数据要copy到新的内存上，地址发生变化
    //所以不能发生可变引用
    er.push(8);

    //上面如果first 不在push后使用，生命周期到println之后就结束了
    //即使是push了元素产生了可变引用，也不会影响
    //但是在push之后first仍然适用
    println!("借用的第一个元素{}",first);
}
```

- 如果不可变引用在可变行为的上面，==是可以的==
- 但是如果在可变行为的后面使用不可变数据是==不行的==，因为数据在内存是连续排列的，当数据发生新增会新开辟地址，将老的数据copy过去，此时数据地址会发生变化

**如果不是不可变引用呢**

```
fn main(){
    let mut er = vec![1,2,3];
    let first = er[0];
    println!("借用的第一个元素{}",first);

    //cannot borrow `er` as mutable because it is also borrowed as immutable
    //因为下边仍然存在不可变引用，新增数据导致Vec发生内存copy，数据要copy到新的内存上，地址发生变化
    //所以不能发生可变引用
    er.push(8);

    //上面如果first 不在push后使用，生命周期到println之后就结束了
    //即使是push了元素产生了可变引用，也不会影响
    //但是在push之后first仍然适用
    println!("借用的第一个元素{}",first);


    //如果是heap的数据
    let mut str = vec![String::from("first"),String::from("two")];
    //move occurs because value has type `String`, which does not implement the `Copy` trait
    //是不可以move的 可以使用&,但是就和基本类型一样了
    let heap_first = str[0];
    println!("heap_first的move-{}",heap_first);

    //进行数据可变
    // str.push(String::from("three"));

    // println!("heap_first的move-{}",heap_first);
}
```



## 23.5 vec的迭代

```
fn main(){
    let er = vec![1,2,3];
    
    for i in er{
        println!("迭代的元素-{}",i);
    }
    //迭代的元素-1
    // 迭代的元素-2
    // 迭代的元素-3

    //迭代中改变元素
    let mut num = vec![1,2,3,4,5];

    for i in &mut num{
         *i  *= 10;
         println!("迭代中改变元素-{}",i);
    }
    println!("迭代中改变元素的数组-{:?}",num)

}

迭代的元素-1
迭代的元素-2
迭代的元素-3
迭代中改变元素-10
迭代中改变元素-20
迭代中改变元素-30
迭代中改变元素-40
迭代中改变元素-50
迭代中改变元素的数组-[10, 20, 30, 40, 50]
```



## 23.6 存储不同类型

```
#[derive(Debug)]
struct Type(String,u32);
fn main(){
    let mut t  = Vec::new();
    let t1 = Type(String::from("str"),32);
    t.push(t1);

    println!("{:?}",t);

    //初始化
    let ts :Vec<Type> = vec![Type(String::from("str"),32),Type(String::from("str2"),34)];
    println!("{:?}",ts);
}

[Type("str", 32)]
[Type("str", 32), Type("str2", 34)]
```

# 24. String

- 基于[]byte的集合
- rust的核心语言层面，只有一个字符串类型，字符串切片str或者&str
- 字符串切片，对存储在其他地方、UTF-8编码的字符串的引用
- ==字符串字面值，存储在二进制中，也是字符串切片==
- String 是可增长的、可修改的、可拥有的
- String不是rust的核心语言层面的
- String是UTF-8类型的

**其他类型的标准库字符串**

- OsString
- OsStr
- CString
- CStr

万般无奈的情况下，只好新设计一套新的字符串类型，用来在做 FFI 接口的时候放弃字符串合法性检查，并把这种允许非法字符存在的编码格式美其名曰——WTF-8。

```
use std::ffi::OsStr;
use std::ffi::OsString;
 
fn main() {
    let a_str: &str = "Hello Str!"; 
    let an_os_str: &OsStr = OsStr::new(a_str); 
    println!("{:?}", an_os_str); 
 
    let a_string = String::from("Hello, String!");
    let an_os_string = OsString::from(a_string); 
    println!("{:?}", an_os_string); 
}
```



## 24.1 slice

切片并不是 Rust 独有的概念，在 Go 语言中就非常流行，它允许你引用集合中部分连续的元素序列，而不是引用整个集合。

对于字符串而言，切片就是对 `String` 类型中某一部分的引用，它看起来像这样：

```
#[derive(Debug)]
struct Type(String,u32);
fn main(){

  let s : String = String::from("hello world");

//   let word = &s[0..6];
  let hello = &s[..6];
  let world = &s[6..];

  println!("hello-{}",hello);
  println!("world-{}",world)


}

hello-hello 
world-world
```
<img width="830" alt="image" src="https://user-images.githubusercontent.com/43371021/231932209-2f1afd9d-e143-42da-9423-967353a77627.png">
<img width="804" alt="image" src="https://user-images.githubusercontent.com/43371021/231932299-468ea92c-1e6f-492e-b636-deeead6b614d.png">
<img width="826" alt="image" src="https://user-images.githubusercontent.com/43371021/231932424-dd0ce2fe-9003-49ef-8cd4-c047d7a414aa.png">

## 24.2 字符串字面量是切片

==字符串字面量是编译到二进制中的，因此不可变==

之前提到过字符串字面量,但是没有提到它的类型：

```rust
let s = "Hello, world!";
```

实际上，`s` 的类型是 `&str`，因此你也可以这样声明：

```rust
let s: &str = "Hello, world!";
```

该切片指向了程序可执行文件中的某个点，这也是为什么字符串字面量是不可变的，因为 `&str` 是一个不可变引用。



## 24.3 String

**Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)**，这样有助于大幅降低字符串所占用的内存空间。

<img width="818" alt="image" src="https://user-images.githubusercontent.com/43371021/231932875-f806aa03-b32a-4449-98e0-658d5f2893fa.png">

## 24.4 String 与 &str 的转换

在之前的代码中，已经见到好几种从 `&str` 类型生成 `String` 类型的操作：

- `String::from("hello,world")`
- `"hello,world".to_string()` //将&str转为String

那么如何将 `String` 类型转为 `&str` 类型呢？答案很简单，取引用即可：

```
#[derive(Debug)]
struct Type(String,u32);
fn main(){

    let s = String::from("hello world");
    say(&s[..6]);

    say(s.as_str());
	
		//as_str 将String转为 &str
		//to_String 将String转为&str
    let str = s.as_str().to_string();
    println!("{}",str)
}

fn say(s:&str){
    println!("say-{}",s)
}

say-hello 
say-hello world
hello world
```
## 24.5 [字符串索引](https://course.rs/basic/compound-type/string-slice.html#字符串索引)

在其它语言中，使用索引的方式访问字符串的某个字符或者子串是很正常的行为，但是在 Rust 中就会报错：

```rust
   let s1 = String::from("hello");
   let h = s1[0];
```

该代码会产生如下错误：
<img width="805" alt="image" src="https://user-images.githubusercontent.com/43371021/231935123-2bbb1498-3290-40fa-ae93-059d72fc37c3.png">

[字符串的不同表现形式](https://course.rs/basic/compound-type/string-slice.html#字符串的不同表现形式)

现在看一下用梵文写的字符串 `“नमस्ते”`, 它底层的字节数组如下形式：

```rust
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

长度是 18 个字节，这也是计算机最终存储该字符串的形式。如果从字符的形式去看，则是：

```rust
['न', 'म', 'स', '्', 'त', 'े']
```

但是这种形式下，第四和六两个字母根本就不存在，没有任何意义，接着再从字母串的形式去看：

```rust
["न", "म", "स्", "ते"]
```

所以，可以看出来 Rust 提供了不同的字符串展现方式，这样程序可以挑选自己想要的方式去使用，而无需去管字符串从人类语言角度看长什么样。

还有一个原因导致了 Rust 不允许去索引字符串：因为索引操作，我们总是期望它的性能表现是 O(1)，然而对于 `String` 类型来说，无法保证这一点，因为 Rust 可能需要从 0 开始去遍历字符串来定位合法的字符。

<img width="816" alt="image" src="https://user-images.githubusercontent.com/43371021/231935340-599e0be3-90fe-40ff-8ca9-4f4aa7e1f79b.png">



## 24.6 字符串操作

### 24.6.1 追加push

- 在字符串尾部可以使用 `push()` 方法追加==字符 `char`==
- 也可以使用 `push_str()` 方法追加字符串字面量。
- 这两个方法都是==**在原有的字符串上追加，并不会返回新的字符串**==。
- 由于字符串追加操作要修改原来的字符串，==则该字符串必须是可变的，即**字符串变量必须由 `mut` 关键字修饰**。==

```
#[derive(Debug)]
struct Type(String,u32);
fn main(){

    let mut str = String::from("push前的字符长");
    say(&str);
    str.push_str("push后的字符长-push_str");
    say(&str);

    //追加字符
    str.push('!');
    say(&str);


}

fn say(s:&str){
    println!("say-{}",s)
}

say-push前的字符长
say-push前的字符长push后的字符长-push_str
say-push前的字符长push后的字符长-push_str!
```

### 24.6.2 insert 插入

- insert插入==单个字符==
- insert_str插入字符串
- 两个方法都需要两个参数，第一个参数是插入的位置，第二个是插入的数据

```
fn main() {
    let mut str= String::from("hello world");
    str.insert(5, ',');

    str.insert_str(str.len(), "!");

    println!("str-{}   ",str)
}

str-hello, world! 
```



### 24.6.3 替换 replace全部、replacen个数、replace_range范围

==replace== 全部替换

- 此方法适用于&str和String两种
- 此方法会有返回值，返回新的，==有&str==肯定返回新的
- 参数1是要修改的字符串，参数2是要修改后的字符串

```
fn main() {
    let  str= String::from("I  like rust");
    let str = str.replace("rust", "RUST");

    println!("{}   ",str);

    //替换&str
    let str = "I like rust";

    let str = str.replace("rust", "RUST");

    println!("{}   ",str);
}

I  like RUST   
I like RUST   
```



==replacen== 指定替换的个数

- 适用于String、&str
- 前两个参数和replace是一样的，第三个是替换的个数

```
fn main() {
    let  str= String::from("I  like rust,must rust");
    let str = str.replacen("rust", "RUST",1);

    println!("{}   ",str);

    //替换&str
    let str = "I like rust,must rust";

    let str = str.replacen("rust", "RUST",2);

    println!("{}   ",str);
}

I  like RUST,must rust   
I like RUST,must RUST   
```



==replace_range== 指定范围替换数据

- 此方法不会返回新值，所以只能适用于String ，还必须是mut的
- 第一个是替换的范围，第二个是要替换后的内容

```
fn main() {
    let  mut str= String::from("I like rust,must rust");
    str.replace_range(7..10, "RUST");

    println!("{}   ",str);

}
I like RUSTt,must rust   
```

### 24.6.4 删除

与字符串删除相关的方法有 4 个，他们分别是 `pop()`，`remove()`，`truncate()`，`clear()`。这四个方法仅适用于== `String` ==类型。



==pop==--删除并返回字符串的最后一个字符

- **该方法是直接操作原来的字符串**。但是存在返回值，其返回值是一个 `Option` 类型，如果字符串为空，则返回 `None`。 示例代码如下：

```
fn main() {
    let   mut str= String::from("I like rust,must rust 中文!");
    let pop = str.pop();
    let pop1 = str.pop();
    dbg!(str);
    dbg!(pop);
    dbg!(pop1);

}

[src/main.rs:5] str = "I like rust,must rust 中"
[src/main.rs:6] pop = Some(
    '!',
)
[src/main.rs:7] pop1 = Some(
    '文',
)
```



**`remove` —— 删除并返回字符串中指定位置的==字符==**

- 有返回值
- 操作原来的数据
- 只接受一个参数，要删除位置

**该方法是直接操作原来的字符串**。但是存在返回值，其返回值是删除位置的字符串，只接收一个参数，表示该字符起始索引位置。`remove()` 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。

```
fn main() {
    let mut str = String::from("remve测试!");
    println!("length-{}",str.len());

    let remove = str.remove(str.len()-1);
    println!("length-{}",str.len());
    println!("{}",str);

    //hread 'main' panicked at 'byte index 10 is not a char boundary; it is inside '试' (bytes 8..11) of `remve测试`',
    //弹出的是字节,但是中文式三子节，取不出一个完整的，所以报错
    let remove1 = str.remove(str.len()-1);
    println!("{}",str);
    println!("{}",remove);
    println!("{}",str);
    println!("{}",remove1);


}

length-12
length-11
remve测试
thread 'main' panicked at 'byte index 10 is not a char boundary; it is inside '试' (bytes 8..11) of `remve测试`', /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/alloc/src/string.rs:1336:24
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```



`truncate` —— 删除字符串中从==指定位置开始到结尾的全部字符==

**该方法是直接操作原来的字符串**。无返回值。该方法 `truncate()` 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。

```
fn main() {
    let mut str = String::from("remve测试!");
    
    println!("length-{}",str.len());
    println!("{}",str);

    //truncate
    //thread 'main' panicked at 'assertion failed: self.is_char_boundary(new_len)', /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/alloc/src/string.rs:1279:13
    //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //原因是5到末尾11都是中文，这里是从字符开始的所以这个范围的都会报错
    // str.truncate(7);
    str.truncate(2);
    println!("length-{}",str.len());
    println!("{}",str);


}

length-12
remve测试!
length-2
re
```



==`clear` —— 清空字符串== 此方法一看就是只能是String

- 清空原来的字符串
- 必须是mut

```
fn main() {
    let mut str = String::from("remve测试!");
    
    println!("length-{}",str.len());
    println!("{:p}",&str);
    println!("{}",&str);

   str.clear();
   println!("{:p}",&str);
   println!("{}",&str);


}

length-12
0x16f95ab80
remve测试!
0x16f95ab80

```

可以看到地址没变，内容变为空了



### 24.6.5 连接 (Concatenate)

1、使用 `+` 或者 `+=` 连接字符串

<img width="823" alt="image" src="https://user-images.githubusercontent.com/43371021/232007814-05b28217-0f9f-43eb-b5d9-38ad87eac60a.png">
<img width="885" alt="image" src="https://user-images.githubusercontent.com/43371021/232008302-88c60810-8134-4aa9-b232-1261fbc3dbc2.png">

2、使用 format! 连接字符串

format! 这种方式适用于 String 和 &str 。format! 的用法与 print! 的用法类似，详见格式化输出。

示例代码如下：

```
fn main() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}
```
代码运行结果：
```
hello rust!
```

### 24.6.6 字符串转义
我们可以通过转义的方式 \ 输出 ASCII 和 Unicode 字符。

```
fn main() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}
```

当然，在某些情况下，可能你会希望保持字符串的原样，不要转义：

```
fn main() {
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
```

### 24.6.7 操作UTF-8数据
- 使用.chars()
<img width="809" alt="image" src="https://user-images.githubusercontent.com/43371021/232009264-f90be6cf-24d9-477d-a682-a27f82dd2fd2.png">

# 25 字符串深度解析
<img width="847" alt="image" src="https://user-images.githubusercontent.com/43371021/232009861-36d171f4-fb0c-45d3-b760-889c79eeb8ce.png">



# 26. HashMap

- H a sh M a p<K,V>
- 所有的K必须是同一个类型
- 所有的V也必须是同一个类型

```
use std::collections::HashMap;

fn main(){
    //type annotations needed for `HashMap<K, V>`
    //空的是不能推断，会有错误
    // let mut map =HashMap::new();

    //可以先创建 空的 添加值后会推断出
    let mut map =HashMap::new();

    //这样不支持
        // map[String::from("key")] = u8;
    map.insert(String::from("key"), 8);
    dbg!(map);

}

[src/main.rs:14] map = {
    "key": 8,
}
```



**另一种创建collect**

```
use std::collections::HashMap;

fn main(){
    let teams  = vec![String::from("blue"),String::from("yellow")];
    let intial_scores = vec![10,50];

    //前边需要之名返回类型
    //因为collect会返回很多种的类型数据
    let scores : HashMap<_,_> = teams.iter().zip(intial_scores.iter()).collect();
    dbg!(scores);

}

[src/main.rs:8] scores = {
    "blue": 10,
    "yellow": 50,
}
```



## 26.1 所有全的移动

```
use std::collections::HashMap;

fn main(){
    let field_name = String::from("key");

    let field_value = String::from("value");

    let mut  map = HashMap::new();

    map.insert(field_name, field_value);

    dbg!(map);

    //此时访问
    //borrow of moved value: `field_name`
    //因为是heap分配的数据，会发生所有全的移动
    // println!("{}",field_name);

    // 但是如果是引用类型就是可以的

    let field_name = String::from("key");

    let field_value = String::from("value");

    let mut  map = HashMap::new();

    map.insert(&field_name, &field_value);

    dbg!(map);
    println!("{}",field_name);

}

[src/main.rs:12] map = {
    "key": "value",
}
[src/main.rs:29] map = {
    "key": "value",
}
key
```



## 26.2 获取值

- 可使用map[k]，获取,但是当get不存在的时候会报错
- 可使用map。get,不存在的时候会返回None

```
use std::collections::HashMap;

fn main(){
    let field_name = String::from("key");

    let field_value = String::from("value");

    let mut  map = HashMap::new();

    map.insert(&field_name, &field_value);

    // dbg!(map);


    let m1 = map[&field_name];
    println!("{}",m1);
    
    //thread 'main' panicked at 'no entry found for key', src/main.rs:15:14
		//note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    let m1 = map[&String::from("value")];
    println!("{}",m1);

    //没有此方法
    // let m1 = map[&field_name];

    let m =map.get(&field_name);
     match m{
        Some(s)=> println!("{}",s),
        None=>println!("empty",)
    };
    dbg!(m);

}

value
value
[src/main.rs:27] m = Some(
    "value",
)
```



## 26.3 遍历

- 可使用tuple的形式进行遍历获取

```
use std::collections::HashMap;

fn main(){
    let field_name = String::from("key");

    let field_value = String::from("value");

    let mut  map = HashMap::new();

    map.insert(&field_name, &field_value);

   for (k,v) in map{
    println!("{}",k);
    println!("{}",v);
   }

}

key
value
```



## 26.4 更新hashmap

![image-20230417211017267](rust-new.assets/image-20230417211017267.png)

**覆盖现有的值**

```
use std::collections::HashMap;

fn main(){
    let field_name = String::from("key");

    let field_value = String::from("value");

    let mut  map = HashMap::new();

    map.insert(&field_name, &field_value);


    let field_value1 = String::from("kkkk");
    map.insert(&field_name,&field_value1);
    dbg!(map);

}

[src/main.rs:15] map = {
    "key": "kkkk",
}
```



**当值存在不替换**

- entry:检查指定的K是否对应一个V
- Or_insert 插入值，如果不存在

```
use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);

    let e =scores.entry(String::from("blue"));

    e.or_insert(50);
    dbg!(scores);
    //[src/main.rs:10] scores = {
    //    "blue": 10,
    //}

    let mut s = HashMap::new();
    s.insert(String::from("keu"), String::from("value"));


    let ec = s.entry(String::from("kes"));

    ec.or_insert(String::from("kkkk"));
    // dbg!(ec);
    println!("{:?}",s);
}

[src/main.rs:10] scores = {
    "blue": 10,
}
{"keu": "value", "kes": "kkkk"}
```

![image-20230417212926822](rust-new.assets/image-20230417212926822.png)



**基于现有的值进行更新**

```
use std::collections::HashMap;

fn main(){
    let text = "hello world wonderful world";

    let mut map = HashMap::with_capacity(4);

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
    print!("{:?}",map)
}

{"hello": 1, "wonderful": 1, "world": 2}%   
```



## 26.5  hash函数-docs攻击

![image-20230417213444644](rust-new.assets/image-20230417213444644.png)



# 27 rust错误处理概述

- rust可靠性-错误处理
- 大部分情况下；在编译阶段提示错误，并处理

- 错误氛围可恢复和不可恢复的错误

- **可吹拂的错误Result<F,E>**



## 27.1 不可恢复的错误

- 打印错误信息及调用栈信息
- 发生panic的时候沿着调用栈往回走，清理每个遇到的函数，**工作量大**
- 立即终止程序
- 内存由os进行回收



==**想让二进制更小**==

- 在cargo.toml中适当profile的位置设置
  - panic='about',发生恐慌，立即终止执行，由os进行回收

```
[package]
name = "code"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[profile.release]
panic = 'abort'
```



![image-20230418212311575](rust-new.assets/image-20230418212311575.png)



## 27.2 RUST_BACKTRACE 追踪错误

![image-20230418212904457](rust-new.assets/image-20230418212904457.png)



![image-20230418212934042](rust-new.assets/image-20230418212934042.png)



现实调用的堆栈信息

![image-20230418213138899](rust-new.assets/image-20230418213138899.png)

## 27.3 可恢复的错误Result枚举

![image-20230418213422289](rust-new.assets/image-20230418213422289.png)

```

use std::{fs::File};
fn main(){
   let f = File::open("hello.txt");

   let _f = match f {
       Ok(file)=>file,
       Err(error)=> match error.kind() {
           std::io::ErrorKind::NotFound=> match File::create("hello.txt") {
               Ok(fc)=>fc,
               Err(e)=>panic!("not found file {:?}",e),
           },
           other_err => panic!("error opening file {:?}",other_err),
       },
      
   };
   
}
```



![image-20230418214246839](rust-new.assets/image-20230418214246839.png)





## 27.4 闭包实现 让带阿妹更简洁

![image-20230418214551063](rust-new.assets/image-20230418214551063.png)

```

use std::{fs::File, io::ErrorKind};
fn main(){
    let _f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("create file is error {:?}",error);
            })
        }else{
            panic!("error open file error {:?}",error);
        }
    }) ;
}
```



## 27.5 unwrap

![image-20230418215056111](rust-new.assets/image-20230418215056111.png)

```

use std::{fs::File};
fn main(){
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file)=>file,
    //     Err(error)=>panic!("{:?}",error),
    // };

    //相当于
    let f = File::open("hello.txt").unwrap();

}


```

![image-20230418215359574](rust-new.assets/image-20230418215359574.png)



## 27.6 expect

![image-20230418215526225](rust-new.assets/image-20230418215526225.png)

```

use std::{fs::File};
fn main(){
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file)=>file,
    //     Err(error)=>panic!("{:?}",error),
    // };

    //相当于
    let f = File::open("hello.txt").expect("无法打开文件");

}
```



![image-20230418215620372](rust-new.assets/image-20230418215620372.png)



## 27.7 传播错误 ？

- 将错误返回给调用者







传统的错误

![image-20230418215922211](rust-new.assets/image-20230418215922211.png)



![image-20230418220037083](rust-new.assets/image-20230418220037083.png)

```

use std::{fs::File};
use std::io::{self, Read};
fn main(){
    
    let res = read_file();
    println!("{:?}",res);
}

fn read_file()->Result<String,io::Error>{
    let mut f = File::open("hello.txt")?;
    let mut s =String::with_capacity(100);

    f.read_to_string(&mut s)?;
    Ok(s)
}
```

![image-20230418220621833](rust-new.assets/image-20230418220621833.png)



## 27.8 ？ from

![image-20230418220757564](rust-new.assets/image-20230418220757564.png)

## 27.8 链式调用

```

use std::{fs::File};
use std::io::{self, Read};
fn main(){
    
    let res = read_file();
    println!("{:?}",res);
}

fn read_file()->Result<String,io::Error>{
    let mut s =String::with_capacity(100);

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

![image-20230418220921046](rust-new.assets/image-20230418220921046.png)



## 27.9 ?只能用于Result<F,E>

![image-20230418221136562](rust-new.assets/image-20230418221136562.png)



![image-20230418221208003](rust-new.assets/image-20230418221208003.png)





# 28 范型

![image-20230419212709459](rust-new.assets/image-20230419212709459.png)



## 28.1 结构体中使用泛型

```
#[derive(Debug)]
#[warn(dead_code)]
struct Point<T,U>{
    x :T,
    y:U,
}
fn main(){
    let int = Point{x:5,y:10.99};

    let float = Point{x:23.3,y:34.5};

    println!("{:#?}",int);
    println!("{:#?}",float);
}
```





## 28.2 在enum中使用泛型

![image-20230419213740564](rust-new.assets/image-20230419213740564.png)

```

enum OPtion<T>{
    Some(T),
    None,
}
enum Result<T,E>{
    Ok(T),
    Err(E),
}
fn main(){
   
}
```





![image-20230419213922568](rust-new.assets/image-20230419213922568.png)





## 28.3 泛型struct impl 方法

```

struct Point <T>{
    x:T,
    y:T,
}
impl <T> Point<T> {
    
    fn Add(&self)->&T{
        &self.x 
    }
}
fn main(){
    let p = Point{x:3,y:5};
    let x = p.Add();
    println!("{}",x)
}
```

![image-20230419214708959](rust-new.assets/image-20230419214708959.png)

![image-20230419214739871](rust-new.assets/image-20230419214739871.png)






**-----**

```

let int_value = 5;
//整型转字符串
let string_value = int_value.to_string();

//字符串转32位有符号整型
let back_int = string_value.parse::<i32>().unwrap();

// 字符串转32位无符号整型
let back_int = string_value.parse::<u32>().unwrap();

//字符串转16位有符号整型
let back_int = string_value.parse::<i16>().unwrap(); 

```

## 28.4 ? Option

`?` 不仅仅可以用于 `Result` 的传播，还能用于 `Option` 的传播，再来回忆下 `Option` 的定义：

```
fn main() {
   let vec = vec![1,2,3];

   let zero = add(&vec);

   if zero == None{
    println!("获取数据空")
   }
}

fn add(i : &[i32])-> Option<&i32> {
    let v = i.get(30)?;
    Some(v)
}

获取数据空
```

上面的函数中，`arr.get` 返回一个 `Option<&i32>` 类型，因为 `?` 的使用，如果 `get` 的结果是 `None`，则直接返回 `None`，如果是 `Some(&i32)`，则把里面的值赋给 `v`。

其实这个函数有些画蛇添足，我们完全可以写出更简单的版本：

```
fn first(arr: &[i32]) -> Option<&i32> {
   arr.get(0)
}
```



```
#![allow(unused)]
fn main() {
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
}

```

上面代码展示了在链式调用中使用 `?` 提前返回 `None` 的用法， `.next` 方法返回的是 `Option` 类型：如果返回 `Some(&str)`，那么继续调用 `chars` 方法,如果返回 `None`，则直接从整个函数中返回 `None`，不再继续进行链式调用。

![image-20230420152204250](rust-new.assets/image-20230420152204250.png)



## 28.5 [带返回值的 main 函数](https://course.rs/basic/result-error/result.html#带返回值的-main-函数)

![image-20230420152500456](rust-new.assets/image-20230420152500456.png)



## 28.6 try!

![image-20230420153037535](rust-new.assets/image-20230420153037535.png)

# 29 模式守卫与@绑定的互换

把模式守卫改为@绑定
修改前

```
fn main() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}
```

修改后

```
fn main() {
    let num = Some(4);
    match num {
        Some(x @ 0..=5) => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}
```

把@绑定改为模式守卫
修改前

```
fn main() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
```


修改后

```
fn main() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable } if (id_variable >= 3 && id_variable <= 7) => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
```

# 30 类型转换

rust提供了一个关键字as

专门用于这样的类型转换

也就是说rust的设计者希望在发生类型转换的时候不是偷偷摸摸进行的

而是显式地标记出来

防止隐藏的bug

```

struct Point {
    x:i8,
    pub y:i8,
}
impl Point {
    fn Add(&self,i:i32)->i8{
       ( self.x + i as *const i8 as i8 )as i8
    }
}
fn main(){
    let p = Point{x:3,y:5};
    let x = p.Add(8);
    println!("{}",x)
}
```





```

struct Point {
    x:i8,
    pub y:i8,
}
impl Point {
    fn Add(&self,i:i32)->i8{
       ( self.x + i as *const i8 as i8 )as i8
    }
}
fn main(){
    let s = String::from("8");

    let i = s.parse::<i32>().unwrap();

    let  s = i.to_string();
}
```



## 28.4 const泛型

[const 详解](https://zh.practice.rs/generics-traits/const-generics.html)

- 定义的语法是 `const N: usize`，表示 const 泛型 `N` ，它基于的值类型是 `usize
- 目前，const 泛型参数只能使用以下形式的实参:
  - 一个单独的 const 泛型参数
  - 一个字面量 (i.e. 整数, 布尔值或字符).
  - 一个具体的 const 表达式( 表达式中不能包含任何 泛型参数)

![image-20230420174141629](rust-new.assets/image-20230420174141629.png)

## [（Rust 1.51 版本引入的重要特性）](https://course.rs/basic/trait/generic.html#const-泛型rust-151-版本引入的重要特性)

![image-20230420173042867](rust-new.assets/image-20230420173042867.png)

![image-20230420173130556](rust-new.assets/image-20230420173130556.png)

==`N` 就是 const 泛型，定义的语法是 `const N: usize`，表示 const 泛型 `N` ，它基于的值类型是 `usize`。在泛型参数之前，Rust 完全不适合复杂矩阵的运算，自从有了 const 泛型，一切即将改变。==



# trait--interface

我们也多次见过特征的使用，例如 `#[derive(Debug)]`，它在我们定义的类型(`struct`)上自动派生 `Debug` 特征，接着可以使用 `println!("{:?}", x)` 打印这个类型；再例如：

```
#![allow(unused)]
fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}
```

通过 `std::ops::Add` 特征来限制 `T`，只有 `T` 实现了 `std::ops::Add` 才能进行合法的加法操作，毕竟不是所有的类型都能进行相加。

这些都说明一个道理，特征定义了**一组可以被共享的行为，只要实现了特征，你就能使用这组行为**。


# 31. trait--interface

我们也多次见过特征的使用，例如 `#[derive(Debug)]`，它在我们定义的类型(`struct`)上自动派生 `Debug` 特征，接着可以使用 `println!("{:?}", x)` 打印这个类型；再例如：

```
#![allow(unused)]
fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}
```

通过 `std::ops::Add` 特征来限制 `T`，只有 `T` 实现了 `std::ops::Add` 才能进行合法的加法操作，毕竟不是所有的类型都能进行相加。

这些都说明一个道理，特征定义了**一组可以被共享的行为，只要实现了特征，你就能使用这组行为**。

## 31.1 定义特征

- 把不同的类型具有相同的行为，定义为一组特征

例如，我们现在有文章 `Post` 和微博 `Weibo` 两种内容载体，而我们想对相应的内容进行总结，也就是无论是文章内容，还是微博内容，都可以在某个时间点进行总结，那么总结这个行为就是共享的，因此可以用特征来定义：

```
pub trait Summary{
	fn summary(&self)->String;
}
```

这里使用 `trait` 关键字来声明一个特征，`Summary` 是特征名。在大括号中定义了该特征的所有方法，在这个例子中是： `fn summarize(&self) -> String`。



## 32.2 为类型实现特征

```
pub trait Summary {
    fn summary(&self)-> String;
}

struct WeiBo {
   pub auther :String,
   pub title : String,
   pub content : String,
}

struct Post{
    pub username : String,
    pub text : String,
}

//结构体关联函数或者方法
impl Post {
    fn new(username : String,text : String)->Post{
        Post { username, text }
    }

    fn get_username(&self) -> &str{
        self.username.as_str()
    }
}

//实现trait
impl Summary for WeiBo {
    fn summary(&self)-> String {
        format!("作者-{} title-{} 内容-{} ",self.auther,self.title,self.content)
    }
}

impl Summary for Post {
    fn summary(&self)-> String {
        format!("username-{} text-{}",self.username,self.text)
    }
}

fn main(){
    //调用结构体的关联函数和方法
    let struct_func = Post::new("username".to_string(), "text".to_string());

    let username = struct_func.get_username();
    
    println!("Post-username-{}",username);

    //调用实现Summary trait的 结构体的行为

    let weibo = WeiBo{
        auther:"zhangSan".to_string(),
        title:"这是title".to_string(),
        content:"这是内容".to_string(),
    };

    let say = weibo.summary();
    println!("weibo-{}",say);


    let post = Post{
        username:"post-username".to_string(),
        text:"post-text".to_string(),
    };

    let post = post.summary();
    println!("post-summary-{}",post)

}

Post-username-username
weibo-作者-zhangSan title-这是title 内容-这是内容 
post-summary-username-post-username text-post-text
```

- 可以看到实现了Summary的类型，都会应有对应Summary的行为

## 32.3 [特征定义与实现的位置(孤儿规则)](https://course.rs/basic/trait/trait.html#特征定义与实现的位置孤儿规则)

![image-20230421144700469](rust-new.assets/image-20230421144700469.png)



## 32.3 默认实现方法

```
pub trait Summary {
    fn summary_auther(&self)->String;

    fn summary(&self){
        println!(" trait Summary");
        self.summary_auther();
    }
}

struct WeiBo {
   pub auther :String,
   pub title : String,
   pub content : String,
}

struct Post{
    pub username : String,
    pub text : String,
}

//实现trait
impl Summary for WeiBo {
    fn summary_auther(&self)-> String {
        format!("作者-{} title-{} 内容-{} ",self.auther,self.title,self.content)
    }
    //重载summary
    fn summary(&self) {
        println!("这是weibo对summary的重载实现")
    }
}

impl Summary for Post {
    fn summary_auther(&self)-> String {
        format!("username-{} text-{}",self.username,self.text)
    }
}

fn main(){
    
    //调用实现Summary trait的 结构体的行为

    let weibo = WeiBo{
        auther:"zhangSan".to_string(),
        title:"这是title".to_string(),
        content:"这是内容".to_string(),
    };

    //默认实现Summary的类型都会继承默认实现内容

    //weibo 实现了重载
    let say = weibo.summary_auther();
    println!("weibo-{}",say);
    weibo.summary();
    


    let post = Post{
        username:"post-username".to_string(),
        text:"post-text".to_string(),
    };

    let say = post.summary_auther();
    println!("post-summary-{}",say);
    post.summary();


}



weibo-作者-zhangSan title-这是title 内容-这是内容 
这是weibo对summary的重载实现
post-summary-username-post-username text-post-text
 trait Summary

```

![image-20230421151048329](rust-new.assets/image-20230421151048329.png)

## 32.4 [使用特征作为函数参数](https://course.rs/basic/trait/trait.html#使用特征作为函数参数)

之前提到过，特征如果仅仅是用来实现方法，那真的有些大材小用，现在我们来讲下，真正可以让特征大放光彩的地方。

现在，先定义一个函数，使用特征作为函数参数：

```
fn apply(item : &impl Summary){
    item.summary();
    item.summary_auther();
}
```

![image-20230421151519253](rust-new.assets/image-20230421151519253.png)



## 32.5 [特征约束(trait bound)](https://course.rs/basic/trait/trait.html#特征约束trait-bound)

- 作为参数传入的形式是`impl Summary`
- 也可以使用特征绑定

```
pub fn apply<T : Summary>(item:T){
	
}
```

此处的`T： Summary`就是特征绑定

![image-20230421152433346](rust-new.assets/image-20230421152433346.png)



## 32.6 多重约束

除了单个约束条件，我们还可以指定多个约束条件，例如除了让参数实现 `Summary` 特征外，还可以让参数实现 `Display` 特征以控制它的格式化输出：

```
fn apply(item : &(impl Summary + Display)){
	
}

或者
fn apply<T : (Summary + Display)>(item : &T){
	
}
```



## 32.7 where

```
fn some_impl<T:Summary + Display,U : Summary + Copy> (item:&T,item2 : &U){

}

fn some_imple<T,U>()
    where   T : Summary+Display,
            U : Summary+Copy,
{
    
}
```

![image-20230421153107091](rust-new.assets/image-20230421153107091.png)



## 32.8 结构体泛型的方法实现约束

```
use std::fmt::Display;
struct Point<T>{
    x:T,
    y:T
}

impl <T :Display+PartialOrd>Point<T> {
    fn com(&self){
        if self.x >self.y{
            println!("x > y")
        }else{
            println!("x < y")
        }
    }
}
```

![image-20230421153930177](rust-new.assets/image-20230421153930177.png)



## 32.9 作为返回参数

![image-20230421154428441](rust-new.assets/image-20230421154428441.png)

==**注意trait bound 是不能作为参数返回的使用的**==



![image-20230421154537450](rust-new.assets/image-20230421154537450.png)



是可以调用返回的Summary类型的函数

```
pub trait Summary {
    fn summary_auther(&self)->String;

    fn summary(&self){
        println!(" trait Summary");
        self.summary_auther();
    }
}

#[derive(Debug)]
struct WeiBo {
   pub auther :String,
   pub title : String,
   pub content : String,
}

struct Post{
    pub username : String,
    pub text : String,
}

//实现trait
impl Summary for WeiBo {
    fn summary_auther(&self)-> String {
        format!("作者-{} title-{} 内容-{} ",self.auther,self.title,self.content)
    }
    //重载summary
    fn summary(&self) {
        println!("这是weibo对summary的重载实现")
    }
}

impl Summary for Post {
    fn summary_auther(&self)-> String {
        format!("username-{} text-{}",self.username,self.text)
    }
}


fn new_summary_post<T : Summary>(item : T)-> impl Summary{
   item
}

fn main(){

    let post_t = new_summary_post(WeiBo{
        auther:"zhangSan".to_string(),
        title:"这是title".to_string(),
        content:"这是内容".to_string(),
    });

    println!("{:?}",post_t.summary());

   

}

这是weibo对summary的重载实现
()
```



## 32.10 [修复上一节中的 `largest` 函数](https://course.rs/basic/trait/trait.html#修复上一节中的-largest-函数)

还记得上一节中的[例子](https://course.rs/basic/trait/generic.html#泛型详解)吧，当时留下一个疑问，该如何解决编译报错：

```rust
error[E0369]: binary operation `>` cannot be applied to type `T` // 无法在 `T` 类型上应用`>`运算符
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
help: consider restricting type parameter `T` // 考虑使用以下的特征来约束 `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
  |             ^^^^^^^^^^^^^^^^^^^^^^
```

在 `largest` 函数体中我们想要使用大于运算符（`>`）比较两个 `T` 类型的值。这个运算符是标准库中特征 `std::cmp::PartialOrd` 的一个默认方法。所以需要在 `T` 的特征约束中指定 `PartialOrd`，这样 `largest` 函数可以用于内部元素类型可比较大小的数组切片。

由于 `PartialOrd` 位于 `prelude` 中所以并不需要通过 `std::cmp` 手动将其引入作用域。所以可以将 `largest` 的签名修改为如下：

```rust
fn largest<T: PartialOrd>(list: &[T]) -> T {}
```

但是此时编译，又会出现新的错误：

```rust
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       help: consider using a reference instead: `&list[0]`

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:4:9
  |
4 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content
```

错误的核心是 `cannot move out of type [T], a non-copy slice`，原因是 `T` 没有[实现 `Copy` 特性](https://course.rs/basic/ownership/ownership.html#拷贝浅拷贝)，因此我们只能把所有权进行转移，毕竟只有 `i32` 等基础类型才实现了 `Copy` 特性，可以存储在栈上，而 `T` 可以指代任何类型（严格来说是实现了 `PartialOrd` 特征的所有类型）。

因此，为了让 `T` 拥有 `Copy` 特性，我们可以增加特征约束：

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

如果并不希望限制 `largest` 函数只能用于实现了 `Copy` 特征的类型，我们可以在 `T` 的特征约束中指定 [`Clone` 特征](https://course.rs/basic/ownership/ownership.html#克隆深拷贝) 而不是 `Copy` 特征。并克隆 `list` 中的每一个值使得 `largest` 函数拥有其所有权。使用 `clone` 函数意味着对于类似 `String` 这样拥有堆上数据的类型，会潜在地分配更多堆上空间，而堆分配在涉及大量数据时可能会相当缓慢。

另一种 `largest` 的实现方式是返回在 `list` 中 `T` 值的引用。如果我们将函数返回值从 `T` 改为 `&T` 并改变函数体使其能够返回一个引用，我们将不需要任何 `Clone` 或 `Copy` 的特征约束而且也不会有任何的堆分配。尝试自己实现这种替代解决方式吧！

![image-20230421161341924](rust-new.assets/image-20230421161341924.png)



## 32.11 [为自定义类型实现 `+` 操作](https://course.rs/basic/trait/trait.html#为自定义类型实现--操作)

```
use std::ops::Add;

#[derive(Debug)]
struct Point<T : Add<T,Output = T>>{
    x:T,
    y:T,
}

impl <T : Add<T,Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Point<T> {
        Point{
            x:self.x+rhs.x,
            y:self.y+rhs.y,
        }
        
    }
}

fn add<T: Add<T ,Output = T>>(a : T,b :T)->T{
    a+b
}

fn main(){
    let f1 = Point{
        x:1.0f32,
        y:2.0f32,
    };
    let f2 = Point{
        x:1.0f32,
        y:2.0f32,
    };

    let f = add(f1, f2);
    println!("{:?}",f);
}

Point { x: 2.0, y: 4.0 }
```



## 32.12 [自定义类型的打印输出](https://course.rs/basic/trait/trait.html#自定义类型的打印输出)

![image-20230421163202854](rust-new.assets/image-20230421163202854.png)

# 32 trait 特征对象



==**特征对象**指向实现了 `Draw` 特征的类型的实例，也就是指向了 `Button` 或者 `SelectBox` 的实例，这种映射关系是存储在一张表中，可以在运行时通过特征对象找到具体调用的类型方法。可以通过 `&` 引用或者 `Box<T>` 智能指针的方式来创建特征对象。==



## 32.1 dyn

**dyn是trait对象类型的前缀**

==dyn关键字用于强调相关trait的方法是动态分配的。==要以这种方式使用trait，它必须是“对象安全”的。

与泛型参数或植入型特质不同，编译器不知道被传递的具体类型。也就是说，该类型已经被抹去。因此，**一个dyn Trait引用包含两个指针。一个指针指向数据（例如，一个结构的实例）。另一个指针指向方法调用名称与函数指针的映射（被称为虚拟方法表各vtable）。**

==impl trait 和 dyn trait 在Rust分别被称为静态分发和动态分发，即当代码涉及多态时，需要某种机制决定实际调动类型。==

![image-20230421165342409](rust-new.assets/image-20230421165342409.png)



![image-20230421165518574](rust-new.assets/image-20230421165518574.png)







![image-20230421170319335](rust-new.assets/image-20230421170319335.png)



```
pub trait Draw {
    fn draw(&self)->String;
}

impl Draw for u8{
    fn draw(&self) ->String{
        println!("u8 - {}",self);
        format!("{}",self)
    }
}

impl Draw for f64 {
    fn draw(&self)->String {
        println!("f64 - {}",self);
        format!("{}",self)
    }
}

fn draw1(x : &dyn Draw){
    x.draw();
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
fn draw2(x : Box<dyn Draw>){
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    x.draw();
}

fn main(){
    let f = 45.67f64;
    let u = 8u8;

    draw1(&f);
    draw1(&u);

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw> 
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw2(Box::new(f));
    draw2(Box::new(u));
    
}

f64 - 45.67
u8 - 8
f64 - 45.67
u8 - 8
```

- `draw1` 函数的参数是 `Box<dyn Draw>` 形式的特征对象，该特征对象是通过 `Box::new(x)` 的方式创建的
- `draw2` 函数的参数是 `&dyn Draw` 形式的特征对象，该特征对象是通过 `&x` 的方式创建的
- ==`dyn` 关键字只用在特征对象的类型声明上，在创建时无需使用 `dyn`==



```
pub trait Draw {
    fn draw(&self)->String;
}

impl Draw for u8{
    fn draw(&self) ->String{
        println!("u8 - {}",self);
        format!("{}",self)
    }
}

impl Draw for f64 {
    fn draw(&self)->String {
        println!("f64 - {}",self);
        format!("{}",self)
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
    //missing lifetime specifier
    //dyn 是动态分配不知道类型的时候来确定的
    // pub components : Vec<&dyn Draw>,
}

impl Screen {
    fn run (&self){
        for item in self.components.iter(){
            item.draw();
        }
    }
}

// fn draw1(x : &dyn Draw){
//     x.draw();
// }

// // 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
// fn draw2(x : Box<dyn Draw>){
//     // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
//     x.draw();
// }

fn main(){
    let f = 45.67f64;
    let u = 8u8;

   let vec = Screen{
    components : vec![Box::new(f),Box::new(u)],
   };

   vec.run()
    
}

f64 - 45.67
u8 - 8
```



## 32.2 dyn 和&dyn区别

![image-20230421173030427](rust-new.assets/image-20230421173030427.png)



![image-20230421173350574](rust-new.assets/image-20230421173350574.png)

![image-20230421173603185](rust-new.assets/image-20230421173603185.png)



## 32.3 self 和Self

- self 是调用者本身
- Self 在谁的作用域内就是谁

```
trait Draw {
    //此时是Drwa
    fn draw(&self) -> Self;
}

#[derive(Clone)]
struct Button;
impl Draw for Button {
    //此时的Self 不是 Draw 是Button
    fn draw(&self) -> Self {
        return self.clone()
    }
}

fn main() {
    let button = Button;
    let newb = button.draw();
}

```

上述代码中，`self`指代的就是当前的实例对象，也就是 `button.draw()` 中的 `button` 实例，`Self` 则指代的是 `Button` 类型。

当理解了 `self` 与 `Self` 的区别后，我们再来看看何为对象安全。

## 32.4 [特征对象的限制](https://course.rs/basic/trait/trait-object.html#特征对象的限制)

不是所有特征都能拥有特征对象，只有对象安全的特征才行。当一个特征的所有方法都有如下属性时，它的对象才是安全的：

- **方法的返回类型不能是 `Self`**
- 方法没有任何泛型参数

![image-20230421174311705](rust-new.assets/image-20230421174311705.png)

# 33 深入trait

## 33.1 关联类型

关联类型是在特征定义的语句块中，申明一个自定义类型，这样就可以在特征的方法签名中使用该类型：

```
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

![image-20230421175203960](rust-new.assets/image-20230421175203960.png)



## 33.2 [默认泛型类型参数](https://course.rs/basic/trait/advance-trait.html#默认泛型类型参数)
## 33.2 [默认泛型类型参数](https://course.rs/basic/trait/advance-trait.html#默认泛型类型参数)

```
trait Add<RHF=Self> {
    type Item;
    fn add(&self,other:RHF)->Self::Item;
}
```

![image-20230423174120994](rust-new.assets/image-20230423174120994.png)



**不同类型相加**

```
use std::ops::Add;

#[derive(Debug)]
struct Millmeters(u32);
struct Meter(u32);

impl Add<Meter> for Millmeters {
    type Output = Millmeters;
    fn add(self,other:Meter)->Self::Output{
        Millmeters(self.0 + (other.0*10000))
    }
}

fn main(){
    let m = Millmeters(43);
    let mm = Meter(5);

    let m1 = m.add(mm);

    println!("{:?}",m1)
}

Millmeters(50043)
```

![image-20230423174841741](rust-new.assets/image-20230423174841741.png)



## 33.3 调用自己和特征的同名方法-self方法

- 调用首先回调调用自己的方法
- 调用实现特征的方法要告诉是调用的哪个特征

```
特征::方法名(实现的实体)
```



```
struct Drive;

trait  Man{
    fn drive(&self);
}

trait Woman {
    fn drive(&self);
}

impl Man for Drive {
    fn drive(&self) {
        println!("man-drive")
    }
}

impl Woman for Drive {
    fn drive(&self) {
        println!("woman-drive")
    }
}

impl Drive {
    fn drive(&self){
        println!("self")
    }
}

fn main(){
   let d = Drive;

   //调用自己的方法
   d.drive();

   //调用特征上的方法
   Man::drive(&d);

   Woman::drive(&d)

}

self
man-drive
woman-drive
```



## 33.4 完全限定语法-关联方法

- 调用自己的方法使用::方法名就可以
- 调用特征的关联函数使用完全限定

```
<实现特征的类型 as 被实现的特征>::特征关联函数();
```



这个时候问题又来了，如果方法没有 `self` 参数呢？稍等，估计有读者会问：还有方法没有 `self` 参数？看到这个疑问，作者的眼泪不禁流了下来，大明湖畔的[关联函数](https://course.rs/basic/method.html#关联函数)，你还记得嘛？

但是成年人的世界，就算再伤心，事还得做，咱们继续：

```
trait Animai {
    fn baby_name();
}

struct Dog;

impl Animai for Dog {
    fn baby_name() {
        println!("impl - animal - dog")
    }
}

impl Dog {
    fn baby_name(){
        println!("self dog")
    }
}

fn main(){

    //调用关联方法用::
    //调用方法用. 含有self的
    Dog::baby_name();


    //调用特征同名的关联函数
    //argument of type `&Dog` unexpected
    // Animai::baby_name(&d);

    //使用as 完全限定方法
    <Dog as Animai>::baby_name();

}

self dog
impl - animal - dog
```



# 34 生命周期

## 34.1 [生命周期标注语法](https://course.rs/basic/lifetime.html#生命周期标注语法)

生命周期的语法也颇为与众不同，以 `'` 开头，名称往往是一个单独的小写字母，大多数人都用 `'a` 来作为生命周期的名称。 如果是引用类型的参数，那么生命周期会位于引用符号 `&` 之后，并用一个空格来将生命周期和引用参数分隔开:

```
&i32        // 一个引用
&'a i32     // 具有显式生命周期的引用
&'a mut i32 // 具有显式生命周期的可变引用

```

![image-20230424174820881](rust-new.assets/image-20230424174820881.png)



## 34.2 函数中的生命周期标注

![image-20230424180004398](rust-new.assets/image-20230424180004398.png)



![image-20230424180324376](rust-new.assets/image-20230424180324376.png)



![image-20230424180721208](rust-new.assets/image-20230424180721208.png)



## 34.3 [结构体中的生命周期](https://course.rs/basic/lifetime.html#结构体中的生命周期)

![image-20230424181421644](rust-new.assets/image-20230424181421644.png)



## 34.4 [生命周期消除](https://course.rs/basic/lifetime.html#生命周期消除)

![image-20230424181826097](rust-new.assets/image-20230424181826097.png)



![image-20230424182108812](rust-new.assets/image-20230424182108812.png)

![image-20230424182245107](rust-new.assets/image-20230424182245107.png)

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:47
  |
1 | fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
  |                       -------     -------     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
note: these named lifetimes are available to use
 --> src/main.rs:1:12
  |
1 | fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
  |            ^^  ^^
help: consider using one of the available lifetimes here
  |
1 | fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'lifetime str {
  |                                                +++++++++

```

## 34.5 [方法中的生命周期](https://course.rs/basic/lifetime.html#方法中的生命周期)

定义

```
struct ImportantExcerpt<'a>{
    port : &'a str,
}

//impl 中必须使用结构体的完整名称，包括 <'a>，因为生命周期标注也是结构体类型的一部分！
// 方法签名中，往往不需要标注生命周期，得益于生命周期消除的第一和第三规则
impl<'a> ImportantExcerpt<'a> {
    fn get_port(&self)->i32{
        3
    }
    
}
```

![image-20230425163410446](rust-new.assets/image-20230425163410446.png)



## 34.6 生命周期对比标注

```
struct ImportantExcerpt<'a>{
    port : &'a str,
}

//impl 中必须使用结构体的完整名称，包括 <'a>，因为生命周期标注也是结构体类型的一部分！
// 方法签名中，往往不需要标注生命周期，得益于生命周期消除的第一和第三规则
impl<'a:'b,'b> ImportantExcerpt<'a> {
    fn get_port(&'a self,others :&'b str)->&'b str{
        //lifetime may not live long enough
        others
    }
    
}

fn main(){

}
```



![image-20230425163810874](rust-new.assets/image-20230425163810874.png)

![image-20230425163524969](rust-new.assets/image-20230425163524969.png)



## 34.7 static 静态生命周期

![image-20230425164200372](rust-new.assets/image-20230425164200372.png)



## 34.8 和泛型一起使用

```
struct Str<'a,T>{
    port : &'a str,
    other : T
}

fn set<'a,T>(x : &'a str,b :&'a str,ann : T)->&'a str
where T : std::fmt::Display,
{
    println!("{}",ann);
    if x.len() > b.len(){
        x
    }else{
        b
    }
}

fn main(){}
```


# 35 格式化输出

## 35.1 [`print!`，`println!`，`format!`](https://course.rs/basic/formatted-output.html#printprintlnformat)

它们是 Rust 中用来格式化输出的三大金刚，用途如下：

- `print!` 将格式化文本输出到标准输出，不带换行符
- `println!` 同上，但是在行的末尾添加换行符
- `format!` 将格式化文本输出到 `String` 字符串

## 35.2 [{} 与 {:?}](https://course.rs/basic/formatted-output.html#-与-)

与 `{}` 类似，`{:?}` 也是占位符：

- `{}` 适用于实现了 `std::fmt::Display` 特征的类型，用来以更优雅、更友好的方式格式化文本，例如展示给用户
- `{:?}` 适用于实现了 `std::fmt::Debug` 特征的类型，用于调试场景

其实两者的选择很简单，当你在写代码需要调试时，使用 `{:?}`，剩下的场景，选择 `{}`。

## 35.3 [`Debug` 特征](https://course.rs/basic/formatted-output.html#debug-特征)

事实上，为了方便我们调试，大多数 Rust 类型都实现了 `Debug` 特征或者支持派生该特征：

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    let i = 3.1415926;
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let p = Person{name: "sunface".to_string(), age: 18};
    println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);
}
```

对于数值、字符串、数组，可以直接使用 `{:?}` 进行输出，但是对于结构体，需要[派生`Debug`](https://course.rs/appendix/derive.html)特征后，才能进行输出，总之很简单。

## 35.4 [`Display` 特征](https://course.rs/basic/formatted-output.html#display-特征)

与大部分类型实现了 `Debug` 不同，实现了 `Display` 特征的 Rust 类型并没有那么多，往往需要我们自定义想要的格式化方式：

```rust
let i = 3.1415926;
let s = String::from("hello");
let v = vec![1, 2, 3];
let p = Person {
    name: "sunface".to_string(),
    age: 18,
};
println!("{}, {}, {}, {}", i, s, v, p);
```

运行后可以看到 `v` 和 `p` 都无法通过编译，因为没有实现 `Display` 特征，但是你又不能像派生 `Debug` 一般派生 `Display`，只能另寻他法：

- 使用 `{:?}` 或 `{:#?}`
- 为自定义类型实现 `Display` 特征
- 使用 `newtype` 为外部类型实现 `Display` 特征

下面来一一看看这三种方式。

## 35.5 [{:#?}](https://course.rs/basic/formatted-output.html#)

`{:#?}` 与 `{:?}` 几乎一样，唯一的区别在于它能更优美地输出内容：

```console
// {:?}
[1, 2, 3], Person { name: "sunface", age: 18 }

// {:#?}
[
    1,
    2,
    3,
], Person {
    name: "sunface",
}
```

因此对于 `Display` 不支持的类型，可以考虑使用 `{:#?}` 进行格式化，虽然理论上它更适合进行调试输出。



## 36.6 为自定义类型实现Display

```
struct Person {
    name: String,
    age: u8,
}

use std::fmt;
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "大佬在上，请受我一拜，小弟姓名{}，年芳{}，家里无田又无车，生活苦哈哈",
            self.name, self.age
        )
    }
}
fn main() {
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{}", p);
}

大佬在上，请受我一拜，小弟姓名sunface，年芳18，家里无田又无车，生活苦哈哈
```

## 36.7 [为外部类型实现 `Display` 特征](https://course.rs/basic/formatted-output.html#为外部类型实现-display-特征)

```
struct Array(Vec<i32>);

use std::fmt;
impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "数组是：{:?}", self.0)
    }
}
fn main() {
    let arr = Array(vec![1, 2, 3]);
    println!("{}", arr);
}
数组是：[1, 2, 3]
```

## 36.8 [位置参数](https://course.rs/basic/formatted-output.html#位置参数)

除了按照依次顺序使用值去替换占位符之外，还能让指定位置的参数去替换某个占位符，例如 `{1}`，表示用第二个参数替换该占位符(索引从 0 开始)：

```rust
fn main() {
    println!("{}{}", 1, 2); // =>"12"
    println!("{1}{0}", 1, 2); // =>"21"
    // => Alice, this is Bob. Bob, this is Alice
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{1}{}{0}{}", 1, 2); // => 2112
}
```

## 36.9 [具名参数](https://course.rs/basic/formatted-output.html#具名参数)

除了像上面那样指定位置外，我们还可以为参数指定名称：

```rust
fn main() {
    println!("{argument}", argument = "test"); // => "test"
    println!("{name} {}", 1, name = 2); // => "2 1"
    println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"
}
```

需要注意的是：**带名称的参数必须放在不带名称参数的后面**，例如下面代码将报错：

```rust
println!("{abc} {1}", abc = "def", 2);
error: positional arguments cannot follow named arguments
 --> src/main.rs:4:36
   |
 4 | println!("{abc} {1}", abc = "def", 2);
   |                             -----  ^ positional arguments must be before named arguments
   |                             |
   |                             named argument
```

## 36.10 [格式化参数](https://course.rs/basic/formatted-output.html#格式化参数)

格式化输出，意味着对输出格式会有更多的要求，例如只输出浮点数的小数点后两位：

```rust
fn main() {
    let v = 3.1415926;
    // Display => 3.14
    println!("{:.2}", v);
    // Debug => 3.14
    println!("{:.2?}", v);
}
```

上面代码只输出小数点后两位。同时我们还展示了 `{}` 和 `{:?}` 的用法，后面如无特殊区别，就只针对 `{}` 提供格式化参数说明。

接下来，让我们一起来看看 Rust 中有哪些格式化参数。



## 36.11 宽度

```
fn main() {
    //-----------------------------------
    // 以下全部输出 "Hello x    !"
    // 为"x"后面填充空格，补齐宽度5
    println!("Hello {:5}!", "x");
    // 使用参数5来指定宽度
    println!("Hello {:1$}!", "x", 5);
    // 使用x作为占位符输出内容，同时使用5作为宽度
    println!("Hello {1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("Hello {:width$}!", "x", width = 5);
    //-----------------------------------

    // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
    println!("Hello {:1$}!{}", "x", 5);
}

Hello x    !
Hello x    !
Hello x    !
Hello x    !
Hello x    !5
```

#### [数字填充:符号和 0](https://course.rs/basic/formatted-output.html#数字填充符号和-0)

数字格式化默认也是使用空格进行填充，但与字符串左对齐不同的是，数字是右对齐。

```rust
fn main() {
    // 宽度是5 => Hello     5!
    println!("Hello {:5}!", 5);
    // 显式的输出正号 => Hello +5!
    println!("Hello {:+}!", 5);
    // 宽度5，使用0进行填充 => Hello 00005!
    println!("Hello {:05}!", 5);
    // 负号也要占用一位宽度 => Hello -0005!
    println!("Hello {:05}!", -5);
}
```

### [对齐](https://course.rs/basic/formatted-output.html#对齐)

```rust
fn main() {
    // 以下全部都会补齐5个字符的长度
    // 左对齐 => Hello x    !
    println!("Hello {:<5}!", "x");
    // 右对齐 => Hello     x!
    println!("Hello {:>5}!", "x");
    // 居中对齐 => Hello   x  !
    println!("Hello {:^5}!", "x");

    // 对齐并使用指定符号填充 => Hello x&&&&!
    // 指定符号填充的前提条件是必须有对齐字符
    println!("Hello {:&<5}!", "x");
}
```

### [精度](https://course.rs/basic/formatted-output.html#精度)

精度可以用于控制浮点数的精度或者字符串的长度

```rust
fn main() {
    let v = 3.1415926;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", v, 4);

    let s = "hi我是Sunface孙飞";
    // 保留字符串前三个字符 => hi我
    println!("{:.3}", s);
    // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
    println!("Hello {:.*}!", 3, "abcdefg");
}
```

### [进制](https://course.rs/basic/formatted-output.html#进制)

可以使用 `#` 号来控制数字的进制输出：

- `#b`, 二进制
- `#o`, 八进制
- `#x`, 小写十六进制
- `#X`, 大写十六进制
- `x`, 不带前缀的小写十六进制

```rust
fn main() {
    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);

    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);

    // 使用0填充二进制，宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);
}
```

### [指数](https://course.rs/basic/formatted-output.html#指数)

```rust
fn main() {
    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9
}
```

### [指针地址](https://course.rs/basic/formatted-output.html#指针地址)

```rust
let v= vec![1, 2, 3];
println!("{:p}", v.as_ptr()) // => 0x600002324050
```

### [转义](https://course.rs/basic/formatted-output.html#转义)

有时需要输出 `{`和`}`，但这两个字符是特殊字符，需要进行转义：

```rust
fn main() {
    // "{{" 转义为 '{'   "}}" 转义为 '}'   "\"" 转义为 '"'
    // => Hello "{World}" 
    println!(" Hello \"{{World}}\" ");

    // 下面代码会报错，因为占位符{}只有一个右括号}，左括号被转义成字符串的内容
    // println!(" {{ Hello } ");
    // 也不可使用 '\' 来转义 "{}"
    // println!(" \{ Hello \} ")
}
```

## [在格式化字符串时捕获环境中的值（Rust 1.58 新增）](https://course.rs/basic/formatted-output.html#在格式化字符串时捕获环境中的值rust-158-新增)

在以前，想要输出一个函数的返回值，你需要这么做：

```rust
fn get_person() -> String {
    String::from("sunface")
}
fn main() {
    let p = get_person();
    println!("Hello, {}!", p);                // implicit position
    println!("Hello, {0}!", p);               // explicit index
    println!("Hello, {person}!", person = p);
}
```

问题倒也不大，但是一旦格式化字符串长了后，就会非常冗余，而在 1.58 后，我们可以这么写：

```rust
fn get_person() -> String {
    String::from("sunface")
}
fn main() {
    let person = get_person();
    println!("Hello, {person}!");
}
```

是不是清晰、简洁了很多？甚至还可以将环境中的值用于格式化参数:

```rust
let (width, precision) = get_format();
for (name, score) in get_scores() {
  println!("{name}: {score:width$.precision$}");
}
```

但也有局限，它只能捕获普通的变量，对于更复杂的类型（例如表达式），可以先将它赋值给一个变量或使用以前的 `name = expression` 形式的格式化参数。 目前除了 `panic!` 外，其它接收格式化参数的宏，都可以使用新的特性。对于 `panic!` 而言，如果还在使用 `2015版本` 或 `2018版本`，那 `panic!("{ident}")` 依然会被当成 正常的字符串来处理，同时编译器会给予 `warn` 提示。而对于 `2021版本` ，则可以正常使用:

```rust
fn get_person() -> String {
    String::from("sunface")
}
fn main() {
    let person = get_person();
    panic!("Hello, {person}!");
}
```

输出:

```console
thread 'main' panicked at 'Hello, sunface!', src/main.rs:6:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```



# 37 实现一个文件匹配

## 37.1 接受传入参数

```
cargo run -- searchstring example-filename.txt
-- 告诉 cargo 后面的参数是给我们的程序使用的，而不是给 cargo 自己使用，例如 -- 前的 run 就是给它用的。
```

## 37.2 [使用环境变量来增强程序](https://course.rs/basic-practice/envs.html#使用环境变量来增强程序)

在上一章节中，留下了一个悬念，该如何实现用户控制的大小写敏感，其实答案很简单，你在其它程序中肯定也遇到过不少，例如如何控制 `panic` 后的栈展开？ Rust 提供的解决方案是通过命令行参数来控制:

```shell
RUST_BACKTRACE=1 cargo run
```

与之类似，我们也可以使用环境变量来控制大小写敏感，例如:

```shell
IGNORE_CASE=1 cargo run -- to poem.txt
```

既然有了目标，那么一起来看看该如何实现吧。

# 38 目录结构

![image-20230426180318972](rust-new.assets/image-20230426180318972.png)

# 39 [&'static 和 T: 'static](https://course.rs/advance/lifetime/static.html#static-和-t-static)

`'static` 在 Rust 中是相当常见的，例如字符串字面值就具有 `'static` 生命周期:

`'static` 在 Rust 中是相当常见的，例如字符串字面值就具有 `'static` 生命周期:

```rust
fn main() {
  let mark_twain: &str = "Samuel Clemens";
  print_author(mark_twain);
}
fn print_author(author: &'static str) {
  println!("{}", author);
}
```

除此之外，特征对象的生命周期也是 `'static`，例如[这里](https://course.rs/compiler/fight-with-compiler/lifetime/closure-with-static.html#特征对象的生命周期)所提到的。

除此之外，特征对象的生命周期也是 `'static`，例如[这里](https://course.rs/compiler/fight-with-compiler/lifetime/closure-with-static.html#特征对象的生命周期)所提到的。

除了 `&'static` 的用法外，我们在另外一种场景中也可以见到 `'static` 的使用:

```rust
use std::fmt::Display;
fn main() {
    let mark_twain = "Samuel Clemens";
    print(&mark_twain);
}

fn print<T: Display + 'static>(message: &T) {
    println!("{}", message);
}
```

在这里，很明显 `'static` 是作为生命周期约束来使用了。 **那么问题来了， `&'static` 和 `T: 'static` 的用法到底有何区别？**

## 39.1 [`&'static`](https://course.rs/advance/lifetime/static.html#static)

`&'static` 对于生命周期有着非常强的要求：一个引用必须要活得跟剩下的程序一样久，才能被标注为 `&'static`。

对于字符串字面量来说，它直接被打包到二进制文件中，永远不会被 `drop`，因此它能跟程序活得一样久，自然它的生命周期是 `'static`。

==但是，**`&'static` 生命周期针对的仅仅是引用，而不是持有该引用的变量，对于变量来说，还是要遵循相应的作用域规则** :==

![image-20230426183411245](rust-new.assets/image-20230426183411245.png)

上面代码有两点值得注意：

- `&'static` 的引用确实可以和程序活得一样久，因为我们通过 `get_str_at_location` 函数直接取到了对应的字符串
- 持有 `&'static` 引用的变量，它的生命周期受到作用域的限制，大家务必不要搞混了



## 39.2 [`T: 'static`](https://course.rs/advance/lifetime/static.html#t-static)

相比起来，这种形式的约束就有些复杂了。

首先，在以下两种情况下，`T: 'static` 与 `&'static` 有相同的约束：`T` 必须活得和程序一样久。

![image-20230426183541788](rust-new.assets/image-20230426183541788.png)



```
use std::fmt::Display;

fn main() {
  let r1;
  let r2;
  {
    static STATIC_EXAMPLE: i32 = 42;
    r1 = &STATIC_EXAMPLE;
    let x = "&'static str";
    r2 = x;
    // r1 和 r2 持有的数据都是 'static 的，因此在花括号结束后，并不会被释放
  }

  println!("&'static i32: {}", r1); // -> 42
  println!("&'static str: {}", r2); // -> &'static str

  let r3: &str;

  {
    let s1 = "String".to_string();

    // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
    // 充分说明这个约束是多么的弱。。
    static_bound(&s1);

    // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
    r3 = &s1;

    // s1 在这里被 drop
  }
  println!("{}", r3);
}

fn static_bound<T: Display + 'static>(t: &T) {
  println!("{}", t);
}

```



![image-20230426183716155](rust-new.assets/image-20230426183716155.png)



# 40 [闭包 Closure](https://course.rs/advance/functional-programing/closure.html#闭包-closure)

闭包是**一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值**，例如：

```rust
fn main() {
   let x = 1;
   let sum = |y| x + y;

    assert_eq!(3, sum(2));
}
```

上面的代码展示了非常简单的闭包 `sum`，它拥有一个入参 `y`，同时捕获了作用域中的 `x` 的值，因此调用 `sum(2)` 意味着将 2（参数 `y`）跟 1（`x`）进行相加,最终返回它们的和：`3`。

可以看到 `sum` 非常符合闭包的定义：可以赋值给变量，允许捕获调用者作用域中的值。



![image-20230426184345477](rust-new.assets/image-20230426184345477.png)




## 40.1 闭包的形式

```
#![allow(unused)]
fn main() {

   fn  add_one_v1   (x: u32) -> u32 { x + 1 }

   let add_one_v2 = |x: u32| -> u32 { x + 1 };

   let add_one_v3 = |x|             { x + 1 };
   
   let add_one_v4 = |x|               x + 1  ;
}


```



## 40.2 struct中的闭包

```
struct Cache<T>{
   query :T,
   value:Option<u32>,
}

impl <T> Cache<T> 
   where T : Fn(u32)->u32,
{
   fn new(query:T)->Cache<T>{
      Cache { query, value: None ,}
   }
    
   fn query(&mut self,args:u32)->u32{
      match self.value {
          Some(v)=>v,
          None=> {
            let v = (self.query)(args);
            self.value = Some(v);
            v
          }
      }
   }
}

fn main() {
    
}



```



**实现泛型**

```
struct Cache<T,E>{
  query:T,
  value:Option<E>
}

impl <T,E> Cache<T,E> 
where T: Fn(E)->E,
{
    fn new(query:T)->Cache<T,E>{
      Cache { query: query
        , value: None }
    }

    fn value(&mut self,query:E)->&Option<E>{
      match &self.value {
          Some(_)=>&self.value,
          None=>{
            let v = (self.query)(query);
            self.value = Some(v);
            &self.value
          }
      }
    }
}


fn main(){
  let fs = |e|->String {e};

  let mut ch = Cache::new(fs);

  if let Some(f) = ch.value(String::from("value")) {
     println!("{}",f)
  }

  let function = |a| -> String { a };
  
  let mut cach= Cache::new(function);
  
  if let Some(a) = cach.value(String::from("a")) {
      println!("{}", a)
  }
  

}

value
a
```



![image-20230427111611078](rust-new.assets/image-20230427111611078.png)



## 40.3 捕获 作用域的值

![image-20230427112107529](rust-new.assets/image-20230427112107529.png)

## 40.4 [闭包对内存的影响](https://course.rs/advance/functional-programing/closure.html#闭包对内存的影响)

当闭包从环境中捕获一个值时，会分配内存去存储这些值。对于有些场景来说，这种额外的内存分配会成为一种负担。与之相比，函数就不会去捕获这些环境值，因此定义和使用函数不会拥有这种内存负担。



## 40.5 [三种 Fn 特征](https://course.rs/advance/functional-programing/closure.html#三种-fn-特征)

闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：转移所有权、可变借用、不可变借用，因此相应的 `Fn` 特征也有三种：



### 40.5.1 FnOnce

#### [三种 Fn 的关系](https://course.rs/advance/functional-programing/closure.html#三种-fn-的关系)

实际上，一个闭包并不仅仅实现某一种 `Fn` 特征，规则如下：

- 所有的闭包都自动实现了 `FnOnce` 特征，因此任何一个闭包都至少可以被调用一次
- 没有移出所捕获变量的所有权的闭包自动实现了 `FnMut` 特征
- 不需要对捕获变量进行改变的闭包自动实现了 `Fn` 特征

1. ==`FnOnce`，该类型的闭包会拿走被捕获变量的所有权。`Once` 顾名思义，说明该闭包只能运行一次：==

```
fn func<F>(fns:F)
where F: FnOnce(usize)->bool,
{
  println!("{}",fns(3));
  //use of moved value: `fns`
  println!("{}",fns(4))
}


fn main(){
  let f = |x|->bool{true};

  func(f);
}
```

![image-20230427112838158](rust-new.assets/image-20230427112838158.png)

**仅**实现 `FnOnce` 特征的闭包在调用时会转移所有权，所以显然不能对已失去所有权的闭包变量进行二次调用：

```
2 | fn func<F>(fns:F)
  |            --- move occurs because `fns` has type `F`, which does not implement the `Copy` trait
...
5 |   println!("{}",fns(3));
  |                 ------ `fns` moved due to this call
6 |   //use of moved value: `fns`
7 |   println!("{}",fns(4))
  |                 ^^^ value used here after move
  |
```



![image-20230427113209312](rust-new.assets/image-20230427113209312.png)



### 40.5.2 FnMut

==`FnMut`，它以可变借用的方式捕获了环境中的值，因此可以修改该值：==

```
fn main(){
  let mut st = String::from("hello ");

  let mut clusor = ||{
    st.push_str("string");
  };
  //cannot mutate immutable variable `clusor`rust-analyzerneed-mut
  //cannot borrow `clusor` as mutable, as it is not declared as mutable   
  clusor();


  println!("{}",st);
}

hello string
```



![image-20230427114003614](rust-new.assets/image-20230427114003614.png)

![image-20230427114203959](rust-new.assets/image-20230427114203959.png)



## 40.5.3 Fn

==`Fn` 特征，它以不可变借用的方式捕获环境中的值 让我们把上面的代码中 `exec` 的 `F` 泛型参数类型修改为 `Fn(&'a str)`：==

![image-20230427115500220](rust-new.assets/image-20230427115500220.png)



```
fn main(){
  let mut str = String::from("hello");

  // ^^^^ this closure implements `FnMut`, not `Fn`   
  let colour = |sr|{
     println!("{}",sr)
  };
  exec(colour)
}

fn exec<'a,F: Fn(&'a str)>(f:F){
  f("word")
}

word

正确代码

```



实际上，一个闭包并不仅仅实现某一种 `Fn` 特征，规则如下：

- 所有的闭包都自动实现了 `FnOnce` 特征，因此任何一个闭包都至少可以被调用一次
- 没有移出所捕获变量的所有权的闭包自动实现了 `FnMut` 特征
- 不需要对捕获变量进行改变的闭包自动实现了 `Fn` 特征

三种关系

```
fn main(){
  let mut str = String::from("hello");

  let c = ||{
    println!("{}",str);
  };
  exec(c);
  exec1(c);
  exec2(c);
}


fn exec<F:Fn()>(f:F){
  f()
}

fn exec1<F:FnOnce()>(f:F){
  f()
}

fn exec2<F:FnMut()>(mut f:F){
  f()
}
```



![image-20230427120358049](rust-new.assets/image-20230427120358049.png)

## 40.6 move转移所有权到其他线程

```
use std::thread;
fn main(){
  let vec = vec![1,2,3];
  let f = thread::spawn(
    move || {
      println!("{:?}",vec)
    }
  );

  f.join().unwrap();

  //borrow of moved value: `vec`
  // value borrowed here after moverustcClick for full compiler diagnostic
  println!("{:?}",vec);
}
```



![image-20230427113512011](rust-new.assets/image-20230427113512011.png)



![image-20230427115743040](rust-new.assets/image-20230427115743040.png)



## 40.7 闭包作为返回值

```
#![allow(unused)]

//rait objects must include the `dyn` keywordr
fn factory() ->  impl Fn(i32) -> i32 {
    let num = 5;

    //是所有权的转让
    |x| x + num
    move  |x| x + num
}

//rait objects must include the `dyn` keywordr
fn factory1(x :i32) -> Box<dyn  Fn(i32) -> i32>  {
  let num = 5;

  //是所有权的转让
  // |x| x + num
 
  let num = 5;

  //expected closure, found a different closure
  if x > 1{
     Box::new(move |x| x + num)
  } else {
     Box::new( move |x| x - num)
  }
}






fn main(){
  let f = factory();

  let answer = f(1);
  assert_eq!(6, answer);
  
}

```



![image-20230427141947881](rust-new.assets/image-20230427141947881.png)

![image-20230427142242289](rust-new.assets/image-20230427142242289.png)



# 41 迭代器Iterator

## 41.1 [惰性初始化](https://course.rs/advance/functional-programing/iterator.html#惰性初始化)

```
fn main(){
  let vec = vec![1,2,3,4];

  let vec_itor = vec.into_iter();
  println!("{:?}",vec_itor);

  for i in vec_itor.into_iter(){
    println!("{}",i)
  }
}

IntoIter([1, 2, 3, 4])
1
2
3
4
```

在 `for` 循环之前，我们只是简单的==创建了一个迭代器 `v1_iter`，此时不会发生任何迭代行为，只有在 `for` 循环开始后，迭代器才会开始迭代其中的元素，最后打印出来。==

这种惰性初始化的方式确保了创建迭代器不会有任何额外的==性能损耗==，其中的元素也不会被消耗，只有使用到该迭代器的时候，一切才开始。



## 41.2 next

```
fn main(){
  let arr = [1,2,3];

  let mut arr_itor = arr.into_iter();

  assert_eq!(arr_itor.next(),Some(1));
  assert_eq!(arr_itor.next(),Some(2));
  assert_eq!(arr_itor.next(),Some(3));
}
```



![image-20230427150559602](rust-new.assets/image-20230427150559602.png)

![image-20230427150737483](rust-new.assets/image-20230427150737483.png)



**实现Initorator**

```
fn main(){
  let arr = [1,2,3];

  let result = match IntoIterator::into_iter(arr){
    mut itor => loop{
      match itor.next(){
        Some(v)=>{println!("{}",v)},
        None =>break,
      }
    },
  };
}
```



## 41.3 [into_iter, iter, iter_mut](https://course.rs/advance/functional-programing/iterator.html#into_iter-iter-iter_mut)

在之前的代码中，我们统一使用了 `into_iter` 的方式将数组转化为迭代器，除此之外，还有 `iter` 和 `iter_mut`，聪明的读者应该大概能猜到这三者的区别：

- ==`into_iter` 会夺走所有权==
- ==`iter` 是借用==
- ==`iter_mut` 是可变借用==

![image-20230428101407786](rust-new.assets/image-20230428101407786.png)

#### [Iterator 和 IntoIterator 的区别](https://course.rs/advance/functional-programing/iterator.html#iterator-和-intoiterator-的区别)

这两个其实还蛮容易搞混的，但我们只需要记住，`Iterator` 就是迭代器特征，只有实现了它才能称为迭代器，才能调用 `next`。

而 `IntoIterator` 强调的是某一个类型如果实现了该特征，它可以通过 `into_iter`，`iter` 等方法变成一个迭代器。



## 41.4 消费者与适配器

![image-20230428101722813](rust-new.assets/image-20230428101722813.png)



### 41.4.1迭代器

- 适配器会调用next消费数据
- 迭代器是惰性的，需要适配器来驱动

```
fn main(){

  //可变迭代
  let  vec = vec![1,2,3,4];

  let res :Vec<i32>= vec.iter().map(|x|x+1).collect();
  println!("{:?}",res);
  println!("{:?}",vec)
}

[2, 3, 4, 5]
[1, 2, 3, 4]
```

![image-20230428102221127](rust-new.assets/image-20230428102221127.png)



## 41.5 collect 和zip

#### [collect](https://course.rs/advance/functional-programing/iterator.html#collect)

上面代码中，使用了 `collect` 方法，该方法就是一个消费者适配器，使用它可以将一个迭代器中的元素收集到指定类型中，**这里我们为 `v2` 标注了 `Vec<_>` 类型，就是为了告诉 `collect`：请把迭代器中的元素消费掉，然后把值收集成 `Vec<_>` 类型，至于为何使用 `_`，因为编译器会帮我们自动推导。**

为何 `collect` 在消费时要指定类型？是因为该方法其实很强大，可以收集成多种不同的集合类型，`Vec<T>` 仅仅是其中之一，因此我们必须显式的告诉编译器我们想要收集成的集合类型。

还有一点值得注意，`map` 会对迭代器中的每一个值进行一系列操作，然后把该值转换成另外一个新值，该操作是通过闭包 `|x| x + 1` 来完成：最终迭代器中的每个值都增加了 `1`，从 `[1, 2, 3]` 变为 `[2, 3, 4]`。

再来看看如何使用 `collect` 收集成 `HashMap` 集合：

```
use std::collections::HashMap;

fn main(){


  //是以最短的自动进行匹配
  let  key = vec![1,2,3,4];

  let val = vec![String::from("a"),String::from("b")];

  let res: HashMap<_,_>= key.iter().zip(val.iter()).collect();

  println!("{:?}",res);

  let res :HashMap<_,_> = val.iter().zip(key.iter()).collect();

  println!("{:?}",res);

}

{2: "b", 1: "a"}
{"a": 1, "b": 2}
```

`zip` 是一个迭代器适配器，它的作用就是将两个迭代器的内容压缩到一起，形成 `Iterator<Item=(ValueFromA, ValueFromB)>` 这样的新的迭代器，在此处就是形如 `[(name1, age1), (name2, age2)]` 的迭代器。

然后再通过 `collect` 将新迭代器中`(K, V)` 形式的值收集成 `HashMap<K, V>`，同样的，这里必须显式声明类型，然后 `HashMap` 内部的 `KV` 类型可以交给编译器去推导，最终编译器会推导出 `HashMap<&str, i32>`，完全正确！

## 41.6 闭包作为适配器参数

#### [闭包作为适配器参数](https://course.rs/advance/functional-programing/iterator.html#闭包作为适配器参数)

之前的 `map` 方法中，我们使用闭包来作为迭代器适配器的参数，它最大的好处不仅在于可以就地实现迭代器中元素的处理，还在于可以捕获环境值：

```rust
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

`filter` 是迭代器适配器，用于对迭代器中的每个值进行过滤。 它使用闭包作为参数，该闭包的参数 `s` 是来自迭代器中的值，然后使用 `s` 跟外部环境中的 `shoe_size` 进行比较，若相等，则在迭代器中保留 `s` 值，若不相等，则从迭代器中剔除 `s` 值，最终通过 `collect` 收集为 `Vec<Shoe>` 类型。



## 41.7 实现迭代器功能

![image-20230428104941803](rust-new.assets/image-20230428104941803.png)



## 41.8 enumerate 获取遍历的key及value

```
#![allow(unused)]

use std::process::id;
fn main() {
  let v = vec![1u64, 2, 3, 4, 5, 6];
  
  let val = v.iter().
  enumerate().
  //filter是对数据进行过滤行为的
  filter(|&(idx,_)|idx %2 == 0).
  //map是对数据进行操作的 返回新数据
  map(|(_,val)|val * 3 ).
  fold(0u64, |sum, acm|{
    println!("{:?}",acm);
    sum *2
  } );

  println!("{:?}",val);
}
3
9
15
0
```



## 41.9 [迭代器的性能](https://course.rs/advance/functional-programing/iterator.html#迭代器的性能)



前面提到，要完成集合遍历，既可以使用 `for` 循环也可以使用迭代器，那么二者之间该怎么选择呢，性能有多大差距呢？

理论分析不会有结果，直接测试最为靠谱：

```rust
#![feature(test)]

extern crate rand;
extern crate test;

fn sum_for(x: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}

fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum::<f64>()
}

#[cfg(test)]
mod bench {
    use test::Bencher;
    use rand::{Rng,thread_rng};
    use super::*;

    const LEN: usize = 1024*1024;

    fn rand_array(cnt: u32) -> Vec<f64> {
        let mut rng = thread_rng();
        (0..cnt).map(|_| rng.gen::<f64>()).collect()
    }

    #[bench]
    fn bench_for(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| {
            sum_for(&samples)
        })
    }

    #[bench]
    fn bench_iter(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| {
            sum_iter(&samples)
        })
    }
}
```

上面的代码对比了 `for` 循环和迭代器 `iterator` 完成同样的求和任务的性能对比，可以看到迭代器还要更快一点。

```console
test bench::bench_for  ... bench:     998,331 ns/iter (+/- 36,250)
test bench::bench_iter ... bench:     983,858 ns/iter (+/- 44,673)
```

迭代器是 Rust 的 **零成本抽象**（zero-cost abstractions）之一，意味着抽象并不会引入运行时开销，这与 `Bjarne Stroustrup`（C++ 的设计和实现者）在 `Foundations of C++（2012）` 中所定义的 **零开销**（zero-overhead）如出一辙：

> In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.
>
> 一般来说，C++的实现遵循零开销原则：没有使用时，你不必为其买单。 更进一步说，需要使用时，你也无法写出更优的代码了。 （翻译一下：用就完事了）

总之，迭代器是 Rust 受函数式语言启发而提供的高级语言特性，可以写出更加简洁、逻辑清晰的代码。编译器还可以通过循环展开（Unrolling）、向量化、消除边界检查等优化手段，使得迭代器和 `for` 循环都有极为高效的执行效率。

所以请放心大胆的使用迭代器，在获得更高的表达力的同时，也不会导致运行时的损失，何乐而不为呢！

# 42 类型转换

不要将大转小 否则会因为长度的不同，导致精度丢失

## 42.1 使用as 类型转换

```rust
fn main() {
    let a = 16_i16;
    let b : u8 = 8;

    if a > (b as i16) {
      println!("a > b")
    }
  
  let  a = String::from("222");

  //non-primitive cast: `String` as `i16`
   let b = a as i16;
   println!("{}",b)
}
```



```
fn main() {
    let i16_max = i16::MAX;
    println!("i16-max{}",i16_max);

    let f = 31.32 as u8;
    let a = 'a' as u8;
    println!("{}-{}",f,a)
}

i16-max32767
31-97
```



## 42.2 内存地址转换指针

```
fn main() {
   let mut a :[i32;2] = [1,2];
   let ptr = a.as_mut_ptr();
   let num = ptr as usize; //i32 占用4个字节

   let num1 = num +4;

   let num2 = num1 as *mut i32;
   unsafe{
    *num2 += 1;
   }
   println!("{}",a[1])


}

3
```



==转换不具有传递性 就算 `e as U1 as U2` 是合法的，也不能说明 `e as U2` 是合法的（`e` 不能直接转换成 `U2`）。==



## 42.3 tryInto

不可以实现String -> number的转换

```
use  std::convert::TryInto;
fn main() {
  let a = 45_u32;

  let b:i8 = a.try_into().unwrap();
  println!("{}",b);

  //  let  a = String::from("222");

  //non-primitive cast: `String` as `i16`
  //the trait bound `i16: From<String>` is not satisfied
  //the following other types implement trait `From<T>`:
  // <i16 as From<NonZeroI16>>
  // <i16 as From<bool>>
  // <i16 as From<i8>>
  // <i16 as From<u8>>
  // required for `String` to implement `Into<i16>`
  // required for `i16` to implement `TryFrom<String>`
  // required for `String` to implement `TryInto<i16>`
   let b : i16 = a.try_into().unwrap();
   println!("{}",b)


}
45
45
```



```
use  std::convert::TryInto;
fn main() {
  let a = 45_u32;

  let b:i8 = a.try_into().unwrap();
  println!("{}",b);



//超出范围的报错
let a = 1500;
let _b:u8 = match a.try_into() {
    Ok(m)=> m,
    Err(e) => {
      //type annotations needed
      //cannot infer type
      println!("{:?}", e.to_string());
      0
    },
};

let b: i16 = 1500;
let b_: u8 = match b.try_into() {
    Ok(b1) => b1,
    Err(e) => {
        println!("{:?}", e.to_string());
        0
    }
};



  //  let  a = String::from("222");

  //non-primitive cast: `String` as `i16`
  //the trait bound `i16: From<String>` is not satisfied
  //the following other types implement trait `From<T>`:
  // <i16 as From<NonZeroI16>>
  // <i16 as From<bool>>
  // <i16 as From<i8>>
  // <i16 as From<u8>>
  // required for `String` to implement `Into<i16>`
  // required for `i16` to implement `TryFrom<String>`
  // required for `String` to implement `TryInto<i16>`
  //  let b : i16 = a.try_into().unwrap();
  //  println!("{}",b)

}
```

tryInto 无法转换String->number,并且返回的是Result类型

![image-20230504143151645](rust-new.assets/image-20230504143151645.png)







## 42.4 通用类型转换

```

struct Foo {
    x: u32,
    y: u16,
}

struct Bar {
    a: u32,
    b: u16,
}

fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}

fn reinterpret1(foo:Foo)->Bar{
  Bar { a: foo.x, b: foo.y}
}

fn main() { 

}

```





## 42.5 字符串与数字类型转换

```
let int_value = 5;
//整型转字符串
let string_value = int_value.to_string();

//字符串转32位有符号整型
let back_int = string_value.parse::<i32>().unwrap();

// 字符串转32位无符号整型
let back_int = string_value.parse::<u32>().unwrap();

//字符串转16位有符号整型
let back_int = string_value.parse::<i16>().unwrap(); 

```



# 43 类型

## 43.1 newType



何为 `newtype`？简单来说，就是使用[元组结构体](https://course.rs/basic/compound-type/struct.html#元组结构体tuple-struct)的方式将已有的类型包裹起来：`struct Meters(u32);`，那么此处 `Meters` 就是一个 `newtype`。

为何需要 `newtype`？Rust 这多如繁星的 Old 类型满足不了我们吗？这是因为：

- 自定义类型可以让我们给出更有意义和可读性的类型名，例如与其使用 `u32` 作为距离的单位类型，我们可以使用 `Meters`，它的可读性要好得多
- 对于某些场景，只有 `newtype` 可以很好地解决
- 隐藏内部类型的细节



![image-20230504144709046](rust-new.assets/image-20230504144709046.png)



![image-20230504144909592](rust-new.assets/image-20230504144909592.png)



![image-20230504144959929](rust-new.assets/image-20230504144959929.png)



## 43.2 类型别名 alias

![image-20230504145123933](rust-new.assets/image-20230504145123933.png)

![image-20230504145202668](rust-new.assets/image-20230504145202668.png)



![image-20230504145300637](rust-new.assets/image-20230504145300637.png)





## 43.3 !用不返回类型

![image-20230504145440507](rust-new.assets/image-20230504145440507.png)

神奇的事发生了，此处 `panic` 竟然通过了编译。难道这两个宏拥有不同的返回类型？

你猜的没错：`panic` 的返回值是 `!`，代表它决不会返回任何值，既然没有任何返回值，那自然不会存在分支类型不匹配的情况。

# 44 [Sized 和不定长类型 DST](https://course.rs/advance/into-types/sized.html#sized-和不定长类型-dst)

- 定长类型( sized )，这些类型的大小在编译时是已知的
- 不定长类型( unsized )，与定长类型相反，它的大小只有到了程序运行时才能动态获知，这种类型又被称之为 DST

## 44.1 dst

读者大大们之前学过的几乎所有类型，都是固定大小的类型，包括集合 `Vec`、`String` 和 `HashMap` 等，而动态大小类型刚好与之相反：**编译器无法在编译期得知该类型值的大小，只有到了程序运行时，才能动态获知**。对于动态类型，我们使用 `DST`(dynamically sized types)或者 `unsized` 类型来称呼它。



![image-20230504153255523](rust-new.assets/image-20230504153255523.png)

![image-20230504153335763](rust-new.assets/image-20230504153335763.png)



![image-20230504153438069](rust-new.assets/image-20230504153438069.png)



## 44.2 sized特征

![image-20230504153626647](rust-new.assets/image-20230504153626647.png)



![image-20230504153710565](rust-new.assets/image-20230504153710565.png)



## 44.3 [`Box`](https://course.rs/advance/into-types/sized.html#boxstr)



```
#![allow(unused)]
fn main() {

  //the size for values of type `str` cannot be known at compilation time
  //the trait `Sized` is not implemented for `str`
  let s1: Box<str> = Box::new("Hello there!" as str);

  let s :Box<str> = "hello world".into();
}

```



# 45 枚举和整数

```
enum MyEnum {
    A = 1,
    B,
    C,
}

fn main() {
    // 将枚举转换成整数，顺利通过
    let x = MyEnum::C as i32;

    // 将整数转换为枚举，失败
    match x {
        MyEnum::A => {}
        MyEnum::B => {}
        MyEnum::C => {}
        _ => {}
    }
}
MyEnum::A => {} mismatched types, expected i32, found enum MyEnum。
```



## 45.1 使用第三方库

```
使用三方库
首先可以想到的肯定是三方库，毕竟 Rust 的生态目前已经发展的很不错，类似的需求总是有的，这里我们先使用num-traits和num-derive来试试。

在Cargo.toml中引入：


[dependencies]
num-traits = "0.2.14"
num-derive = "0.3.3"
代码如下:


use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
enum MyEnum {
    A = 1,
    B,
    C,
}

fn main() {
    let x = 2;

    match FromPrimitive::from_i32(x) {
        Some(MyEnum::A) => println!("Got A"),
        Some(MyEnum::B) => println!("Got B"),
        Some(MyEnum::C) => println!("Got C"),
        None            => println!("Couldn't convert {}", x),
    }
}
除了上面的库，还可以使用一个较新的库: num_enums。


```



## 45.2 [TryFrom + 宏](https://course.rs/advance/into-types/enum-int.html#tryfrom--宏)



在 Rust 1.34 后，可以实现`TryFrom`特征来做转换:

```rust
use std::convert::TryFrom;

impl TryFrom<i32> for MyEnum {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}
```

以上代码定义了从`i32`到`MyEnum`的转换，接着就可以使用`TryInto`来实现转换：

```rust
use std::convert::TryInto;

fn main() {
    let x = MyEnum::C as i32;

    match x.try_into() {
        Ok(MyEnum::A) => println!("a"),
        Ok(MyEnum::B) => println!("b"),
        Ok(MyEnum::C) => println!("c"),
        Err(_) => eprintln!("unknown number"),
    }
}
```

但是上面的代码有个问题，你需要为每个枚举成员都实现一个转换分支，非常麻烦。好在可以使用宏来简化，自动根据枚举的定义来实现`TryFrom`特征:

```rust
#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {
    enum MyEnum {
        A = 1,
        B,
        C,
    }
}
```



## 45.3 [邪恶之王 std::mem::transmute](https://course.rs/advance/into-types/enum-int.html#邪恶之王-stdmemtransmute)

**这个方法原则上并不推荐，但是有其存在的意义，如果要使用，你需要清晰的知道自己为什么使用**。

在之前的类型转换章节，我们提到过非常邪恶的[`transmute`转换](https://course.rs/advance/into-types/converse.html#变形记transmutes)，其实，当你知道数值一定不会超过枚举的范围时(例如枚举成员对应 1，2，3，传入的整数也在这个范围内)，就可以使用这个方法完成变形。

> 最好使用#[repr(..)]来控制底层类型的大小，免得本来需要 i32，结果传入 i64，最终内存无法对齐，产生奇怪的结果

```rust
#[repr(i32)]
enum MyEnum {
    A = 1, B, C
}

fn main() {
    let x = MyEnum::C;
    let y = x as i32;
    let z: MyEnum = unsafe { std::mem::transmute(y) };

    // match the enum that came from an int
    match z {
        MyEnum::A => { println!("Found A"); }
        MyEnum::B => { println!("Found B"); }
        MyEnum::C => { println!("Found C"); }
    }
}
```

既然是邪恶之王，当然得有真本事，无需标准库、也无需 unstable 的 Rust 版本，我们就完成了转换！awesome!??



# 46 Box<T>



## 46.1 rust 堆栈

![image-20230504161211521](rust-new.assets/image-20230504161211521.png)



## 46.2 堆栈的性能

![image-20230504161337115](rust-new.assets/image-20230504161337115.png)



## 46.3 [Box 的使用场景](https://course.rs/advance/smart-pointer/box.html#box-的使用场景)

![image-20230504161428636](rust-new.assets/image-20230504161428636.png)



## 46.4 将数据存储在堆上

如果一个变量拥有一个数值 `let a = 3`，那变量 `a` 必然是存储在栈上的，那如果我们想要 `a` 的值存储在堆上就需要使用 `Box<T>`：

```rust
fn main() {
  //分配在栈上
  let x = 2;

  //分配在堆上
  let x :Box<i32> = Box::new(32);
   // 下面一行代码将报错
    // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`
}

```

![image-20230504161901925](rust-new.assets/image-20230504161901925.png)



## 46.5 [避免栈上数据的拷贝](https://course.rs/advance/smart-pointer/box.html#避免栈上数据的拷贝)

- ==栈数据所有权转换，是copy一份新的==
- ==堆上的所有权转移，是指针的转移==

当栈上数据转移所有权时，实际上是把数据拷贝了一份，最终新旧变量各自拥有不同的数据，因此所有权并未转移。

而堆上则不然，底层数据并不会被拷贝，转移所有权仅仅是复制一份栈中的指针，再将新的指针赋予新的变量，然后让拥有旧指针的变量失效，最终完成了所有权的转移：

```
fn main(){
  let arr = [0;1000];

  let arr1 = arr;

  println!("arr的地址是-{:?},arr1的地址是-{:?}",arr.as_ptr(),arr1.as_ptr());
  //因为是发生拷贝，所以不会有问题
  println!("arr.len-{},arr1.len-{}",arr.len(),arr1.len());

  let arr = Box::new([0;1000]);
  println!("arr的地址是-{:p}",&arr);
  let arr1 = arr;
  println!("arr1的地址是-{:p}",&arr1);

   //因为是发生拷贝，所以不会有问题
   //borrow of moved value: `arr`
  //value borrowed here after move    
  //  println!("arr.len-{:?},arr1.len-{}",arr.len(),arr1.len());
}
```



## 46.6 [将动态大小类型变为 Sized 固定大小类型](https://course.rs/advance/smart-pointer/box.html#将动态大小类型变为-sized-固定大小类型)

![image-20230504163030826](rust-new.assets/image-20230504163030826.png)



## 46.7 特征对象

```
trait Draw {
    fn draw(&self);
}

struct Button{}


struct Top{}

impl Draw for Button {
    
    fn draw(&self) {
        println!("Button-draw")
    }
}

impl Draw for Top {
    fn draw(&self) {
      println!("Top-draw")

    }
}

fn main(){
  let list:Vec<Box<dyn Draw>> =  vec![Box::new(Button{}),Box::new(Top{})];

  for i in list{
    i.draw()
  }
  
}

Button-draw
Top-draw
```



以上代码将不同类型的 `Button` 和 `Select` 包装成 `Draw` 特征的特征对象，放入一个数组中，`Box<dyn Draw>` 就是特征对象。

其实，特征也是 DST 类型，而特征对象在做的就是将 DST 类型转换为固定大小类型。



## 46.8 内存布局

![image-20230504171133724](rust-new.assets/image-20230504171133724.png)

## 46.9 Box::leak

![image-20230504171432693](rust-new.assets/image-20230504171432693.png)



# 47 cargo 发布配置

![image-20230504205421710](rust-new.assets/image-20230504205421710-3208266.png)

## 47.1 通过release profile自定义构建

![image-20230504205546551](rust-new.assets/image-20230504205546551-3208266.png)



![image-20230504210042357](rust-new.assets/image-20230504210042357-3208266.png)

![image-20230504205945727](rust-new.assets/image-20230504205945727-3208266.png)



## 47.2 文档注释

![image-20230504210314287](rust-new.assets/image-20230504210314287-3208266.png)



![image-20230504210339901](rust-new.assets/image-20230504210339901-3208266.png)



![image-20230504210506746](rust-new.assets/image-20230504210506746-3208266.png)



![image-20230504210621779](rust-new.assets/image-20230504210621779-3208266.png)



![image-20230504210825166](rust-new.assets/image-20230504210825166-3208266.png)

![image-20230504210857626](rust-new.assets/image-20230504210857626-3208266.png)



## 47.3 pub use

![image-20230504211112345](rust-new.assets/image-20230504211112345-3208266.png)



![image-20230504211240475](rust-new.assets/image-20230504211240475-3208266.png)





![image-20230504211338310](rust-new.assets/image-20230504211338310-3208266.png)



![image-20230504211351022](rust-new.assets/image-20230504211351022-3208266.png)



![image-20230504211444161](rust-new.assets/image-20230504211444161-3208266.png)





## 47.4 发布crate 到crate.io

crate.io 创建账号

进行登录

用户设置-》api-token

![image-20230504211554305](rust-new.assets/image-20230504211554305-3208266.png)

![image-20230504211614517](rust-new.assets/image-20230504211614517-3208266.png)



在package 修改内容

- name-唯一性
- describe 描述 
- license 许可标识值
- vension 
- auther

![image-20230504211743909](rust-new.assets/image-20230504211743909-3208266.png)

发布

![image-20230504211904033](rust-new.assets/image-20230504211904033-3208266.png)

版本发布

![image-20230504212022017](rust-new.assets/image-20230504212022017-3208266.png)



撤回发布版本

![image-20230504212147258](rust-new.assets/image-20230504212147258-3208266.png)

## 47.5 工作空间workspace

![image-20230504212248846](rust-new.assets/image-20230504212248846-3208266.png)



![image-20230504212335200](rust-new.assets/image-20230504212335200-3208266.png)



![image-20230504212610362](rust-new.assets/image-20230504212610362-3208266.png)



![image-20230504212718200](rust-new.assets/image-20230504212718200-3208266.png)



依赖外部的crate

![image-20230504212911365](rust-new.assets/image-20230504212911365-3208266.png)


# 48 deref解引用

![image-20230506151306246](rust-new.assets/image-20230506151306246.png)



## 48.1 [通过 `*` 获取引用背后的值](https://course.rs/advance/smart-pointer/deref.html#通过--获取引用背后的值)

![image-20230506151344295](rust-new.assets/image-20230506151344295.png)



# 49 [Rc 与 Arc 实现 1vN 所有权机制](https://course.rs/advance/smart-pointer/rc-arc.html)

==`Rc` 和 `Arc` 的区别在于，后者是原子化实现的引用计数，因此是线程安全的，可以用于多线程中共享数据。==

==这两者都是只读的，如果想要实现内部数据可修改，必须配合内部可变性 `RefCell` 或者互斥锁 `Mutex` 来一起使用。==

## 49.1 Rc单线程

```
use std::rc::Rc;
fn main(){

  let a = String::from("value");

  let b = a;

  //use of moved value: `a`
  //value used here after moverustcClick for full compiler diagnostic
  // let c = a;



  let a = Rc::new(String::from("value"));

  let b = Rc::clone(&a);
  let c = a.clone();

}
```

解决单线程的 变量被单线程同事拥有多个拥有者

以上代码我们使用 `Rc::new` 创建了一个新的 `Rc<String>` 智能指针并赋给变量 `a`，该指针指向底层的字符串数据。

智能指针 `Rc<T>` 在创建时，还会将引用计数加 1，此时获取引用计数的关联函数 `Rc::strong_count` 返回的值将是 `1`。





## 49.2 Rc::clone

Rc::clone 不是真的克隆底层数据，而是对引用进行+1操作

```
use std::rc::Rc;
fn main(){

  let a = Rc::new(String::from("value"));
  println!("{}",Rc::strong_count(&a)); //1

  let b = Rc::clone(&a);
  println!("{}",Rc::strong_count(&b));//2

  {
    let d = Rc::clone(&a);
    println!("{}",Rc::strong_count(&d));//3
  
  }

  let c = a.clone();//3
  println!("{}",Rc::strong_count(&c));

}
1
2
3
3
```

事实上，==`Rc<T>` 是指向底层数据的不可变的引用，因此你无法通过它来修改数据==，这也符合 Rust 的借用规则：要么存在多个不可变借用，要么只能存在一个可变借用。

但是实际开发中我们往往需要对数据进行修改，这时单独使用 `Rc<T>` 无法满足我们的需求，需要配合其它数据类型来一起使用，例如内部可变性的 `RefCell<T>` 类型以及互斥锁 `Mutex<T>`。事实上，在多线程编程中，`Arc` 跟 `Mutex` 锁的组合使用非常常见，它们既可以让我们在不同的线程中共享数据，又允许在各个线程中对其进行修改。



## 49.3 例子

```
use std::rc::Rc;

struct Owner {
  name :String
}

struct Gadget{
  id : i32,
  onwer : Rc<Owner>
}

fn main(){
  let onwer = Rc::new(Owner{
    name : String::from("onwer")
  });

  let gadget = Gadget{
    id:1,
    onwer:Rc::clone(&onwer)
  };

  let gadget1 = Gadget{
    id:2,
    onwer:Rc::clone(&onwer)
  };
  drop(onwer);

  println!("id-{},onwer-{}",gadget.id,gadget.onwer.name);
  println!("id-{},onwer-{}",gadget1.id,gadget1.onwer.name);
}
id-1,onwer-onwer
id-2,onwer-onwer
```



[Rc 简单总结](https://course.rs/advance/smart-pointer/rc-arc.html#rc-简单总结)

- ==`Rc/Arc` 是不可变引用，你无法修改它指向的值，只能进行读取==，如果要修改，需要配合后面章节的==内部可变性 `RefCell` 或互斥锁 `Mutex`==
- 一旦最后一个拥有者消失，则资源会自动被回收，这个生命周期是在编译期就确定下来的
- ==`Rc` 只能用于同一线程内部，想要用于线程之间的对象共享，你需要使用 `Arc`==
- `Rc<T>` 是一个智能指针，实现了 `Deref` 特征，因此你无需先解开 `Rc` 指针，再使用里面的 `T`，而是可以直接使用 `T`，例如上例中的 `gadget1.owner.name`

## 49.4 Arc

![image-20230506170329309](rust-new.assets/image-20230506170329309.png)



`Arc` 是 `Atomic Rc` 的缩写，顾名思义：原子化的 `Rc<T>` 智能指针。原子化是一种并发原语

![image-20230506170428529](rust-new.assets/image-20230506170428529.png)



```
use std::sync::Arc;
use std::thread;


fn main(){
  let s = Arc::new(String::from("多线程"));

  //`Rc<String>` cannot be sent between threads safely
  for i in 1..10{
    let s1 = Arc::clone(&s);
    let handle = thread::spawn(move ||{
      println!("{}-{}",i,s1)
    });
  };
}

1-多线程
3-多线程
4-多线程
2-多线程
5-多线程
6-多线程
7-多线程
8-多线程
9-多线程
```





# 50 [Cell 与 RefCell 内部可变性](https://course.rs/advance/smart-pointer/cell-refcell.html)



## 50.1 Cell 实现copy的基础类型

```
use std::{self, cell::Cell};


fn main(){
  let s = Cell::new("asdv");

  let s1 = s.get(); //asdv

  s.set("val");

  let s2 = s.get();//val
  
  println!("s1-{} s2{}",s1,s2)
}

s1-asdv s2val

```



![image-20230506175117026](rust-new.assets/image-20230506175117026.png)





## 50.2 Refcell 未实现copy的heap类型

由于 `Cell` 类型针对的是实现了 `Copy` 特征的值类型，因此在实际开发中，`Cell` 使用的并不多，因为我们要解决的往往==是可变、不可变引用共存==导致的问题，此时就需要借助于 `RefCell` 来达成目的。

[RefCell 简单总结](https://course.rs/advance/smart-pointer/cell-refcell.html#refcell-简单总结)

- 与 `Cell` 用于可 `Copy` 的值不同，`RefCell` 用于引用
- `RefCell` 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
- `RefCell` 适用于编译期误报或者一个引用被在多处代码使用、修改以至于难于管理借用关系时
- 使用 `RefCell` 时，违背借用规则会导致运行期的 `panic`

![image-20230506180230732](rust-new.assets/image-20230506180230732.png)



![image-20230506180343694](rust-new.assets/image-20230506180343694.png)



## 50.3 [选择 `Cell` 还是 `RefCell`](https://course.rs/advance/smart-pointer/cell-refcell.html#选择-cell-还是-refcell)

根据本文的内容，我们可以大概总结下两者的区别：

- `Cell` 只适用于 `Copy` 类型，用于提供值，而 `RefCell` 用于提供引用
- `Cell` 不会 `panic`，而 `RefCell` 会

![image-20230506180719957](rust-new.assets/image-20230506180719957.png)



## 50.4 内部可用性

![image-20230506181158335](rust-new.assets/image-20230506181158335.png)



```
use std::cell::RefCell;
trait Messager {
    fn send(&self,msg :String);
}

struct Message {
  mes_cache : RefCell<Vec<String>>
}

impl Messager for Message {
    fn send(&self,msg :String) {
        self.mes_cache.borrow_mut().push(msg)
    }
}


fn main(){
    let msg = Message{
      mes_cache : RefCell::new(vec!["hello".to_string(),"world".to_string()]),
    };
    msg.send("你好".to_string());

    println!("{:#?}",msg.mes_cache)
}

RefCell {
    value: [
        "hello",
        "world",
        "你好",
    ],
}

```



![image-20230506182108535](rust-new.assets/image-20230506182108535.png)



## 50.5 Rc+RefCell 单线程不可变多拥有者+引用类型多可变

```
use std::rc::Rc;
use std::cell::RefCell;


fn main(){
  let s  = Rc::new(RefCell::new(String::from("这是单线程rc，一个变量多所有者 和 引用类型多可变RefCell（可解决编译器错误报警），基本类型用cell，实现copy的")));

  let s2 = s.clone();
  println!("s2 -{:?}",s2);

  s2.borrow_mut().push_str("end!!!");

  let s3 = s2.clone();


  println!("因为是引用类型，所以导致最终一致");
  println!("s -{:?}  ",s);
  println!("s2 -{:?}",s2);
  println!("s3-{:?}",s3);
}

s2 -RefCell { value: "这是单线程rc，一个变量多所有者 和 引用类型多可变RefCell（可解决编译器错误报警），基本类型用cell，实现copy的" }
因为是引用类型，所以导致最终一致
s -RefCell { value: "这是单线程rc，一个变量多所有者 和 引用类型多可变RefCell（可解决编译器错误报警），基本类型用cell，实现copy的end!!!" }  
s2 -RefCell { value: "这是单线程rc，一个变量多所有者 和 引用类型多可变RefCell（可解决编译器错误报警），基本类型用cell，实现copy的end!!!" }
s3-RefCell { value: "这是单线程rc，一个变量多所有者 和 引用类型多可变RefCell（可解决编译器错误报警），基本类型用cell，实现copy的end!!!" }
```



## 50.6 内存消耗

![image-20230506183541836](rust-new.assets/image-20230506183541836.png)



## 50.7 [通过 `Cell::from_mut` 解决借用冲突](https://course.rs/advance/smart-pointer/cell-refcell.html#通过-cellfrom_mut-解决借用冲突)

在 Rust 1.37 版本中新增了两个非常实用的方法：

- Cell::from_mut，该方法将 `&mut T` 转为 `&Cell<T>`
- Cell::as_slice_of_cells，该方法将 `&Cell<[T]>` 转为 `&[Cell<T>]`

这里我们不做深入的介绍，但是来看看如何使用这两个方法来解决一个常见的借用冲突问题：

```rust
fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn retain_even(nums: &mut Vec<i32>) {
    let mut i = 0;
    for num in nums.iter().filter(|&num| is_even(*num)) {
        nums[i] = *num;
        i += 1;
    }
    nums.truncate(i);
}
```

以上代码会报错：

```console
error[E0502]: cannot borrow `*nums` as mutable because it is also borrowed as immutable
 --> src/main.rs:8:9
  |
7 |     for num in nums.iter().filter(|&num| is_even(*num)) {
  |                ----------------------------------------
  |                |
  |                immutable borrow occurs here
  |                immutable borrow later used here
8 |         nums[i] = *num;
  |         ^^^^ mutable borrow occurs here
```

很明显，报错是因为同时借用了不可变与可变引用，你可以通过索引的方式来避免这个问题：

```rust
fn retain_even(nums: &mut Vec<i32>) {
    let mut i = 0;
    for j in 0..nums.len() {
        if is_even(nums[j]) {
            nums[i] = nums[j];
            i += 1;
        }
    }
    nums.truncate(i);
}
```

但是这样就违背我们的初衷了，毕竟迭代器会让代码更加简洁，那么还有其它的办法吗？

这时就可以使用 `Cell` 新增的这两个方法：

```rust
use std::cell::Cell;

fn retain_even(nums: &mut Vec<i32>) {
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..])
        .as_slice_of_cells();

    let mut i = 0;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}
```

此时代码将不会报错，因为 `Cell` 上的 `set` 方法获取的是不可变引用 `pub fn set(&self, val: T)`。

当然，以上代码的本质还是对 `Cell` 的运用，只不过这两个方法可以很方便的帮我们把 `&mut [T]` 类型转换成 `&[Cell<T>]` 类型。

# 51 [Weak 与循环引用](https://course.rs/advance/circle-self-ref/circle-reference.html#weak-与循环引用)



## 51.1 weak

`Weak` 通过 `use std::rc::Weak` 来引入，它具有以下特点:

- 可访问，但没有所有权，不增加引用计数，因此不会影响被引用值的释放回收
- 可由 `Rc<T>` 调用 `downgrade` 方法转换成 `Weak<T>`
- `Weak<T>` 可使用 `upgrade` 方法转换成 `Option<Rc<T>>`，如果资源已经被释放，则 `Option` 的值是 `None`
- 常用于解决循环引用的问题

![image-20230508115806966](rust-new.assets/image-20230508115806966.png)

==有待考察==

```
use std::rc::Rc;
fn main(){
  let five = Rc::new(5);

  //创建weak指针
  let down = Rc::downgrade(&five);

  //获取值
  let down_1 = down.upgrade();

  println!("{:?}",down_1);

  drop(five);

  let up = down.upgrade();

  println!("{:?}",up);


  // 创建Rc，持有一个值5
  let five = Rc::new(5);

  // 通过Rc，创建一个Weak指针
  let weak_five = Rc::downgrade(&five);

  // Weak引用的资源依然存在，取到值5
  let strong_five: Option<Rc<_>> = weak_five.upgrade();
 println!("{:?}",strong_five);

  // 手动释放资源`five`
  drop(five);
  // println!("{}",five);

  // Weak引用的资源已不存在，因此返回None
  let strong_five: Option<Rc<_>> = weak_five.upgrade();
  match strong_five {
      Some(v)=> println!("{}",v),
      None=>println!("none")
  }
  // println!("{:?}",strong_five);

}

Some(5)
Some(5)
Some(5)
5
```



# 52 多线程并发编程

## 52.1 线程屏障 barrier

在 Rust 中，可以使用 `Barrier` 让多个线程都执行到某个点后，才继续一起往后执行：

```
use std::sync::{Arc, Barrier};
use std::thread;


fn main(){
  let mut handles = Vec::with_capacity(6);

  let barrier = Arc::new(Barrier::new(6));

  for i in 0..6{
    //增加多线程的引用计数
    let b = barrier.clone();

    let handle = thread::spawn(move ||{
      println!("befer-{}",i);
      b.wait();
      println!("after-{}",i);
    });

    handles.push(handle);
  }

  for i in handles{
    i.join().unwrap();
  }

}

befer-1
befer-0
befer-5
befer-2
befer-3
befer-4
after-4
after-2
after-0
after-5
after-3
after-1
```



## 52.2 [线程局部变量(Thread Local Variable)](https://course.rs/advance/concurrency-with-threads/thread.html#线程局部变量thread-local-variable)

### 52.1 ==标准库thread_local==

```
use  std::thread;
use std::cell::RefCell;

fn main(){
  thread_local! {static FOO : RefCell<i32> = RefCell::new(1)};

  FOO.with(|f|{
      assert_eq!(*f.borrow(),1);
      *f.borrow_mut() = 2;
  });

  //每个线程拿到Foo的初始值main的 调用前面的
  let t = thread::spawn(move ||{
    FOO.with(|f|{
      assert_eq!(*f.borrow(),1);
      *f.borrow_mut() = 3;
    })
  });

  t.join().unwrap();

  FOO.with(|f|{
    assert_eq!(*f.borrow(),2);
  })

}
```

注意 `FOO` 使用 `static` 声明为生命周期为 `'static` 的静态变量。

![image-20230508161424873](rust-new.assets/image-20230508161424873.png)



### 52.2 [三方库 thread-local](https://course.rs/advance/concurrency-with-threads/thread.html#三方库-thread-local)

[thread-local](https://github.com/Amanieu/thread_local-rs)

```
use std::{sync::Arc, thread};
use thread_local::ThreadLocal;
use std::cell::Cell;



fn main(){
  let thread_local = Arc::new(ThreadLocal::new());

  for _ in 0..5{
    let tls2 = thread_local.clone();
    thread::spawn(move ||{
        //将计数器+1
        let cell = tls2.get_or(|| Cell::new(0));
        cell.set(cell.get()+1);
    }).join().unwrap();
  }

  //自线程结束，收集子线程改变的值
  let tls = Arc::try_unwrap(thread_local).unwrap();
  let total = tls.into_iter().fold(0, |x,y|{
      x + y.get()
  });

  println!("{}",total)
}

5
```

![image-20230508170649239](rust-new.assets/image-20230508170649239.png)



## 52.3 [用条件控制线程的挂起和执行](https://course.rs/advance/concurrency-with-threads/thread.html#用条件控制线程的挂起和执行)

条件变量(Condition Variables)经常和 `Mutex` 一起使用，可以让线程挂起，直到某个条件发生后再继续执行：

```
use std::sync::{Arc, Mutex, Condvar};
use std::thread;



fn main(){
  let pair = Arc::new((Mutex::new(false),Condvar::new()));

  let pair2 = pair.clone();

  thread::spawn(move ||{
    let (lock,cvar) = &*pair2;
    let mut locked = lock.lock().unwrap();
    println!("changing locked");
    *locked = true;
    cvar.notify_one();
  });

  let (lock ,cvar)= &*pair;
  
  // 获取锁
  // lock2.lock().unwrap().push_str(" thread ");
  // 释放锁
  // 不用主动释放的原因是，Rust 会记住 lock() 时的作用域，离开作用域会自动释放
  
  let mut started = lock.lock().unwrap();
  while !*started {
      started = cvar.wait(started).unwrap();//true
      println!("{}",started)
  }
  println!("started changed");
}
```



![image-20230508180431268](rust-new.assets/image-20230508180431268.png)





![image-20230508180743053](rust-new.assets/image-20230508180743053.png)





## 52.4 [只被调用一次的函数](https://course.rs/advance/concurrency-with-threads/thread.html#只被调用一次的函数)

```
use std::{sync::Once, thread};

fn main(){
  static mut VALUE :usize = 0;
  static INIT:Once = Once::new();

  let h = thread::spawn(||{
    INIT.call_once(||{
      unsafe{
        VALUE = 1
      }
    });
  });


  let h1 = thread::spawn(||{
    INIT.call_once(||{
      unsafe{
        VALUE = 2
      }
    });
  });

  h.join().unwrap();
  h1.join().unwrap();
  println!("{}", unsafe { VALUE });
}


```

![image-20230508181651461](rust-new.assets/image-20230508181651461.png)


# 53 线程间消息传递

标准库提供了通道`std::sync::mpsc`，其中`mpsc`是*multiple producer, single consumer*的缩写

通道支持多个发送者，但是只支持唯一的接收者。



## 53.1 [多发送者，单接收者](https://course.rs/advance/concurrency-with-threads/message-passing.html#多发送者单接收者)

```
use std::{sync::mpsc, thread,time};


fn main(){

  //创建接收者和发送者
  let (tx,rx) = mpsc::channel();

  let tx1 = tx.clone();

  thread::spawn(move ||{
    //发送数字，send返回的Result类型
    tx1.send(1).unwrap();

    //mismatched types
    //expected type `{integer}`
    //found enum `Option<{integer}>
    // tx.send(Some(1)).unwrap();

  });
  let tx2 = tx.clone();
  thread::spawn(move ||{
    //发送数字，send返回的Result类型
    tx2.send(2).unwrap();

    //mismatched types
    //expected type `{integer}`
    //found enum `Option<{integer}>
    // tx.send(Some(1)).unwrap();
    
  });

  println!("recv-{}",rx.recv().unwrap());
  println!("recv-{}",rx.recv().unwrap());

  thread::sleep(time::Duration::from_secs(3))
}

recv-1
recv-2
```

![image-20230509110044521](rust-new.assets/image-20230509110044521.png)



## 53.2 [不阻塞的 try_recv 方法](https://course.rs/advance/concurrency-with-threads/message-passing.html#不阻塞的-try_recv-方法)

除了上述`recv`方法，还可以使用`try_recv`尝试接收一次消息，该方法并**不会阻塞线程**，当通道中没有消息时，它会立刻返回一个错误：

```
use std::{sync::mpsc, thread,time};


fn main(){

  //创建接收者和发送者
  let (tx,rx) = mpsc::channel();

  let tx1 = tx.clone();

  // println!("{}",Arc::strong_count(&tx1));
  thread::spawn(move ||{
    //发送数字，send返回的Result类型
    tx1.send(1).unwrap();

    //mismatched types
    //expected type `{integer}`
    //found enum `Option<{integer}>
    // tx.send(Some(1)).unwrap();

  });
  let tx2 = tx.clone();
  thread::spawn(move ||{
    //发送数字，send返回的Result类型
    tx2.send(2).unwrap();

    //mismatched types
    //expected type `{integer}`
    //found enum `Option<{integer}>
    // tx.send(Some(1)).unwrap();
    
  });

  println!("recv-{:?}",rx.try_recv());
  println!("recv-{:?}",rx.try_recv().unwrap());
  //recv-Err(Empty)
  //recv-1

  thread::sleep(time::Duration::from_secs(3))
}

recv-Err(Empty)
recv-1
```

![image-20230509110418067](rust-new.assets/image-20230509110418067.png)



## 53.3 传输具有所有权的数据

使用通道来传输数据，一样要遵循 Rust 的所有权规则：

- 若值的类型实现了`Copy`特征，则直接复制一份该值，然后传输过去，例如之前的`i32`类型
- ==若值没有实现`Copy`，则它的所有权会被转移给接收端，在发送端继续使用该值将报错==

```
use std::{sync::mpsc, thread,time};


fn main(){

  //创建接收者和发送者
  let (tx,rx) = mpsc::channel();

  let tx1 = tx.clone();

  // println!("{}",Arc::strong_count(&tx1));
  thread::spawn(move ||{

    let s = "我，飞了".to_string();
    //发送数字，send返回的Result类型
    tx1.send(s).unwrap();

    //orrow of moved value: `s`
    //value borrowed here after move
    // println!("{}",s);

  });
  let tx2 = tx.clone();
  thread::spawn(move ||{
    let s = "我，飞了2".to_string();

    //发送数字，send返回的Result类型
    tx2.send(s).unwrap();

  });

  println!("recv-{:?}",rx.recv());
  println!("recv-{:?}",rx.recv().unwrap());
  //recv-Ok("我，飞了")
  //recv-"我，飞了2"

  thread::sleep(time::Duration::from_secs(3))
}
```



## 53.4 使用for进行接收

```
use std::{sync::mpsc, thread,time::Duration};


fn main(){

  //创建接收者和发送者
  let (tx,rx) = mpsc::channel();

  thread::spawn(move ||{
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
  ];

  for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
  }

  });
  

  for reve in rx{
    println!("recv-{:?}",reve);
  }
  //recv-"hi"
  //recv-"from"
  //recv-"the"
  //recv-"thread"


}
```



## 54.5 多发送者

由于子线程会拿走发送者的所有权，因此我们必须对发送者进行克隆，然后让每个线程拿走它的一份拷贝:

![image-20230509112033379](rust-new.assets/image-20230509112033379.png)



## 54.6 同步与异步

```
use std::sync::{mpsc};
use std::thread;
use std::time::Duration;

fn main() {
  //异步接收
  async_func();

  println!("");
  println!("-----------------");
  println!("");


  //同步接收
  sync_func();


  thread::sleep(Duration::from_secs(3));

   
}

fn async_func(){
  let (tx,rx) = mpsc::channel();

  let t1 = tx.clone();
  thread::spawn(move ||{

    println!("异步发送前");
    t1.send(1).unwrap();
    println!("异步发送后")

  });

  println!("异步接受前");
  println!("sleep-3s");
  thread::sleep(Duration::from_secs(3));
  println!("revc-{}",rx.recv().unwrap());
  println!("异步接受后");

}

fn sync_func(){
  let (tx,rx) = mpsc::sync_channel(0);

  thread::spawn(move ||{
    println!("同步发送前");
    tx.send(1).unwrap();
    println!("同步发送后");

  });

  println!("同步接收前");
  println!("sleep-3s");
  thread::sleep(Duration::from_secs(3));
  println!("revc-{}",rx.recv().unwrap());
  println!("同步接受后");


}

异步接受前
sleep-3s
异步发送前
异步发送后
revc-1
异步接受后

-----------------

同步接收前
sleep-3s
同步发送前
revc-1
同步接受后
同步发送后

```

![image-20230509143024554](rust-new.assets/image-20230509143024554.png)

![image-20230509142753032](rust-new.assets/image-20230509142753032.png)

## 54.7 [关闭通道](https://course.rs/advance/concurrency-with-threads/message-passing.html#关闭通道)

**所有发送者被`drop`或者所有接收者被`drop`后，通道会自动关闭**。



## 54.8 传递多样数据

```
use std::sync::{mpsc};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Person{
  name : String,
  age : i32,
}

fn main() {
  let (tx,rx) = mpsc::channel();

  let t1 = tx.clone();
  thread::spawn(move ||{


    t1.send(Person{
      name : "zhangSan".to_string(),
      age :28,
    }).unwrap();

  });
  drop(tx);

  println!("revc-{:?}",rx.recv().unwrap());

  match rx.recv() {
    Ok(v)=> println!("{:?}",v),
    Err(e)=>println!("err-{}",e),
      
  };

  thread::sleep(Duration::from_secs(3));

   
}

revc-Person { name: "zhangSan", age: 28 }
err-receiving on a closed channel
```



**enum**

```
use std::sync::mpsc::{self, Receiver, Sender};

enum Fruit {
    Apple(u8),
    Orange(String)
}

fn main() {
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("received {} oranges", flavor),
        }
    }
}

```



## 54.9 容易发生的坑

![image-20230509145343454](rust-new.assets/image-20230509145343454.png)

## 54.10 [mpmc 更好的性能](https://course.rs/advance/concurrency-with-threads/message-passing.html#mpmc-更好的性能)

如果你需要 mpmc(多发送者，多接收者)或者需要更高的性能，可以考虑第三方库:

- [**crossbeam-channel**](https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel), 老牌强库，功能较全，性能较强，之前是独立的库，但是后面合并到了`crossbeam`主仓库中
- [**flume**](https://github.com/zesterer/flume), 官方给出的性能数据某些场景要比 crossbeam 更好些

# 54 线程同步

## 54.1 锁 Mutex

`Mutex`让多个线程并发的访问同一个值变成了排队访问：同一时间，只允许一个线程`A`访问该值，其它线程需要等待`A`访问完成后才能继续。

![image-20230509161248401](rust-new.assets/image-20230509161248401.png)



## 54.2 死锁

![image-20230509161505251](rust-new.assets/image-20230509161505251.png)

```
use std::sync::Mutex;



fn main(){
    let mutx = Mutex::new(5);

    let mut num = mutx.lock().unwrap();

    *num = 6;

    drop(num);

    let mut num1 = mutx.lock().unwrap();
    *num1 = 7;
    println!("{}",num1);

}
7
```



## 54.3 多线程间使用mutex

```
use std::sync::{Arc, Mutex};
use std::thread;




fn main(){
    let mut handles = Vec::with_capacity(6);

    let mtx = Arc::new(Mutex::new(6));


    for i in 0..6{
        let lock = mtx.clone();
        handles.push(thread::spawn(move ||{
            let mut num = lock.lock().unwrap();
                *num = i
        }));
    }

    for i in handles{
        i.join().unwrap();
    }

    println!("{:?}",mtx)
}

Mutex { data: 4, poisoned: false, .. }
```

## 54.4 内部可变性

![image-20230509164736990](rust-new.assets/image-20230509164736990.png)



## 54.5 死锁

![image-20230509164831293](rust-new.assets/image-20230509164831293.png)

**多线程死锁**

当我们拥有两个锁，且两个线程各自使用了其中一个锁，然后试图去访问另一个锁时，就可能发生死锁：

```
use std::{sync::{Mutex, MutexGuard}, thread};
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;
lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

fn main() {
    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                // 线程1
                if i_thread % 2 == 0 {
                    // 锁住MUTEX1
                    let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                    // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                    sleep(Duration::from_millis(10));

                    // 去锁MUTEX2
                    let guard = MUTEX2.lock().unwrap();
                // 线程2
                } else {
                    // 锁住MUTEX2
                    let _guard = MUTEX2.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);

                    let _guard = MUTEX1.lock().unwrap();
                }
            }
        }));
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");
}

```

![image-20230509165028781](rust-new.assets/image-20230509165028781.png)



## 54.5 [try_lock](https://course.rs/advance/concurrency-with-threads/sync1.html#try_lock)

与`lock`方法不同，`try_lock`会**尝试**去获取一次锁，如果无法获取会返回一个错误，因此**不会发生阻塞**:

```
use std::sync::{Arc, Mutex};
use std::thread;




fn main(){
    let mut handles = Vec::with_capacity(6);

    let mtx = Arc::new(Mutex::new(6));


    for i in 0..6{
        let lock = mtx.clone();
        handles.push(thread::spawn(move ||{
            thread::sleep(std::time::Duration::from_secs(5));
            let mut num = lock.lock().unwrap();
                *num = i
        }));
    }

    //等待子线程执行
    thread::sleep(std::time::Duration::from_secs(1));

    let try_lock = mtx.try_lock().unwrap();
    println!("----{}",try_lock);
    drop(try_lock);

    for i in handles{
        i.join().unwrap();
    }

    println!("{:?}",mtx)
}
----6
Mutex { data: 2, poisoned: false, .. }
```



## 54.6[读写锁 RwLock](https://course.rs/advance/concurrency-with-threads/sync1.html#读写锁-rwlock)

`Mutex`会对每次读写都进行加锁，但某些时候，我们需要大量的并发读，`Mutex`就无法满足需求了，此时就可以使用`RwLock`:

```
use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);

    // 同一时间允许多个读
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 读锁在此处被drop

    // 同一时间只允许一个写
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);

        // 以下代码会panic，因为读和写不允许同时存在
        // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
        // let r1 = lock.read();
        // println!("{:?}",r1);
    }// 写锁在此处被drop
}

```

![image-20230509171010230](rust-new.assets/image-20230509171010230.png)



## 54.7 [三方库提供的锁实现](https://course.rs/advance/concurrency-with-threads/sync1.html#三方库提供的锁实现)

标准库在设计时总会存在取舍，因为往往性能并不是最好的，如果你追求性能，可以使用三方库提供的并发原语:

- [parking_lot](https://crates.io/crates/parking_lot), 功能更完善、稳定，社区较为活跃，star 较多，更新较为活跃
- [spin](https://crates.io/crates/spin), 在多数场景中性能比`parking_lot`高一点，最近没怎么更新

如果不是追求特别极致的性能，建议选择前者。



## 54.8 [用条件变量(Condvar)控制线程的同步](https://course.rs/advance/concurrency-with-threads/sync1.html#用条件变量condvar控制线程的同步)

`Mutex`用于解决资源安全访问的问题，但是我们还需要一个手段来解决资源访问顺序的问题。而 Rust 考虑到了这一点，为我们提供了条件变量(Condition Variables)，它经常和`Mutex`一起使用，可以让线程挂起，直到某个条件发生后再继续执行，其实`Condvar`我们在之前的多线程章节就已经见到过，现在再来看一个不同的例子：

```rust
use std::sync::{Arc,Mutex,Condvar};
use std::thread::{spawn,sleep};
use std::time::Duration;

fn main() {
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = spawn(move || {
        let mut m = { *cflag.lock().unwrap() };
        let mut counter = 0;

        while counter < 3 {
            while !m {
                m = *ccond.wait(cflag.lock().unwrap()).unwrap();
            }

            {
                m = false;
                *cflag.lock().unwrap() = false;
            }

            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }
    hdl.join().unwrap();
    println!("{:?}", flag);
}
```

例子中通过主线程来触发子线程实现交替打印输出：

```console
outside counter: 1
inner counter: 1
outside counter: 2
inner counter: 2
outside counter: 3
inner counter: 3
Mutex { data: true, poisoned: false, .. }
```

## 54.9 [信号量 Semaphore](https://course.rs/advance/concurrency-with-threads/sync1.html#信号量-semaphore)

![image-20230509171933624](rust-new.assets/image-20230509171933624.png)



# 55 [线程同步：Atomic 原子类型与内存顺序](https://course.rs/advance/concurrency-with-threads/sync2.html#线程同步atomic-原子类型与内存顺序)

## 55.1 Atomic

![image-20230509173411337](rust-new.assets/image-20230509173411337.png)



```
use std::sync::atomic::{AtomicU64,Ordering};
use std::thread::{self,JoinHandle};
use std::time::Instant;
use std::ops::Sub;



const N_TIMES : u64 = 100000;
const N_THREAD : usize = 10;

static R : AtomicU64 = AtomicU64::new(0);

fn add_time(n:u64)-> JoinHandle<()>{
    
        thread::spawn(move ||{
            for _ in 0..n{
                R.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        })
}

fn main(){
    let s  = Instant::now();
    println!("{:?}",s);

    let mut handles = Vec::with_capacity(N_THREAD);

    for _ in 0..N_THREAD{
        handles.push(add_time(N_TIMES));
    }

    for i in handles{
        i.join().unwrap();

    }

    assert_eq!(N_TIMES * N_THREAD as u64, R.load(Ordering::Relaxed));
    println!("{}-{}",N_TIMES * N_THREAD as u64, R.load(Ordering::Relaxed));
    

    //use std::ops::Sub; 才能用sub
    println!("{:?}",Instant::now().sub(s));

}

Instant { tv_sec: 8807, tv_nsec: 732274916 }
1000000-1000000
77.497209ms

```

还有一点值得注意: **和`Mutex`一样，`Atomic`的值具有内部可变性**，你无需将其声明为`mut`：

![image-20230509174856258](rust-new.assets/image-20230509174856258.png)



## 55.2 内存顺序

内存顺序是指 CPU 在访问内存时的顺序，该顺序可能受以下因素的影响：

- 代码中的先后顺序
- 编译器优化导致在编译阶段发生改变(内存重排序 reordering)
- 运行阶段因 CPU 的缓存机制导致顺序被打乱

**[限定内存顺序的 5 个规则](https://course.rs/advance/concurrency-with-threads/sync2.html#限定内存顺序的-5-个规则)**

在理解了内存顺序可能存在的改变后，你就可以明白为什么 Rust 提供了`Ordering::Relaxed`用于限定内存顺序了，事实上，该枚举有 5 个成员:

- **Relaxed**， 这是最宽松的规则，它对编译器和 CPU 不做任何限制，可以乱序
- **Release 释放**，设定内存屏障(Memory barrier)，保证它之前的操作永远在它之前，但是它后面的操作可能被重排到它前面
- **Acquire 获取**, 设定内存屏障，保证在它之后的访问永远在它之后，但是它之前的操作却有可能被重排到它后面，往往和`Release`在不同线程中联合使用
- **AcqRel**, 是 *Acquire* 和 *Release* 的结合，同时拥有它们俩提供的保证。比如你要对一个 `atomic` 自增 1，同时希望该操作之前和之后的读取或写入操作不会被重新排序
- **SeqCst 顺序一致性**， `SeqCst`就像是`AcqRel`的加强版，它不管原子操作是属于读取还是写入的操作，只要某个线程有用到`SeqCst`的原子操作，线程中该`SeqCst`操作前的数据操作绝对不会被重新排在该`SeqCst`操作之后，且该`SeqCst`操作后的数据操作也绝对不会被重新排在`SeqCst`操作前。

这些规则由于是系统提供的，因此其它语言提供的相应规则也大同小异，大家如果不明白可以看看其它语言的相关解释。

## 55.3 [多线程中使用 Atomic](https://course.rs/advance/concurrency-with-threads/sync2.html#多线程中使用-atomic)

```
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};

fn main() {
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move|| {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // 等待其它线程释放锁
    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}

```



# 56 [基于 Send 和 Sync 的线程安全](https://course.rs/advance/concurrency-with-threads/send-sync.html#基于-send-和-sync-的线程安全)

![image-20230509180822772](rust-new.assets/image-20230509180822772.png)



![image-20230509181001312](rust-new.assets/image-20230509181001312.png)



![image-20230509181056923](rust-new.assets/image-20230509181056923.png)



## 56.1 [为裸指针实现`Send`](https://course.rs/advance/concurrency-with-threads/send-sync.html#为裸指针实现send)

```
use std::thread;

#[derive(Debug)]
struct MyBox(*mut u8);
unsafe impl Send for MyBox {    }
fn main(){
    let x = 5;
    let my = MyBox(x as *mut u8);

    let handle = thread::spawn(move ||{
        println!("{:?}",my);
    });

    handle.join().unwrap();

}
MyBox(0x5)

```



![image-20230509181226742](rust-new.assets/image-20230509181226742.png)



此时，我们的指针已经可以欢快的在多线程间撒欢，以上代码很简单，但有一点需要注意：`Send`和`Sync`是`unsafe`特征，实现时需要用`unsafe`代码块包裹。



## 56.2 [为裸指针实现`Sync`](https://course.rs/advance/concurrency-with-threads/send-sync.html#为裸指针实现sync)

![image-20230509182319096](rust-new.assets/image-20230509182319096.png)

![image-20230509182525350](rust-new.assets/image-20230509182525350.png)

```
use std::{thread, sync::Arc};

#[derive(Debug)]
struct MyBox(*const u8);
unsafe impl Send for MyBox {    }
unsafe impl Sync for MyBox {    }
fn main(){
    let x = 5;
    let my = MyBox(x as *mut u8);
    let ac = Arc::new(my);

    let acs = ac.clone();
    let handle = thread::spawn(move ||{
        println!("{:?}",acs);
    });
    //`*mut u8` cannot be shared between threads safely
    //within `MyBox`, the trait `Sync` is not implemented for `*mut u8`
    //required for `Arc<MyBox>` to implement `Send`


    handle.join().unwrap();

}
```



# 57 裸指针

![image-20230509182117098](rust-new.assets/image-20230509182117098.png)

（1）解引用后的区别，*const T指针解引用对应&T，*mut T解引用对应&mut T；

（2）在安全的代码中，可以将*const T转换为*mut T，因为对于解引用之前，都是原始裸指针，原始裸指针之间转换时可以的；

（3）为什么Unique的实现中，使用*const T而不是*mut T，其原因如下：


# 58 全局变量

## 58.1**静态常量**

全局常量可以在程序任何一部分使用，当然，如果它是定义在某个模块中，你需要引入对应的模块才能使用。常量，顾名思义它是不可变的，很适合用作静态配置：



- 关键字是`const`而不是`let`
- 定义常量必须指明类型（如 i32）不能省略
- 定义常量时变量的命名规则一般是全部大写
- ==常量可以在任意作用域进行定义，其生命周期贯穿整个程序的生命周期。编译时编译器会尽可能`将其内联到代码中`，所以在不同地方对同一常量的引用并不能保证引用到相同的内存地址==
- 常量的赋值只能是常量表达式/数学表达式，也就是说必须是在编译期就能计算出的值，如果需要在运行时才能得出结果的值比如函数，则不能赋值给常量表达式
- 对于变量出现重复的定义(绑定)会发生变量遮盖，后面定义的变量会遮住前面定义的变量，常量则不允许出现重复的定义

```
const NUMBER : usize = usize::MAX /2;

fn main(){

    println!("{}",NUMBER);
}
```



## 58.2 静态变量

静态变量允许声明一个全局的变量，常用于全局数据统计，例如我们希望用一个变量来统计程序当前的总请求数：

```

const NUMBER : usize = usize::MAX /2;
static mut NUM :i32 = 0;

fn main(){

    println!("{}",NUMBER);
    let mut n = 0;
    unsafe{
        NUM += 1;
    println!("{}",NUM);
     n = NUM.clone();


    }
    println!("{}",n);
}
9223372036854775807
1
1
```

![image-20230509184427994](rust-new.assets/image-20230509184427994.png)

## 58.3 [原子类型](https://course.rs/advance/global-variable.html#原子类型)

想要全局计数器、状态控制等功能，又想要线程安全的实现，原子类型是非常好的办法。

```
use std::sync::atomic::AtomicI64;
static REQUEST_RECV : AtomicI64 = AtomicI64::new(0);

fn main(){

//    for _ in 0..=100{//101
   for _ in 0..100{//100
    REQUEST_RECV.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
   }

   println!("{:?}",REQUEST_RECV);
}

```



## 58.4 全局id生成器

```
use std::sync::atomic::{Ordering, AtomicUsize};


static GOABLE_NUM :AtomicUsize= AtomicUsize::new(0);
const MAX_ID: usize = usize::MAX/2;

#[derive(Debug)]
struct Factroy{
    factroy_id : usize,
}

impl Factroy {
    fn new()->Self{
        Self { factroy_id: generate_id() }
    }
}

fn generate_id()->usize{
    let current = GOABLE_NUM.load(Ordering::Relaxed);
    if current > MAX_ID{
        panic!("the current number is bigger MAX_ID")
    }
    GOABLE_NUM.fetch_add(1, Ordering::Relaxed);
    let goable_num = GOABLE_NUM.load(Ordering::Relaxed);
    if goable_num > MAX_ID {
        panic!("the current number is bigger MAX_ID")
    }
    println!("{}",goable_num);
    goable_num
}

fn main(){
    let factory = Factroy::new();
    println!("{:?}",factory);
    println!("{:?}",factory);
}
Factroy { factroy_id: 1 }
Factroy { factroy_id: 1 }
```

## 58.5 可变更的全局变量

[lazy_static](https://course.rs/advance/global-variable.html#lazy_static)

[`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs)是社区提供的非常强大的宏，用于懒初始化静态变量，之前的静态变量都是在编译期初始化的，因此无法使用函数调用进行赋值，而`lazy_static`允许我们在运行期初始化静态变量！

```
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static!{
    #[derive(Debug)]
    static ref CONF: Mutex<String> = Mutex::new(String::from("the global config "));
}


fn main(){
    let mut name = CONF.lock().unwrap();
    name.push_str("change hello");
    println!("{}",name);

    drop(name);

    println!("{:?}",CONF.lock());
}
    

```

当然，使用`lazy_static`在每次访问静态变量时，会有轻微的性能损失，因为其内部实现用了一个底层的并发原语`std::sync::Once`，在每次访问该变量时，程序都会执行一次原子指令用于确认静态变量的初始化是否完成。

`lazy_static`宏，匹配的是`static ref`，所以定义的静态变量都是不可变引用



**内存缓存的实现**

```
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref HASHMAPS : HashMap <&'static str,&'static str> = {
        let mut m = HashMap::new();
        m.insert("path", "./");
        m.insert("name", "name");
        m
    };
}

fn main(){
    println!("{:?}",HASHMAPS.get("path"));
    println!("{:?}",HASHMAPS.get("name"));
}
Some("./")
Some("name")
```

需要注意的是，`lazy_static`直到运行到`main`中的第一行代码时，才进行初始化，非常`lazy static`。



## 58.6 [Box::leak](https://course.rs/advance/global-variable.html#boxleak) 全局变量

在`Box`智能指针章节中，我们提到了`Box::leak`可以用于全局变量

好在`Rust`为我们提供了`Box::leak`方法，它可以将一个变量从内存中泄漏(听上去怪怪的，竟然做主动内存泄漏)，然后将其变为`'static`生命周期，最终该变量将和程序活得一样久，因此可以赋值给全局静态变量`CONFIG`。



```
#[derive(Debug)]
struct Config  {
     a  : String,
    b :String,
}

static   mut  CONF : Option<&mut Config>  = None;


fn main(){

    let  c = Box::new(Config{
        a : "a".to_string(),
        b : "b".to_string(),
    });
    unsafe{
        CONF = Some(Box::leak(c));
        println!("{:?}",CONF);

    }


}

Some(Config { a: "a", b: "b" })
```

## 58.7 从函数返回全局变量

```
#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut CONFIG: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    Some(Box::leak(c))
}


fn main() {
    unsafe {
        CONFIG = init();

        println!("{:?}", CONFIG)
    }
}

```



## 58.8 [标准库中的 OnceCell](https://course.rs/advance/global-variable.html#标准库中的-oncecell)

在 `Rust` 标准库中提供 `lazy::OnceCell` 和 `lazy::SyncOnceCell` 两种 `Cell`，前者用于单线程，后者用于多线程，它们用来存储堆上的信息，并且具有最多只能赋值一次的特性。 如实现一个多线程的日志组件 `Logger`：

```
#![feature(once_cell)]

use std::{lazy::SyncOnceCell, thread};

fn main() {
    // 子线程中调用
    let handle = thread::spawn(|| {
        let logger = Logger::global();
        logger.log("thread message".to_string());
    });

    // 主线程调用
    let logger = Logger::global();
    logger.log("some message".to_string());

    let logger2 = Logger::global();
    logger2.log("other message".to_string());

    handle.join().unwrap();
}

#[derive(Debug)]
struct Logger;

static LOGGER: SyncOnceCell<Logger> = SyncOnceCell::new();

impl Logger {
    fn global() -> &'static Logger {
        // 获取或初始化 Logger
        LOGGER.get_or_init(|| {
            println!("Logger is being created..."); // 初始化打印
            Logger
        })
    }

    fn log(&self, message: String) {
        println!("{}", message)
    }
}

```

以上代码我们声明了一个 `global()` 关联函数，并在其内部调用 `get_or_init` 进行初始化 `Logger`，之后在不同线程上多次调用 `Logger::global()` 获取其实例：

```console
Logger is being created...
some message
other message
thread message
```

可以看到，`Logger is being created...` 在多个线程中使用也只被打印了一次。

特别注意，目前 `OnceCell` 和 `SyncOnceCell` API 暂未稳定，需启用特性 `#![feature(once_cell)]`。

# 59 Rust 中的 & 和 ref

1. & 和 ref 都是用来定义指针的
   废话少说，先看代码：



    fn main() {
        let mut a: i32 = 111;
        let b = &a;
        println!("{}", *b); //111
    
        let ref c = a;
        println!("{}", *c); //111
    }

而这结果一样，都是在声明一个指针。区别在那里？& 放在等号的右边， ref 放在等好的左边。

在看个例子，看看如何用指针修改变量：

    fn main() {
        let mut a: i32 = 111;
        let b = &mut a; // 指针 b 本身是不可以修改的，但它指向的内容是可以修改的。
        *b = 222;
        println!("{}", a); // 222
    
        let ref mut c = a; // 指针 c 本身是不可以修改的，但它指向的内容是可以修改的。
        *c = 333;
        println!("{}", a); //333
    }



在代码行里，二者没有任何区别。但是，为什么弄出两个来呢？

2. 只能用 & 定义指针的地方
   看一段代码：

```
fn foo(x: &mut i32) {
    *x = 999;
}
fn main() {
    let mut a: i32 = 111;
    foo(&mut a);
    println!("{}", a); // 999
}
```



在函数传参的时候用到了 & 来表示传入参数栈的是一个可修改变量的地址。

下面我们修改一下，改成下面的代码：

```
fn foo(ref mut x: i32) {
    *x = 999;
}
fn main() {
    let mut a: i32 = 111;
    foo(a);
    println!("{}", a); // 111
}
```

foo(a) 的调用语义是说，要把 a 的值复制到栈空间，因此，fn foo(ref mut x: i32) 中参数 x 引用的是栈上的数据。也就是说，不管函数的参数 x 如何声明，foo(a) 这种形式传入参数 a，都不可能修改变量 a 的值。我觉得这个规定是合理的，比 C++ 的引用的语义声明简单、合理多了。

我们再修改一下：

```
fn foo(ref x: &mut i32) {
    **x = 999;
}
fn main() {
    let mut a: i32 = 111;
    foo(&mut a);
    println!("{}", a); // 999
}
```

这次又成功了。但是这个 **x 你不觉得麻烦吧？因此，在函数参数声明中，一般只用 & 来传入变量的地址。

3. 只能用 ref 定义指针的地方
   看下面的代码：

```
fn main() {
    let s = Some(String::from("Hello!"));
    match s {
        Some(t) => println!("t = {}", t),
        _ => {}
    }
    println!("s = {}", s.unwrap());
}


```



这个是无法编译的。因为 match s 语句中，已经把 s 的所有权给转移了，导致最后一条语句无法执行。编译提示如下：

编译期建议在模式匹配中把变量 t 改成 ref t，也就是说把 t 声明成指针即可解决问题。修改后代码如下：

```
fn main() {
    let s = Some(String::from("Hello!"));
    match s {
        Some(ref t) => println!("t = {}", t),
        _ => {}
    }
    println!("s = {}", s.unwrap());
}
```

因为在模式匹配代码块中，我们没有机会声明变量类型，只能用 ref 表示变量 t 是个指针。

我试了一下，不用 ref 的话，还有一个变通的方法，就是把 match s 改成 match &s。代码如下：

```
fn main() {
    let s = Some(String::from("Hello!"));
    match &s {
        Some(t) => println!("t = {}", t),
        _ => {}
    }
    println!("s = {}", s.unwrap());
}


```



这个时候 t 前面加不加 ref 结果都一样。因为 match 只是借用 s，所以不会影响 s 的生命周期。

4. 更多的试验
   下面给出了一组代码，我们看看那些是合法的，那些是非法的。



    fn main() {
        let v = 123;
        let x: &i32 = &v; 					// OK!
        let x: &i32 = &(123 + 456); 		// OK!
        if let Some(x:&i32) = Some(&123); 	// Error！
    
        let ref x: i32 = v;					// OK!
        let ref x: i32 = 123 + 456;			// OK!
        if let Some(ref x) = Some(123) {}	// OK!
    }

5. 指针变量的解引用
   看下面代码，道理我不多讲了，rust 会自动解多层嵌套引用，这个太方便了。

```
fn main() {
    let a: &i32 = &123;
    let b: &&i32 = &a;
    let c: &&&i32 = &b;
    println!("a = {}, b = {}, c = {}", a, b, c);
	println!("*a = {}, **b = {}, ***c = {}", *a, **b, ***c);
}

/* output
a = 123, b = 123, c = 123
*a = 123, **b = 123, ***c = 123
*/
```


# 60 错误处理



## 60.1 or and

跟布尔关系的与/或很像，这两个方法会对两个表达式做逻辑组合，最终返回 `Option` / `Result`。

- `or()`，表达式按照顺序求值，若任何一个表达式的结果是 `Some` 或 `Ok`，则该值会立刻返回
- `and()`，若两个表达式的结果都是 `Some` 或 `Ok`，则**第二个表达式中的值被返回**。若任何一个的结果是 `None` 或 `Err` ，则立刻返回。

实际上，只要将布尔表达式的 `true` / `false`，替换成 `Some` / `None` 或 `Ok` / `Err` 就很好理解了。

```
fn main() {
  let s1 = Some("some1");
  let s2 = Some("some2");
  let n: Option<&str> = None;

  let o1: Result<&str, &str> = Ok("ok1");
  let o2: Result<&str, &str> = Ok("ok2");
  let e1: Result<&str, &str> = Err("error1");
  let e2: Result<&str, &str> = Err("error2");

  assert_eq!(s1.or(s2), s1); // Some1 or Some2 = Some1
  assert_eq!(s1.or(n), s1);  // Some or None = Some
  assert_eq!(n.or(s1), s1);  // None or Some = Some
  assert_eq!(n.or(n), n);    // None1 or None2 = None2

  assert_eq!(o1.or(o2), o1); // Ok1 or Ok2 = Ok1
  assert_eq!(o1.or(e1), o1); // Ok or Err = Ok
  assert_eq!(e1.or(o1), o1); // Err or Ok = Ok
  assert_eq!(e1.or(e2), e2); // Err1 or Err2 = Err2

  assert_eq!(s1.and(s2), s2); // Some1 and Some2 = Some2
  assert_eq!(s1.and(n), n);   // Some and None = None
  assert_eq!(n.and(s1), n);   // None and Some = None
  assert_eq!(n.and(n), n);    // None1 and None2 = None1

  assert_eq!(o1.and(o2), o2); // Ok1 and Ok2 = Ok2
  assert_eq!(o1.and(e1), e1); // Ok and Err = Err
  assert_eq!(e1.and(o1), e1); // Err and Ok = Err
  assert_eq!(e1.and(e2), e1); // Err1 and Err2 = Err1
}
```

除了 `or` 和 `and` 之外，Rust 还为我们提供了 `xor` ，但是它只能应用在 `Option` 上，其实想想也是这个理，如果能应用在 `Result` 上，那你又该如何对一个值和错误进行异或操作？

## 60.2 [or_else() 和 and_then()](https://course.rs/advance/errors.html#or_else-和-and_then)

它们跟 `or()` 和 `and()` 类似，唯一的区别在于，它们的第二个表达式是一个闭包。

```
fn main() {
    // or_else with Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = || Some("some2"); // 类似于: let fn_some = || -> Option<&str> { Some("some2") };

    let n: Option<&str> = None;
    let fn_none = || None;

    assert_eq!(s1.or_else(fn_some), s1);  // Some1 or_else Some2 = Some1
    assert_eq!(s1.or_else(fn_none), s1);  // Some or_else None = Some
    assert_eq!(n.or_else(fn_some), s2);   // None or_else Some = Some
    assert_eq!(n.or_else(fn_none), None); // None1 or_else None2 = None2

    // or_else with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.or_else(fn_ok), o1);  // Ok1 or_else Ok2 = Ok1
    assert_eq!(o1.or_else(fn_err), o1); // Ok or_else Err = Ok
    assert_eq!(e1.or_else(fn_ok), o2);  // Err or_else Ok = Ok
    assert_eq!(e1.or_else(fn_err), e2); // Err1 or_else Err2 = Err2
}

```



```
fn main() {
    // and_then with Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = |_| Some("some2"); // 类似于: let fn_some = |_| -> Option<&str> { Some("some2") };

    let n: Option<&str> = None;
    let fn_none = |_| None;

    assert_eq!(s1.and_then(fn_some), s2); // Some1 and_then Some2 = Some2
    assert_eq!(s1.and_then(fn_none), n);  // Some and_then None = None
    assert_eq!(n.and_then(fn_some), n);   // None and_then Some = None
    assert_eq!(n.and_then(fn_none), n);   // None1 and_then None2 = None1

    // and_then with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.and_then(fn_ok), o2);  // Ok1 and_then Ok2 = Ok2
    assert_eq!(o1.and_then(fn_err), e2); // Ok and_then Err = Err
    assert_eq!(e1.and_then(fn_ok), e1);  // Err and_then Ok = Err
    assert_eq!(e1.and_then(fn_err), e1); // Err1 and_then Err2 = Err1
}
```

## 60.3 [filter](https://course.rs/advance/errors.html#filter)

`filter` 用于对 `Option` 进行过滤：

```
fn main() {
    let s1 = Some(3);
    let s2 = Some(6);
    let n = None;

    let fn_is_even = |x: &i8| x % 2 == 0;

    assert_eq!(s1.filter(fn_is_even), n);  // Some(3) -> 3 is not even -> None
    assert_eq!(s2.filter(fn_is_even), s2); // Some(6) -> 6 is even -> Some(6)
    assert_eq!(n.filter(fn_is_even), n);   // None -> no value -> None
}

```



## 60.4 [map() 和 map_err()](https://course.rs/advance/errors.html#map-和-map_err)

==Map 只有在Some。Ok的情况下才会进行下一步计算，否侧返回原值，不进行处理==

```
fn main() {
    let s1 = Some("abcde");
    let s2 = Some(5);

    let n1: Option<&str> = None;
    let n2: Option<usize> = None;

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<usize, &str> = Ok(5);

    let e1: Result<&str, &str> = Err("abcde");
    let e2: Result<usize, &str> = Err("abcde");

    let fn_character_count = |s: &str| s.chars().count();


    assert_eq!(s1.map(fn_character_count), s2); // Some1 map = Some2
    assert_eq!(n1.map(fn_character_count), n2); // None1 map = None2
    println!("{:?}",s1.map(fn_character_count)); //Some(5)
    println!("{:?}",n1.map(fn_character_count)); //None


    assert_eq!(o1.map(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map(fn_character_count), e2); // Err1 map = Err2

    println!("{:?}",o1.map(fn_character_count)); //Ok(5)
    println!("{:?}",e1.map(fn_character_count)); //Err("abced")

}

```



==但是如果你想要将 `Err` 中的值进行改变， `map` 就无能为力了，此时我们需要用 `map_err`：==

```rust
fn main() {
    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize

    assert_eq!(o1.map_err(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map_err(fn_character_count), e2); // Err1 map = Err2
}
```

==通过对 `o1` 的操作可以看出，与 `map` 面对 `Err` 时的短小类似， `map_err` 面对 `Ok` 时也是相当无力的。==

## 60.5 [map_or() 和 map_or_else()](https://course.rs/advance/errors.html#map_or-和-map_or_else)

`map_or` 在 `map` 的基础上提供了一个默认值:

```rust
fn main() {
    const V_DEFAULT: u32 = 1;

    let s: Result<u32, ()> = Ok(10);
    let n: Option<u32> = None;
    let fn_closure = |v: u32| v + 2;

    assert_eq!(s.map_or(V_DEFAULT, fn_closure), 12);
    assert_eq!(n.map_or(V_DEFAULT, fn_closure), V_DEFAULT);
}
```

如上所示，当处理 `None` 的时候，`V_DEFAULT` 作为默认值被直接返回。

`map_or_else` 与 `map_or` 类似，但是它是通过一个闭包来提供默认值:

```rust
fn main() {
    let s = Some(10);
    let n: Option<i8> = None;

    let fn_closure = |v: i8| v + 2;
    let fn_default = || 1;

    assert_eq!(s.map_or_else(fn_default, fn_closure), 12);
    assert_eq!(n.map_or_else(fn_default, fn_closure), 1);

    let o = Ok(10);
    let e = Err(5);
    let fn_default_for_result = |v: i8| v + 1; // 闭包可以对 Err 中的值进行处理，并返回一个新值

    assert_eq!(o.map_or_else(fn_default_for_result, fn_closure), 12);
    assert_eq!(e.map_or_else(fn_default_for_result, fn_closure), 6);
}
```



## 60.6 [ok_or() and ok_or_else()](https://course.rs/advance/errors.html#ok_or-and-ok_or_else)

这两兄弟可以将 `Option` 类型转换为 `Result` 类型。其中 `ok_or` 接收一个默认的 `Err` 参数:

```rust
fn main() {
    const ERR_DEFAULT: &str = "error message";

    let s = Some("abcde");
    let n: Option<&str> = None;

    let o: Result<&str, &str> = Ok("abcde");
    let e: Result<&str, &str> = Err(ERR_DEFAULT);

    assert_eq!(s.ok_or(ERR_DEFAULT), o); // Some(T) -> Ok(T)
    assert_eq!(n.ok_or(ERR_DEFAULT), e); // None -> Err(default)
}
```

而 `ok_or_else` 接收一个闭包作为 `Err` 参数:

```rust
fn main() {
    let s = Some("abcde");
    let n: Option<&str> = None;
    let fn_err_message = || "error message";

    let o: Result<&str, &str> = Ok("abcde");
    let e: Result<&str, &str> = Err("error message");

    assert_eq!(s.ok_or_else(fn_err_message), o); // Some(T) -> Ok(T)
    assert_eq!(n.ok_or_else(fn_err_message), e); // None -> Err(default)
}
```

以上列出的只是常用的一部分，强烈建议大家看看标准库中有哪些可用的 API，在实际项目中，这些 API 将会非常有用: [Option](https://doc.rust-lang.org/stable/std/option/enum.Option.html) 和 [Result](https://doc.rust-lang.org/stable/std/result/enum.Result.html)。

# 61 自定义错误

```
use core::fmt;

struct AppErr{
    name : String,
}

impl fmt::Display for AppErr {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("self---{}",self.name);
        write!(f,"error")
    }
}

fn make_error()->Result<(), AppErr>{
    Err(AppErr{name:String::from("err")})
}

fn main(){
    match make_error(){
        Ok(v) => println!("ok -{:?}",v),
        Err(e)=> println!("e-{}",e)
    }

}

e-self---err
error
```



```
use std::fmt;

// AppError 是自定义错误类型，它可以是当前包中定义的任何类型，在这里为了简化，我们使用了单元结构体作为例子。
// 为 AppError 自动派生 Debug 特征
#[derive(Debug)]
struct AppError;

// 为 AppError 实现 std::fmt::Display 特征
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

// 一个示例函数用于产生 AppError 错误
fn produce_error() -> Result<(), AppError> {
    Err(AppError)
}

fn main(){
    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("No error"),
    }

    eprintln!("{:?}", produce_error()); // Err({ file: src/main.rs, line: 17 })
}

```

![image-20230510173639910](rust-new.assets/image-20230510173639910.png)





```
use std::fmt;

struct AppError {
    code: usize,
    message: String,
}

// 根据错误码显示不同的错误信息
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppError {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}

fn produce_error() -> Result<(), AppError> {
    Err(AppError {
        code: 404,
        message: String::from("Page not found"),
    })
}

fn main() {
    match produce_error() {
        Err(e) => eprintln!("{}", e), // 抱歉，未找到指定的页面!
        _ => println!("No error"),
    }

    eprintln!("{:?}", produce_error()); // Err(AppError { code: 404, message: Page not found })

    eprintln!("{:#?}", produce_error());
    // Err(
    //     AppError { code: 404, message: Page not found }
    // )
}

```

## 61.1 [错误转换 `From` 特征](https://course.rs/advance/errors.html#错误转换-from-特征)

![image-20230510180613850](rust-new.assets/image-20230510180613850.png)



```
use std::{io, fs::File};

#[derive(Debug)]
struct AppError{
    kind : String,
    msg : String,
}

impl From<io::Error> for AppError {
    // 为 AppError 实现 std::convert::From 特征，由于 From 包含在 std::prelude 中，因此可以直接简化引入。
    // 实现 From<io::Error> 意味着我们可以将 io::Error 错误转换成自定义的 AppError 错误
    fn from(value: io::Error) -> Self {
        AppError { kind: "io-error".to_string(), msg: value.to_string() }
    }
}

fn main() -> Result<(),AppError>{
    let _file = File::open("nonexistent_file.txt")?;

    Ok(())
}
Error: AppError { kind: "io-error", msg: "No such file or directory (os error 2)" }
```



## 61.2 [归一化不同的错误类型](https://course.rs/advance/errors.html#归一化不同的错误类型)

![image-20230511172054676](rust-new.assets/image-20230511172054676.png)



## 61.2 Box \<dyn Error> 返回不同错误

```
use std::fs::read_to_string;
use std::error::Error;


fn render() -> Result<String,Box<dyn Error>>{
    let file = std::env::var("MAEKDOWN")?;

    let str = read_to_string(file)?;

    Ok(str)
}

fn main()-> Result<(),Box<dyn Error>>{
   let html = render()?;
   println!("{}",html);
   Ok(())
}
```

==但是有一个问题：`Result` 实际上不会限制错误的类型，也就是一个类型就算不实现 `Error` 特征，它依然可以在 `Result<T, E>` 中作为 `E` 来使用，此时这种特征对象的解决方案就无能为力了。==



## 61.3 [自定义错误类型](https://course.rs/advance/errors.html#自定义错误类型-1)

```rust
use std::fs::read_to_string;

fn main() -> Result<(), MyError> {
  let html = render()?;
  println!("{}", html);
  Ok(())
}

fn render() -> Result<String, MyError> {
  let file = std::env::var("MARKDOWN")?;
  let source = read_to_string(file)?;
  Ok(source)
}

#[derive(Debug)]
enum MyError {
  EnvironmentVariableNotFound,
  IOError(std::io::Error),
}

impl From<std::env::VarError> for MyError {
  fn from(_: std::env::VarError) -> Self {
    Self::EnvironmentVariableNotFound
  }
}

impl From<std::io::Error> for MyError {
  fn from(value: std::io::Error) -> Self {
    Self::IOError(value)
  }
}

impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      MyError::EnvironmentVariableNotFound => write!(f, "Environment variable not found"),
      MyError::IOError(err) => write!(f, "IO Error: {}", err.to_string()),
    }
  }
}

```

![image-20230511172946144](rust-new.assets/image-20230511172946144.png)



## 61.4 [简化错误处理](https://course.rs/advance/errors.html#简化错误处理)

### [thiserror](https://course.rs/advance/errors.html#thiserror)

[`thiserror`](https://github.com/dtolnay/thiserror)可以帮助我们简化上面的第二种解决方案：

```rust
use std::fs::read_to_string;

fn main() -> Result<(), MyError> {
  let html = render()?;
  println!("{}", html);
  Ok(())
}

fn render() -> Result<String, MyError> {
  let file = std::env::var("MARKDOWN")?;
  let source = read_to_string(file)?;
  Ok(source)
}

#[derive(thiserror::Error, Debug)]
enum MyError {
  #[error("Environment variable not found")]
  EnvironmentVariableNotFound(#[from] std::env::VarError),
  #[error(transparent)]
  IOError(#[from] std::io::Error),
}
```

如上所示，只要简单写写注释，就可以实现错误处理了，惊不惊喜？

### [error-chain](https://course.rs/advance/errors.html#error-chain)

[`error-chain`](https://github.com/rust-lang-deprecated/error-chain) 也是简单好用的库，可惜不再维护了，但是我觉得它依然可以在合适的地方大放光彩，值得大家去了解下。

```rust
use std::fs::read_to_string;

error_chain::error_chain! {
  foreign_links {
    EnvironmentVariableNotFound(::std::env::VarError);
    IOError(::std::io::Error);
  }
}

fn main() -> Result<()> {
  let html = render()?;
  println!("{}", html);
  Ok(())
}

fn render() -> Result<String> {
  let file = std::env::var("MARKDOWN")?;
  let source = read_to_string(file)?;
  Ok(source)
}
```

喏，简单吧？使用 `error-chain` 的宏你可以获得：`Error` 结构体，错误类型 `ErrorKind` 枚举 以及一个自定义的 `Result` 类型。

### [anyhow](https://course.rs/advance/errors.html#anyhow)

[`anyhow`](https://github.com/dtolnay/anyhow) 和 `thiserror` 是同一个作者开发的，这里是作者关于 `anyhow` 和 `thiserror` 的原话：

> 如果你想要设计自己的错误类型，同时给调用者提供具体的信息时，就使用 `thiserror`，例如当你在开发一个三方库代码时。如果你只想要简单，就使用 `anyhow`，例如在自己的应用服务中。

```rust
use std::fs::read_to_string;

use anyhow::Result;

fn main() -> Result<()> {
    let html = render()?;
    println!("{}", html);
    Ok(())
}

fn render() -> Result<String> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}
```

关于如何选用 `thiserror` 和 `anyhow` 只需要遵循一个原则即可：**是否关注自定义错误消息**，关注则使用 `thiserror`（常见业务代码），否则使用 `anyhow`（编写第三方库代码）。


# 62 unsafe

![image-20230511173856177](rust-new.assets/image-20230511173856177.png)

## 62.1 [解引用裸指针](https://course.rs/advance/unsafe/superpowers.html#解引用裸指针)

==`*const T` 和 `*mut T`，它们分别代表了不可变和可变。==

![image-20230511175337546](rust-new.assets/image-20230511175337546.png)



![image-20230511175421450](rust-new.assets/image-20230511175421450.png)



**创建裸指针**

```
fn main(){

    let mut five = 5;

    let con_prt = five as *const i32;//不可变
    let mut_prt = five as *mut i32;//可变
}
```

![image-20230511175818274](rust-new.assets/image-20230511175818274.png)



## 62.2 使用*解引用

![image-20230511180051758](rust-new.assets/image-20230511180051758.png)



## 62.3 [基于智能指针创建裸指针](https://course.rs/advance/unsafe/superpowers.html#基于智能指针创建裸指针)

![image-20230511180143661](rust-new.assets/image-20230511180143661.png)



## 62.4 [调用 unsafe 函数或方法](https://course.rs/advance/unsafe/superpowers.html#调用-unsafe-函数或方法)

![image-20230511180549061](rust-new.assets/image-20230511180549061.png)



```
use std::slice;



fn splait(arr   : &mut [i32],length : usize)-> (&mut [i32],&mut [i32]){
    let len = arr.len();
    let prt = arr.as_mut_ptr();

    unsafe{
        (
            slice::from_raw_parts_mut(prt, length),
            slice::from_raw_parts_mut(prt.add(length), len-length)
        )
    }
}

fn main(){

    let mut arr =vec![1,2,3,4,5,6];
    

    //expected &mut [i32], found &Vec<i32, Global>
    let  r = &mut arr[..];

    let (first,sencond) = splait(r, 3);

    assert_eq!(first,[1,2,3]);
    assert_eq!(sencond,[4,5,6]);

}
```

![image-20230511181948083](rust-new.assets/image-20230511181948083.png)

## 62.5 调用C代码

![image-20230511182145215](rust-new.assets/image-20230511182145215.png)

![image-20230511182528066](rust-new.assets/image-20230511182528066.png)



## 62.6 [实现 unsafe 特征](https://course.rs/advance/unsafe/superpowers.html#实现-unsafe-特征)

![image-20230511182848535](rust-new.assets/image-20230511182848535.png)



# 63 [Macro 宏编程](https://course.rs/advance/macro.html#macro-宏编程)

![image-20230511183309800](rust-new.assets/image-20230511183309800.png)



## 63.1 [声明式宏 `macro_rules!`](https://course.rs/advance/macro.html#声明式宏-macro_rules)
![image-20230512093656296](rust-new.assets/image-20230512093656296.png)



```rust
#[macro_export]
macro_rules! slice  {
   ($($x:expr),*) => {
        {
            let mut slice = Vec::new();
            $(
                slice.push($x);
                
            )*
            slice
        }
    };
}
```





- ==`#[macro_export]` 注释将宏进行了导出，这样其它的包就可以将该宏引入到当前作用域中==，然后才能使用。可能有同学会提问：我们在使用标准库 `vec!` 时也没有引入宏啊，那是因为 Rust 已经通过 [`std::prelude`](https://course.rs/appendix/prelude.html) 的方式为我们自动引入了。
- ==`macro_rules!` 进行了宏定义==，需要注意的是宏的名称是 `vec`，而不是 `vec!`，后者的感叹号只在调用时才需要。



虽然宏和 `match` 都称之为模式，但是前者跟[后者](https://course.rs/basic/match-pattern/all-patterns.html)的模式规则是不同的。如果大家想要更深入的了解宏的模式，可以查看[这里](https://doc.rust-lang.org/reference/macros-by-example.html)。

`( $( $x:expr ),* )` 的含义

- 我们使用圆括号 `()` 将整个宏模式包裹其中。紧随其后的是 `$()`，==跟括号中模式相匹配的值(传入的 Rust 源代码)会被捕获==，然后用于代码替换。在这里，模式 `$x:expr` 会匹配任何 Rust 表达式并给予该模式一个名称：`$x`。
- `$()` 之后的逗号说明在 `$()` 所匹配的代码的后面会有一个==可选的逗号分隔符，紧随逗号之后的 `*` 说明 `*` 之前的模式会被匹配零次或任意多次(类似正则表达式)。==

![image-20230512095207165](rust-new.assets/image-20230512095207165.png)



![image-20230512095306665](rust-new.assets/image-20230512095306665.png)



## 63.2 [用过程宏为属性标记生成代码](https://course.rs/advance/macro.html#用过程宏为属性标记生成代码)

第二种常用的宏就是[*过程宏*](https://doc.rust-lang.org/reference/procedural-macros.html) ( *procedural macros* )，从形式上来看，过程宏跟函数较为相像，但过程宏是使用源代码作为输入参数，基于代码进行一系列操作后，再输出一段全新的代码。**注意，过程宏中的 derive 宏输出的代码并不会替换之前的代码，这一点与声明宏有很大的不同！**

至于前文提到的过程宏的三种类型(自定义 `derive`、属性宏、函数宏)，它们的工作方式都是类似的。

当**创建过程宏**时，它的定义必须要放入一个==独立的包==中，且包的类型也是特殊的，这么做的原因相当复杂，大家只要知道这种限制在未来可能会有所改变即可。

![image-20230512095626514](rust-new.assets/image-20230512095626514.png)

































































































































































































