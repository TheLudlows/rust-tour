use tokio::fs;


#[tokio::test]
async fn test_async() {
    let r = fs::read_to_string("./Cargo.yaml").await;
    println!("{:?}", r);
    
}


