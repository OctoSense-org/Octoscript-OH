# Octoscript-OH

[English](README.md) | 简体中文

## 共享的 Octoscript-Makepad 运行时

`native-runtime.lock.json` 选定一个
[Octoscript-Makepad](https://github.com/OctoSense-org/Octoscript-Makepad)
发布版本。该版本的 `runtime.json` 负责锁定确切的 Makepad 和 Octoscript 版本，
AppCards、Mail 以及其他 OctoSense 应用共用同一份。

构建之前先运行 `python3 tools/setup-native.py`（需要 Python 3.9+）。框架仓库与本应用
并列存放：`../octoscript-makepad`、`../makepad` 和 `../octoscript`。本地改动会被保留；
`--update` 只更新没有改动的 checkout。CI 会校验所选的发布版本，并拒绝重复的 Makepad 来源。
用 `python3 tools/setup-native.py --check --cargo-manifest Cargo.toml`
检查本地依赖图。现有的各平台渲染后端仍然属于各自的应用；框架负责的是共享的 VM 和 UI 源码。


做一个鸿蒙应用：前端用网页，其余全部交给 Rust。

页面里调 `octoscript.invoke('device.info')`，答案来自 Rust。页面周围是真正的原生
ArkUI 控件——同样由 Rust 构建，不经过 ArkTS。整体形态和 Tauri 一致；控件那一层
是 Tauri 没有的。

```js
const info = await octoscript.invoke('device.info')
// { productModel: "SUP-AL90", osFullName: "OpenHarmony-6.1.1.120", ... }
```

## 上手

```sh
octoscript-oh new my-app
cd my-app
npm install && npm run build
./build.sh              # 编译、安装并在已连接的手机上启动
```

`./build.sh` 需要一份 Octoscript-OH 源码作为构建依托——在项目旁边克隆一份，或设置
`OCTOSCRIPT_OH`。详见 **[docs/building-an-app.zh-CN.md](docs/building-an-app.zh-CN.md)**。

开发过程中可以完全跳过重新构建：

```sh
octoscript-oh dev           # 通过 USB 把手机打通到你的机器
npm run dev
OCTOSCRIPT_DEV_SERVER=http://127.0.0.1:5173 ./build.sh
```

此后改前端，手机上大约一秒就刷新。改 Rust 仍然要重新构建。

## 文档

| | |
|---|---|
| [做一个应用](docs/building-an-app.zh-CN.md) | 模板、开发回路、`octoscript.toml`、构建 |
| [插件](docs/plugins.zh-CN.md) | 自己的原生工具，同步与异步 |
| [能力](docs/capabilities.zh-CN.md) | 页面能做什么，以及如何强制执行 |
| [发布](docs/releasing.zh-CN.md) | 签名、AGC，以及尚未完成的部分 |

每个页面都有中文版：每个 `docs/*.md` 旁边都有对应的 `docs/*.zh-CN.md`。

## 页面能碰到什么

49 个内置工具（其中少数是内部用的）：设备信息、显示、电池、传感器、振动、定位、
蜂窝网络、Wi-Fi、网络请求、文件系统、文件选择器、剪贴板、HUKS 密钥库、SQLite、
蓝牙、相机、音频、视频、加解密，以及 Octoscript VM。你自己的工具和它们并列——见
[docs/plugins.zh-CN.md](docs/plugins.zh-CN.md)。

页面只能拿到它所在的那个 surface 被授予的东西。信任不是一个布尔值：每个 surface
自己声明可以调用哪些工具、可以访问哪些目录、可以访问哪些主机。见
[docs/capabilities.zh-CN.md](docs/capabilities.zh-CN.md)。

## 目录结构

```
crates/octoscript-oh-arkui/       渲染器                       rlib
crates/octoscript-oh-core/         注册表、Args、Responder      rlib
crates/octoscript-oh-webview/      桥接层与各项能力              cdylib -> liboctoscript_oh.so
crates/octoscript-oh-plugin-demo/  插件示例                     rlib
examples/<app>/                每个示例应用一个目录
tools/octoscript-oh-cli/           宿主机侧工具                  bin: octoscript-oh
deveco/                        ArkTS 外壳
```

依赖是单向的，这正是插件机制能成立的原因。`octoscript-oh-arkui` 不知道 webview 的
存在。`octoscript-oh-core` 不知道桥接层的存在——所以插件可以依赖它，而不必依赖应用
本体。`octoscript-oh` 是 `cdylib`，是最终产物，没有别的东西链接它，因此由它来决定
这次构建包含哪些插件。

最后只产出一个 `.so`，因为 ArkTS 只加载一个。

### octoscript-oh-arkui

用 Rust 把 UI 树渲染成 ArkUI 原生控件。ArkTS 在启动时交出一个 `NodeContent`，
之后每个控件的创建、配置、布局和事件绑定都由原生代码完成，没有逐控件、也没有
逐帧的 ArkTS 调用。

其中包含 ArkUI NDK 绑定、Octoscript DSL 遍历器、控件构造器、四个移植过来的参考
应用（微信、淘宝、抖音、Wonderous），以及它们存在的目的——Rust 与 ArkTS 的对比
基准测试。

#### 组件目录

`assets/catalog.octoscript` 是用 DSL 写、渲染成原生 ArkUI 的 Material 组件目录：
一个索引页加 28 个页面，没有 makepad，没有 ArkTS 控件。28 个页面全部拍在
`catalog-screens.png` 里，并且每一个都在真机上亲眼看过，而不只是"能打开"。

第一次这样检查时发现有两个是错的，现已修复：Badges 页面在写着"数字徽标"的说明
下画的是没有数字的色块；Text picker 因为从来没有人给它设置 range，画出来是空
的几行。`Index.ets` 里的 `CATALOG_WALK_MS` 可以重跑那次把它们找出来的巡检。

## ArkTS 还在哪里、不在哪里

| | |
|---|---|
| 完全不需要 | 控件树、全部能力、承载相机与视频的 XComponent surface |
| 结构上做不到 | `Web` 组件——不存在 `ARKUI_NODE_WEB` |
| 只是因为没有 NDK | 文件选择器、剪贴板、运行时权限申请、BLE 扫描 |

`OH_NativeArkWeb_RunJavaScript` 在真机上能解析到符号，但 controller 的 web tag
始终绑不上，所以桥接流量仍然经由 ArkTS 中转。这是实测结论，不是推断——见
`crates/octoscript-oh-webview/src/arkweb.rs`。

## 老实说的现状

这套东西跑在真机上，本文提到的每一条都是在 HarmonyOS 6.1 设备上验证过的，不是
推想出来的。还没做完的部分：

- **发布签名没有接通。** `sign-hap.sh` 里已有无 IDE 的 AGC 签名路径，
  `octoscript.toml` 里也已有 `[signing]` 段，但两者还没连起来。见
  [docs/releasing.zh-CN.md](docs/releasing.zh-CN.md)。
- **外壳是一份源码，不是一个依赖。** 项目是"依托"一份 Octoscript-OH 源码来构建的，
  而把你自己的插件链接进去目前还需要在那份源码里手工改两处。
- **没有多窗口、自动更新、托盘。** 鸿蒙上的对应能力还没去查，谈不上有计划。
- **`cargo test` 在这里跑不起来。** 这些 crate 只为
  `aarch64-unknown-linux-ohos` 构建，宿主机执行不了，推到设备上又被 SELinux
  拒绝。所以真正重要的检查改成在启动时运行并把结果写进日志——在 hilog 里搜
  `selftest`，而且要立刻去看：大约一分钟内 hilog 就会被 Chromium 的输出灌满，把这些日志挤掉。
