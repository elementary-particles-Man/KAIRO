pub mod router {
    pub fn assign() {
        // A2: Pアドレス取得
        // GET /assign_p_address を1度だけ実行
        println!("KAIROBOT API: Pアドレス取得を実行します。");
        // TODO: HTTP GETリクエストの実装
    }

    pub fn send(json: &str) {
        // A3: JSON送信
        // 任意のJSONをUIから貼り付け、POST /send
        println!("KAIROBOT API: JSON送信を実行します。JSON: {}", json);
        // TODO: HTTP POSTリクエストの実装
    }
}