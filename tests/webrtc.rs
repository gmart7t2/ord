#[tokio::test]
async fn test_webrtc_offer_creation() {
    let webrtc_service = WebRTCService::new();
    let offer = webrtc_service.create_offer();
    assert!(!offer.is_empty(), "Offer should not be empty");
}
