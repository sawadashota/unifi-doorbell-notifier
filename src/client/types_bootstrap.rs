use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapResponse {
    // pub auth_user_id: String,
    // pub access_key: String,
    pub cameras: Vec<Camera>,
    // pub users: Vec<User>,
    // pub groups: Vec<Group>,
    // pub liveviews: Vec<Liveview>,
    // pub nvr: Nvr,
    // #[serde(rename = "legacyUFVs")]
    // pub legacy_ufvs: Vec<Value>,
    // pub last_update_id: String,
    // pub viewers: Vec<Value>,
    // pub displays: Vec<Value>,
    // pub lights: Vec<Value>,
    // pub bridges: Vec<Value>,
    // pub sensors: Vec<Value>,
    // pub doorlocks: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    pub is_deleting: bool,
    pub mac: String,
    pub host: String,
    pub connection_host: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub up_since: i64,
    pub uptime: i64,
    pub last_seen: i64,
    pub connected_since: i64,
    pub state: String,
    pub hardware_revision: String,
    pub firmware_version: String,
    pub latest_firmware_version: String,
    pub firmware_build: String,
    pub is_updating: bool,
    pub is_adopting: bool,
    pub is_adopted: bool,
    pub is_adopted_by_other: bool,
    pub is_provisioned: bool,
    pub is_rebooting: bool,
    pub is_ssh_enabled: bool,
    pub can_adopt: bool,
    pub is_attempting_to_connect: bool,
    pub last_motion: i64,
    pub mic_volume: i64,
    pub is_mic_enabled: bool,
    pub is_recording: bool,
    pub is_motion_detected: bool,
    pub phy_rate: i64,
    pub hdr_mode: bool,
    pub video_mode: String,
    pub is_probing_for_wifi: bool,
    pub ap_mac: Value,
    pub ap_rssi: Value,
    pub element_info: Value,
    pub chime_duration: i64,
    pub is_dark: bool,
    pub last_privacy_zone_position_id: Value,
    pub last_ring: Option<i64>,
    pub is_live_heatmap_enabled: bool,
    pub anonymous_device_id: String,
    pub event_stats: EventStats,
    pub wired_connection_state: WiredConnectionState,
    pub channels: Vec<Channel>,
    pub isp_settings: IspSettings,
    pub talkback_settings: TalkbackSettings,
    pub osd_settings: OsdSettings,
    pub led_settings: LedSettings,
    pub speaker_settings: SpeakerSettings,
    pub recording_settings: RecordingSettings,
    pub smart_detect_settings: SmartDetectSettings,
    pub recording_schedule: Value,
    pub motion_zones: Vec<MotionZone>,
    pub privacy_zones: Vec<Value>,
    pub smart_detect_zones: Vec<SmartDetectZone>,
    pub smart_detect_lines: Vec<Value>,
    pub stats: Stats,
    pub feature_flags: FeatureFlags,
    pub pir_settings: PirSettings,
    pub lcd_message: LcdMessage,
    pub wifi_connection_state: WifiConnectionState,
    pub id: String,
    pub is_connected: bool,
    pub platform: String,
    pub has_speaker: bool,
    pub has_wifi: bool,
    pub audio_bitrate: i64,
    pub can_manage: bool,
    pub is_managed: bool,
    pub model_key: String,
}

