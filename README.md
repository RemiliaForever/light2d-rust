用C语言画光 rust学习
===
原版见[miloyip/light2d](https://github.com/miloyip/light2d)

## 说明
以下运行结果基于
* CPU : AMD R7 1700
* 内存 : 2400MHz 16G * 2
* OS : ArchLinux
* Rust: Rustc Nightly

release编译，通过rayon启用多线程渲染，1024x1024分辨率，仅统计渲染所花时间。

## chapter 1
运行时间:

|算法|N=16|N=64|N=256|N=1024|
|:-|:-:|:-:|:-:|:-:|
|**uniform sampling**|0.145946|0.509972|1.965942|7.904334|
|**stratified sampling**|0.108399|0.361274|1.405259|5.554561|
|**jittered sampling**|0.138914|0.477013|1.861866|7.377027|

运行效果：
![chapter1.png](https://github.com/RemiliaForever/light2d-rust/raw/master/img/chapter1.png)

## chapter 2
![chapter2.png](https://github.com/RemiliaForever/light2d-rust/raw/master/img/chapter2.png)
