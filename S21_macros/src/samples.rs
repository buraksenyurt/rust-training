/*
   Bu macro blok olarak girilen kodun çalışma zamanını ölçer.
*/
macro_rules! wt {
    ($block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        println!("Total execution time: {:?}", duration);
        result
    }};
}

/*
   Aşağıdaki macro ise { } bloğu içerisindeki veri modelini alıp XML şekline dönüştürür.
*/
macro_rules! xml {
    ($tag:ident { $($key:ident : $value:expr),* }) => {
        format!(
            "<{tag} {attributes} />",
            tag = stringify!($tag),
            attributes = vec![$(format!("{}=\"{}\"", stringify!($key), $value)),*].join(" ")
        )
    };
}

// cargo test -- --nocapture
#[cfg(test)]
mod sample_tests {
    #[test]
    fn wt_test() {
        let sum = wt!({
            let mut total = 0;
            for i in 1..100 {
                total += i;
            }
            total
        });
        assert_eq!(sum, 4950);
    }

    #[test]
    fn xml_test() {
        let data = xml! {
            game {
                id:1,
                title:"Pacman 1983",
                rate:9.6
            }
        };
        assert_eq!(
            data,
            "<game id=\"1\" title=\"Pacman 1983\" rate=\"9.6\" />".to_string()
        );
    }
}
