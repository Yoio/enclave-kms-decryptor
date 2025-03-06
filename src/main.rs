use anyhow::Result;
use aws_sdk_kms::Client as KmsClient;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use vsock::{VsockAddr, VsockListener, VsockStream};
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

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化 AWS KMS 客户端
    let config = aws_config::load_from_env().await;
    let kms_client = KmsClient::new(&config);

    // 获取 KMS 密钥的公钥
    let get_public_key_response = kms_client
        .get_public_key()
        .key_id("YOUR_KEY_ID") // 替换为实际的 KMS 密钥 ID
        .send()
        .await?;
    
    let public_key = BASE64.encode(get_public_key_response.public_key().unwrap());
    println!("KMS 公钥: {}", public_key);

    // 创建 vsock 监听器
    let listener = VsockListener::bind(&VsockAddr::new(8000))?;
    println!("Enclave 正在监听 vsock 端口 8000...");

    loop {
        let (mut stream, _) = listener.accept()?;
        println!("收到新的连接");

        // 处理每个连接
        tokio::spawn(async move {
            if let Err(e) = handle_connection(&mut stream, &kms_client, &public_key).await {
                eprintln!("处理连接时出错: {}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: &mut VsockStream,
    kms_client: &KmsClient,
    public_key: &str,
) -> Result<()> {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    let request: DecryptRequest = serde_json::from_slice(&buffer)?;
    
    // 解码加密数据
    let encrypted_data = BASE64.decode(request.encrypted_data)?;

    // 使用 KMS 解密数据
    let decrypt_response = kms_client
        .decrypt()
        .ciphertext_blob(encrypted_data)
        .send()
        .await?;

    let decrypted_data = decrypt_response.plaintext().unwrap();
    let decrypted_string = String::from_utf8(decrypted_data.to_vec())?;

    // 构建响应
    let response = DecryptResponse {
        decrypted_data: decrypted_string,
        public_key: public_key.to_string(),
    };

    // 发送响应
    let response_json = serde_json::to_vec(&response)?;
    stream.write_all(&response_json)?;
    stream.flush()?;

    Ok(())
}
