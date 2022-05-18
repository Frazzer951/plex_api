use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    #[serde(rename = "MediaContainer")]
    pub media_container: MediaContainer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaContainer {
    pub size: i32,
    #[serde(rename = "Directory")]
    pub directory: Vec<Keys>,
    pub allowCameraUpload: bool,
    pub allowChannelAccess: bool,
    pub allowMediaDeletion: bool,
    pub allowSharing: bool,
    pub allowSync: bool,
    pub allowTuners: bool,
    pub backgroundProcessing: bool,
    pub certificate: bool,
    pub companionProxy: bool,
    pub countryCode: String,
    pub diagnostics: String,
    pub eventStream: bool,
    pub friendlyName: String,
    pub hubSearch: bool,
    pub itemClusters: bool,
    pub livetv: u32,
    pub machineIdentifier: String,
    pub mediaProviders: bool,
    pub multiuser: bool,
    pub musicAnalysis: u32,
    pub myPlex: bool,
    pub myPlexMappingState: String,
    pub myPlexSigninState: String,
    pub myPlexSubscription: bool,
    pub myPlexUsername: String,
    pub offlineTranscode: u32,
    pub ownerFeatures: String,
    pub photoAutoTag: bool,
    pub platform: String,
    pub platformVersion: String,
    pub pluginHost: bool,
    pub pushNotifications: bool,
    pub readOnlyLibraries: bool,
    pub streamingBrainABRVersion: u32,
    pub streamingBrainVersion: u32,
    pub sync: bool,
    pub transcoderActiveVideoSessions: u32,
    pub transcoderAudio: bool,
    pub transcoderLyrics: bool,
    pub transcoderPhoto: bool,
    pub transcoderSubtitles: bool,
    pub transcoderVideo: bool,
    pub transcoderVideoBitrates: String,
    pub transcoderVideoQualities: String,
    pub transcoderVideoResolutions: String,
    pub updatedAt: u32,
    pub updater: bool,
    pub version: String,
    pub voiceSearch: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keys {
    pub count: u32,
    pub key: String,
    pub title: String,
}
