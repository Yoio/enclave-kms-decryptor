const net = require('net');
const vsock = require('vsock');

// 连接到 enclave
const client = vsock.connect(8000, () => {
    console.log('已连接到 enclave');

    // 准备加密数据（这里使用示例数据，实际使用时需要替换为真实的加密数据）
    const encryptedData = Buffer.from('示例加密数据').toString('base64');

    // 构建请求
    const request = {
        encrypted_data: encryptedData
    };

    // 发送请求
    client.write(JSON.stringify(request));
    client.end();
});

// 接收响应
let data = '';
client.on('data', (chunk) => {
    data += chunk;
});

client.on('end', () => {
    try {
        const response = JSON.parse(data);
        console.log('解密后的数据:', response.decrypted_data);
        console.log('公钥:', response.public_key);
    } catch (error) {
        console.error('处理响应时出错:', error);
    }
}); 