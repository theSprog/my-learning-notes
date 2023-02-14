素性判定算法，时间复杂度($O(log^3(n))$)

注意该算法不是确定性算法，而是随机算法，也就是说结果可能有误。具体来说，如果输出 `false`, 那么不会出错；如果输出 `true`, 则有概率出错。平均来看，出错概率不会大于$\frac{1}{4}$. 我们可以多运行几次该算法来缩小错误率：一旦输出 `false`那么必然不是素数。

```rust
extern crate rand;	// 随机包

use rand::Rng;
use std::io::stdin;

//自定义乘法，代替默认乘法，防止 a * b爆炸
fn q_mul(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut ans: u64 = 0;
    while b != 0 {
        if b & 1 == 1 {
            ans = (ans + a) % m;
        }
        a = (a + a) % m;
        b = b >> 1;
    }
    return ans;
}

//自定义幂，代替默认幂，防止 base ** power 爆炸
fn q_pow(mut base: u64, mut power: u64, m: u64) -> u64 {
    let mut ans: u64 = 1;
    while power != 0 {
        if power & 1 == 1 {
            ans = q_mul(ans, base, m);
        }
        base = q_mul(base, base, m);
        power = power >> 1;
    }
    return ans % m;
}

fn btest(a: u64, n: u64) -> bool {
    let mut s: u64 = 0;
    let mut t: u64 = n - 1;
    while t % 2 != 1 {
        s = s + 1;
        t = t >> 1;
    }
    // x = (a ^ t) mod n, 问题是 a ^ t 有可能爆炸
    let mut x = q_pow(a, t, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..s {
        x = x * x % n;		// a^((2^i)*t) mod n
        if x == n - 1 {
            return true;
        }
    }
    return false;
}

// 算法入口
fn mill_rab(n: u64) -> bool {
    if n % 2 == 0 || n < 4 {
        return true;
    } // n == 2 or 3 or 4
    let mut rng = rand::thread_rng();
    let a: u64 = rng.gen_range(2, n - 1);
    return btest(a, n);
}
```

