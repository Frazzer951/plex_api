use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    #[serde(rename = "MediaContainer")]
    pub media_container: MediaContainer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaContainer {
    pub size: i32,
    #[serde(rename = "allowCameraUpload")]
    pub allow_camera_upload: bool,
    #[serde(rename = "allowChannelAccess")]
    pub allow_channel_access: bool,
    #[serde(rename = "allowMediaDeletion")]
    pub allow_media_deletion: bool,
    #[serde(rename = "allowSharing")]
    pub allow_sharing: bool,
    #[serde(rename = "allowSync")]
    pub allow_sync: bool,
    #[serde(rename = "allowTuners")]
    pub allow_tuners: bool,
    #[serde(rename = "backgroundProcessing")]
    pub background_processing: bool,
    pub certificate: bool,
    #[serde(rename = "companionProxy")]
    pub companion_proxy: bool,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    pub diagnostics: String,
    #[serde(rename = "eventStream")]
    pub event_stream: bool,
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    #[serde(rename = "hubSearch")]
    pub hub_search: bool,
    #[serde(rename = "itemClusters")]
    pub item_clusters: bool,
    pub livetv: u32,
    #[serde(rename = "machineIdentifier")]
    pub machine_identifier: String,
    #[serde(rename = "mediaProviders")]
    pub media_providers: bool,
    pub multiuser: bool,
    #[serde(rename = "musicAnalysis")]
    pub music_analysis: u32,
    #[serde(rename = "myPlex")]
    pub my_plex: bool,
    #[serde(rename = "myPlexMappingState")]
    pub my_plex_mapping_state: String,
    #[serde(rename = "myPlexSigninState")]
    pub my_plex_signin_state: String,
    #[serde(rename = "myPlexSubscription")]
    pub my_plex_subscription: bool,
    #[serde(rename = "myPlexUsername")]
    pub my_plex_username: String,
    #[serde(rename = "offlineTranscode")]
    pub offline_transcode: u32,
    #[serde(rename = "ownerFeatures")]
    pub owner_features: String,
    #[serde(rename = "photoAutoTag")]
    pub photo_autotag: bool,
    pub platform: String,
    #[serde(rename = "platformVersion")]
    pub platform_version: String,
    #[serde(rename = "pluginHost")]
    pub plugin_host: bool,
    #[serde(rename = "pushNotifications")]
    pub push_notifications: bool,
    #[serde(rename = "readOnlyLibraries")]
    pub read_only_libraries: bool,
    #[serde(rename = "streamingBrainABRVersion")]
    pub streaming_brain_abr_version: u32,
    #[serde(rename = "streamingBrainVersion")]
    pub streaming_brain_version: u32,
    pub sync: bool,
    #[serde(rename = "transcoderActiveVideoSessions")]
    pub transcoder_active_video_sessions: u32,
    #[serde(rename = "transcoderAudio")]
    pub transcoder_audio: bool,
    #[serde(rename = "transcoderLyrics")]
    pub transcoder_lyrics: bool,
    #[serde(rename = "transcoderPhoto")]
    pub transcoder_photo: bool,
    #[serde(rename = "transcoderSubtitles")]
    pub transcoder_subtitles: bool,
    #[serde(rename = "transcoderVideo")]
    pub transcoder_video: bool,
    #[serde(rename = "transcoderVideoBitrates")]
    pub transcoder_video_bitrates: String,
    #[serde(rename = "transcoderVideoQualities")]
    pub transcoder_video_qualities: String,
    #[serde(rename = "transcoderVideoResolutions")]
    pub transcoder_video_resolutions: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: u32,
    pub updater: bool,
    pub version: String,
    #[serde(rename = "voiceSearch")]
    pub voice_search: bool,
    #[serde(rename = "Directory")]
    pub directory: Vec<Keys>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keys {
    pub count: u32,
    pub key: String,
    pub title: String,
}
