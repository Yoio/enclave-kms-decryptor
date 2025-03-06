const { KMSClient, GetPublicKeyCommand, DecryptCommand } = require('@aws-sdk/client-kms');
const net = require('net');
const vsock = require('vsock');

// 初始化 AWS KMS 客户端
const kmsClient = new KMSClient({
    region: process.env.AWS_REGION || 'ap-northeast-1'
});

// 创建 vsock 服务器
const server = vsock.createServer((socket) => {
    console.log('收到新的连接');

    let data = '';
    socket.on('data', async (chunk) => {
        data += chunk;
    });

    socket.on('end', async () => {
        try {
            const request = JSON.parse(data);
            
            // 获取 KMS 公钥
            const getPublicKeyResponse = await kmsClient.send(
                new GetPublicKeyCommand({
                    KeyId: process.env.KMS_KEY_ID
                })
            );
            
            const publicKey = getPublicKeyResponse.PublicKey.toString('base64');

            // 解密数据
            const decryptResponse = await kmsClient.send(
                new DecryptCommand({
                    CiphertextBlob: Buffer.from(request.encrypted_data, 'base64')
                })
            );

            const decryptedData = decryptResponse.Plaintext.toString('utf-8');

            // 构建响应
            const response = {
                decrypted_data: decryptedData,
                public_key: publicKey
            };

            // 发送响应
            socket.write(JSON.stringify(response));
            socket.end();
        } catch (error) {
            console.error('处理请求时出错:', error);
            socket.write(JSON.stringify({ error: error.message }));
            socket.end();
        }
    });
});

// 监听 vsock 端口
const PORT = 8000;
server.listen(PORT, () => {
    console.log(`Enclave 正在监听 vsock 端口 ${PORT}...`);
}); 