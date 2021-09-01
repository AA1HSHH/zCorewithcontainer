

## 做了什么

对容器技术进行了初步的探索

### 什么是容器

容器技术可以对应用及其整个运行时环境（包括全部所需文件）一起进行打包或隔离。可以在开发、测试和生产等环境之间便捷迁移。

### 虚拟化  vs  容器

![virtualization-vs-containers](summary.assets/virtualization-vs-containers.png)

虚拟化技术可以运行不同的操作系统。其使用虚拟机监控程序模拟硬件，开销比容器大。

容器与宿主机共享相同的操作系统

## 结果

Ubuntu上Go语言初步实现了文件隔离，进程隔离

[代码](https://github.com/AA1HSHH/zCorewithcontainer/tree/main/trydocker)

## 体会

对操作系统有实践层面的理解

尽早动手写代码，少纸上谈兵

## 想法

1 Go到Rust

2 Ubuntu到zCore（linux ，zircon）



Linux 内核   namespace  cgroup

Zircon 内核   进程隔离jobs   

