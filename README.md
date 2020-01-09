# 对账系统
针对区块链交易信息与用户账户体系构建的对账系统，主要为了帮助定位分布式系统中，由于物理制约和时间跨度，所造成的账务数据不一致的问题。
## 环境准备
- rust 1.40.0

## 相关命令
build:
```
cargo build --release
```

test:
```
cargo test
```

## 代码规范
- Rust下的code style
- Rust下的工程项目、模块实践

## Git使用规范

### Git分支策略

推荐使用Trunk Based Flow，所有的开发工作都在同一个`master`分支上进行，同时利用[Continuous Integration](http://www.martinfowler.com/articles/continuousIntegration.html)在`master`分支上进行构建与测试，确保`master`上的代码随时都是production ready的。另外，可以从`master`上拉出`release`分支进行发布的追踪。

![02](http://insights.thoughtworkers.org/wp-content/uploads/2016/02/02.jpg)

> 参考文章
>
> - [Gitflow有害论](http://insights.thoughtworkers.org/gitflow-consider-harmful/)
> - [Trunk-Based-Development](http://paulhammant.com/2013/04/05/what-is-trunk-based-development/)

### Git commit message规范

- format: `type(<Trello-Number>) message`
> example: `feat(175) add the request validation`

- type可选范围

```
feat: 新功能（feature）
fix: 修复bug
docs: 文档
style: 格式
refactor: 重构
test: 增加测试
pref: 性能调优
chore: 构建过程或辅助工具的变动（如pipeline配置等）
```

### Git push前置要求
1. 保证代码测试覆盖率100%
2. 保证所有代码通过测试

### Git hooks