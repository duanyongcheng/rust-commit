# Rust Commit

一个智能的 Git 提交工具，可以：
- 🔍 检测当前目录是否为 Git 仓库
- 📝 显示未提交的文件修改
- 🤖 使用 AI 生成结构化的 commit message
- 🎨 彩色输出，便于阅读

## 功能特性

### 1. 状态检查
```bash
# 检查当前目录
rust-commit

# 检查指定目录
rust-commit -p /path/to/repo

# 详细输出
rust-commit -v
```

### 2. 查看 Diff
```bash
# 查看所有修改
rust-commit diff

# 仅查看已暂存的修改
rust-commit diff --staged
```

### 3. AI 生成 Commit Message
```bash
# 使用 AI 生成 commit message
rust-commit commit

# 指定 API key
rust-commit commit --api-key YOUR_API_KEY

# 使用不同的模型
rust-commit commit --model gpt-4

# 自动提交（不需要确认）
rust-commit commit --auto

# 显示 diff 预览
rust-commit commit --show-diff
```

## 安装

```bash
# 克隆仓库
git clone <repository>
cd rust-commit

# 编译
cargo build --release

# 可执行文件位于
./target/release/rust-commit
```

## 快速开始

```bash
# 初始化配置文件（推荐）
rust-commit init

# 或在当前项目创建配置
rust-commit init --local

# 设置 API Key
export OPENAI_API_KEY="your-api-key"

# 使用工具
rust-commit                    # 查看状态
rust-commit commit              # 生成 AI commit message
```

## 配置

创建配置文件 `.rust-commit.toml`：

```toml
[ai]
provider = "openai"  # 或 "anthropic"
model = "gpt-4"
api_key_env = "OPENAI_API_KEY"

[commit]
format = "conventional"
include_emoji = false
max_diff_size = 4000
auto_stage = false
```

配置文件查找顺序：
1. 当前目录 `.rust-commit.toml`
2. `~/.config/rust-commit/config.toml`
3. `~/.rust-commit.toml`

## 环境变量

设置 API Key：
```bash
export OPENAI_API_KEY="your-api-key"
# 或
export ANTHROPIC_API_KEY="your-api-key"
```

## Commit Message 格式

生成的 commit message 遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建或辅助工具的变动
- `perf`: 性能优化

格式示例：
```
feat(auth): add user login functionality

Implemented JWT-based authentication system with
refresh token support and session management.

BREAKING CHANGE: API endpoints now require auth token
```

## 使用示例

1. **检查仓库状态**
```bash
$ rust-commit
Checking: /Users/bary/project

Git repository detected ✓
Uncommitted changes detected ✗

Modified files:
  M src/main.rs
  M README.md

New files:
  A config.toml

Total uncommitted changes: 3

Current branch: main
Tracking: origin/main
Status: 2 ahead, 1 behind
```

2. **生成 AI Commit Message**
```bash
$ rust-commit commit --show-diff

Diff Preview:
──────────────────────────────────────────────────
+fn new_feature() {
+    println!("Hello, World!");
+}
──────────────────────────────────────────────────
Continue with commit generation? [Y/n]

ℹ Generating commit message with AI...

Generated Commit Message:
──────────────────────────────────────────────────
feat: add new feature function

Added a simple hello world function for demonstration
──────────────────────────────────────────────────

What would you like to do?
> Accept and commit
  Edit message
  Regenerate with different AI
  Cancel

✓ Changes committed successfully!
```

## 开发

```bash
# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

## License

MIT