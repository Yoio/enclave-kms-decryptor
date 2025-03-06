use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use vsock::{VsockAddr, VsockStream};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

#[derive(Debug, Serialize, Deserialize)]
struct DecryptRequest {
    encrypted_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DecryptResponse {
    decrypted_data: String,
    public_key: String,
}

fn main() -> Result<()> {
    // 连接到 enclave
    let mut stream = VsockStream::connect(&VsockAddr::new(8000))?;
    println!("已连接到 enclave");

    // 准备加密数据（这里使用示例数据，实际使用时需要替换为真实的加密数据）
    let encrypted_data = "示例加密数据";
    let encrypted_base64 = BASE64.encode(encrypted_data);

    // 构建请求
    let request = DecryptRequest {
        encrypted_data: encrypted_base64,
    };

    // 发送请求
    let request_json = serde_json::to_vec(&request)?;
    stream.write_all(&request_json)?;
    stream.flush()?;

    // 读取响应
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    let response: DecryptResponse = serde_json::from_slice(&buffer)?;
    println!("解密后的数据: {}", response.decrypted_data);
    println!("公钥: {}", response.public_key);

    Ok(())
} 