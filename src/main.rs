fn main() {
    let df:i32 = 1;
    let prob:f32 = 0.05; 

    println!("df = {}",df);
    println!("prob = {}",prob);

    println!("tvalue(1,0.05) = {}", tvalue(df, prob));
    println!("tvalue(2,0,05) = {}", tvalue(2, prob));
    println!("tvalue(3,0.05) = {}", tvalue(3, prob));
    println!("tvalue(4,0,05) = {}", tvalue(4, prob));
    println!("tvalue(5,0.05) = {}", tvalue(5, prob));
    println!("tvalue(6,0,05) = {}", tvalue(6, prob));
    //println!("tvalue(INF,0.05) = {}", tvalue(core::f32::INFINITY as i32, prob));
}

fn tvalue(df: i32, prob: f32,) -> f32 {

    const HALFPI:f32 = core::f32::consts::PI / 2.0;
    assert!((df >= 1));
    assert!((prob > 0.0) && (prob < 1.0));

    let t = if df == 1 {
            (prob * HALFPI).cos() / (prob * HALFPI).sin()
        } else if df == 2 {
            (2.0/(prob*(2.0-prob))-2.0).sqrt()
        } else {
            let xn = df as f32;
        let a = 1.0 / (xn - 0.5);
        let b = 48.0 / (a * a);
        let mut c = ((20700.0 * a / b - 98.0) * a - 16.0) * a + 96.36;
        let d = ((94.5 / (b + c) - 3.0) / b + 1.0) * (a * HALFPI).sqrt() * xn;
        let mut x = d * prob;
        let mut y = x.powf(2.0 / xn);

        if y <= a + 0.05 {
            y = ((1.0/(((xn + 6.0) / (xn * y) - 0.089 * d - 0.822) * (xn + 2.0) * 3.0) + 0.5 / (xn + 4.0)) * y - 1.0) * (xn + 1.0) / (xn + 2.0) + 1.0 / y;
            } else {
                let q = prob * 0.5;
                let qt = ( 1.0 / (q * q)).ln().sqrt();
                x = -qt + (((0.802853 + qt*0.010328) * qt + 2.515517) / (((0.189269 + qt * 0.001308) * qt + 1.432788) * qt + 1.0));
                y = x * x;
                if df < 5 { 
                    c = c + 0.3 * (xn - 4.5) * (x + 0.6);
                } else { };
                c = (((0.05 * d * x - 5.0) * x - 7.0) * x - 2.0) * x + b + c;
                y = (((((0.4 * y + 6.3) * y + 36.0) * y + 94.5) / c - y- 3.0) / b + 1.0) * x;
                y = a * y * y;
                if y > 0.002 { y = y.exp() - 1.0;
                } 
                else{ 
                y = 0.5 * y * y + y;
                };
            };
        (xn * y).sqrt()
        };
        return t
}