impl Camera {
    pub fn is_doorbell(&self) -> bool {
        self.is_managed && self.type_field == "UVC G4 Doorbell"
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventStats {
    pub motion: Motion,
    pub smart: Smart,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Motion {
    pub today: i64,
    pub average: i64,
    pub last_days: Vec<i64>,
    pub recent_hours: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Smart {
    pub today: i64,
    pub average: i64,
    pub last_days: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WiredConnectionState {
    pub phy_rate: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: i64,
    pub video_id: String,
    pub name: String,
    pub enabled: bool,
    pub is_rtsp_enabled: bool,
    pub rtsp_alias: Value,
    pub width: i64,
    pub height: i64,
    pub fps: i64,
    pub bitrate: i64,
    pub min_bitrate: i64,
    pub max_bitrate: i64,
    pub min_client_adaptive_bit_rate: i64,
    pub min_motion_adaptive_bit_rate: i64,
    pub fps_values: Vec<i64>,
    pub idr_interval: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IspSettings {
    pub ae_mode: String,
    pub ir_led_mode: String,
    pub ir_led_level: i64,
    pub wdr: i64,
    pub icr_sensitivity: i64,
    pub brightness: i64,
    pub contrast: i64,
    pub hue: i64,
    pub saturation: i64,
    pub sharpness: i64,
    pub denoise: i64,
    pub is_flipped_vertical: bool,
    pub is_flipped_horizontal: bool,
    pub is_auto_rotate_enabled: bool,
    pub is_ldc_enabled: bool,
    pub is3dnr_enabled: bool,
    pub is_external_ir_enabled: bool,
    pub is_aggressive_anti_flicker_enabled: bool,
    pub is_pause_motion_enabled: bool,
    pub d_zoom_center_x: i64,
    pub d_zoom_center_y: i64,
    pub d_zoom_scale: i64,
    pub d_zoom_stream_id: i64,
    pub focus_mode: String,
    pub focus_position: i64,
    pub touch_focus_x: i64,
    pub touch_focus_y: i64,
    pub zoom_position: i64,
    pub mount_position: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TalkbackSettings {
    pub type_fmt: String,
    pub type_in: String,
    pub bind_addr: String,
    pub bind_port: i64,
    pub filter_addr: Option<String>,
    pub filter_port: Option<i64>,
    pub channels: i64,
    pub sampling_rate: i64,
    pub bits_per_sample: i64,
    pub quality: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsdSettings {
    pub is_name_enabled: bool,
    pub is_date_enabled: bool,
    pub is_logo_enabled: bool,
    pub is_debug_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LedSettings {
    pub is_enabled: bool,
    pub blink_rate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerSettings {
    pub is_enabled: bool,
    pub are_system_sounds_enabled: bool,
    pub volume: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingSettings {
    pub pre_padding_secs: i64,
    pub post_padding_secs: i64,
    pub min_motion_event_trigger: i64,
    pub end_motion_event_delay: i64,
    pub suppress_illumination_surge: bool,
    pub mode: String,
    pub geofencing: String,
    pub motion_algorithm: String,
    pub enable_pir_timelapse: bool,
    pub use_new_motion_algorithm: bool,
    pub retention_duration_ms: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartDetectSettings {
    pub object_types: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MotionZone {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub points: Vec<Vec<i64>>,
    pub sensitivity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartDetectZone {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub points: Vec<Vec<i64>>,
    pub sensitivity: i64,
    pub object_types: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub rx_bytes: i64,
    pub tx_bytes: i64,
    pub wifi: Wifi,
    pub battery: Battery,
    pub video: Video,
    pub storage: Storage,
    pub wifi_quality: i64,
    pub wifi_strength: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wifi {
    pub channel: Option<i64>,
    pub frequency: Option<i64>,
    pub link_speed_mbps: Value,
    pub signal_quality: i64,
    pub signal_strength: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
    pub percentage: Value,
    pub is_charging: bool,
    pub sleep_state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub recording_start: i64,
    pub recording_end: i64,
    #[serde(rename = "recordingStartLQ")]
    pub recording_start_lq: i64,
    #[serde(rename = "recordingEndLQ")]
    pub recording_end_lq: i64,
    pub timelapse_start: i64,
    pub timelapse_end: i64,
    #[serde(rename = "timelapseStartLQ")]
    pub timelapse_start_lq: i64,
    #[serde(rename = "timelapseEndLQ")]
    pub timelapse_end_lq: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Storage {
    pub used: i64,
    pub rate: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureFlags {
    pub can_adjust_ir_led_level: bool,
    pub can_magic_zoom: bool,
    pub can_optical_zoom: bool,
    pub can_touch_focus: bool,
    pub has_accelerometer: bool,
    pub has_aec: bool,
    pub has_battery: bool,
    pub has_bluetooth: bool,
    pub has_chime: bool,
    pub has_external_ir: bool,
    pub has_icr_sensitivity: bool,
    pub has_ldc: bool,
    pub has_led_ir: bool,
    pub has_led_status: bool,
    pub has_line_in: bool,
    pub has_mic: bool,
    pub has_privacy_mask: bool,
    pub has_rtc: bool,
    pub has_sd_card: bool,
    pub has_speaker: bool,
    pub has_wifi: bool,
    pub has_hdr: bool,
    #[serde(rename = "hasAutoICROnly")]
    pub has_auto_icronly: bool,
    pub video_modes: Vec<String>,
    pub video_mode_max_fps: Vec<i64>,
    pub has_motion_zones: bool,
    pub has_lcd_screen: bool,
    pub mount_positions: Vec<Value>,
    pub smart_detect_types: Vec<String>,
    pub motion_algorithms: Vec<String>,
    pub has_square_event_thumbnail: bool,
    pub privacy_mask_capability: PrivacyMaskCapability,
    pub focus: Focus,
    pub pan: Pan,
    pub tilt: Tilt,
    pub zoom: Zoom,
    pub has_smart_detect: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyMaskCapability {
    pub max_masks: i64,
    pub rectangle_only: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Focus {
    pub steps: Steps,
    pub degrees: Degrees,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Steps {
    pub max: Value,
    pub min: Value,
    pub step: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Degrees {
    pub max: Value,
    pub min: Value,
    pub step: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pan {
    pub steps: Steps2,
    pub degrees: Degrees2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Steps2 {
    pub max: Value,
    pub min: Value,
    pub step: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Degrees2 {
    pub max: Value,
    pub min: Value,
    pub step: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tilt {
    pub steps: Steps3,
    pub degrees: Degrees3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Steps3 {
    pub max: Value,
    pub min: Value,
    pub step: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Degrees3 {
    pub max: Value,
    pub min: Value,
    pub step: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zoom {
    pub steps: Steps4,
    pub degrees: Degrees4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Steps4 {
    pub max: Value,
    pub min: Value,
    pub step: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Degrees4 {
    pub max: Value,
    pub min: Value,
    pub step: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PirSettings {
    pub pir_sensitivity: i64,
    pub pir_motion_clip_length: i64,
    pub timelapse_frame_interval: i64,
    pub timelapse_transfer_interval: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdMessage {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiConnectionState {
    pub channel: Option<i64>,
    pub frequency: Option<i64>,
    pub phy_rate: Option<i64>,
    pub signal_quality: Option<i64>,
    pub signal_strength: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub permissions: Vec<String>,
    pub last_login_ip: Option<String>,
    pub last_login_time: Option<i64>,
    pub is_owner: bool,
    pub enable_notifications: bool,
    pub settings: Option<Settings>,
    pub groups: Vec<String>,
    pub location: Option<Location>,
    pub alert_rules: Vec<AlertRule>,
    pub id: String,
    pub has_accepted_invite: bool,
    pub all_permissions: Vec<String>,
    pub cloud_account: Option<CloudAccount>,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub local_username: String,
    pub model_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub flags: Option<Flags>,
    #[serde(default)]
    pub camera_order: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flags {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub is_away: bool,
    pub latitude: Value,
    pub longitude: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertRule {
    pub id: String,
    pub name: String,
    pub when: String,
    pub schedule: Schedule,
    pub system: System,
    pub cameras: Vec<Camera2>,
    pub users: Vec<Value>,
    pub geofencing: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub items: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    #[serde(default)]
    pub connect_disconnect: Vec<Value>,
    #[serde(default)]
    pub update: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera2 {
    #[serde(default)]
    pub ring: Vec<String>,
    pub camera: Option<String>,
    #[serde(default)]
    pub connect_disconnect: Vec<String>,
    #[serde(default)]
    pub motion: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudAccount {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub profile_img: Option<String>,
    pub user: String,
    pub id: String,
    pub cloud_id: String,
    pub name: String,
    pub model_key: String,
    pub location: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub name: String,
    pub permissions: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub is_default: bool,
    pub id: String,
    pub model_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Liveview {
    pub name: String,
    pub is_default: bool,
    pub is_global: bool,
    pub layout: i64,
    pub slots: Vec<Slot>,
    pub owner: String,
    pub id: String,
    pub model_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    pub cameras: Vec<String>,
    pub cycle_mode: String,
    pub cycle_interval: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nvr {
    pub mac: String,
    pub host: String,
    pub name: String,
    pub can_auto_update: bool,
    pub is_stats_gathering_enabled: bool,
    pub timezone: String,
    pub version: String,
    pub ucore_version: String,
    pub firmware_version: String,
    pub ui_version: Value,
    pub hardware_platform: String,
    pub ports: Ports,
    pub uptime: i64,
    pub last_seen: i64,
    pub is_updating: bool,
    pub last_update_at: i64,
    pub is_station: bool,
    pub enable_automatic_backups: bool,
    pub enable_stats_reporting: bool,
    pub is_ssh_enabled: bool,
    pub error_code: Value,
    pub release_channel: String,
    pub hosts: Vec<String>,
    pub enable_bridge_auto_adoption: bool,
    pub hardware_id: String,
    pub hardware_revision: String,
    pub host_type: i64,
    pub host_shortname: String,
    pub is_hardware: bool,
    pub is_wireless_uplink_enabled: bool,
    pub time_format: String,
    pub temperature_unit: String,
    pub recording_retention_duration_ms: Value,
    pub enable_crash_reporting: bool,
    pub disable_audio: Value,
    pub analytics_data: String,
    pub anonymous_device_id: String,
    pub camera_utilization: i64,
    pub is_recycling: bool,
    pub avg_motions: Vec<f64>,
    pub disable_auto_link: bool,
    pub wifi_settings: WifiSettings,
    pub location_settings: LocationSettings,
    pub feature_flags: FeatureFlags2,
    pub system_info: SystemInfo,
    pub doorbell_settings: DoorbellSettings,
    pub smart_detect_agreement: SmartDetectAgreement,
    pub storage_stats: StorageStats,
    pub id: String,
    pub is_away: bool,
    pub is_setup: bool,
    pub network: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub up_since: i64,
    pub is_recording_disabled: bool,
    pub is_recording_motion_only: bool,
    pub max_camera_capacity: MaxCameraCapacity,
    pub model_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ports {
    pub ump: i64,
    pub http: i64,
    pub https: i64,
    pub rtsp: i64,
    pub rtsps: i64,
    pub rtmp: i64,
    pub devices_wss: i64,
    pub camera_https: i64,
    pub camera_tcp: i64,
    pub live_ws: i64,
    pub live_wss: i64,
    pub tcp_streams: i64,
    pub playback: i64,
    #[serde(rename = "emsCLI")]
    pub ems_cli: i64,
    #[serde(rename = "emsLiveFLV")]
    pub ems_live_flv: i64,
    pub camera_events: i64,
    pub tcp_bridge: i64,
    pub ucore: i64,
    pub discovery_client: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiSettings {
    pub use_third_party_wifi: bool,
    pub ssid: Value,
    pub password: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationSettings {
    pub is_away: bool,
    pub is_geofencing_enabled: bool,
    pub latitude: Value,
    pub longitude: Value,
    pub radius: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureFlags2 {
    pub beta: bool,
    pub dev: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub cpu: Cpu,
    pub memory: Memory,
    pub storage: Storage2,
    pub tmpfs: Tmpfs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cpu {
    pub average_load: f64,
    pub temperature: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    pub available: i64,
    pub free: i64,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Storage2 {
    pub available: i64,
    pub is_recycling: bool,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub used: i64,
    pub devices: Vec<Device>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub model: String,
    pub size: i64,
    pub healthy: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tmpfs {
    pub available: i64,
    pub total: i64,
    pub used: i64,
    pub path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoorbellSettings {
    pub default_message_text: String,
    pub default_message_reset_timeout_ms: i64,
    pub custom_messages: Vec<String>,
    pub all_messages: Vec<AllMessage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartDetectAgreement {
    pub status: String,
    pub last_update_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageStats {
    pub utilization: f64,
    pub capacity: i64,
    pub remaining_capacity: i64,
    pub recording_space: RecordingSpace,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingSpace {
    pub total: i64,
    pub used: i64,
    pub available: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxCameraCapacity {
    #[serde(rename = "4K")]
    pub n4k: i64,
    #[serde(rename = "HD")]
    pub hd: i64,
}
