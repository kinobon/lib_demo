use rand::Rng;

pub fn gen_ran() -> u8 {
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen(); // 型を明示するとその範囲で乱数を返す
    n
}
