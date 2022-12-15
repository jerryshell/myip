use myip::*;

#[tokio::test]
async fn test_ip() {
    let (status_code, json_value) = ip("114.114.114.114").await;
    assert_eq!(status_code.as_u16(), 200);
    assert_eq!(
        json_value.get("country").unwrap().as_str().unwrap(),
        "China"
    );
}
