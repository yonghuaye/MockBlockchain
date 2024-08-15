有幸得到冲哥指点 非常感谢～

项目主要分为以下：
1、main文件夹：区块链运行
2、utils文件夹：负责序列化操作、哈希运算、时间戳计算等
3、core文件夹： 核心逻辑，包括blockheader、block等业务逻辑

项目执行：
cargo run

运行结果：
Hello to Jeffy BlockChain!
Start mining ...
Produce blocking ...
Start mining ...
Produce blocking ...
================================================
Block {
    header: BlockHeader {
        time: 1723704924,
        tx_hash: "668e5cb7294ae339c468bd1e683438af8da220822a7cef2a323d335ab7088391",
        pre_hash: "",
    },
    hash: "328c61f80d417e62911524cb9701e41e297be74eef1b2c8f61b45ff39c9d97cc",
    data: "Genesis Block",
}

================================================
Block {
    header: BlockHeader {
        time: 1723704934,
        tx_hash: "e524a30f5d9c89c4e29ee7ad4f5d34a3130fa1fb837309ea691c022cb3799f0f",
        pre_hash: "328c61f80d417e62911524cb9701e41e297be74eef1b2c8f61b45ff39c9d97cc",
    },
    hash: "e405f96f577da94ee31475eda3a175f2bdb1511e7657980ad1d4a905eb448818",
    data: "TanqiLong => Jeffy: 1 BTC",
}

================================================
Block {
    header: BlockHeader {
        time: 1723704944,
        tx_hash: "63f3dba1b1d0b19d300a703ec7604d7f652cded207f1bcba9f4f5f6552295b24",
        pre_hash: "e405f96f577da94ee31475eda3a175f2bdb1511e7657980ad1d4a905eb448818",
    },
    hash: "58013e8e2565a4933105bf1b130bc1425aabe07e64d51551873f2783743b6d1f",
    data: "YaleXin => Jeffy: 2 BTC",
}