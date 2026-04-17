# body-throwing-movement

一个工程化的 Rust 命令行工具，用于抛体运动分析。

给定以下参数：
- 出手高度（`h_0`）
- 落点高度（`h_1`）
- 水平距离（`d`）

程序会在可配置的角度范围内扫描，并基于经典力学计算每个**可行角度**对应的所需出手速度。

同时输出：
- **最慢可行出手速度**及其角度
- **最快可行出手速度**及其角度

## 物理模型

对每一个发射角 $\theta$，所需出手速度为：

$$
v_0 = \sqrt{\frac{g d^2}{2\cos^2(\theta)\left(d\tan(\theta) - (h_1 - h_0)\right)}}
$$

仅当分母为正且数值有效时，该角度才是可行解。

## 功能特性

- 使用 Rust 编写的 CLI，参数校验清晰
- 输出所有可行“角度-速度”结果表
- 自动汇总最慢/最快速度与对应角度
- 支持可选 CSV 导出
- 支持可选速度-角度可视化导出（`.png` 或 `.svg`）
- 包含单元测试与集成测试
- CI 检查（`fmt`、`clippy`、`test`、`release build`）
- Release 工作流支持多平台二进制打包

## 快速开始

```bash
cargo run -- \
  --launch-height 1.8 \
  --landing-height 1.0 \
  --distance 20 \
  --angle-min 5 \
  --angle-max 85 \
  --angle-step 0.5 \
  --csv-out output/results.csv \
  --plot-out output/speed-angle.png
```

## CLI 参数说明

```text
--launch-height   出手高度（米）
--landing-height  落点高度（米）
--distance        水平距离（米）
--angle-min       扫描最小角度（度，默认: 1）
--angle-max       扫描最大角度（度，默认: 89）
--angle-step      角度步长（度，默认: 0.5）
--gravity         重力加速度（m/s^2，默认: 9.80665）
--csv-out         可选 CSV 输出路径
--plot-out        可选图表输出路径（.png 或 .svg）
```

## 项目结构

```text
src/
  cli.rs                  # CLI 参数定义
  error.rs                # 错误类型
  main.rs                 # 程序入口
  physics/
    ballistics.rs         # 抛体求解核心
  output/
    table.rs              # 控制台输出与 CSV 导出
    plot.rs               # PNG/SVG 可视化
tests/
  cli.rs                  # 集成测试
.github/workflows/
  ci.yml                  # CI 流水线
  release.yml             # 基于 tag 的发布打包
```

## 开发命令

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run -- --help
```

## CI/CD

- `CI` 工作流在 push/PR 时执行：
  - 格式检查
  - clippy 静态检查
  - 测试
  - release 构建健康检查
- `Release` 工作流在 `v0.1.0` 这类 tag 推送时执行：
  - 构建 Linux/macOS/Windows 二进制
  - 打包并附带 README 与 LICENSE
  - 生成 SHA256 校验文件
  - 发布到 GitHub Releases
