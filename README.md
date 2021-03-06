# zCorewithcontainer

[代码](./trydocker/)

# 现状
Go/Rust已经可以在Ubuntu上运行，可实现文件&&进程隔离

TODO:
- [x] Go移植到Rust
- [x] 文档&&总结
- [x] 更新图片
- [ ] 网络隔离

# 规划
## 路线
1.从linux走，实现cgroups以及namespaces

2.从Zircon走，实现jobs扩展

## 初步计划
1.先加入zCore 文档与测试 小组入门zcore

2.GO ----> RUST

考虑到容器知识储备不足
参考从**linux**走的 [自己动手写Docker/陈显鹭，王炳燊，秦妤嘉著.—北京：电子工业出版社，2017.7](https://github.com/xianlubird/mydocker)做一些尝试

3.Ubuntu ---> zCore



# 打卡

|日期|做了什么|
|--|--|
|0806|1.阅读[自己动手写Docker](https://weread.qq.com/web/reader/a8932240721e42b5a89f479kc81322c012c81e728d9d180)和[Docker进阶与实战](https://weread.qq.com/web/reader/89c324e05c428889cbf40e9kc81322c012c81e728d9d180)容器基本原理部分|
|0808|1.阅读许中兴博士Fuchsia介绍，了解到原生进程沙箱等，在碎碎念可见<br/>2.参加文档组组会，可以通过linux-object操作文件，通过zircon-object来实现管理操作|
|0809|1.听王润基学长交流。具体感受就是，脚踏实地仰望星空。<br/>2.在助教的帮助下运行zCore|
|0812|1.重点实现libcontainer && aufs这一部分的东西<br/>2. 这几天手头稍微有点忙，下周开始每天可以投入更多的时间参与开发<br/>3. 了解了一些Rust社区的进展|
|0816|linux下的参考资料比较多，先在ubuntu上搞。后边移植过来。<br/>容器这些主要用Go语言，在ubuntu上先做到可以Rust切换根目录这件事。|
|0820|运行了Ubuntu上的简易容器，隔离了进程，没隔离文件|
|0823|修BUG，尝试移植|
|0901|RUST移植基本完成，已经隔离进程&&文件 |
|0902|基于[系列文章](https://windsock.io/net-namespace/)及其[代码](https://github.com/nbrownuk/Namespaces)现在打算进一步支持网络隔离|
|0904|添加流程图|

