#  碎碎念

# 0806

> 进程和文件是一定要隔离

**访问隔离** Namespace又称为命名空间（也可翻译为名字空间），它主要做访问隔离。其原理是针对一类资源进行抽象，并将其封装在一起提供给一个容器使用，对于这类资源，因为每个容器都有自己的抽象，而它们彼此之间是不可见的，所以就可以做到访问隔离。

资源抽象并封装

**资源控制**  Cgroup是control group的简称，又称为控制组，它主要是做资源控制。其原理是将一组进程放在一个控制组里，通过给这个控制组分配指定的可用资源，达到控制这一组进程可用资源的目的。    



今天大概看了下 我感觉cgroup和job应该是一个东西

## 几个链接

[namespaces链接](https://fuchsia.dev/fuchsia-src/concepts/process/namespaces)

[job链接](https://fuchsia.dev/fuchsia-src/reference/kernel_objects/job)



# 0808

昨天休息了

早上起来再看[Fuchsia简介](https://xuzhongxing.github.io/201806fuchsia.pdf)

几个点

1.原生进程沙箱

2.文件和文件系统成为一个局部概念，namespace来定义一个进程能够访问的资源

3.User从而在进程里没有uid

Linux: namespace, control group, unionfs => docker



Capability-based access control 

• 能访问的资源是父进程赋予的namespace 

• 看不到初始namespace之外的任何资源

感觉zircon已经有了系统级的支持，应该对于实现会容易很多。



