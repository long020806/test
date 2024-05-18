pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[macro_export] 
macro_rules! vec_test { 
    ($elem:expr; $n:expr) => ( 
        $crate::vec::from_elem($elem, $n) 
    ); 
    // 匹配的为 (1,2,3) {1,2,3} [1,2,3] 序列
    // ($($x:expr),*) => ( 
    //     <[_]>::into_vec(box [$($x),*]) 
    // );
    // 匹配的为 (1,2,3,) {1,2,3,} [1,2,3,]
    // $($x),* 表示 将1,2,3, 转为 1,2,3序列 同时 满足上一匹配
    ($($x:expr,)*) => (vec![$($x),*]) 
}
#[macro_export] 
macro_rules! map { 
    ( $( $k:expr => $v:expr ),* ) => { 
        { 
            let mut map = ::std::collections::HashMap::new(); 
            $( 
                map.insert($k, $v); 
            )* 
            map 
        } 
    }; 
    ( $( $k:expr => $v:expr ,)* ) => { 
        { 
            let mut map = ::std::collections::HashMap::new(); 
            $( 
                map.insert($k, $v); 
            )* 
            map 
        } 
    }; 
} 

macro_rules! func_test {
    ( "vec" , $($x:expr),* ) => {
        vec![$($x),*]
    };
    ("map" ,  $( $k:expr => $v:expr ),*) =>{
        {
            let mut map = ::std::collections::HashMap::new(); 
            $( 
                map.insert($k, $v); 
            )* 
            map 
        } 
    }
}

#[macro_export] 
macro_rules! http_test { 
    ($url:tt GET => $code:expr) => { 
        let request = reqwest::get($url).unwrap(); 
        println!("Testing GET {} => {}", $url, $code); 
        assert_eq!(request.status().as_u16(), $code); 
    }; 
    ($url:tt POST => $code:expr, $($k:expr => $v:expr),*) => { 
        let params = [$(($k, $v),)*]; 
        let client = reqwest::Client::new(); 
        let res = client.post($url) 
            .form(&params) 
            .send().unwrap(); 
        println!("Testing POST {} => {}", $url, $code); 
        assert_eq!(res.status().as_u16(), $code); 
    }; 
} 
#[cfg(test)] 
mod tests { 
    #[test] 
    fn test_map_macro() { 
        let a = map! {
            "1" => 1, 
            "2" => 2
        };
        let b = map! {
            "1" => 1, 
            "2" => 2,
        };
        let c = vec![1;3];
        let d = func_test!("vec",1,2,3);
        let e = func_test!("map","1"=>1);

        http_test!("http://duckduckgo.com" GET => 200); 
        http_test!("http://httpbin.org/post" POST => 200, "hello" => "world", "foo" => "bar"); 

        assert_eq!(a["1"], 1); 
        assert_eq!(a["2"], 2); 
    } 
}