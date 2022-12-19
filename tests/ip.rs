use myip::*;

#[tokio::test]
async fn test_ip() {
    let ip_info_map = ip("114.114.114.114").await.unwrap();
    assert_eq!(
        ip_info_map.get("country").unwrap().as_str().unwrap(),
        "China"
    );
}
