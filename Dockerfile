# 构建阶段
FROM node:18-slim as builder

# 设置工作目录
WORKDIR /usr/src/app

# 复制 package.json 和 package-lock.json
COPY package*.json ./

# 安装依赖
RUN npm ci

# 复制源代码
COPY . .

# 运行阶段
FROM amazonlinux:2023

# 安装 Node.js 和必要的运行时依赖
RUN dnf update -y && dnf install -y \
    nodejs \
    ca-certificates \
    shadow-utils \
    && dnf clean all

# 创建非 root 用户
RUN useradd -m -u 1000 enclave

# 设置工作目录
WORKDIR /app

# 从构建阶段复制文件
COPY --from=builder /usr/src/app/node_modules ./node_modules
COPY --from=builder /usr/src/app/src ./src
COPY --from=builder /usr/src/app/package*.json ./

# 设置权限
RUN chown -R enclave:enclave /app

# 切换到非 root 用户
USER enclave

# 设置环境变量
ENV AWS_REGION=ap-northeast-1
ENV NODE_ENV=production
ENV KMS_KEY_ID=YOUR_KEY_ID

# 暴露 vsock 端口
EXPOSE 8000

# 启动命令
CMD ["node", "src/enclave.js"] 