
# clickflow

**clickflow** 是一个用于自动化执行键盘和鼠标操作的 Rust 库。通过读取用户定义的配置文件，你可以轻松设置复杂的任务流程，并自动运行这些操作。非常适合需要重复执行键鼠操作的任务场景，如测试、自动化脚本、游戏辅助等。

## 功能

- 自动化执行鼠标点击、键盘输入等操作
- 支持通过配置文件定义操作流程
- 用户可自定义操作顺序、延时和重复次数
- 易于集成到现有项目中

## 安装

在 `Cargo.toml` 中添加以下依赖：

```toml
[dependencies]
clickflow = "0.1"
```

然后在项目中引入：

```rust
extern crate clickflow;
```

## 快速开始

1. 创建一个配置文件（如 `config.json`），定义一系列键盘和鼠标操作：

```json
{
  "actions": [
    { "type": "click", "button": "left", "x": 100, "y": 200, "delay": 500 },
    { "type": "keypress", "key": "A", "delay": 1000 },
    { "type": "click", "button": "right", "x": 300, "y": 400, "delay": 500 }
  ]
}
```

2. 使用 `clickflow` 运行这些自动化操作：

```rust
use clickflow::TaskRunner;

fn main() {
    let runner = TaskRunner::from_file("config.json").unwrap();
    runner.run();
}
```

## 配置说明

你可以通过 JSON 文件配置 `clickflow` 的任务流程。以下是支持的操作类型和参数：

- **click**: 模拟鼠标点击操作
  - `button`: 鼠标按钮 (`"left"` 或 `"right"`)
  - `x`, `y`: 鼠标点击的屏幕坐标
  - `delay`: 在执行该操作前等待的毫秒数

- **keypress**: 模拟键盘按键操作
  - `key`: 按键名称（如 `"A"`, `"Enter"`）
  - `delay`: 在执行该操作前等待的毫秒数

## 未来计划

- 支持鼠标拖拽操作
- 支持更多复杂的键盘组合（如 Ctrl+C）
- 提供更灵活的配置选项（如循环执行、条件分支）

## 贡献

欢迎贡献！如果你有任何问题或改进建议，请随时提出 [Issue](https://github.com/yourusername/clickflow/issues)。

## 许可证

本项目基于 MIT 许可证发布，详情请查看 [LICENSE](LICENSE) 文件。
```