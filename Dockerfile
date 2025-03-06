# 构建阶段
FROM rust:1.75-slim-bullseye as builder

# 安装必要的构建工具
RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /usr/src/app

# 复制项目文件
COPY . .

# 构建项目
RUN cargo build --release

# 运行阶段
FROM amazonlinux:2

# 安装必要的运行时依赖
RUN yum update -y && yum install -y \
    ca-certificates \
    && yum clean all

# 创建非 root 用户
RUN useradd -m -u 1000 enclave

# 设置工作目录
WORKDIR /app

# 从构建阶段复制编译好的二进制文件
COPY --from=builder /usr/src/app/target/release/enclave-kms-decryptor /app/
COPY --from=builder /usr/src/app/target/release/parent /app/

# 设置权限
RUN chown -R enclave:enclave /app

# 切换到非 root 用户
USER enclave

# 设置环境变量
ENV AWS_REGION=ap-northeast-1
ENV RUST_LOG=info

# 暴露 vsock 端口
EXPOSE 8000

# 启动命令
CMD ["./enclave-kms-decryptor"] 