mod downloader;

async fn async_main() {
    let mut dl = downloader::Downloader::new();
    let links = [
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/b2e0806148dad19dc8e6d9f9627dfbb1_MIT6_262S11_assn08_sol.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/89783a68df74118697ba4c21f03632f0_MIT6_262S11_assn09_sol.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/58bf52271ded6aab691d1d4732064d8f_MIT6_262S11_assn09.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/5fa1a2f7e494ff01df67e48e97042c57_MIT6_262S11_assn07.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/0902956aa08f67a954b28efdcbaaa472_MIT6_262S11_assn01.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/fd5dafd71de53dd56a51d9fac4e71f75_MIT6_262S11_assn06.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/f11101eaa72a3b7b4ee7636215d363b0_MIT6_262S11_assn08.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/1d1071239a9ca4c77e453bc477e58037_MIT6_262S11_assn12.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/ef4b4a04da62606c54dfe1600b5ef74a_MIT6_262S11_assn03.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/20d33d56935e00f746a40bca6442f81b_MIT6_262S11_assn04.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/53eae5402936a3f9c6251493ce35dfa5_MIT6_262S11_assn05.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/0c561f1e4128ac98a65bc42e289a266b_MIT6_262S11_assn02.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/c12643e48449ee92da0cba905e0ba5ca_MIT6_262S11_assn01_sol.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/3e1405bd1754b64a27ec0689e7804930_MIT6_262S11_assn02_sol.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/5a1f468c5e2da7aa0201a8d8e41dce51_MIT6_262S11_assn03_sol.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/8c50f40e1dbf6f198bd98a3b87158bc6_MIT6_262S11_assn07_sol.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/5ac94bbbf158ac3b6829d0a1ac7d8e28_MIT6_262S11_assn06_sol.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/62715a03a3efc0d8464d4af217565752_MIT6_262S11_assn12_sol.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/ee3a0ddf80bb3af54b73895625fb7375_MIT6_262S11_assn11.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/0e44be4e3d2d66eb8a9991606b996793_MIT6_262S11_assn04_sol.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/a256d3b584e71f207587481ab0ee328a_MIT6_262S11_assn10_sol.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/ddb3c7af4b7030d44634cc763dcb39ac_MIT6_262S11_assn11_sol.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/99a367788475e6d8ff0251b81d484661_MIT6_262S11_assn05_sol.pdf",
        "https://ocw.mit.edu/courses/6-262-discrete-stochastic-processes-spring-2011/b3d1124a522d208dfad2accb8e44ae19_MIT6_262S11_assn10.pdf",
    ];
    for link in links {
        use std::path::Path;
        let p = Path::new(link);
        dl.add(link, format!("{}", p.file_name().unwrap().to_string_lossy()));
    }
    dl.run(10).await;
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_io()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async_main());
}
