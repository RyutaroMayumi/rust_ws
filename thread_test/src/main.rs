use chrono::NaiveDateTime; // タイムゾーンなしの日時
use std::sync::{Arc, Mutex}; // スレッドセーフな共有参照とMutexロックを使用
#[derive(Copy, Clone, Debug)]
// 観測データの構造体(データの組)を定義
struct Measurement {
    // 日時(タイムゾーン無し)
    time: NaiveDateTime,
    // 観測値
    value: f64,
    // データを観測したスレッドID
    thread_id: usize,
}
impl Measurement {
    // Measurement構造体の関連関数(メソッド)を定義
    fn new(value: f64, thread_id: usize) -> Self {
        Measurement {
            // 生成時の現在時刻を記録
            time: chrono::Utc::now().naive_utc(),
            // 観測値を登録, キーと変数名と同じなら省略記法が使える
            value,
            // データを生成したスレッドを記録
            thread_id,
        }
    }
}
fn main() -> anyhow::Result<()> {
    // キューの生成、キューの所有権はメインスレッドが持つ
    let queue = Vec::new();
    // （1）：キューのロック付(Mutex)の共有参照(Arc)を定義する
    //! 20220508 mayumi
    //! Arc -> Atomically Reference Counted（原始的参照カウント）
    //! 「原始的」とはスレッド間で共有しても安全に機能することを示す
    //! Rc -> Reference Counted（参照カウント）も存在するが、スレッド間での共有はコンパイルエラーになる
    //! clone()メソッドで参照のコピーを生成し、カウントを一つ増加する
    //! https://qiita.com/qnighy/items/4bbbb20e71cf4ae527b9
    //! 20220508 mayumi
    //! Mutex -> Mutual Exclusion（相互排他）
    //! 資源共有の仕組みの一つ
    //! Mutexで鍵付きのキューへの参照を生成し、さらにArcでカウント可能な共有参照を生成する
    //! すなわち、セマフォを定義している
    let arc_queue = Arc::new(Mutex::new(queue));
    // キューの共有参照をコピー（そうしないと複数のスレッドからアクセスできない）
    let arc_queue1 = arc_queue.clone();
    // データ記録スレッド1を作成
    // moveで共有参照を別スレッドに移動
    // 共有参照を移動＝キューの参照権限を別スレッドにも渡したと考えると分かりやすい
    //! 20220508 mayumi
    //! "||{}"はクロージャの定義
    //! クロージャとは変数や引数として取り扱うことのできる無名関数
    //! キーワードmoveとクロージャを組み合わせることで、クロージャに変数の所有権を移動する
    //! https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/64c6f3
    let record_thread1 = std::thread::spawn(move || {
        for i in 1..=10000 {
            let m = Measurement::new(i as f64, 1);
            // （2）:キューのロックを取りキューに観測値を記録
            arc_queue1.lock().unwrap().push(m);
        }
    });
    let arc_queue2 = arc_queue.clone();
    // データ記録スレッド2を作成
    let record_thread2 = std::thread::spawn(move || {
        for i in 1..=10000 {
            let m = Measurement::new(i as f64, 2);
            arc_queue2.lock().unwrap().push(m);
        }
    });
    // データ観測スレッドを作成
    // 1ミリ秒のスリープを挟みつつ、キューの最新値を出力
    //! 20220508 mayumi
    //! 警告回避のための修正
    //! observe_threadは以降で参照されないため、無名スレッドに変更
    // let observe_thread = std::thread::spawn(move || loop {
    std::thread::spawn(move || loop {
            // ここで{}と書いてスコープを作ることには意味がある
        {
            // ロックを取得
            let queue_lock = arc_queue.lock().unwrap();
            let latest = queue_lock.last();
            //! 20220508 mayumi
            //! 警告回避のための修正
            //! Measurement構造体のすべての要素にアクセスする
            // println!("{:?}", latest);
            if let Some(v) = latest {
                println!("{{time:{},\tvalue:{},\tthread_id:{}}}", v.time, v.value, v.thread_id);
            }
            // スコープによりここでqueue_lock変数が無効になる→ロックが解放される
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    });
    // 2つのデータ記録スレッドの終了を待つ
    for thread in [record_thread1, record_thread2] {
        let _ = thread.join();
    }
    Ok(())
}
