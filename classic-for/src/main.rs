macro_rules! cfor {
    (($stmt:stmt; $cond:expr; $counter:ident++) $code:block) => {
        $stmt
        while $cond {
            $code;
            $counter+=1;
        }
    };

    (($stmt:stmt; $cond:expr; $counter:ident--) $code:block) => {
        $stmt
        while $cond {
            $code;
            $counter-=1;
        }
    };

    (($stmt:stmt; $cond:expr; $incr:stmt) $code:block) => {
        $stmt
        while $cond {
            $code;
            $incr
        }
    };

}

fn main() {
    cfor!{

        (let mut a = 1; a < 10; a++) {
            println!("{}", a);
        }

    }
}
