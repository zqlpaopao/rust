fn main() {
let  a = 5;
a = a +1;
println!("{}",a);


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