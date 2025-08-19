# Rust Commit

一个智能的 Git 提交工具，支持中英文双语提交信息生成。

## ✨ 特性

- 🔍 **智能检测** - 自动检测 Git 仓库状态和未提交的更改
- 🤖 **AI 生成** - 支持 OpenAI、Anthropic、DeepSeek 等 AI 提供商
- 🌏 **双语支持** - 生成中英文双语的提交信息
- 🎨 **彩色输出** - 美观的终端输出，便于阅读
- 🔧 **灵活配置** - 支持多种配置方式和自定义 API 端点
- 🐛 **调试模式** - 内置调试功能，方便排查问题

## 📦 安装

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/yourusername/rust-commit.git
cd rust-commit

# 编译发布版本
cargo build --release

# 安装到系统（可选）
cargo install --path .

# 可执行文件位于
./target/release/rust-commit
```

### 系统要求

- Rust 1.70+
- Git 2.0+

## 🚀 快速开始

### 1. 初始化配置

```bash
# 在用户目录创建全局配置
rust-commit init

# 或在当前项目创建本地配置
rust-commit init --local
```

### 2. 设置 API Key

```bash
# 方式一：环境变量（推荐）
export OPENAI_API_KEY="your-api-key"

# 方式二：配置文件
# 编辑 ~/.config/rust-commit/config.toml
```

### 3. 使用工具

```bash
# 查看仓库状态
rust-commit

# 生成 AI 提交信息
rust-commit commit

# 显示差异后再生成
rust-commit commit --show-diff

# 调试模式（显示 AI 原始响应）
rust-commit commit --debug
```

## 📖 使用指南

### 基本命令

#### 状态检查
```bash
# 检查当前目录
rust-commit
rust-commit status

# 检查指定目录
rust-commit -p /path/to/repo

# 详细输出
rust-commit -v
```

#### 查看差异
```bash
# 查看所有修改
rust-commit diff

# 仅查看已暂存的修改
rust-commit diff --staged
```

#### AI 提交
```bash
# 基本使用
rust-commit commit

# 指定 API key（临时）
rust-commit commit --api-key YOUR_KEY

# 使用特定模型
rust-commit commit --model gpt-4
rust-commit commit --model deepseek-v3

# 自动提交（跳过确认）
rust-commit commit --auto

# 显示差异预览
rust-commit commit --show-diff

# 调试模式
rust-commit commit --debug
```

### 配置文件

配置文件查找顺序：
1. 当前目录 `.rust-commit.toml`
2. `~/.config/rust-commit/config.toml`
3. `~/.rust-commit.toml`

#### 配置示例

```toml
[ai]
# AI 提供商：openai 或 anthropic
provider = "openai"

# 模型名称
model = "deepseek-v3"  # 或 gpt-4, claude-3-opus 等

# API Key 环境变量名
api_key_env = "OPENAI_API_KEY"

# 直接设置 API Key（不推荐）
# api_key = "sk-..."

# 自定义 API 端点（用于代理或私有部署）
base_url = "https://api.deepseek.com/v1"

[commit]
# 提交信息格式
format = "conventional"

# 是否包含 emoji
include_emoji = false

# 发送给 AI 的最大差异大小
max_diff_size = 4000

# 是否自动暂存所有更改
auto_stage = false
```

### API Key 配置优先级

1. **命令行参数**（最高优先级）
   ```bash
   rust-commit commit --api-key YOUR_KEY
   ```

2. **配置文件**
   ```toml
   api_key = "your-key"
   ```

3. **环境变量**
   ```bash
   export OPENAI_API_KEY="your-key"
   export ANTHROPIC_API_KEY="your-key"
   ```

4. **交互式输入**（最低优先级）

## 🌏 双语提交信息

工具会生成符合 Conventional Commits 规范的中英文双语提交信息：

### 格式示例

```
feat(auth): 添加用户认证功能
Add user authentication feature

实现了JWT令牌验证
Implement JWT token validation
添加了用户登录接口
Add user login endpoint
集成了OAuth2.0支持
Integrate OAuth2.0 support
```

### 提交类型

- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式调整（不影响功能）
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建或辅助工具的变动
- `perf`: 性能优化

## 🐛 调试功能

使用 `--debug` 标志可以查看 AI 的原始响应，方便排查问题：

```bash
rust-commit commit --debug
```

调试模式会显示：
- 原始 HTTP 响应
- AI 返回的 JSON 内容
- 解析前的消息内容

## 💻 开发

### 开发命令

```bash
# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 自动修复
cargo fix

# 开发模式运行
cargo run -- commit
```

### 项目结构

```
src/
├── main.rs        # 程序入口和命令分发
├── cli.rs         # 命令行参数定义
├── config.rs      # 配置文件管理
├── git.rs         # Git 操作封装
├── ui.rs          # 用户交互界面
└── ai/
    ├── mod.rs     # AI 客户端接口
    ├── openai.rs  # OpenAI 实现
    └── anthropic.rs # Anthropic 实现
```

## 🔧 故障排除

### 常见问题

1. **API 连接失败**
   - 检查网络连接
   - 验证 API key 是否正确
   - 使用 `--debug` 查看详细错误

2. **JSON 解析错误**
   - 使用 `--debug` 查看 AI 返回的原始内容
   - 检查模型是否支持 JSON 格式输出
   - 尝试更换模型

3. **配置文件未生效**
   - 检查配置文件位置是否正确
   - 验证 TOML 格式是否有效
   - 注意 `api_key_env` 是环境变量名，不是 key 本身

### 获取帮助

```bash
# 查看帮助信息
rust-commit --help
rust-commit commit --help
```

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 🙏 致谢

- [git2-rs](https://github.com/rust-lang/git2-rs) - Git 操作
- [clap](https://github.com/clap-rs/clap) - 命令行解析
- [dialoguer](https://github.com/console-rs/dialoguer) - 交互式界面
- [colored](https://github.com/colored-rs/colored) - 彩色输出