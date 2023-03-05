use clap::Args;

// use crate::base64::table;

#[derive(Args, Debug)]
#[command(about = "Encoder", long_about = None)]
pub struct Encode {
    text: String,
}

pub fn exec(cli: &Encode) {
    let text = &cli.text;
    println!("{:?}", text);

    // https://news.mynavi.jp/techplus/article/rustalgorithm-5/
    // Base64の変換テーブルを1文字ずつに区切る --- (*3)
    let t = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let table: Vec<char> = t.chars().collect::<Vec<char>>();
    // 変換結果を保持する文字列 --- (*4)
    let mut result = String::new();
    // 入力文字列をバイト列に変換 --- (*5)
    let bin8 = text.as_bytes();
    // 繰り返し24bitごと(3文字ずつ)に処理する --- (*6)
    let cnt = bin8.len() / 3;
    for i in 0..cnt {
        let n = i * 3; // 3文字(24bit)ずつ処理 --- (*7)
        let b24 = ((bin8[n+0] as usize) << 16) +
                  ((bin8[n+1] as usize) <<  8) +
                  ((bin8[n+2] as usize) <<  0);
        result.push(table[(b24 >> 18) & 0x3f]); // 6bitずつ変換 --- (*8)
        result.push(table[(b24 >> 12) & 0x3f]);
        result.push(table[(b24 >>  6) & 0x3f]);
        result.push(table[(b24 >>  0) & 0x3f]);
    }
    // 3バイトずつに割り切れなかった余りの部分を処理 --- (*9)
    match bin8.len() % 3 {
        1 => {
            let b24 = (bin8[cnt*3] as usize) << 16;
            result.push(table[(b24 >> 18) & 0x3f]);
            result.push(table[(b24 >> 12) & 0x3f]);
            result.push_str("==");
        },
        2 => {
            let b24 = ((bin8[cnt*3+0] as usize) << 16) +
                      ((bin8[cnt*3+1] as usize) << 8);
            result.push(table[(b24 >> 18) & 0x3f]);
            result.push(table[(b24 >> 12) & 0x3f]);
            result.push(table[(b24 >>  6) & 0x3f]);
            result.push('=');
        },
        _ => {},
    }
    println!("{:?}", result);
}
