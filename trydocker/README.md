# trydocker

GO语言的一些尝试,正考虑移植到RUST
# Go版本
## 环境
go version go1.16.7 linux/amd64
Ubuntu 21.04

Go语言代码[参考](https://www.katacoda.com/lizrice/courses/containers-and-go/scratch)
## 运行命令
```
go run main.go run sh
```
## 效果
![image-20210825080456088](README.assets/image-20210825080456088.png)

## 关键技术

### namespace

syscall.CLONE_NEWUTS  主机名称隔离

syscall.CLONE_NEWPID   进程隔离

syscall.CLONE_NEWNS    挂载点隔离

### chroot                                
文件隔离