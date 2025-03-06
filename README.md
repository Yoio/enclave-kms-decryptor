# Enclave KMS 解密器

这个项目实现了一个在 AWS Nitro Enclave 中运行的解密服务，使用 AWS KMS 进行数据解密，并通过 vsock 与父实例进行通信。

## 功能特点

- 在 enclave 中获取 AWS KMS 私钥
- 通过 vsock 接收加密数据
- 使用 KMS 进行数据解密
- 返回解密后的数据和公钥

## 前置要求

- Node.js 18+
- AWS 凭证配置
- AWS KMS 密钥
- Docker
- AWS Nitro CLI
- AWS Nitro Enclaves 环境

## 配置

1. 设置环境变量：
   - `AWS_REGION`：AWS 区域（默认为 ap-northeast-1）
   - `KMS_KEY_ID`：KMS 密钥 ID

2. 确保 AWS 凭证已正确配置（可以通过环境变量或 AWS 凭证文件）

## 构建 Enclave 镜像

```bash
# 安装依赖
npm install

# 构建 Docker 镜像
docker build -t enclave-kms-decryptor .

# 将镜像转换为 Nitro Enclave 镜像
nitro-cli build-enclave --docker-uri enclave-kms-decryptor:latest --output-file enclave-kms-decryptor.eif
```

## 运行 Enclave

1. 启动 Nitro Enclave：
```bash
nitro-cli run-enclave --eif-path enclave-kms-decryptor.eif --cpu-count 2 --memory-size 512 --attach
```

2. 在父实例中运行测试程序：
```bash
npm test
```

## 安全说明

- 确保 AWS 凭证安全存储
- 在生产环境中使用适当的错误处理
- 定期更新依赖项以修复安全漏洞
- 使用非 root 用户运行 enclave 程序
- 最小化运行时依赖

## 注意事项

- 确保 enclave 有足够的权限访问 AWS KMS
- vsock 通信仅在同一实例内的 enclave 和父实例之间有效
- 建议在生产环境中添加适当的日志记录和监控
- 确保 AWS 区域设置正确
- 在生产环境中使用适当的资源限制（CPU、内存等） 