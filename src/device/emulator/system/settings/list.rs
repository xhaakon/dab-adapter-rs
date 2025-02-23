// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct SettingsListRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct OutputResolution{
// pub width: u32,
// pub height: u32,
// pub frequency: f32,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize)]
// pub enum MatchContentFrameRate{#[default]
//     EnabledAlways,
//     EnabledSeamlessOnly,
//     Disabled,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize)]
// pub enum HdrOutputMode{#[default]
//     AlwaysHdr,
//     HdrOnPlayback,
//     DisableHdr,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize)]
// pub enum PictureMode{#[default]
//     Standard,
//     Dynamic,
//     Movie,
//     Sports,
//     FilmMaker,
//     Game,
//     Auto,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize)]
// pub enum AudioOutputMode{#[default]
//     Stereo,
//     MultichannelPcm,
//     PassThrough,
//     Auto,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize)]
// pub enum AudioOutputSource{#[default]
//     NativeSpeaker,
//     Arc,
//     EArc,
//     Optical,
//     Aux,
//     Bluetooth,
//     Auto,
//     HDMI,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize)]
// pub enum VideoInputSource{#[default]
//     Tuner,
//     HDMI1,
//     HDMI2,
//     HDMI3,
//     HDMI4,
//     Composite,
//     Component,
//     Home,
//     Cast,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct AudioVolume{
// pub min: u32,
// pub max: u32,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct ListSystemSettings{
// pub language: Vec<String>,
// pub outputResolution: Vec<OutputResolution>,
// pub memc: bool,
// pub cec: bool,
// pub lowLatencyMode: bool,
// pub matchContentFrameRate: Vec<MatchContentFrameRate>,
// pub hdrOutputMode: Vec<HdrOutputMode>,
// pub pictureMode: Vec<PictureMode>,
// pub audioOutputMode: Vec<AudioOutputMode>,
// pub audioOutputSource: Vec<AudioOutputSource>,
// pub videoInputSource: Vec<VideoInputSource>,
// pub audioVolume: AudioVolume,
// pub mute: bool,
// pub textToSpeech: bool,
// }

#[allow(unused_imports)]
use crate::dab::structs::AudioVolume;
use crate::dab::structs::ListSystemSettings;
#[allow(unused_imports)]
use crate::dab::structs::OutputResolution;
#[allow(unused_imports)]
use crate::dab::structs::SettingsListRequest;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = ListSystemSettings::default();
    // *** Fill in the fields of the struct ListSystemSettings here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
