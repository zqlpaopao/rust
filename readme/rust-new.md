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





