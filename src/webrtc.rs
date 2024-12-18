use std::sync::Arc;
use webrtc::api::APIBuilder;
use webrtc::data_channel::data_channel_init::RTCDataChannelInit;
use webrtc::data_channel::RTCDataChannel;
use webrtc::ice_transport::ice_candidate::RTCIceCandidateInit;
use webrtc::peer_connection::sdp::sdp_type::RTCSdpType;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::Error;

pub struct WebRTCService {
  pub peer_connection: RTCPeerConnection,
}

impl WebRTCService {
  /// Creates a new WebRTCService with a PeerConnection.
  pub async fn new() -> Result<Self, Error> {
    // Create a new WebRTC API instance
    let api = APIBuilder::new().build();
    let peer_connection = api.new_peer_connection(Default::default()).await?;
    Ok(WebRTCService { peer_connection })
  }

  /// Creates an SDP offer for the PeerConnection.
  pub async fn create_offer(&self) -> Result<String, Error> {
    let offer = self.peer_connection.create_offer(None).await?;
    self
      .peer_connection
      .set_local_description(offer.clone())
      .await?;
    Ok(offer.sdp)
  }

  /// Sets the remote SDP description.
  pub async fn set_remote_description(
    &self,
    sdp: String,
    sdp_type: RTCSdpType,
  ) -> Result<(), Error> {
    // Use the correct constructor method depending on the sdp_type
    let remote_description = match sdp_type {
      RTCSdpType::Offer => RTCSessionDescription::offer(sdp).map_err(|e| {
        Error::new(format!(
          "Failed to create RTCSessionDescription (Offer): {:?}",
          e
        ))
      })?,
      RTCSdpType::Answer => RTCSessionDescription::answer(sdp).map_err(|e| {
        Error::new(format!(
          "Failed to create RTCSessionDescription (Answer): {:?}",
          e
        ))
      })?,
      RTCSdpType::Pranswer => RTCSessionDescription::pranswer(sdp).map_err(|e| {
        Error::new(format!(
          "Failed to create RTCSessionDescription (PrAnswer): {:?}",
          e
        ))
      })?,
      _ => return Err(Error::new("Unsupported SDP type".to_string())),
    };

    // Set the remote description
    self
      .peer_connection
      .set_remote_description(remote_description)
      .await
  }

  /// Adds an ICE candidate to the PeerConnection.
  pub async fn handle_ice_candidate(&self, candidate: RTCIceCandidateInit) -> Result<(), Error> {
    let candidate_init = candidate;
    self
      .peer_connection
      .add_ice_candidate(candidate_init)
      .await?;

    Ok(()) // Return Ok(()) to match the expected Result type
  }

  /// Creates a data channel for the PeerConnection.
  pub async fn create_data_channel(&self, label: &str) -> Result<Arc<RTCDataChannel>, Error> {
    let data_channel_init = RTCDataChannelInit {
      ..Default::default()
    };
    self
      .peer_connection
      .create_data_channel(label, Some(data_channel_init))
      .await
  }
}
