use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An object of this type can be returned on every function call, in case of an error"]
pub struct Error {
    #[doc = "Error code; subject to future changes. If the error code is 406, the error message must not be processed in any way and must not be displayed to the user"]
    pub code: i32,
    #[doc = "Error message; subject to future changes"]
    pub message: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An object of this type is returned on a successful function call for certain functions"]
pub struct Ok {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains parameters for TDLib initialization"]
pub struct TdlibParameters {
    #[doc = "If set to true, the Telegram test environment will be used instead of the production environment"]
    #[serde(default)]
    pub use_test_dc: bool,
    #[doc = "The path to the directory for the persistent database; if empty, the current working directory will be used"]
    pub database_directory: String,
    #[doc = "The path to the directory for storing files; if empty, database_directory will be used"]
    pub files_directory: String,
    #[doc = "If set to true, information about downloaded and uploaded files will be saved between application restarts"]
    #[serde(default)]
    pub use_file_database: bool,
    #[doc = "If set to true, the library will maintain a cache of users, basic groups, supergroups, channels and secret chats. Implies use_file_database"]
    #[serde(default)]
    pub use_chat_info_database: bool,
    #[doc = "If set to true, the library will maintain a cache of chats and messages. Implies use_chat_info_database"]
    #[serde(default)]
    pub use_message_database: bool,
    #[doc = "If set to true, support for secret chats will be enabled"]
    #[serde(default)]
    pub use_secret_chats: bool,
    #[doc = "Application identifier for Telegram API access, which can be obtained at https://my.telegram.org"]
    pub api_id: i32,
    #[doc = "Application identifier hash for Telegram API access, which can be obtained at https://my.telegram.org"]
    pub api_hash: String,
    #[doc = "IETF language tag of the user's operating system language; must be non-empty"]
    pub system_language_code: String,
    #[doc = "Model of the device the application is being run on; must be non-empty"]
    pub device_model: String,
    #[doc = "Version of the operating system the application is being run on; must be non-empty"]
    pub system_version: String,
    #[doc = "Application version; must be non-empty"]
    pub application_version: String,
    #[doc = "If set to true, old files will automatically be deleted"]
    #[serde(default)]
    pub enable_storage_optimizer: bool,
    #[doc = "If set to true, original file names will be ignored. Otherwise, downloaded files will be saved under names as close as possible to the original name"]
    #[serde(default)]
    pub ignore_file_names: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An authentication code is delivered via a private Telegram message, which can be viewed in another client "]
pub struct AuthenticationCodeTypeTelegramMessage {
    #[doc = "Length of the code"]
    pub length: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An authentication code is delivered via an SMS message to the specified phone number "]
pub struct AuthenticationCodeTypeSms {
    #[doc = "Length of the code"]
    pub length: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An authentication code is delivered via a phone call to the specified phone number "]
pub struct AuthenticationCodeTypeCall {
    #[doc = "Length of the code"]
    pub length: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An authentication code is delivered by an immediately cancelled call to the specified phone number. The number from which the call was made is the code "]
pub struct AuthenticationCodeTypeFlashCall {
    #[doc = "Pattern of the phone number from which the call will be made"]
    pub pattern: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Information about the authentication code that was sent "]
pub struct AuthenticationCodeInfo {
    #[doc = "A phone number that is being authenticated "]
    pub phone_number: String,
    #[serde(rename = "type")]
    #[doc = "Describes the way the code was sent to the user "]
    pub type_: AuthenticationCodeType,
    #[doc = "Describes the way the next code will be sent to the user; may be null "]
    pub next_type: Option<AuthenticationCodeType>,
    #[doc = "Timeout before the code should be re-sent, in seconds"]
    pub timeout: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Information about the email address authentication code that was sent "]
pub struct EmailAddressAuthenticationCodeInfo {
    #[doc = "Pattern of the email address to which an authentication code was sent "]
    pub email_address_pattern: String,
    #[doc = "Length of the code; 0 if unknown"]
    pub length: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a part of the text that needs to be formatted in some unusual way "]
pub struct TextEntity {
    #[doc = "Offset of the entity in UTF-16 code units "]
    pub offset: i32,
    #[doc = "Length of the entity, in UTF-16 code units "]
    pub length: i32,
    #[serde(rename = "type")]
    #[doc = "Type of the entity"]
    pub type_: TextEntityType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of text entities "]
pub struct TextEntities {
    #[doc = "List of text entities"]
    pub entities: Vec<TextEntity>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A text with some entities "]
pub struct FormattedText {
    #[doc = "The text "]
    pub text: String,
    #[doc = "Entities contained in the text. Entities can be nested, but must not mutually intersect with each other.\n Pre, Code and PreCode entities can't contain other entities. Bold, Italic, Underline and Strikethrough entities can contain and to be contained in all other entities. All other entities can't contain each other"]
    pub entities: Vec<TextEntity>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains Telegram terms of service "]
pub struct TermsOfService {
    #[doc = "Text of the terms of service "]
    pub text: FormattedText,
    #[doc = "The minimum age of a user to be able to accept the terms; 0 if any "]
    pub min_user_age: i32,
    #[doc = "True, if a blocking popup with terms of service must be shown to the user"]
    #[serde(default)]
    pub show_popup: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "TDLib needs TdlibParameters for initialization"]
pub struct AuthorizationStateWaitTdlibParameters {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "TDLib needs an encryption key to decrypt the local database "]
pub struct AuthorizationStateWaitEncryptionKey {
    #[doc = "True, if the database is currently encrypted"]
    #[serde(default)]
    pub is_encrypted: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "TDLib needs the user's phone number to authorize. Call `setAuthenticationPhoneNumber` to provide the phone number, or use `requestQrCodeAuthentication`, or `checkAuthenticationBotToken` for other authentication options"]
pub struct AuthorizationStateWaitPhoneNumber {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "TDLib needs the user's authentication code to authorize "]
pub struct AuthorizationStateWaitCode {
    #[doc = "Information about the authorization code that was sent"]
    pub code_info: AuthenticationCodeInfo,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link "]
pub struct AuthorizationStateWaitOtherDeviceConfirmation {
    #[doc = "A tg:// URL for the QR code. The link will be updated frequently"]
    pub link: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is unregistered and need to accept terms of service and enter their first name and last name to finish registration "]
pub struct AuthorizationStateWaitRegistration {
    #[doc = "Telegram terms of service"]
    pub terms_of_service: TermsOfService,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user has been authorized, but needs to enter a password to start using the application "]
pub struct AuthorizationStateWaitPassword {
    #[doc = "Hint for the password; may be empty "]
    pub password_hint: String,
    #[doc = "True, if a recovery email address has been set up"]
    #[serde(default)]
    pub has_recovery_email_address: bool,
    #[doc = "Pattern of the email address to which the recovery email was sent; empty until a recovery email has been sent"]
    pub recovery_email_address_pattern: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user has been successfully authorized. TDLib is now ready to answer queries"]
pub struct AuthorizationStateReady {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is currently logging out"]
pub struct AuthorizationStateLoggingOut {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received"]
pub struct AuthorizationStateClosing {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to\n with error code 500. To continue working, one should create a new instance of the TDLib client"]
pub struct AuthorizationStateClosed {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents the current state of 2-step verification "]
pub struct PasswordState {
    #[doc = "True, if a 2-step verification password is set "]
    #[serde(default)]
    pub has_password: bool,
    #[doc = "Hint for the password; may be empty"]
    pub password_hint: String,
    #[doc = "True, if a recovery email is set "]
    #[serde(default)]
    pub has_recovery_email_address: bool,
    #[doc = "True, if some Telegram Passport elements were saved"]
    #[serde(default)]
    pub has_passport_data: bool,
    #[doc = "Information about the recovery email address to which the confirmation email was sent; may be null"]
    pub recovery_email_address_code_info: Option<EmailAddressAuthenticationCodeInfo>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about the current recovery email address "]
pub struct RecoveryEmailAddress {
    #[doc = "Recovery email address"]
    pub recovery_email_address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about the availability of a temporary password, which can be used for payments "]
pub struct TemporaryPasswordState {
    #[doc = "True, if a temporary password is available "]
    #[serde(default)]
    pub has_password: bool,
    #[doc = "Time left before the temporary password expires, in seconds"]
    pub valid_for: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a local file"]
pub struct LocalFile {
    #[doc = "Local path to the locally available file part; may be empty"]
    pub path: String,
    #[doc = "True, if it is possible to try to download or generate the file"]
    #[serde(default)]
    pub can_be_downloaded: bool,
    #[doc = "True, if the file can be deleted"]
    #[serde(default)]
    pub can_be_deleted: bool,
    #[doc = "True, if the file is currently being downloaded (or a local copy is being generated by some other means)"]
    #[serde(default)]
    pub is_downloading_active: bool,
    #[doc = "True, if the local copy is fully available"]
    #[serde(default)]
    pub is_downloading_completed: bool,
    #[doc = "Download will be started from this offset. downloaded_prefix_size is calculated from this offset"]
    pub download_offset: i32,
    #[doc = "If is_downloading_completed is false, then only some prefix of the file starting from download_offset is ready to be read. downloaded_prefix_size is the size of that prefix"]
    pub downloaded_prefix_size: i32,
    #[doc = "Total downloaded file bytes. Should be used only for calculating download progress. The actual file size may be bigger, and some parts of it may contain garbage"]
    pub downloaded_size: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a remote file"]
pub struct RemoteFile {
    #[doc = "Remote file identifier; may be empty. Can be used by the current user across application restarts or even from other devices. Uniquely identifies a file, but a file can have a lot of different valid identifiers.\n If the ID starts with \"http://\" or \"https://\", it represents the HTTP URL of the file. TDLib is currently unable to download files if only their URL is known.\n If downloadFile is called on such a file or if it is sent to a secret chat, TDLib starts a file generation process by sending updateFileGenerationStart to the client with the HTTP URL in the original_path and \"#url#\" as the conversion string. Clients should generate the file by downloading it to the specified location"]
    pub id: String,
    #[doc = "Unique file identifier; may be empty if unknown. The unique file identifier which is the same for the same file even for different users and is persistent over time"]
    pub unique_id: String,
    #[doc = "True, if the file is currently being uploaded (or a remote copy is being generated by some other means)"]
    #[serde(default)]
    pub is_uploading_active: bool,
    #[doc = "True, if a remote copy is fully available"]
    #[serde(default)]
    pub is_uploading_completed: bool,
    #[doc = "Size of the remote available part of the file; 0 if unknown"]
    pub uploaded_size: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a file"]
pub struct File {
    #[doc = "Unique file identifier"]
    pub id: i32,
    #[doc = "File size; 0 if unknown"]
    pub size: i32,
    #[doc = "Expected file size in case the exact file size is unknown, but an approximate size is known. Can be used to show download/upload progress"]
    pub expected_size: i32,
    #[doc = "Information about the local copy of the file"]
    pub local: LocalFile,
    #[doc = "Information about the remote copy of the file"]
    pub remote: RemoteFile,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A file defined by its unique ID "]
pub struct InputFileId {
    #[doc = "Unique file identifier"]
    pub id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A file defined by its remote ID. The remote ID is guaranteed to be usable only if the corresponding file is still accessible to the user and known to TDLib.\n For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the client"]
pub struct InputFileRemote {
    #[doc = "Remote file identifier"]
    pub id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A file defined by a local path "]
pub struct InputFileLocal {
    #[doc = "Local path to the file"]
    pub path: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A file generated by the client "]
pub struct InputFileGenerated {
    #[doc = "Local path to a file from which the file is generated; may be empty if there is no such file"]
    pub original_path: String,
    #[doc = "String specifying the conversion applied to the original file; should be persistent across application restarts. Conversions beginning with '#' are reserved for internal TDLib usage"]
    pub conversion: String,
    #[doc = "Expected size of the generated file; 0 if unknown"]
    pub expected_size: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Photo description "]
pub struct PhotoSize {
    #[serde(rename = "type")]
    #[doc = "Thumbnail type (see https://core.telegram.org/constructor/photoSize) "]
    pub type_: String,
    #[doc = "Information about the photo file "]
    pub photo: File,
    #[doc = "Photo width "]
    pub width: i32,
    #[doc = "Photo height"]
    pub height: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Thumbnail image of a very poor quality and low resolution "]
pub struct Minithumbnail {
    #[doc = "Thumbnail width, usually doesn't exceed 40 "]
    pub width: i32,
    #[doc = "Thumbnail height, usually doesn't exceed 40 "]
    pub height: i32,
    #[doc = "The thumbnail in JPEG format"]
    pub data: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A mask should be placed relatively to the forehead"]
pub struct MaskPointForehead {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A mask should be placed relatively to the eyes"]
pub struct MaskPointEyes {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A mask should be placed relatively to the mouth"]
pub struct MaskPointMouth {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A mask should be placed relatively to the chin"]
pub struct MaskPointChin {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Position on a photo where a mask should be placed "]
pub struct MaskPosition {
    #[doc = "Part of the face, relative to which the mask should be placed"]
    pub point: MaskPoint,
    #[doc = "Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. (For example, -1.0 will place the mask just to the left of the default mask position)"]
    pub x_shift: f64,
    #[doc = "Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. (For example, 1.0 will place the mask just below the default mask position)"]
    pub y_shift: f64,
    #[doc = "Mask scaling coefficient. (For example, 2.0 means a doubled size)"]
    pub scale: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes one answer option of a poll "]
pub struct PollOption {
    #[doc = "Option text, 1-100 characters "]
    pub text: String,
    #[doc = "Number of voters for this option, available only for closed or voted polls "]
    pub voter_count: i32,
    #[doc = "The percentage of votes for this option, 0-100"]
    pub vote_percentage: i32,
    #[doc = "True, if the option was chosen by the user "]
    #[serde(default)]
    pub is_chosen: bool,
    #[doc = "True, if the option is being chosen by a pending setPollAnswer request"]
    #[serde(default)]
    pub is_being_chosen: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A regular poll "]
pub struct PollTypeRegular {
    #[doc = "True, if multiple answer options can be chosen simultaneously"]
    #[serde(default)]
    pub allow_multiple_answers: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A poll in quiz mode, which has exactly one correct answer option and can be answered only once"]
pub struct PollTypeQuiz {
    #[doc = "0-based identifier of the correct answer option; -1 for a yet unanswered poll"]
    pub correct_option_id: i32,
    #[doc = "Text that is shown when the user chooses an incorrect answer or taps on the lamp icon, 0-200 characters with at most 2 line feeds; empty for a yet unanswered poll"]
    pub explanation: FormattedText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes an animation file. The animation must be encoded in GIF or MPEG4 format "]
pub struct Animation {
    #[doc = "Duration of the animation, in seconds; as defined by the sender "]
    pub duration: i32,
    #[doc = "Width of the animation "]
    pub width: i32,
    #[doc = "Height of the animation"]
    pub height: i32,
    #[doc = "Original name of the file; as defined by the sender "]
    pub file_name: String,
    #[doc = "MIME type of the file, usually \"image/gif\" or \"video/mp4\""]
    pub mime_type: String,
    #[doc = "Animation minithumbnail; may be null "]
    pub minithumbnail: Option<Minithumbnail>,
    #[doc = "Animation thumbnail; may be null "]
    pub thumbnail: Option<PhotoSize>,
    #[doc = "File containing the animation"]
    pub animation: File,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes an audio file. Audio is usually in MP3 or M4A format "]
pub struct Audio {
    #[doc = "Duration of the audio, in seconds; as defined by the sender "]
    pub duration: i32,
    #[doc = "Title of the audio; as defined by the sender "]
    pub title: String,
    #[doc = "Performer of the audio; as defined by the sender"]
    pub performer: String,
    #[doc = "Original name of the file; as defined by the sender "]
    pub file_name: String,
    #[doc = "The MIME type of the file; as defined by the sender "]
    pub mime_type: String,
    #[doc = "The minithumbnail of the album cover; may be null "]
    pub album_cover_minithumbnail: Option<Minithumbnail>,
    #[doc = "The thumbnail of the album cover; as defined by the sender. The full size thumbnail should be extracted from the downloaded file; may be null "]
    pub album_cover_thumbnail: Option<PhotoSize>,
    #[doc = "File containing the audio"]
    pub audio: File,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a document of any type "]
pub struct Document {
    #[doc = "Original name of the file; as defined by the sender "]
    pub file_name: String,
    #[doc = "MIME type of the file; as defined by the sender"]
    pub mime_type: String,
    #[doc = "Document minithumbnail; may be null "]
    pub minithumbnail: Option<Minithumbnail>,
    #[doc = "Document thumbnail in JPEG or PNG format (PNG will be used only for background patterns); as defined by the sender; may be null "]
    pub thumbnail: Option<PhotoSize>,
    #[doc = "File containing the document"]
    pub document: File,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a photo "]
pub struct Photo {
    #[doc = "True, if stickers were added to the photo "]
    #[serde(default)]
    pub has_stickers: bool,
    #[doc = "Photo minithumbnail; may be null "]
    pub minithumbnail: Option<Minithumbnail>,
    #[doc = "Available variants of the photo, in different sizes"]
    pub sizes: Vec<PhotoSize>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a sticker "]
pub struct Sticker {
    #[doc = "The identifier of the sticker set to which the sticker belongs; 0 if none "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub set_id: i64,
    #[doc = "Sticker width; as defined by the sender "]
    pub width: i32,
    #[doc = "Sticker height; as defined by the sender"]
    pub height: i32,
    #[doc = "Emoji corresponding to the sticker "]
    pub emoji: String,
    #[doc = "True, if the sticker is an animated sticker in TGS format "]
    #[serde(default)]
    pub is_animated: bool,
    #[doc = "True, if the sticker is a mask "]
    #[serde(default)]
    pub is_mask: bool,
    #[doc = "Position where the mask should be placed; may be null "]
    pub mask_position: Option<MaskPosition>,
    #[doc = "Sticker thumbnail in WEBP or JPEG format; may be null "]
    pub thumbnail: Option<PhotoSize>,
    #[doc = "File containing the sticker"]
    pub sticker: File,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a video file "]
pub struct Video {
    #[doc = "Duration of the video, in seconds; as defined by the sender "]
    pub duration: i32,
    #[doc = "Video width; as defined by the sender "]
    pub width: i32,
    #[doc = "Video height; as defined by the sender"]
    pub height: i32,
    #[doc = "Original name of the file; as defined by the sender "]
    pub file_name: String,
    #[doc = "MIME type of the file; as defined by the sender "]
    pub mime_type: String,
    #[doc = "True, if stickers were added to the video"]
    #[serde(default)]
    pub has_stickers: bool,
    #[doc = "True, if the video should be tried to be streamed "]
    #[serde(default)]
    pub supports_streaming: bool,
    #[doc = "Video minithumbnail; may be null "]
    pub minithumbnail: Option<Minithumbnail>,
    #[doc = "Video thumbnail; as defined by the sender; may be null "]
    pub thumbnail: Option<PhotoSize>,
    #[doc = "File containing the video"]
    pub video: File,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a video note. The video must be equal in width and height, cropped to a circle, and stored in MPEG4 format "]
pub struct VideoNote {
    #[doc = "Duration of the video, in seconds; as defined by the sender "]
    pub duration: i32,
    #[doc = "Video width and height; as defined by the sender "]
    pub length: i32,
    #[doc = "Video minithumbnail; may be null "]
    pub minithumbnail: Option<Minithumbnail>,
    #[doc = "Video thumbnail; as defined by the sender; may be null "]
    pub thumbnail: Option<PhotoSize>,
    #[doc = "File containing the video"]
    pub video: File,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a voice note. The voice note must be encoded with the Opus codec, and stored inside an OGG container. Voice notes can have only a single audio channel "]
pub struct VoiceNote {
    #[doc = "Duration of the voice note, in seconds; as defined by the sender"]
    pub duration: i32,
    #[doc = "A waveform representation of the voice note in 5-bit format "]
    pub waveform: String,
    #[doc = "MIME type of the file; as defined by the sender "]
    pub mime_type: String,
    #[doc = "File containing the voice note"]
    pub voice: File,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a user contact "]
pub struct Contact {
    #[doc = "Phone number of the user "]
    pub phone_number: String,
    #[doc = "First name of the user; 1-255 characters in length "]
    pub first_name: String,
    #[doc = "Last name of the user "]
    pub last_name: String,
    #[doc = "Additional data about the user in a form of vCard; 0-2048 bytes in length "]
    pub vcard: String,
    #[doc = "Identifier of the user, if known; otherwise 0"]
    pub user_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a location on planet Earth "]
pub struct Location {
    #[doc = "Latitude of the location in degrees; as defined by the sender "]
    pub latitude: f64,
    #[doc = "Longitude of the location, in degrees; as defined by the sender"]
    pub longitude: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a venue "]
pub struct Venue {
    #[doc = "Venue location; as defined by the sender "]
    pub location: Location,
    #[doc = "Venue name; as defined by the sender "]
    pub title: String,
    #[doc = "Venue address; as defined by the sender "]
    pub address: String,
    #[doc = "Provider of the venue database; as defined by the sender. Currently only \"foursquare\" needs to be supported"]
    pub provider: String,
    #[doc = "Identifier of the venue in the provider database; as defined by the sender "]
    pub id: String,
    #[serde(rename = "type")]
    #[doc = "Type of the venue in the provider database; as defined by the sender"]
    pub type_: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a game "]
pub struct Game {
    #[doc = "Game ID "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "Game short name. To share a game use the URL https://t.me/{bot_username}?game={game_short_name} "]
    pub short_name: String,
    #[doc = "Game title "]
    pub title: String,
    #[doc = "Game text, usually containing scoreboards for a game"]
    pub text: FormattedText,
    #[doc = "Game description "]
    pub description: String,
    #[doc = "Game photo "]
    pub photo: Photo,
    #[doc = "Game animation; may be null"]
    pub animation: Option<Animation>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a poll "]
pub struct Poll {
    #[doc = "Unique poll identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "Poll question, 1-255 characters "]
    pub question: String,
    #[doc = "List of poll answer options"]
    pub options: Vec<PollOption>,
    #[doc = "Total number of voters, participating in the poll "]
    pub total_voter_count: i32,
    #[doc = "User identifiers of recent voters, if the poll is non-anonymous"]
    pub recent_voter_user_ids: Vec<i32>,
    #[doc = "True, if the poll is anonymous "]
    #[serde(default)]
    pub is_anonymous: bool,
    #[serde(rename = "type")]
    #[doc = "Type of the poll"]
    pub type_: PollType,
    #[doc = "Amount of time the poll will be active after creation, in seconds "]
    pub open_period: i32,
    #[doc = "Point in time (Unix timestamp) when the poll will be automatically closed "]
    pub close_date: i32,
    #[doc = "True, if the poll is closed"]
    #[serde(default)]
    pub is_closed: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a user profile photo "]
pub struct ProfilePhoto {
    #[doc = "Photo identifier; 0 for an empty photo. Can be used to find a photo in a list of userProfilePhotos"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "A small (160x160) user profile photo. The file can be downloaded only before the photo is changed "]
    pub small: File,
    #[doc = "A big (640x640) user profile photo. The file can be downloaded only before the photo is changed"]
    pub big: File,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes the photo of a chat "]
pub struct ChatPhoto {
    #[doc = "A small (160x160) chat photo. The file can be downloaded only before the photo is changed "]
    pub small: File,
    #[doc = "A big (640x640) chat photo. The file can be downloaded only before the photo is changed"]
    pub big: File,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A regular user"]
pub struct UserTypeRegular {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A deleted user or deleted bot. No information on the user besides the user identifier is available. It is not possible to perform any active actions on this type of user"]
pub struct UserTypeDeleted {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A bot (see https://core.telegram.org/bots) "]
pub struct UserTypeBot {
    #[doc = "True, if the bot can be invited to basic group and supergroup chats"]
    #[serde(default)]
    pub can_join_groups: bool,
    #[doc = "True, if the bot can read all messages in basic group or supergroup chats and not just those addressed to the bot. In private and channel chats a bot can always read all messages"]
    #[serde(default)]
    pub can_read_all_group_messages: bool,
    #[doc = "True, if the bot supports inline queries "]
    #[serde(default)]
    pub is_inline: bool,
    #[doc = "Placeholder for inline queries (displayed on the client input field) "]
    pub inline_query_placeholder: String,
    #[doc = "True, if the location of the user should be sent with every inline query to this bot"]
    #[serde(default)]
    pub need_location: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "No information on the user besides the user identifier is available, yet this user has not been deleted. This object is extremely rare and must be handled like a deleted user. It is not possible to perform any actions on users of this type"]
pub struct UserTypeUnknown {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a command supported by a bot "]
pub struct BotCommand {
    #[doc = "Text of the bot command "]
    pub command: String,
    #[doc = "Description of the bot command"]
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Provides information about a bot and its supported commands "]
pub struct BotInfo {
    #[doc = "Long description shown on the user info page "]
    pub description: String,
    #[doc = "A list of commands supported by the bot"]
    pub commands: Vec<BotCommand>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a location to which a chat is connected "]
pub struct ChatLocation {
    #[doc = "The location "]
    pub location: Location,
    #[doc = "Location address; 1-64 characters, as defined by the chat owner"]
    pub address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a user "]
pub struct User {
    #[doc = "User identifier "]
    pub id: i32,
    #[doc = "First name of the user "]
    pub first_name: String,
    #[doc = "Last name of the user "]
    pub last_name: String,
    #[doc = "Username of the user"]
    pub username: String,
    #[doc = "Phone number of the user "]
    pub phone_number: String,
    #[doc = "Current online status of the user "]
    pub status: UserStatus,
    #[doc = "Profile photo of the user; may be null"]
    pub profile_photo: Option<ProfilePhoto>,
    #[doc = "The user is a contact of the current user"]
    #[serde(default)]
    pub is_contact: bool,
    #[doc = "The user is a contact of the current user and the current user is a contact of the user"]
    #[serde(default)]
    pub is_mutual_contact: bool,
    #[doc = "True, if the user is verified "]
    #[serde(default)]
    pub is_verified: bool,
    #[doc = "True, if the user is Telegram support account"]
    #[serde(default)]
    pub is_support: bool,
    #[doc = "If non-empty, it contains a human-readable description of the reason why access to this user must be restricted"]
    pub restriction_reason: String,
    #[doc = "True, if many users reported this user as a scam"]
    #[serde(default)]
    pub is_scam: bool,
    #[doc = "If false, the user is inaccessible, and the only information known about the user is inside this class. It can't be passed to any method except GetUser "]
    #[serde(default)]
    pub have_access: bool,
    #[serde(rename = "type")]
    #[doc = "Type of the user "]
    pub type_: UserType,
    #[doc = "IETF language tag of the user's language; only available to bots"]
    pub language_code: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains full information about a user (except the full list of profile photos) "]
pub struct UserFullInfo {
    #[doc = "True, if the user is blacklisted by the current user"]
    #[serde(default)]
    pub is_blocked: bool,
    #[doc = "True, if the user can be called "]
    #[serde(default)]
    pub can_be_called: bool,
    #[doc = "True, if the user can't be called due to their privacy settings"]
    #[serde(default)]
    pub has_private_calls: bool,
    #[doc = "True, if the current user needs to explicitly allow to share their phone number with the user when the method addContact is used"]
    #[serde(default)]
    pub need_phone_number_privacy_exception: bool,
    #[doc = "A short user bio "]
    pub bio: String,
    #[doc = "For bots, the text that is included with the link when users share the bot "]
    pub share_text: String,
    #[doc = "Number of group chats where both the other user and the current user are a member; 0 for the current user "]
    pub group_in_common_count: i32,
    #[doc = "If the user is a bot, information about the bot; may be null"]
    pub bot_info: Option<BotInfo>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains full information about a user profile photo "]
pub struct UserProfilePhoto {
    #[doc = "Unique user profile photo identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "Point in time (Unix timestamp) when the photo has been added "]
    pub added_date: i32,
    #[doc = "Available variants of the user photo, in different sizes"]
    pub sizes: Vec<PhotoSize>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains part of the list of user photos "]
pub struct UserProfilePhotos {
    #[doc = "Total number of user profile photos "]
    pub total_count: i32,
    #[doc = "A list of photos"]
    pub photos: Vec<UserProfilePhoto>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a list of users "]
pub struct Users {
    #[doc = "Approximate total count of users found "]
    pub total_count: i32,
    #[doc = "A list of user identifiers"]
    pub user_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a chat administrator "]
pub struct ChatAdministrator {
    #[doc = "User identifier of the administrator "]
    pub user_id: i32,
    #[doc = "Custom title of the administrator "]
    pub custom_title: String,
    #[doc = "True, if the user is the owner of the chat"]
    #[serde(default)]
    pub is_owner: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a list of chat administrators "]
pub struct ChatAdministrators {
    #[doc = "A list of chat administrators"]
    pub administrators: Vec<ChatAdministrator>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes actions that a user is allowed to take in a chat"]
pub struct ChatPermissions {
    #[doc = "True, if the user can send text messages, contacts, locations, and venues"]
    #[serde(default)]
    pub can_send_messages: bool,
    #[doc = "True, if the user can send audio files, documents, photos, videos, video notes, and voice notes. Implies can_send_messages permissions"]
    #[serde(default)]
    pub can_send_media_messages: bool,
    #[doc = "True, if the user can send polls. Implies can_send_messages permissions"]
    #[serde(default)]
    pub can_send_polls: bool,
    #[doc = "True, if the user can send animations, games, and stickers and use inline bots. Implies can_send_messages permissions"]
    #[serde(default)]
    pub can_send_other_messages: bool,
    #[doc = "True, if the user may add a web page preview to their messages. Implies can_send_messages permissions"]
    #[serde(default)]
    pub can_add_web_page_previews: bool,
    #[doc = "True, if the user can change the chat title, photo, and other settings"]
    #[serde(default)]
    pub can_change_info: bool,
    #[doc = "True, if the user can invite new users to the chat"]
    #[serde(default)]
    pub can_invite_users: bool,
    #[doc = "True, if the user can pin messages"]
    #[serde(default)]
    pub can_pin_messages: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is the owner of a chat and has all the administrator privileges"]
pub struct ChatMemberStatusCreator {
    #[doc = "A custom title of the owner; 0-16 characters without emojis; applicable to supergroups only"]
    pub custom_title: String,
    #[doc = "True, if the user is a member of the chat"]
    #[serde(default)]
    pub is_member: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is a member of a chat and has some additional privileges. In basic groups, administrators can edit and delete messages sent by others, add new members, and ban unprivileged members. In supergroups and channels, there are more detailed options for administrator privileges"]
pub struct ChatMemberStatusAdministrator {
    #[doc = "A custom title of the administrator; 0-16 characters without emojis; applicable to supergroups only"]
    pub custom_title: String,
    #[doc = "True, if the current user can edit the administrator privileges for the called user"]
    #[serde(default)]
    pub can_be_edited: bool,
    #[doc = "True, if the administrator can change the chat title, photo, and other settings"]
    #[serde(default)]
    pub can_change_info: bool,
    #[doc = "True, if the administrator can create channel posts; applicable to channels only"]
    #[serde(default)]
    pub can_post_messages: bool,
    #[doc = "True, if the administrator can edit messages of other users and pin messages; applicable to channels only"]
    #[serde(default)]
    pub can_edit_messages: bool,
    #[doc = "True, if the administrator can delete messages of other users"]
    #[serde(default)]
    pub can_delete_messages: bool,
    #[doc = "True, if the administrator can invite new users to the chat"]
    #[serde(default)]
    pub can_invite_users: bool,
    #[doc = "True, if the administrator can restrict, ban, or unban chat members"]
    #[serde(default)]
    pub can_restrict_members: bool,
    #[doc = "True, if the administrator can pin messages; applicable to groups only"]
    #[serde(default)]
    pub can_pin_messages: bool,
    #[doc = "True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that were directly or indirectly promoted by them"]
    #[serde(default)]
    pub can_promote_members: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is a member of a chat, without any additional privileges or restrictions"]
pub struct ChatMemberStatusMember {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is under certain restrictions in the chat. Not supported in basic groups and channels"]
pub struct ChatMemberStatusRestricted {
    #[doc = "True, if the user is a member of the chat"]
    #[serde(default)]
    pub is_member: bool,
    #[doc = "Point in time (Unix timestamp) when restrictions will be lifted from the user; 0 if never. If the user is restricted for more than 366 days or for less than 30 seconds from the current time, the user is considered to be restricted forever"]
    pub restricted_until_date: i32,
    #[doc = "User permissions in the chat"]
    pub permissions: ChatPermissions,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is not a chat member"]
pub struct ChatMemberStatusLeft {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user was banned (and hence is not a member of the chat). Implies the user can't return to the chat or view messages"]
pub struct ChatMemberStatusBanned {
    #[doc = "Point in time (Unix timestamp) when the user will be unbanned; 0 if never. If the user is banned for more than 366 days or for less than 30 seconds from the current time, the user is considered to be banned forever"]
    pub banned_until_date: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A user with information about joining/leaving a chat "]
pub struct ChatMember {
    #[doc = "User identifier of the chat member "]
    pub user_id: i32,
    #[doc = "Identifier of a user that invited/promoted/banned this member in the chat; 0 if unknown"]
    pub inviter_user_id: i32,
    #[doc = "Point in time (Unix timestamp) when the user joined a chat "]
    pub joined_chat_date: i32,
    #[doc = "Status of the member in the chat "]
    pub status: ChatMemberStatus,
    #[doc = "If the user is a bot, information about the bot; may be null. Can be null even for a bot if the bot is not a chat member"]
    pub bot_info: Option<BotInfo>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of chat members "]
pub struct ChatMembers {
    #[doc = "Approximate total count of chat members found "]
    pub total_count: i32,
    #[doc = "A list of chat members"]
    pub members: Vec<ChatMember>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns contacts of the user"]
pub struct ChatMembersFilterContacts {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the owner and administrators"]
pub struct ChatMembersFilterAdministrators {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns all chat members, including restricted chat members"]
pub struct ChatMembersFilterMembers {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns users under certain restrictions in the chat; can be used only by administrators in a supergroup"]
pub struct ChatMembersFilterRestricted {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns users banned from the chat; can be used only by administrators in a supergroup or in a channel"]
pub struct ChatMembersFilterBanned {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns bot members of the chat"]
pub struct ChatMembersFilterBots {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns recently active users in reverse chronological order"]
pub struct SupergroupMembersFilterRecent {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns contacts of the user, which are members of the supergroup or channel "]
pub struct SupergroupMembersFilterContacts {
    #[doc = "Query to search for"]
    pub query: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the owner and administrators"]
pub struct SupergroupMembersFilterAdministrators {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Used to search for supergroup or channel members via a (string) query "]
pub struct SupergroupMembersFilterSearch {
    #[doc = "Query to search for"]
    pub query: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns restricted supergroup members; can be used only by administrators "]
pub struct SupergroupMembersFilterRestricted {
    #[doc = "Query to search for"]
    pub query: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns users banned from the supergroup or channel; can be used only by administrators "]
pub struct SupergroupMembersFilterBanned {
    #[doc = "Query to search for"]
    pub query: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns bot members of the supergroup or channel"]
pub struct SupergroupMembersFilterBots {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)"]
pub struct BasicGroup {
    #[doc = "Group identifier"]
    pub id: i32,
    #[doc = "Number of members in the group"]
    pub member_count: i32,
    #[doc = "Status of the current user in the group"]
    pub status: ChatMemberStatus,
    #[doc = "True, if the group is active"]
    #[serde(default)]
    pub is_active: bool,
    #[doc = "Identifier of the supergroup to which this group was upgraded; 0 if none"]
    pub upgraded_to_supergroup_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains full information about a basic group "]
pub struct BasicGroupFullInfo {
    #[doc = "Group description "]
    pub description: String,
    #[doc = "User identifier of the creator of the group; 0 if unknown "]
    pub creator_user_id: i32,
    #[doc = "Group members "]
    pub members: Vec<ChatMember>,
    #[doc = "Invite link for this group; available only after it has been generated at least once and only for the group creator"]
    pub invite_link: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a supergroup or channel with zero or more members (subscribers in the case of channels). From the point of view of the system, a channel is a special kind of a supergroup: only administrators can post and see the list of members, and posts from all administrators use the name and photo of the channel instead of individual names and profile photos. Unlike supergroups, channels can have an unlimited number of subscribers"]
pub struct Supergroup {
    #[doc = "Supergroup or channel identifier"]
    pub id: i32,
    #[doc = "Username of the supergroup or channel; empty for private supergroups or channels"]
    pub username: String,
    #[doc = "Point in time (Unix timestamp) when the current user joined, or the point in time when the supergroup or channel was created, in case the user is not a member"]
    pub date: i32,
    #[doc = "Status of the current user in the supergroup or channel; custom title will be always empty"]
    pub status: ChatMemberStatus,
    #[doc = "Number of members in the supergroup or channel; 0 if unknown. Currently it is guaranteed to be known only if the supergroup or channel was found through SearchPublicChats"]
    pub member_count: i32,
    #[doc = "True, if the channel has a discussion group, or the supergroup is the designated discussion group for a channel"]
    #[serde(default)]
    pub has_linked_chat: bool,
    #[doc = "True, if the supergroup is connected to a location, i.e. the supergroup is a location-based supergroup"]
    #[serde(default)]
    pub has_location: bool,
    #[doc = "True, if messages sent to the channel should contain information about the sender. This field is only applicable to channels"]
    #[serde(default)]
    pub sign_messages: bool,
    #[doc = "True, if the slow mode is enabled in the supergroup"]
    #[serde(default)]
    pub is_slow_mode_enabled: bool,
    #[doc = "True, if the supergroup is a channel"]
    #[serde(default)]
    pub is_channel: bool,
    #[doc = "True, if the supergroup or channel is verified"]
    #[serde(default)]
    pub is_verified: bool,
    #[doc = "If non-empty, contains a human-readable description of the reason why access to this supergroup or channel must be restricted"]
    pub restriction_reason: String,
    #[doc = "True, if many users reported this supergroup as a scam"]
    #[serde(default)]
    pub is_scam: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains full information about a supergroup or channel"]
pub struct SupergroupFullInfo {
    #[doc = "Supergroup or channel description"]
    pub description: String,
    #[doc = "Number of members in the supergroup or channel; 0 if unknown"]
    pub member_count: i32,
    #[doc = "Number of privileged users in the supergroup or channel; 0 if unknown"]
    pub administrator_count: i32,
    #[doc = "Number of restricted users in the supergroup; 0 if unknown"]
    pub restricted_count: i32,
    #[doc = "Number of users banned from chat; 0 if unknown"]
    pub banned_count: i32,
    #[doc = "Chat identifier of a discussion group for the channel, or a channel, for which the supergroup is the designated discussion group; 0 if none or unknown"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub linked_chat_id: i64,
    #[doc = "Delay between consecutive sent messages for non-administrator supergroup members, in seconds"]
    pub slow_mode_delay: i32,
    #[doc = "Time left before next message can be sent in the supergroup, in seconds. An updateSupergroupFullInfo update is not triggered when value of this field changes, but both new and old values are non-zero"]
    pub slow_mode_delay_expires_in: f64,
    #[doc = "True, if members of the chat can be retrieved"]
    #[serde(default)]
    pub can_get_members: bool,
    #[doc = "True, if the chat username can be changed"]
    #[serde(default)]
    pub can_set_username: bool,
    #[doc = "True, if the supergroup sticker set can be changed"]
    #[serde(default)]
    pub can_set_sticker_set: bool,
    #[doc = "True, if the supergroup location can be changed"]
    #[serde(default)]
    pub can_set_location: bool,
    #[doc = "True, if the channel statistics is available"]
    #[serde(default)]
    pub can_view_statistics: bool,
    #[doc = "True, if new chat members will have access to old messages. In public or discussion groups and both public and private channels, old messages are always available, so this option affects only private supergroups without a linked chat. The value of this field is only available for chat administrators"]
    #[serde(default)]
    pub is_all_history_available: bool,
    #[doc = "Identifier of the supergroup sticker set; 0 if none"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub sticker_set_id: i64,
    #[doc = "Location to which the supergroup is connected; may be null"]
    pub location: Option<ChatLocation>,
    #[doc = "Invite link for this chat"]
    pub invite_link: String,
    #[doc = "Identifier of the basic group from which supergroup was upgraded; 0 if none"]
    pub upgraded_from_basic_group_id: i32,
    #[doc = "Identifier of the last message in the basic group from which supergroup was upgraded; 0 if none"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub upgraded_from_max_message_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The secret chat is not yet created; waiting for the other user to get online"]
pub struct SecretChatStatePending {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The secret chat is ready to use"]
pub struct SecretChatStateReady {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The secret chat is closed"]
pub struct SecretChatStateClosed {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a secret chat"]
pub struct SecretChat {
    #[doc = "Secret chat identifier"]
    pub id: i32,
    #[doc = "Identifier of the chat partner"]
    pub user_id: i32,
    #[doc = "State of the secret chat"]
    pub state: SecretChatState,
    #[doc = "True, if the chat was created by the current user; otherwise false"]
    #[serde(default)]
    pub is_outbound: bool,
    #[doc = "Current message Time To Live setting (self-destruct timer) for the chat, in seconds"]
    pub ttl: i32,
    #[doc = "Hash of the currently used key for comparison with the hash of the chat partner's key. This is a string of 36 little-endian bytes, which must be split into groups of 2 bits, each denoting a pixel of one of 4 colors FFFFFF, D5E6F3, 2D5775, and 2F99C9.\n The pixels must be used to make a 12x12 square image filled from left to right, top to bottom. Alternatively, the first 32 bytes of the hash can be converted to the hexadecimal format and printed as 32 2-digit hex numbers"]
    pub key_hash: String,
    #[doc = "Secret chat layer; determines features supported by the other client. Video notes are supported if the layer >= 66; nested text entities and underline and strikethrough entities are supported if the layer >= 101"]
    pub layer: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The message was originally written by a known user "]
pub struct MessageForwardOriginUser {
    #[doc = "Identifier of the user that originally sent the message"]
    pub sender_user_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The message was originally written by a user, which is hidden by their privacy settings "]
pub struct MessageForwardOriginHiddenUser {
    #[doc = "Name of the sender"]
    pub sender_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The message was originally a post in a channel"]
pub struct MessageForwardOriginChannel {
    #[doc = "Identifier of the chat from which the message was originally forwarded"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Message identifier of the original message; 0 if unknown"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "Original post author signature"]
    pub author_signature: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a forwarded message"]
pub struct MessageForwardInfo {
    #[doc = "Origin of a forwarded message"]
    pub origin: MessageForwardOrigin,
    #[doc = "Point in time (Unix timestamp) when the message was originally sent"]
    pub date: i32,
    #[doc = "The type of a public service announcement for the forwarded message"]
    pub public_service_announcement_type: String,
    #[doc = "For messages forwarded to the chat with the current user (Saved Messages) or to the channel's discussion group, the identifier of the chat from which the message was forwarded last time; 0 if unknown"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub from_chat_id: i64,
    #[doc = "For messages forwarded to the chat with the current user (Saved Messages) or to the channel's discussion group, the identifier of the original message from which the new message was forwarded last time; 0 if unknown"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub from_message_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The message is being sent now, but has not yet been delivered to the server"]
pub struct MessageSendingStatePending {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The message failed to be sent "]
pub struct MessageSendingStateFailed {
    #[doc = "An error code; 0 if unknown "]
    pub error_code: i32,
    #[doc = "Error message"]
    pub error_message: String,
    #[doc = "True, if the message can be re-sent "]
    #[serde(default)]
    pub can_retry: bool,
    #[doc = "Time left before the message can be re-sent, in seconds. No update is sent when this field changes"]
    pub retry_after: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a message"]
pub struct Message {
    #[doc = "Message identifier, unique for the chat to which the message belongs"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "Identifier of the user who sent the message; 0 if unknown. Currently, it is unknown for channel posts and for channel posts automatically forwarded to discussion group"]
    pub sender_user_id: i32,
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Information about the sending state of the message; may be null"]
    pub sending_state: Option<MessageSendingState>,
    #[doc = "Information about the scheduling state of the message; may be null"]
    pub scheduling_state: Option<MessageSchedulingState>,
    #[doc = "True, if the message is outgoing"]
    #[serde(default)]
    pub is_outgoing: bool,
    #[doc = "True, if the message can be edited. For live location and poll messages this fields shows whether editMessageLiveLocation or stopPoll can be used with this message by the client"]
    #[serde(default)]
    pub can_be_edited: bool,
    #[doc = "True, if the message can be forwarded"]
    #[serde(default)]
    pub can_be_forwarded: bool,
    #[doc = "True, if the message can be deleted only for the current user while other users will continue to see it"]
    #[serde(default)]
    pub can_be_deleted_only_for_self: bool,
    #[doc = "True, if the message can be deleted for all users"]
    #[serde(default)]
    pub can_be_deleted_for_all_users: bool,
    #[doc = "True, if the message is a channel post. All messages to channels are channel posts, all other messages are not channel posts"]
    #[serde(default)]
    pub is_channel_post: bool,
    #[doc = "True, if the message contains an unread mention for the current user"]
    #[serde(default)]
    pub contains_unread_mention: bool,
    #[doc = "Point in time (Unix timestamp) when the message was sent"]
    pub date: i32,
    #[doc = "Point in time (Unix timestamp) when the message was last edited"]
    pub edit_date: i32,
    #[doc = "Information about the initial message sender; may be null"]
    pub forward_info: Option<MessageForwardInfo>,
    #[doc = "If non-zero, the identifier of the message this message is replying to; can be the identifier of a deleted message"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub reply_to_message_id: i64,
    #[doc = "For self-destructing messages, the message's TTL (Time To Live), in seconds; 0 if none. TDLib will send updateDeleteMessages or updateMessageContent once the TTL expires"]
    pub ttl: i32,
    #[doc = "Time left before the message expires, in seconds"]
    pub ttl_expires_in: f64,
    #[doc = "If non-zero, the user identifier of the bot through which this message was sent"]
    pub via_bot_user_id: i32,
    #[doc = "For channel posts, optional author signature"]
    pub author_signature: String,
    #[doc = "Number of times this message was viewed"]
    pub views: i32,
    #[doc = "Unique identifier of an album this message belongs to. Only photos and videos can be grouped together in albums"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub media_album_id: i64,
    #[doc = "If non-empty, contains a human-readable description of the reason why access to this message must be restricted"]
    pub restriction_reason: String,
    #[doc = "Content of the message"]
    pub content: MessageContent,
    #[doc = "Reply markup for the message; may be null"]
    pub reply_markup: Option<ReplyMarkup>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of messages "]
pub struct Messages {
    #[doc = "Approximate total count of messages found "]
    pub total_count: i32,
    #[doc = "List of messages; messages may be null"]
    pub messages: Option<Vec<Message>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of messages found by a search "]
pub struct FoundMessages {
    #[doc = "List of messages "]
    pub messages: Vec<Message>,
    #[doc = "Value to pass as from_search_id to get more results"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub next_from_search_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Notification settings applied to all private and secret chats when the corresponding chat setting has a default value"]
pub struct NotificationSettingsScopePrivateChats {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Notification settings applied to all basic groups and supergroups when the corresponding chat setting has a default value"]
pub struct NotificationSettingsScopeGroupChats {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Notification settings applied to all channels when the corresponding chat setting has a default value"]
pub struct NotificationSettingsScopeChannelChats {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about notification settings for a chat"]
pub struct ChatNotificationSettings {
    #[doc = "If true, mute_for is ignored and the value for the relevant type of chat is used instead "]
    #[serde(default)]
    pub use_default_mute_for: bool,
    #[doc = "Time left before notifications will be unmuted, in seconds"]
    pub mute_for: i32,
    #[doc = "If true, sound is ignored and the value for the relevant type of chat is used instead "]
    #[serde(default)]
    pub use_default_sound: bool,
    #[doc = "The name of an audio file to be used for notification sounds; only applies to iOS applications"]
    pub sound: String,
    #[doc = "If true, show_preview is ignored and the value for the relevant type of chat is used instead "]
    #[serde(default)]
    pub use_default_show_preview: bool,
    #[doc = "True, if message content should be displayed in notifications"]
    #[serde(default)]
    pub show_preview: bool,
    #[doc = "If true, disable_pinned_message_notifications is ignored and the value for the relevant type of chat is used instead "]
    #[serde(default)]
    pub use_default_disable_pinned_message_notifications: bool,
    #[doc = "If true, notifications for incoming pinned messages will be created as for an ordinary unread message"]
    #[serde(default)]
    pub disable_pinned_message_notifications: bool,
    #[doc = "If true, disable_mention_notifications is ignored and the value for the relevant type of chat is used instead "]
    #[serde(default)]
    pub use_default_disable_mention_notifications: bool,
    #[doc = "If true, notifications for messages with mentions will be created as for an ordinary unread message"]
    #[serde(default)]
    pub disable_mention_notifications: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about notification settings for several chats"]
pub struct ScopeNotificationSettings {
    #[doc = "Time left before notifications will be unmuted, in seconds"]
    pub mute_for: i32,
    #[doc = "The name of an audio file to be used for notification sounds; only applies to iOS applications"]
    pub sound: String,
    #[doc = "True, if message content should be displayed in notifications"]
    #[serde(default)]
    pub show_preview: bool,
    #[doc = "True, if notifications for incoming pinned messages will be created as for an ordinary unread message"]
    #[serde(default)]
    pub disable_pinned_message_notifications: bool,
    #[doc = "True, if notifications for messages with mentions will be created as for an ordinary unread message"]
    #[serde(default)]
    pub disable_mention_notifications: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a message draft"]
pub struct DraftMessage {
    #[doc = "Identifier of the message to reply to; 0 if none"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub reply_to_message_id: i64,
    #[doc = "Point in time (Unix timestamp) when the draft was created"]
    pub date: i32,
    #[doc = "Content of the message draft; this should always be of type inputMessageText"]
    pub input_message_text: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An ordinary chat with a user "]
pub struct ChatTypePrivate {
    #[doc = "User identifier"]
    pub user_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A basic group (i.e., a chat with 0-200 other users) "]
pub struct ChatTypeBasicGroup {
    #[doc = "Basic group identifier"]
    pub basic_group_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A supergroup (i.e. a chat with up to GetOption(\"supergroup_max_size\") other users), or channel (with unlimited members) "]
pub struct ChatTypeSupergroup {
    #[doc = "Supergroup or channel identifier "]
    pub supergroup_id: i32,
    #[doc = "True, if the supergroup is a channel"]
    #[serde(default)]
    pub is_channel: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A secret chat with a user "]
pub struct ChatTypeSecret {
    #[doc = "Secret chat identifier "]
    pub secret_chat_id: i32,
    #[doc = "User identifier of the secret chat peer"]
    pub user_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A main list of chats"]
pub struct ChatListMain {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A list of chats usually located at the top of the main chat list. Unmuted chats are automatically moved from the Archive to the Main chat list when a new message arrives"]
pub struct ChatListArchive {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat is sponsored by the user's MTProxy server"]
pub struct ChatSourceMtprotoProxy {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat contains a public service announcement "]
pub struct ChatSourcePublicServiceAnnouncement {
    #[serde(rename = "type")]
    #[doc = "The type of the announcement "]
    pub type_: String,
    #[doc = "The text of the announcement"]
    pub text: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat. (Can be a private chat, basic group, supergroup, or secret chat)"]
pub struct Chat {
    #[doc = "Chat unique identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[serde(rename = "type")]
    #[doc = "Type of the chat"]
    pub type_: ChatType,
    #[doc = "A chat list to which the chat belongs; may be null"]
    pub chat_list: Option<ChatList>,
    #[doc = "Chat title"]
    pub title: String,
    #[doc = "Chat photo; may be null"]
    pub photo: Option<ChatPhoto>,
    #[doc = "Actions that non-administrator chat members are allowed to take in the chat"]
    pub permissions: ChatPermissions,
    #[doc = "Last message in the chat; may be null"]
    pub last_message: Option<Message>,
    #[doc = "Descending parameter by which chats are sorted in the main chat list. If the order number of two chats is the same, they must be sorted in descending order by ID. If 0, the position of the chat in the list is undetermined"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub order: i64,
    #[doc = "Source of the chat in a chat list; may be null"]
    pub source: Option<ChatSource>,
    #[doc = "True, if the chat is pinned"]
    #[serde(default)]
    pub is_pinned: bool,
    #[doc = "True, if the chat is marked as unread"]
    #[serde(default)]
    pub is_marked_as_unread: bool,
    #[doc = "True, if the chat has scheduled messages"]
    #[serde(default)]
    pub has_scheduled_messages: bool,
    #[doc = "True, if the chat messages can be deleted only for the current user while other users will continue to see the messages"]
    #[serde(default)]
    pub can_be_deleted_only_for_self: bool,
    #[doc = "True, if the chat messages can be deleted for all users"]
    #[serde(default)]
    pub can_be_deleted_for_all_users: bool,
    #[doc = "True, if the chat can be reported to Telegram moderators through reportChat"]
    #[serde(default)]
    pub can_be_reported: bool,
    #[doc = "Default value of the disable_notification parameter, used when a message is sent to the chat"]
    #[serde(default)]
    pub default_disable_notification: bool,
    #[doc = "Number of unread messages in the chat"]
    pub unread_count: i32,
    #[doc = "Identifier of the last read incoming message"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub last_read_inbox_message_id: i64,
    #[doc = "Identifier of the last read outgoing message"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub last_read_outbox_message_id: i64,
    #[doc = "Number of unread messages with a mention/reply in the chat"]
    pub unread_mention_count: i32,
    #[doc = "Notification settings for this chat"]
    pub notification_settings: ChatNotificationSettings,
    #[doc = "Describes actions which should be possible to do through a chat action bar; may be null"]
    pub action_bar: Option<ChatActionBar>,
    #[doc = "Identifier of the pinned message in the chat; 0 if none"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub pinned_message_id: i64,
    #[doc = "Identifier of the message from which reply markup needs to be used; 0 if there is no default custom reply markup in the chat"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub reply_markup_message_id: i64,
    #[doc = "A draft of a message in the chat; may be null"]
    pub draft_message: Option<DraftMessage>,
    #[doc = "Contains client-specific data associated with the chat. (For example, the chat position or local chat notification settings can be stored here.) Persistent if the message database is used"]
    pub client_data: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a list of chats "]
pub struct Chats {
    #[doc = "List of chat identifiers"]
    pub chat_ids: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a chat located nearby "]
pub struct ChatNearby {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Distance to the chat location in meters"]
    pub distance: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a list of chats located nearby "]
pub struct ChatsNearby {
    #[doc = "List of users nearby "]
    pub users_nearby: Vec<ChatNearby>,
    #[doc = "List of location-based supergroups nearby"]
    pub supergroups_nearby: Vec<ChatNearby>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a chat invite link "]
pub struct ChatInviteLink {
    #[doc = "Chat invite link"]
    pub invite_link: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a chat invite link"]
pub struct ChatInviteLinkInfo {
    #[doc = "Chat identifier of the invite link; 0 if the user is not a member of this chat"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[serde(rename = "type")]
    #[doc = "Contains information about the type of the chat"]
    pub type_: ChatType,
    #[doc = "Title of the chat"]
    pub title: String,
    #[doc = "Chat photo; may be null"]
    pub photo: Option<ChatPhoto>,
    #[doc = "Number of members in the chat"]
    pub member_count: i32,
    #[doc = "User identifiers of some chat members that may be known to the current user"]
    pub member_user_ids: Vec<i32>,
    #[doc = "True, if the chat is a public supergroup or channel, i.e. it has a username or it is a location-based supergroup"]
    #[serde(default)]
    pub is_public: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat is public, because it has username"]
pub struct PublicChatTypeHasUsername {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat is public, because it is a location-based supergroup"]
pub struct PublicChatTypeIsLocationBased {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat can be reported as spam using the method reportChat with the reason chatReportReasonSpam"]
pub struct ChatActionBarReportSpam {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat is a location-based supergroup, which can be reported as having unrelated location using the method reportChat with the reason chatReportReasonUnrelatedLocation"]
pub struct ChatActionBarReportUnrelatedLocation {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be added to the contact list using the method addContact, or the other user can be blocked using the method blockUser"]
pub struct ChatActionBarReportAddBlock {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat is a private or secret chat and the other user can be added to the contact list using the method addContact"]
pub struct ChatActionBarAddContact {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat is a private or secret chat with a mutual contact and the user's phone number can be shared with the other user using the method sharePhoneNumber"]
pub struct ChatActionBarSharePhoneNumber {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A simple button, with text that should be sent when the button is pressed"]
pub struct KeyboardButtonTypeText {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A button that sends the user's phone number when pressed; available only in private chats"]
pub struct KeyboardButtonTypeRequestPhoneNumber {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A button that sends the user's location when pressed; available only in private chats"]
pub struct KeyboardButtonTypeRequestLocation {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A button that allows the user to create and send a poll when pressed; available only in private chats "]
pub struct KeyboardButtonTypeRequestPoll {
    #[doc = "If true, only regular polls must be allowed to create "]
    #[serde(default)]
    pub force_regular: bool,
    #[doc = "If true, only polls in quiz mode must be allowed to create"]
    #[serde(default)]
    pub force_quiz: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a single button in a bot keyboard "]
pub struct KeyboardButton {
    #[doc = "Text of the button "]
    pub text: String,
    #[serde(rename = "type")]
    #[doc = "Type of the button"]
    pub type_: KeyboardButtonType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A button that opens a specified URL "]
pub struct InlineKeyboardButtonTypeUrl {
    #[doc = "HTTP or tg:// URL to open"]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A button that opens a specified URL and automatically logs in in current user if they allowed to do that "]
pub struct InlineKeyboardButtonTypeLoginUrl {
    #[doc = "An HTTP URL to open "]
    pub url: String,
    #[doc = "Unique button identifier "]
    pub id: i32,
    #[doc = "If non-empty, new text of the button in forwarded messages"]
    pub forward_text: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A button that sends a special callback query to a bot "]
pub struct InlineKeyboardButtonTypeCallback {
    #[doc = "Data to be sent to the bot via a callback query"]
    pub data: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A button with a game that sends a special callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame"]
pub struct InlineKeyboardButtonTypeCallbackGame {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A button that forces an inline query to the bot to be inserted in the input field "]
pub struct InlineKeyboardButtonTypeSwitchInline {
    #[doc = "Inline query to be sent to the bot "]
    pub query: String,
    #[doc = "True, if the inline query should be sent from the current chat"]
    #[serde(default)]
    pub in_current_chat: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageInvoice"]
pub struct InlineKeyboardButtonTypeBuy {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a single button in an inline keyboard "]
pub struct InlineKeyboardButton {
    #[doc = "Text of the button "]
    pub text: String,
    #[serde(rename = "type")]
    #[doc = "Type of the button"]
    pub type_: InlineKeyboardButtonType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Instructs clients to remove the keyboard once this message has been received. This kind of keyboard can't be received in an incoming message; instead, UpdateChatReplyMarkup with message_id == 0 will be sent"]
pub struct ReplyMarkupRemoveKeyboard {
    #[doc = "True, if the keyboard is removed only for the mentioned users or the target user of a reply"]
    #[serde(default)]
    pub is_personal: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Instructs clients to force a reply to this message"]
pub struct ReplyMarkupForceReply {
    #[doc = "True, if a forced reply must automatically be shown to the current user. For outgoing messages, specify true to show the forced reply only for the mentioned users and for the target user of a reply"]
    #[serde(default)]
    pub is_personal: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a custom keyboard layout to quickly reply to bots"]
pub struct ReplyMarkupShowKeyboard {
    #[doc = "A list of rows of bot keyboard buttons"]
    pub rows: Vec<Vec<KeyboardButton>>,
    #[doc = "True, if the client needs to resize the keyboard vertically"]
    #[serde(default)]
    pub resize_keyboard: bool,
    #[doc = "True, if the client needs to hide the keyboard after use"]
    #[serde(default)]
    pub one_time: bool,
    #[doc = "True, if the keyboard must automatically be shown to the current user. For outgoing messages, specify true to show the keyboard only for the mentioned users and for the target user of a reply"]
    #[serde(default)]
    pub is_personal: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains an inline keyboard layout"]
pub struct ReplyMarkupInlineKeyboard {
    #[doc = "A list of rows of inline keyboard buttons"]
    pub rows: Vec<Vec<InlineKeyboardButton>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An HTTP url needs to be open "]
pub struct LoginUrlInfoOpen {
    #[doc = "The URL to open "]
    pub url: String,
    #[doc = "True, if there is no need to show an ordinary open URL confirm"]
    #[serde(default)]
    pub skip_confirm: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An authorization confirmation dialog needs to be shown to the user "]
pub struct LoginUrlInfoRequestConfirmation {
    #[doc = "An HTTP URL to be opened "]
    pub url: String,
    #[doc = "A domain of the URL"]
    pub domain: String,
    #[doc = "User identifier of a bot linked with the website "]
    pub bot_user_id: i32,
    #[doc = "True, if the user needs to be requested to give the permission to the bot to send them messages"]
    #[serde(default)]
    pub request_write_access: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A plain text "]
pub struct RichTextPlain {
    #[doc = "Text"]
    pub text: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A bold rich text "]
pub struct RichTextBold {
    #[doc = "Text"]
    pub text: Box<RichText>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An italicized rich text "]
pub struct RichTextItalic {
    #[doc = "Text"]
    pub text: Box<RichText>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An underlined rich text "]
pub struct RichTextUnderline {
    #[doc = "Text"]
    pub text: Box<RichText>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A strikethrough rich text "]
pub struct RichTextStrikethrough {
    #[doc = "Text"]
    pub text: Box<RichText>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A fixed-width rich text "]
pub struct RichTextFixed {
    #[doc = "Text"]
    pub text: Box<RichText>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A rich text URL link "]
pub struct RichTextUrl {
    #[doc = "Text "]
    pub text: Box<RichText>,
    #[doc = "URL "]
    pub url: String,
    #[doc = "True, if the URL has cached instant view server-side"]
    #[serde(default)]
    pub is_cached: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A rich text email link "]
pub struct RichTextEmailAddress {
    #[doc = "Text "]
    pub text: Box<RichText>,
    #[doc = "Email address"]
    pub email_address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A subscript rich text "]
pub struct RichTextSubscript {
    #[doc = "Text"]
    pub text: Box<RichText>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A superscript rich text "]
pub struct RichTextSuperscript {
    #[doc = "Text"]
    pub text: Box<RichText>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A marked rich text "]
pub struct RichTextMarked {
    #[doc = "Text"]
    pub text: Box<RichText>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A rich text phone number "]
pub struct RichTextPhoneNumber {
    #[doc = "Text "]
    pub text: Box<RichText>,
    #[doc = "Phone number"]
    pub phone_number: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A small image inside the text "]
pub struct RichTextIcon {
    #[doc = "The image represented as a document. The image can be in GIF, JPEG or PNG format"]
    pub document: Document,
    #[doc = "Width of a bounding box in which the image should be shown; 0 if unknown"]
    pub width: i32,
    #[doc = "Height of a bounding box in which the image should be shown; 0 if unknown"]
    pub height: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A rich text reference of a text on the same web page "]
pub struct RichTextReference {
    #[doc = "The text "]
    pub text: Box<RichText>,
    #[doc = "The text to show on click "]
    pub reference_text: Box<RichText>,
    #[doc = "An HTTP URL, opening the reference"]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An anchor "]
pub struct RichTextAnchor {
    #[doc = "Anchor name"]
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A link to an anchor on the same web page "]
pub struct RichTextAnchorLink {
    #[doc = "The link text "]
    pub text: Box<RichText>,
    #[doc = "The anchor name. If the name is empty, the link should bring back to top "]
    pub name: String,
    #[doc = "An HTTP URL, opening the anchor"]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A concatenation of rich texts "]
pub struct RichTexts {
    #[doc = "Texts"]
    pub texts: Vec<RichText>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a caption of an instant view web page block, consisting of a text and a trailing credit "]
pub struct PageBlockCaption {
    #[doc = "Content of the caption "]
    pub text: RichText,
    #[doc = "Block credit (like HTML tag <cite>)"]
    pub credit: RichText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes an item of a list page block "]
pub struct PageBlockListItem {
    #[doc = "Item label "]
    pub label: String,
    #[doc = "Item blocks"]
    pub page_blocks: Vec<PageBlock>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The content should be left-aligned"]
pub struct PageBlockHorizontalAlignmentLeft {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The content should be center-aligned"]
pub struct PageBlockHorizontalAlignmentCenter {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The content should be right-aligned"]
pub struct PageBlockHorizontalAlignmentRight {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The content should be top-aligned"]
pub struct PageBlockVerticalAlignmentTop {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The content should be middle-aligned"]
pub struct PageBlockVerticalAlignmentMiddle {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The content should be bottom-aligned"]
pub struct PageBlockVerticalAlignmentBottom {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a cell of a table "]
pub struct PageBlockTableCell {
    #[doc = "Cell text; may be null. If the text is null, then the cell should be invisible "]
    pub text: Option<RichText>,
    #[doc = "True, if it is a header cell"]
    #[serde(default)]
    pub is_header: bool,
    #[doc = "The number of columns the cell should span "]
    pub colspan: i32,
    #[doc = "The number of rows the cell should span"]
    pub rowspan: i32,
    #[doc = "Horizontal cell content alignment "]
    pub align: PageBlockHorizontalAlignment,
    #[doc = "Vertical cell content alignment"]
    pub valign: PageBlockVerticalAlignment,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a related article "]
pub struct PageBlockRelatedArticle {
    #[doc = "Related article URL "]
    pub url: String,
    #[doc = "Article title; may be empty "]
    pub title: String,
    #[doc = "Article description; may be empty"]
    pub description: String,
    #[doc = "Article photo; may be null "]
    pub photo: Option<Photo>,
    #[doc = "Article author; may be empty "]
    pub author: String,
    #[doc = "Point in time (Unix timestamp) when the article was published; 0 if unknown"]
    pub publish_date: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The title of a page "]
pub struct PageBlockTitle {
    #[doc = "Title"]
    pub title: RichText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The subtitle of a page "]
pub struct PageBlockSubtitle {
    #[doc = "Subtitle"]
    pub subtitle: RichText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The author and publishing date of a page "]
pub struct PageBlockAuthorDate {
    #[doc = "Author "]
    pub author: RichText,
    #[doc = "Point in time (Unix timestamp) when the article was published; 0 if unknown"]
    pub publish_date: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A header "]
pub struct PageBlockHeader {
    #[doc = "Header"]
    pub header: RichText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A subheader "]
pub struct PageBlockSubheader {
    #[doc = "Subheader"]
    pub subheader: RichText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A kicker "]
pub struct PageBlockKicker {
    #[doc = "Kicker"]
    pub kicker: RichText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A text paragraph "]
pub struct PageBlockParagraph {
    #[doc = "Paragraph text"]
    pub text: RichText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A preformatted text paragraph "]
pub struct PageBlockPreformatted {
    #[doc = "Paragraph text "]
    pub text: RichText,
    #[doc = "Programming language for which the text should be formatted"]
    pub language: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The footer of a page "]
pub struct PageBlockFooter {
    #[doc = "Footer"]
    pub footer: RichText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An empty block separating a page"]
pub struct PageBlockDivider {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An invisible anchor on a page, which can be used in a URL to open the page from the specified anchor "]
pub struct PageBlockAnchor {
    #[doc = "Name of the anchor"]
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A list of data blocks "]
pub struct PageBlockList {
    #[doc = "The items of the list"]
    pub items: Vec<PageBlockListItem>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A block quote "]
pub struct PageBlockBlockQuote {
    #[doc = "Quote text "]
    pub text: RichText,
    #[doc = "Quote credit"]
    pub credit: RichText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A pull quote "]
pub struct PageBlockPullQuote {
    #[doc = "Quote text "]
    pub text: RichText,
    #[doc = "Quote credit"]
    pub credit: RichText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An animation "]
pub struct PageBlockAnimation {
    #[doc = "Animation file; may be null "]
    pub animation: Option<Animation>,
    #[doc = "Animation caption "]
    pub caption: PageBlockCaption,
    #[doc = "True, if the animation should be played automatically"]
    #[serde(default)]
    pub need_autoplay: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An audio file "]
pub struct PageBlockAudio {
    #[doc = "Audio file; may be null "]
    pub audio: Option<Audio>,
    #[doc = "Audio file caption"]
    pub caption: PageBlockCaption,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A photo "]
pub struct PageBlockPhoto {
    #[doc = "Photo file; may be null "]
    pub photo: Option<Photo>,
    #[doc = "Photo caption "]
    pub caption: PageBlockCaption,
    #[doc = "URL that needs to be opened when the photo is clicked"]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A video "]
pub struct PageBlockVideo {
    #[doc = "Video file; may be null "]
    pub video: Option<Video>,
    #[doc = "Video caption "]
    pub caption: PageBlockCaption,
    #[doc = "True, if the video should be played automatically "]
    #[serde(default)]
    pub need_autoplay: bool,
    #[doc = "True, if the video should be looped"]
    #[serde(default)]
    pub is_looped: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A voice note "]
pub struct PageBlockVoiceNote {
    #[doc = "Voice note; may be null "]
    pub voice_note: Option<VoiceNote>,
    #[doc = "Voice note caption"]
    pub caption: PageBlockCaption,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A page cover "]
pub struct PageBlockCover {
    #[doc = "Cover"]
    pub cover: Box<PageBlock>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An embedded web page "]
pub struct PageBlockEmbedded {
    #[doc = "Web page URL, if available "]
    pub url: String,
    #[doc = "HTML-markup of the embedded page "]
    pub html: String,
    #[doc = "Poster photo, if available; may be null "]
    pub poster_photo: Option<Photo>,
    #[doc = "Block width; 0 if unknown "]
    pub width: i32,
    #[doc = "Block height; 0 if unknown "]
    pub height: i32,
    #[doc = "Block caption "]
    pub caption: PageBlockCaption,
    #[doc = "True, if the block should be full width "]
    #[serde(default)]
    pub is_full_width: bool,
    #[doc = "True, if scrolling should be allowed"]
    #[serde(default)]
    pub allow_scrolling: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An embedded post "]
pub struct PageBlockEmbeddedPost {
    #[doc = "Web page URL "]
    pub url: String,
    #[doc = "Post author "]
    pub author: String,
    #[doc = "Post author photo; may be null "]
    pub author_photo: Option<Photo>,
    #[doc = "Point in time (Unix timestamp) when the post was created; 0 if unknown "]
    pub date: i32,
    #[doc = "Post content "]
    pub page_blocks: Vec<PageBlock>,
    #[doc = "Post caption"]
    pub caption: PageBlockCaption,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A collage "]
pub struct PageBlockCollage {
    #[doc = "Collage item contents "]
    pub page_blocks: Vec<PageBlock>,
    #[doc = "Block caption"]
    pub caption: PageBlockCaption,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A slideshow "]
pub struct PageBlockSlideshow {
    #[doc = "Slideshow item contents "]
    pub page_blocks: Vec<PageBlock>,
    #[doc = "Block caption"]
    pub caption: PageBlockCaption,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A link to a chat "]
pub struct PageBlockChatLink {
    #[doc = "Chat title "]
    pub title: String,
    #[doc = "Chat photo; may be null "]
    pub photo: Option<ChatPhoto>,
    #[doc = "Chat username, by which all other information about the chat should be resolved"]
    pub username: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A table "]
pub struct PageBlockTable {
    #[doc = "Table caption "]
    pub caption: RichText,
    #[doc = "Table cells "]
    pub cells: Vec<Vec<PageBlockTableCell>>,
    #[doc = "True, if the table is bordered "]
    #[serde(default)]
    pub is_bordered: bool,
    #[doc = "True, if the table is striped"]
    #[serde(default)]
    pub is_striped: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A collapsible block "]
pub struct PageBlockDetails {
    #[doc = "Always visible heading for the block "]
    pub header: RichText,
    #[doc = "Block contents "]
    pub page_blocks: Vec<PageBlock>,
    #[doc = "True, if the block is open by default"]
    #[serde(default)]
    pub is_open: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Related articles "]
pub struct PageBlockRelatedArticles {
    #[doc = "Block header "]
    pub header: RichText,
    #[doc = "List of related articles"]
    pub articles: Vec<PageBlockRelatedArticle>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A map "]
pub struct PageBlockMap {
    #[doc = "Location of the map center "]
    pub location: Location,
    #[doc = "Map zoom level "]
    pub zoom: i32,
    #[doc = "Map width "]
    pub width: i32,
    #[doc = "Map height "]
    pub height: i32,
    #[doc = "Block caption"]
    pub caption: PageBlockCaption,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes an instant view page for a web page"]
pub struct WebPageInstantView {
    #[doc = "Content of the web page"]
    pub page_blocks: Vec<PageBlock>,
    #[doc = "Number of the instant view views; 0 if unknown"]
    pub view_count: i32,
    #[doc = "Version of the instant view, currently can be 1 or 2"]
    pub version: i32,
    #[doc = "True, if the instant view must be shown from right to left"]
    #[serde(default)]
    pub is_rtl: bool,
    #[doc = "True, if the instant view contains the full page. A network request might be needed to get the full web page instant view"]
    #[serde(default)]
    pub is_full: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a web page preview"]
pub struct WebPage {
    #[doc = "Original URL of the link"]
    pub url: String,
    #[doc = "URL to display"]
    pub display_url: String,
    #[serde(rename = "type")]
    #[doc = "Type of the web page. Can be: article, photo, audio, video, document, profile, app, or something else"]
    pub type_: String,
    #[doc = "Short name of the site (e.g., Google Docs, App Store)"]
    pub site_name: String,
    #[doc = "Title of the content"]
    pub title: String,
    #[doc = "Description of the content"]
    pub description: FormattedText,
    #[doc = "Image representing the content; may be null"]
    pub photo: Option<Photo>,
    #[doc = "URL to show in the embedded preview"]
    pub embed_url: String,
    #[doc = "MIME type of the embedded preview, (e.g., text/html or video/mp4)"]
    pub embed_type: String,
    #[doc = "Width of the embedded preview"]
    pub embed_width: i32,
    #[doc = "Height of the embedded preview"]
    pub embed_height: i32,
    #[doc = "Duration of the content, in seconds"]
    pub duration: i32,
    #[doc = "Author of the content"]
    pub author: String,
    #[doc = "Preview of the content as an animation, if available; may be null"]
    pub animation: Option<Animation>,
    #[doc = "Preview of the content as an audio file, if available; may be null"]
    pub audio: Option<Audio>,
    #[doc = "Preview of the content as a document, if available (currently only available for small PDF files and ZIP archives); may be null"]
    pub document: Option<Document>,
    #[doc = "Preview of the content as a sticker for small WEBP files, if available; may be null"]
    pub sticker: Option<Sticker>,
    #[doc = "Preview of the content as a video, if available; may be null"]
    pub video: Option<Video>,
    #[doc = "Preview of the content as a video note, if available; may be null"]
    pub video_note: Option<VideoNote>,
    #[doc = "Preview of the content as a voice note, if available; may be null"]
    pub voice_note: Option<VoiceNote>,
    #[doc = "Version of instant view, available for the web page (currently can be 1 or 2), 0 if none"]
    pub instant_view_version: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes an action associated with a bank card number "]
pub struct BankCardActionOpenUrl {
    #[doc = "Action text "]
    pub text: String,
    #[doc = "The URL to be opened"]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Information about a bank card "]
pub struct BankCardInfo {
    #[doc = "Title of the bank card description "]
    pub title: String,
    #[doc = "Actions that can be done with the bank card number"]
    pub actions: Vec<BankCardActionOpenUrl>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes an address "]
pub struct Address {
    #[doc = "A two-letter ISO 3166-1 alpha-2 country code "]
    pub country_code: String,
    #[doc = "State, if applicable "]
    pub state: String,
    #[doc = "City "]
    pub city: String,
    #[doc = "First line of the address "]
    pub street_line1: String,
    #[doc = "Second line of the address "]
    pub street_line2: String,
    #[doc = "Address postal code"]
    pub postal_code: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Portion of the price of a product (e.g., \"delivery cost\", \"tax amount\") "]
pub struct LabeledPricePart {
    #[doc = "Label for this portion of the product price "]
    pub label: String,
    #[doc = "Currency amount in minimal quantity of the currency"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub amount: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Product invoice "]
pub struct Invoice {
    #[doc = "ISO 4217 currency code "]
    pub currency: String,
    #[doc = "A list of objects used to calculate the total price of the product "]
    pub price_parts: Vec<LabeledPricePart>,
    #[doc = "True, if the payment is a test payment"]
    #[serde(default)]
    pub is_test: bool,
    #[doc = "True, if the user's name is needed for payment "]
    #[serde(default)]
    pub need_name: bool,
    #[doc = "True, if the user's phone number is needed for payment "]
    #[serde(default)]
    pub need_phone_number: bool,
    #[doc = "True, if the user's email address is needed for payment"]
    #[serde(default)]
    pub need_email_address: bool,
    #[doc = "True, if the user's shipping address is needed for payment "]
    #[serde(default)]
    pub need_shipping_address: bool,
    #[doc = "True, if the user's phone number will be sent to the provider"]
    #[serde(default)]
    pub send_phone_number_to_provider: bool,
    #[doc = "True, if the user's email address will be sent to the provider "]
    #[serde(default)]
    pub send_email_address_to_provider: bool,
    #[doc = "True, if the total price depends on the shipping method"]
    #[serde(default)]
    pub is_flexible: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Order information "]
pub struct OrderInfo {
    #[doc = "Name of the user "]
    pub name: String,
    #[doc = "Phone number of the user "]
    pub phone_number: String,
    #[doc = "Email address of the user "]
    pub email_address: String,
    #[doc = "Shipping address for this order; may be null"]
    pub shipping_address: Option<Address>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "One shipping option "]
pub struct ShippingOption {
    #[doc = "Shipping option identifier "]
    pub id: String,
    #[doc = "Option title "]
    pub title: String,
    #[doc = "A list of objects used to calculate the total shipping costs"]
    pub price_parts: Vec<LabeledPricePart>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about saved card credentials "]
pub struct SavedCredentials {
    #[doc = "Unique identifier of the saved credentials "]
    pub id: String,
    #[doc = "Title of the saved credentials"]
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password "]
pub struct InputCredentialsSaved {
    #[doc = "Identifier of the saved credentials"]
    pub saved_credentials_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Applies if a user enters new credentials on a payment provider website "]
pub struct InputCredentialsNew {
    #[doc = "Contains JSON-encoded data with a credential identifier from the payment provider "]
    pub data: String,
    #[doc = "True, if the credential identifier can be saved on the server side"]
    #[serde(default)]
    pub allow_save: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Applies if a user enters new credentials using Android Pay "]
pub struct InputCredentialsAndroidPay {
    #[doc = "JSON-encoded data with the credential identifier"]
    pub data: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Applies if a user enters new credentials using Apple Pay "]
pub struct InputCredentialsApplePay {
    #[doc = "JSON-encoded data with the credential identifier"]
    pub data: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Stripe payment provider "]
pub struct PaymentsProviderStripe {
    #[doc = "Stripe API publishable key "]
    pub publishable_key: String,
    #[doc = "True, if the user country must be provided "]
    #[serde(default)]
    pub need_country: bool,
    #[doc = "True, if the user ZIP/postal code must be provided "]
    #[serde(default)]
    pub need_postal_code: bool,
    #[doc = "True, if the cardholder name must be provided"]
    #[serde(default)]
    pub need_cardholder_name: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about an invoice payment form "]
pub struct PaymentForm {
    #[doc = "Full information of the invoice "]
    pub invoice: Invoice,
    #[doc = "Payment form URL "]
    pub url: String,
    #[doc = "Contains information about the payment provider, if available, to support it natively without the need for opening the URL; may be null"]
    pub payments_provider: Option<PaymentsProviderStripe>,
    #[doc = "Saved server-side order information; may be null "]
    pub saved_order_info: Option<OrderInfo>,
    #[doc = "Contains information about saved card credentials; may be null "]
    pub saved_credentials: Option<SavedCredentials>,
    #[doc = "True, if the user can choose to save credentials "]
    #[serde(default)]
    pub can_save_credentials: bool,
    #[doc = "True, if the user will be able to save credentials protected by a password they set up"]
    #[serde(default)]
    pub need_password: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a temporary identifier of validated order information, which is stored for one hour. Also contains the available shipping options "]
pub struct ValidatedOrderInfo {
    #[doc = "Temporary identifier of the order information "]
    pub order_info_id: String,
    #[doc = "Available shipping options"]
    pub shipping_options: Vec<ShippingOption>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains the result of a payment request "]
pub struct PaymentResult {
    #[doc = "True, if the payment request was successful; otherwise the verification_url will be not empty "]
    #[serde(default)]
    pub success: bool,
    #[doc = "URL for additional payment credentials verification"]
    pub verification_url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a successful payment "]
pub struct PaymentReceipt {
    #[doc = "Point in time (Unix timestamp) when the payment was made "]
    pub date: i32,
    #[doc = "User identifier of the payment provider bot "]
    pub payments_provider_user_id: i32,
    #[doc = "Contains information about the invoice"]
    pub invoice: Invoice,
    #[doc = "Contains order information; may be null "]
    pub order_info: Option<OrderInfo>,
    #[doc = "Chosen shipping option; may be null "]
    pub shipping_option: Option<ShippingOption>,
    #[doc = "Title of the saved credentials"]
    pub credentials_title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "File with the date it was uploaded "]
pub struct DatedFile {
    #[doc = "The file "]
    pub file: File,
    #[doc = "Point in time (Unix timestamp) when the file was uploaded"]
    pub date: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's personal details"]
pub struct PassportElementTypePersonalDetails {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's passport"]
pub struct PassportElementTypePassport {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's driver license"]
pub struct PassportElementTypeDriverLicense {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's identity card"]
pub struct PassportElementTypeIdentityCard {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's internal passport"]
pub struct PassportElementTypeInternalPassport {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's address"]
pub struct PassportElementTypeAddress {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's utility bill"]
pub struct PassportElementTypeUtilityBill {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's bank statement"]
pub struct PassportElementTypeBankStatement {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's rental agreement"]
pub struct PassportElementTypeRentalAgreement {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the registration page of the user's passport"]
pub struct PassportElementTypePassportRegistration {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's temporary registration"]
pub struct PassportElementTypeTemporaryRegistration {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's phone number"]
pub struct PassportElementTypePhoneNumber {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's email address"]
pub struct PassportElementTypeEmailAddress {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a date according to the Gregorian calendar "]
pub struct Date {
    #[doc = "Day of the month, 1-31 "]
    pub day: i32,
    #[doc = "Month, 1-12 "]
    pub month: i32,
    #[doc = "Year, 1-9999"]
    pub year: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains the user's personal details"]
pub struct PersonalDetails {
    #[doc = "First name of the user written in English; 1-255 characters "]
    pub first_name: String,
    #[doc = "Middle name of the user written in English; 0-255 characters "]
    pub middle_name: String,
    #[doc = "Last name of the user written in English; 1-255 characters"]
    pub last_name: String,
    #[doc = "Native first name of the user; 1-255 characters "]
    pub native_first_name: String,
    #[doc = "Native middle name of the user; 0-255 characters "]
    pub native_middle_name: String,
    #[doc = "Native last name of the user; 1-255 characters"]
    pub native_last_name: String,
    #[doc = "Birthdate of the user "]
    pub birthdate: Date,
    #[doc = "Gender of the user, \"male\" or \"female\" "]
    pub gender: String,
    #[doc = "A two-letter ISO 3166-1 alpha-2 country code of the user's country "]
    pub country_code: String,
    #[doc = "A two-letter ISO 3166-1 alpha-2 country code of the user's residence country"]
    pub residence_country_code: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An identity document "]
pub struct IdentityDocument {
    #[doc = "Document number; 1-24 characters "]
    pub number: String,
    #[doc = "Document expiry date; may be null "]
    pub expiry_date: Option<Date>,
    #[doc = "Front side of the document"]
    pub front_side: DatedFile,
    #[doc = "Reverse side of the document; only for driver license and identity card "]
    pub reverse_side: DatedFile,
    #[doc = "Selfie with the document; may be null "]
    pub selfie: Option<DatedFile>,
    #[doc = "List of files containing a certified English translation of the document"]
    pub translation: Vec<DatedFile>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An identity document to be saved to Telegram Passport "]
pub struct InputIdentityDocument {
    #[doc = "Document number; 1-24 characters "]
    pub number: String,
    #[doc = "Document expiry date, if available "]
    pub expiry_date: Date,
    #[doc = "Front side of the document"]
    pub front_side: InputFile,
    #[doc = "Reverse side of the document; only for driver license and identity card "]
    pub reverse_side: InputFile,
    #[doc = "Selfie with the document, if available "]
    pub selfie: InputFile,
    #[doc = "List of files containing a certified English translation of the document"]
    pub translation: Vec<InputFile>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A personal document, containing some information about a user "]
pub struct PersonalDocument {
    #[doc = "List of files containing the pages of the document "]
    pub files: Vec<DatedFile>,
    #[doc = "List of files containing a certified English translation of the document"]
    pub translation: Vec<DatedFile>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A personal document to be saved to Telegram Passport "]
pub struct InputPersonalDocument {
    #[doc = "List of files containing the pages of the document "]
    pub files: Vec<InputFile>,
    #[doc = "List of files containing a certified English translation of the document"]
    pub translation: Vec<InputFile>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's personal details "]
pub struct PassportElementPersonalDetails {
    #[doc = "Personal details of the user"]
    pub personal_details: PersonalDetails,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's passport "]
pub struct PassportElementPassport {
    #[doc = "Passport"]
    pub passport: IdentityDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's driver license "]
pub struct PassportElementDriverLicense {
    #[doc = "Driver license"]
    pub driver_license: IdentityDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's identity card "]
pub struct PassportElementIdentityCard {
    #[doc = "Identity card"]
    pub identity_card: IdentityDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's internal passport "]
pub struct PassportElementInternalPassport {
    #[doc = "Internal passport"]
    pub internal_passport: IdentityDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's address "]
pub struct PassportElementAddress {
    #[doc = "Address"]
    pub address: Address,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's utility bill "]
pub struct PassportElementUtilityBill {
    #[doc = "Utility bill"]
    pub utility_bill: PersonalDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's bank statement "]
pub struct PassportElementBankStatement {
    #[doc = "Bank statement"]
    pub bank_statement: PersonalDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's rental agreement "]
pub struct PassportElementRentalAgreement {
    #[doc = "Rental agreement"]
    pub rental_agreement: PersonalDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's passport registration pages "]
pub struct PassportElementPassportRegistration {
    #[doc = "Passport registration pages"]
    pub passport_registration: PersonalDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's temporary registration "]
pub struct PassportElementTemporaryRegistration {
    #[doc = "Temporary registration"]
    pub temporary_registration: PersonalDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's phone number "]
pub struct PassportElementPhoneNumber {
    #[doc = "Phone number"]
    pub phone_number: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element containing the user's email address "]
pub struct PassportElementEmailAddress {
    #[doc = "Email address"]
    pub email_address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's personal details "]
pub struct InputPassportElementPersonalDetails {
    #[doc = "Personal details of the user"]
    pub personal_details: PersonalDetails,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's passport "]
pub struct InputPassportElementPassport {
    #[doc = "The passport to be saved"]
    pub passport: InputIdentityDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's driver license "]
pub struct InputPassportElementDriverLicense {
    #[doc = "The driver license to be saved"]
    pub driver_license: InputIdentityDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's identity card "]
pub struct InputPassportElementIdentityCard {
    #[doc = "The identity card to be saved"]
    pub identity_card: InputIdentityDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's internal passport "]
pub struct InputPassportElementInternalPassport {
    #[doc = "The internal passport to be saved"]
    pub internal_passport: InputIdentityDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's address "]
pub struct InputPassportElementAddress {
    #[doc = "The address to be saved"]
    pub address: Address,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's utility bill "]
pub struct InputPassportElementUtilityBill {
    #[doc = "The utility bill to be saved"]
    pub utility_bill: InputPersonalDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's bank statement "]
pub struct InputPassportElementBankStatement {
    #[doc = "The bank statement to be saved"]
    pub bank_statement: InputPersonalDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's rental agreement "]
pub struct InputPassportElementRentalAgreement {
    #[doc = "The rental agreement to be saved"]
    pub rental_agreement: InputPersonalDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's passport registration "]
pub struct InputPassportElementPassportRegistration {
    #[doc = "The passport registration page to be saved"]
    pub passport_registration: InputPersonalDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's temporary registration "]
pub struct InputPassportElementTemporaryRegistration {
    #[doc = "The temporary registration document to be saved"]
    pub temporary_registration: InputPersonalDocument,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's phone number "]
pub struct InputPassportElementPhoneNumber {
    #[doc = "The phone number to be saved"]
    pub phone_number: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Telegram Passport element to be saved containing the user's email address "]
pub struct InputPassportElementEmailAddress {
    #[doc = "The email address to be saved"]
    pub email_address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about saved Telegram Passport elements "]
pub struct PassportElements {
    #[doc = "Telegram Passport elements"]
    pub elements: Vec<PassportElement>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The element contains an error in an unspecified place. The error will be considered resolved when new data is added"]
pub struct PassportElementErrorSourceUnspecified {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "One of the data fields contains an error. The error will be considered resolved when the value of the field changes "]
pub struct PassportElementErrorSourceDataField {
    #[doc = "Field name"]
    pub field_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The front side of the document contains an error. The error will be considered resolved when the file with the front side changes"]
pub struct PassportElementErrorSourceFrontSide {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The reverse side of the document contains an error. The error will be considered resolved when the file with the reverse side changes"]
pub struct PassportElementErrorSourceReverseSide {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The selfie with the document contains an error. The error will be considered resolved when the file with the selfie changes"]
pub struct PassportElementErrorSourceSelfie {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "One of files with the translation of the document contains an error. The error will be considered resolved when the file changes "]
pub struct PassportElementErrorSourceTranslationFile {
    #[doc = "Index of a file with the error"]
    pub file_index: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The translation of the document contains an error. The error will be considered resolved when the list of translation files changes"]
pub struct PassportElementErrorSourceTranslationFiles {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file contains an error. The error will be considered resolved when the file changes "]
pub struct PassportElementErrorSourceFile {
    #[doc = "Index of a file with the error"]
    pub file_index: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The list of attached files contains an error. The error will be considered resolved when the list of files changes"]
pub struct PassportElementErrorSourceFiles {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains the description of an error in a Telegram Passport element "]
pub struct PassportElementError {
    #[serde(rename = "type")]
    #[doc = "Type of the Telegram Passport element which has the error "]
    pub type_: PassportElementType,
    #[doc = "Error message "]
    pub message: String,
    #[doc = "Error source"]
    pub source: PassportElementErrorSource,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a Telegram Passport element that was requested by a service "]
pub struct PassportSuitableElement {
    #[serde(rename = "type")]
    #[doc = "Type of the element "]
    pub type_: PassportElementType,
    #[doc = "True, if a selfie is required with the identity document"]
    #[serde(default)]
    pub is_selfie_required: bool,
    #[doc = "True, if a certified English translation is required with the document "]
    #[serde(default)]
    pub is_translation_required: bool,
    #[doc = "True, if personal details must include the user's name in the language of their country of residence"]
    #[serde(default)]
    pub is_native_name_required: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a description of the required Telegram Passport element that was requested by a service "]
pub struct PassportRequiredElement {
    #[doc = "List of Telegram Passport elements any of which is enough to provide"]
    pub suitable_elements: Vec<PassportSuitableElement>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a Telegram Passport authorization form that was requested "]
pub struct PassportAuthorizationForm {
    #[doc = "Unique identifier of the authorization form"]
    pub id: i32,
    #[doc = "Information about the Telegram Passport elements that need to be provided to complete the form"]
    pub required_elements: Vec<PassportRequiredElement>,
    #[doc = "URL for the privacy policy of the service; may be empty"]
    pub privacy_policy_url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a Telegram Passport elements and corresponding errors "]
pub struct PassportElementsWithErrors {
    #[doc = "Telegram Passport elements "]
    pub elements: Vec<PassportElement>,
    #[doc = "Errors in the elements that are already available"]
    pub errors: Vec<PassportElementError>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains encrypted Telegram Passport data credentials "]
pub struct EncryptedCredentials {
    #[doc = "The encrypted credentials "]
    pub data: String,
    #[doc = "The decrypted data hash "]
    pub hash: String,
    #[doc = "Secret for data decryption, encrypted with the service's public key"]
    pub secret: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about an encrypted Telegram Passport element; for bots only "]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    #[doc = "Type of Telegram Passport element "]
    pub type_: PassportElementType,
    #[doc = "Encrypted JSON-encoded data about the user "]
    pub data: String,
    #[doc = "The front side of an identity document "]
    pub front_side: DatedFile,
    #[doc = "The reverse side of an identity document; may be null "]
    pub reverse_side: Option<DatedFile>,
    #[doc = "Selfie with the document; may be null "]
    pub selfie: Option<DatedFile>,
    #[doc = "List of files containing a certified English translation of the document "]
    pub translation: Vec<DatedFile>,
    #[doc = "List of attached files "]
    pub files: Vec<DatedFile>,
    #[doc = "Unencrypted data, phone number or email address "]
    pub value: String,
    #[doc = "Hash of the entire element"]
    pub hash: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The element contains an error in an unspecified place. The error will be considered resolved when new data is added "]
pub struct InputPassportElementErrorSourceUnspecified {
    #[doc = "Current hash of the entire element"]
    pub element_hash: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A data field contains an error. The error is considered resolved when the field's value changes "]
pub struct InputPassportElementErrorSourceDataField {
    #[doc = "Field name "]
    pub field_name: String,
    #[doc = "Current data hash"]
    pub data_hash: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The front side of the document contains an error. The error is considered resolved when the file with the front side of the document changes "]
pub struct InputPassportElementErrorSourceFrontSide {
    #[doc = "Current hash of the file containing the front side"]
    pub file_hash: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The reverse side of the document contains an error. The error is considered resolved when the file with the reverse side of the document changes "]
pub struct InputPassportElementErrorSourceReverseSide {
    #[doc = "Current hash of the file containing the reverse side"]
    pub file_hash: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The selfie contains an error. The error is considered resolved when the file with the selfie changes "]
pub struct InputPassportElementErrorSourceSelfie {
    #[doc = "Current hash of the file containing the selfie"]
    pub file_hash: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "One of the files containing the translation of the document contains an error. The error is considered resolved when the file with the translation changes "]
pub struct InputPassportElementErrorSourceTranslationFile {
    #[doc = "Current hash of the file containing the translation"]
    pub file_hash: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The translation of the document contains an error. The error is considered resolved when the list of files changes "]
pub struct InputPassportElementErrorSourceTranslationFiles {
    #[doc = "Current hashes of all files with the translation"]
    pub file_hashes: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file contains an error. The error is considered resolved when the file changes "]
pub struct InputPassportElementErrorSourceFile {
    #[doc = "Current hash of the file which has the error"]
    pub file_hash: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The list of attached files contains an error. The error is considered resolved when the file list changes "]
pub struct InputPassportElementErrorSourceFiles {
    #[doc = "Current hashes of all attached files"]
    pub file_hashes: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains the description of an error in a Telegram Passport element; for bots only "]
pub struct InputPassportElementError {
    #[serde(rename = "type")]
    #[doc = "Type of Telegram Passport element that has the error "]
    pub type_: PassportElementType,
    #[doc = "Error message "]
    pub message: String,
    #[doc = "Error source"]
    pub source: InputPassportElementErrorSource,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A text message "]
pub struct MessageText {
    #[doc = "Text of the message "]
    pub text: FormattedText,
    #[doc = "A preview of the web page that's mentioned in the text; may be null"]
    pub web_page: Option<WebPage>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An animation message (GIF-style). "]
pub struct MessageAnimation {
    #[doc = "The animation description "]
    pub animation: Animation,
    #[doc = "Animation caption "]
    pub caption: FormattedText,
    #[doc = "True, if the animation thumbnail must be blurred and the animation must be shown only while tapped"]
    #[serde(default)]
    pub is_secret: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An audio message "]
pub struct MessageAudio {
    #[doc = "The audio description "]
    pub audio: Audio,
    #[doc = "Audio caption"]
    pub caption: FormattedText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A document message (general file) "]
pub struct MessageDocument {
    #[doc = "The document description "]
    pub document: Document,
    #[doc = "Document caption"]
    pub caption: FormattedText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A photo message "]
pub struct MessagePhoto {
    #[doc = "The photo description "]
    pub photo: Photo,
    #[doc = "Photo caption "]
    pub caption: FormattedText,
    #[doc = "True, if the photo must be blurred and must be shown only while tapped"]
    #[serde(default)]
    pub is_secret: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An expired photo message (self-destructed after TTL has elapsed)"]
pub struct MessageExpiredPhoto {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A sticker message "]
pub struct MessageSticker {
    #[doc = "The sticker description"]
    pub sticker: Sticker,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A video message "]
pub struct MessageVideo {
    #[doc = "The video description "]
    pub video: Video,
    #[doc = "Video caption "]
    pub caption: FormattedText,
    #[doc = "True, if the video thumbnail must be blurred and the video must be shown only while tapped"]
    #[serde(default)]
    pub is_secret: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An expired video message (self-destructed after TTL has elapsed)"]
pub struct MessageExpiredVideo {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A video note message "]
pub struct MessageVideoNote {
    #[doc = "The video note description "]
    pub video_note: VideoNote,
    #[doc = "True, if at least one of the recipients has viewed the video note "]
    #[serde(default)]
    pub is_viewed: bool,
    #[doc = "True, if the video note thumbnail must be blurred and the video note must be shown only while tapped"]
    #[serde(default)]
    pub is_secret: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A voice note message "]
pub struct MessageVoiceNote {
    #[doc = "The voice note description "]
    pub voice_note: VoiceNote,
    #[doc = "Voice note caption "]
    pub caption: FormattedText,
    #[doc = "True, if at least one of the recipients has listened to the voice note"]
    #[serde(default)]
    pub is_listened: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a location "]
pub struct MessageLocation {
    #[doc = "The location description "]
    pub location: Location,
    #[doc = "Time relative to the message sent date until which the location can be updated, in seconds"]
    pub live_period: i32,
    #[doc = "Left time for which the location can be updated, in seconds. updateMessageContent is not sent when this field changes"]
    pub expires_in: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with information about a venue "]
pub struct MessageVenue {
    #[doc = "The venue description"]
    pub venue: Venue,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a user contact "]
pub struct MessageContact {
    #[doc = "The contact description"]
    pub contact: Contact,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A dice message. The dice value is randomly generated by the server"]
pub struct MessageDice {
    #[doc = "The animated sticker with the initial dice animation; may be null if unknown. updateMessageContent will be sent when the sticker became known"]
    pub initial_state_sticker: Option<Sticker>,
    #[doc = "The animated sticker with the final dice animation; may be null if unknown. updateMessageContent will be sent when the sticker became known"]
    pub final_state_sticker: Option<Sticker>,
    #[doc = "Emoji on which the dice throw animation is based"]
    pub emoji: String,
    #[doc = "The dice value. If the value is 0, the dice don't have final state yet"]
    pub value: i32,
    #[doc = "Number of frame after which a success animation like a shower of confetti needs to be shown on updateMessageSendSucceeded"]
    pub success_animation_frame_number: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a game "]
pub struct MessageGame {
    #[doc = "The game description"]
    pub game: Game,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a poll "]
pub struct MessagePoll {
    #[doc = "The poll description"]
    pub poll: Poll,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with an invoice from a bot "]
pub struct MessageInvoice {
    #[doc = "Product title "]
    pub title: String,
    #[doc = "Product description "]
    pub description: String,
    #[doc = "Product photo; may be null "]
    pub photo: Option<Photo>,
    #[doc = "Currency for the product price "]
    pub currency: String,
    #[doc = "Product total price in the minimal quantity of the currency"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub total_amount: i64,
    #[doc = "Unique invoice bot start_parameter. To share an invoice use the URL https://t.me/{bot_username}?start={start_parameter} "]
    pub start_parameter: String,
    #[doc = "True, if the invoice is a test invoice"]
    #[serde(default)]
    pub is_test: bool,
    #[doc = "True, if the shipping address should be specified "]
    #[serde(default)]
    pub need_shipping_address: bool,
    #[doc = "The identifier of the message with the receipt, after the product has been purchased"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub receipt_message_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with information about an ended call "]
pub struct MessageCall {
    #[doc = "Reason why the call was discarded "]
    pub discard_reason: CallDiscardReason,
    #[doc = "Call duration, in seconds"]
    pub duration: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A newly created basic group "]
pub struct MessageBasicGroupChatCreate {
    #[doc = "Title of the basic group "]
    pub title: String,
    #[doc = "User identifiers of members in the basic group"]
    pub member_user_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A newly created supergroup or channel "]
pub struct MessageSupergroupChatCreate {
    #[doc = "Title of the supergroup or channel"]
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An updated chat title "]
pub struct MessageChatChangeTitle {
    #[doc = "New chat title"]
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An updated chat photo "]
pub struct MessageChatChangePhoto {
    #[doc = "New chat photo"]
    pub photo: Photo,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A deleted chat photo"]
pub struct MessageChatDeletePhoto {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "New chat members were added "]
pub struct MessageChatAddMembers {
    #[doc = "User identifiers of the new members"]
    pub member_user_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new member joined the chat by invite link"]
pub struct MessageChatJoinByLink {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat member was deleted "]
pub struct MessageChatDeleteMember {
    #[doc = "User identifier of the deleted chat member"]
    pub user_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A basic group was upgraded to a supergroup and was deactivated as the result "]
pub struct MessageChatUpgradeTo {
    #[doc = "Identifier of the supergroup to which the basic group was upgraded"]
    pub supergroup_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A supergroup has been created from a basic group "]
pub struct MessageChatUpgradeFrom {
    #[doc = "Title of the newly created supergroup "]
    pub title: String,
    #[doc = "The identifier of the original basic group"]
    pub basic_group_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message has been pinned "]
pub struct MessagePinMessage {
    #[doc = "Identifier of the pinned message, can be an identifier of a deleted message or 0"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A screenshot of a message in the chat has been taken"]
pub struct MessageScreenshotTaken {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The TTL (Time To Live) setting messages in a secret chat has been changed "]
pub struct MessageChatSetTtl {
    #[doc = "New TTL"]
    pub ttl: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A non-standard action has happened in the chat "]
pub struct MessageCustomServiceAction {
    #[doc = "Message text to be shown in the chat"]
    pub text: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new high score was achieved in a game "]
pub struct MessageGameScore {
    #[doc = "Identifier of the message with the game, can be an identifier of a deleted message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub game_message_id: i64,
    #[doc = "Identifier of the game; may be different from the games presented in the message with the game "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub game_id: i64,
    #[doc = "New score"]
    pub score: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A payment has been completed "]
pub struct MessagePaymentSuccessful {
    #[doc = "Identifier of the message with the corresponding invoice; can be an identifier of a deleted message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub invoice_message_id: i64,
    #[doc = "Currency for the price of the product "]
    pub currency: String,
    #[doc = "Total price for the product, in the minimal quantity of the currency"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub total_amount: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A payment has been completed; for bots only "]
pub struct MessagePaymentSuccessfulBot {
    #[doc = "Identifier of the message with the corresponding invoice; can be an identifier of a deleted message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub invoice_message_id: i64,
    #[doc = "Currency for price of the product"]
    pub currency: String,
    #[doc = "Total price for the product, in the minimal quantity of the currency "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub total_amount: i64,
    #[doc = "Invoice payload "]
    pub invoice_payload: String,
    #[doc = "Identifier of the shipping option chosen by the user; may be empty if not applicable "]
    pub shipping_option_id: String,
    #[doc = "Information about the order; may be null"]
    pub order_info: Option<OrderInfo>,
    #[doc = "Telegram payment identifier "]
    pub telegram_payment_charge_id: String,
    #[doc = "Provider payment identifier"]
    pub provider_payment_charge_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A contact has registered with Telegram"]
pub struct MessageContactRegistered {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The current user has connected a website by logging in using Telegram Login Widget on it "]
pub struct MessageWebsiteConnected {
    #[doc = "Domain name of the connected website"]
    pub domain_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Telegram Passport data has been sent "]
pub struct MessagePassportDataSent {
    #[doc = "List of Telegram Passport element types sent"]
    pub types: Vec<PassportElementType>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Telegram Passport data has been received; for bots only "]
pub struct MessagePassportDataReceived {
    #[doc = "List of received Telegram Passport elements "]
    pub elements: Vec<EncryptedPassportElement>,
    #[doc = "Encrypted data credentials"]
    pub credentials: EncryptedCredentials,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Message content that is not supported by the client"]
pub struct MessageUnsupported {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A mention of a user by their username"]
pub struct TextEntityTypeMention {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A hashtag text, beginning with \"#\""]
pub struct TextEntityTypeHashtag {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A cashtag text, beginning with \"$\" and consisting of capital english letters (i.e. \"$USD\")"]
pub struct TextEntityTypeCashtag {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A bot command, beginning with \"/\". This shouldn't be highlighted if there are no bots in the chat"]
pub struct TextEntityTypeBotCommand {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An HTTP URL"]
pub struct TextEntityTypeUrl {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An email address"]
pub struct TextEntityTypeEmailAddress {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A phone number"]
pub struct TextEntityTypePhoneNumber {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A bank card number. The getBankCardInfo method can be used to get information about the bank card"]
pub struct TextEntityTypeBankCardNumber {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A bold text"]
pub struct TextEntityTypeBold {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An italic text"]
pub struct TextEntityTypeItalic {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An underlined text"]
pub struct TextEntityTypeUnderline {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A strikethrough text"]
pub struct TextEntityTypeStrikethrough {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Text that must be formatted as if inside a code HTML tag"]
pub struct TextEntityTypeCode {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Text that must be formatted as if inside a pre HTML tag"]
pub struct TextEntityTypePre {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Text that must be formatted as if inside pre, and code HTML tags "]
pub struct TextEntityTypePreCode {
    #[doc = "Programming language of the code; as defined by the sender"]
    pub language: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A text description shown instead of a raw URL "]
pub struct TextEntityTypeTextUrl {
    #[doc = "HTTP or tg:// URL to be opened when the link is clicked"]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A text shows instead of a raw mention of the user (e.g., when the user has no username) "]
pub struct TextEntityTypeMentionName {
    #[doc = "Identifier of the mentioned user"]
    pub user_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A thumbnail to be sent along with a file; should be in JPEG or WEBP format for stickers, and less than 200 KB in size "]
pub struct InputThumbnail {
    #[doc = "Thumbnail file to send. Sending thumbnails by file_id is currently not supported"]
    pub thumbnail: InputFile,
    #[doc = "Thumbnail width, usually shouldn't exceed 320. Use 0 if unknown "]
    pub width: i32,
    #[doc = "Thumbnail height, usually shouldn't exceed 320. Use 0 if unknown"]
    pub height: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The message will be sent at the specified date "]
pub struct MessageSchedulingStateSendAtDate {
    #[doc = "Date the message will be sent. The date must be within 367 days in the future"]
    pub send_date: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The message will be sent when the peer will be online. Applicable to private chats only and when the exact online status of the peer is known"]
pub struct MessageSchedulingStateSendWhenOnline {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Options to be used when a message is send"]
pub struct SendMessageOptions {
    #[doc = "Pass true to disable notification for the message. Must be false if the message is sent to a secret chat"]
    #[serde(default)]
    pub disable_notification: bool,
    #[doc = "Pass true if the message is sent from the background"]
    #[serde(default)]
    pub from_background: bool,
    #[doc = "Message scheduling state. Messages sent to a secret chat, live location messages and self-destructing messages can't be scheduled"]
    pub scheduling_state: MessageSchedulingState,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A text message "]
pub struct InputMessageText {
    #[doc = "Formatted text to be sent; 1-GetOption(\"message_text_length_max\") characters. Only Bold, Italic, Underline, Strikethrough, Code, Pre, PreCode, TextUrl and MentionName entities are allowed to be specified manually"]
    pub text: FormattedText,
    #[doc = "True, if rich web page previews for URLs in the message text should be disabled "]
    #[serde(default)]
    pub disable_web_page_preview: bool,
    #[doc = "True, if a chat message draft should be deleted"]
    #[serde(default)]
    pub clear_draft: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An animation message (GIF-style). "]
pub struct InputMessageAnimation {
    #[doc = "Animation file to be sent "]
    pub animation: InputFile,
    #[doc = "Animation thumbnail, if available "]
    pub thumbnail: InputThumbnail,
    #[doc = "Duration of the animation, in seconds "]
    pub duration: i32,
    #[doc = "Width of the animation; may be replaced by the server "]
    pub width: i32,
    #[doc = "Height of the animation; may be replaced by the server "]
    pub height: i32,
    #[doc = "Animation caption; 0-GetOption(\"message_caption_length_max\") characters"]
    pub caption: FormattedText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An audio message "]
pub struct InputMessageAudio {
    #[doc = "Audio file to be sent "]
    pub audio: InputFile,
    #[doc = "Thumbnail of the cover for the album, if available "]
    pub album_cover_thumbnail: InputThumbnail,
    #[doc = "Duration of the audio, in seconds; may be replaced by the server "]
    pub duration: i32,
    #[doc = "Title of the audio; 0-64 characters; may be replaced by the server"]
    pub title: String,
    #[doc = "Performer of the audio; 0-64 characters, may be replaced by the server "]
    pub performer: String,
    #[doc = "Audio caption; 0-GetOption(\"message_caption_length_max\") characters"]
    pub caption: FormattedText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A document message (general file) "]
pub struct InputMessageDocument {
    #[doc = "Document to be sent "]
    pub document: InputFile,
    #[doc = "Document thumbnail, if available "]
    pub thumbnail: InputThumbnail,
    #[doc = "Document caption; 0-GetOption(\"message_caption_length_max\") characters"]
    pub caption: FormattedText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A photo message "]
pub struct InputMessagePhoto {
    #[doc = "Photo to send "]
    pub photo: InputFile,
    #[doc = "Photo thumbnail to be sent, this is sent to the other party in secret chats only "]
    pub thumbnail: InputThumbnail,
    #[doc = "File identifiers of the stickers added to the photo, if applicable "]
    pub added_sticker_file_ids: Vec<i32>,
    #[doc = "Photo width "]
    pub width: i32,
    #[doc = "Photo height "]
    pub height: i32,
    #[doc = "Photo caption; 0-GetOption(\"message_caption_length_max\") characters"]
    pub caption: FormattedText,
    #[doc = "Photo TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats"]
    pub ttl: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A sticker message "]
pub struct InputMessageSticker {
    #[doc = "Sticker to be sent "]
    pub sticker: InputFile,
    #[doc = "Sticker thumbnail, if available "]
    pub thumbnail: InputThumbnail,
    #[doc = "Sticker width "]
    pub width: i32,
    #[doc = "Sticker height"]
    pub height: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A video message "]
pub struct InputMessageVideo {
    #[doc = "Video to be sent "]
    pub video: InputFile,
    #[doc = "Video thumbnail, if available "]
    pub thumbnail: InputThumbnail,
    #[doc = "File identifiers of the stickers added to the video, if applicable"]
    pub added_sticker_file_ids: Vec<i32>,
    #[doc = "Duration of the video, in seconds "]
    pub duration: i32,
    #[doc = "Video width "]
    pub width: i32,
    #[doc = "Video height "]
    pub height: i32,
    #[doc = "True, if the video should be tried to be streamed"]
    #[serde(default)]
    pub supports_streaming: bool,
    #[doc = "Video caption; 0-GetOption(\"message_caption_length_max\") characters "]
    pub caption: FormattedText,
    #[doc = "Video TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats"]
    pub ttl: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A video note message "]
pub struct InputMessageVideoNote {
    #[doc = "Video note to be sent "]
    pub video_note: InputFile,
    #[doc = "Video thumbnail, if available "]
    pub thumbnail: InputThumbnail,
    #[doc = "Duration of the video, in seconds "]
    pub duration: i32,
    #[doc = "Video width and height; must be positive and not greater than 640"]
    pub length: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A voice note message "]
pub struct InputMessageVoiceNote {
    #[doc = "Voice note to be sent "]
    pub voice_note: InputFile,
    #[doc = "Duration of the voice note, in seconds "]
    pub duration: i32,
    #[doc = "Waveform representation of the voice note, in 5-bit format "]
    pub waveform: String,
    #[doc = "Voice note caption; 0-GetOption(\"message_caption_length_max\") characters"]
    pub caption: FormattedText,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a location "]
pub struct InputMessageLocation {
    #[doc = "Location to be sent "]
    pub location: Location,
    #[doc = "Period for which the location can be updated, in seconds; should be between 60 and 86400 for a live location and 0 otherwise"]
    pub live_period: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with information about a venue "]
pub struct InputMessageVenue {
    #[doc = "Venue to send"]
    pub venue: Venue,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message containing a user contact "]
pub struct InputMessageContact {
    #[doc = "Contact to send"]
    pub contact: Contact,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A dice message "]
pub struct InputMessageDice {
    #[doc = "Emoji on which the dice throw animation is based "]
    pub emoji: String,
    #[doc = "True, if a chat message draft should be deleted"]
    #[serde(default)]
    pub clear_draft: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a game; not supported for channels or secret chats "]
pub struct InputMessageGame {
    #[doc = "User identifier of the bot that owns the game "]
    pub bot_user_id: i32,
    #[doc = "Short name of the game"]
    pub game_short_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with an invoice; can be used only by bots and only in private chats "]
pub struct InputMessageInvoice {
    #[doc = "Invoice "]
    pub invoice: Invoice,
    #[doc = "Product title; 1-32 characters "]
    pub title: String,
    #[doc = "Product description; 0-255 characters "]
    pub description: String,
    #[doc = "Product photo URL; optional "]
    pub photo_url: String,
    #[doc = "Product photo size "]
    pub photo_size: i32,
    #[doc = "Product photo width "]
    pub photo_width: i32,
    #[doc = "Product photo height"]
    pub photo_height: i32,
    #[doc = "The invoice payload "]
    pub payload: String,
    #[doc = "Payment provider token "]
    pub provider_token: String,
    #[doc = "JSON-encoded data about the invoice, which will be shared with the payment provider "]
    pub provider_data: String,
    #[doc = "Unique invoice bot start_parameter for the generation of this invoice"]
    pub start_parameter: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a poll. Polls can't be sent to secret chats. Polls can be sent only to a private chat with a bot "]
pub struct InputMessagePoll {
    #[doc = "Poll question, 1-255 characters "]
    pub question: String,
    #[doc = "List of poll answer options, 2-10 strings 1-100 characters each"]
    pub options: Vec<String>,
    #[doc = "True, if the poll voters are anonymous. Non-anonymous polls can't be sent or forwarded to channels "]
    #[serde(default)]
    pub is_anonymous: bool,
    #[serde(rename = "type")]
    #[doc = "Type of the poll"]
    pub type_: PollType,
    #[doc = "Amount of time the poll will be active after creation, in seconds; for bots only"]
    pub open_period: Option<i32>,
    #[doc = "Point in time (Unix timestamp) when the poll will be automatically closed; for bots only"]
    pub close_date: Option<i32>,
    #[doc = "True, if the poll needs to be sent already closed; for bots only"]
    #[serde(default)]
    pub is_closed: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A forwarded message "]
pub struct InputMessageForwarded {
    #[doc = "Identifier for the chat this forwarded message came from "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub from_chat_id: i64,
    #[doc = "Identifier of the message to forward"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "True, if a game message should be shared within a launched game; applies only to game messages"]
    #[serde(default)]
    pub in_game_share: bool,
    #[doc = "True, if content of the message needs to be copied without a link to the original message. Always true if the message is forwarded to a secret chat"]
    #[serde(default)]
    pub send_copy: bool,
    #[doc = "True, if media caption of the message copy needs to be removed. Ignored if send_copy is false"]
    #[serde(default)]
    pub remove_caption: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns all found messages, no filter is applied"]
pub struct SearchMessagesFilterEmpty {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only animation messages"]
pub struct SearchMessagesFilterAnimation {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only audio messages"]
pub struct SearchMessagesFilterAudio {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only document messages"]
pub struct SearchMessagesFilterDocument {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only photo messages"]
pub struct SearchMessagesFilterPhoto {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only video messages"]
pub struct SearchMessagesFilterVideo {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only voice note messages"]
pub struct SearchMessagesFilterVoiceNote {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only photo and video messages"]
pub struct SearchMessagesFilterPhotoAndVideo {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only messages containing URLs"]
pub struct SearchMessagesFilterUrl {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only messages containing chat photos"]
pub struct SearchMessagesFilterChatPhoto {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only call messages"]
pub struct SearchMessagesFilterCall {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only incoming call messages with missed/declined discard reasons"]
pub struct SearchMessagesFilterMissedCall {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only video note messages"]
pub struct SearchMessagesFilterVideoNote {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only voice and video note messages"]
pub struct SearchMessagesFilterVoiceAndVideoNote {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only messages with mentions of the current user, or messages that are replies to their messages"]
pub struct SearchMessagesFilterMention {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only messages with unread mentions of the current user, or messages that are replies to their messages. When using this filter the results can't be additionally filtered by a query or by the sending user"]
pub struct SearchMessagesFilterUnreadMention {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns only failed to send messages. This filter can be used only if the message database is used"]
pub struct SearchMessagesFilterFailedToSend {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is typing a message"]
pub struct ChatActionTyping {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is recording a video"]
pub struct ChatActionRecordingVideo {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is uploading a video "]
pub struct ChatActionUploadingVideo {
    #[doc = "Upload progress, as a percentage"]
    pub progress: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is recording a voice note"]
pub struct ChatActionRecordingVoiceNote {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is uploading a voice note "]
pub struct ChatActionUploadingVoiceNote {
    #[doc = "Upload progress, as a percentage"]
    pub progress: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is uploading a photo "]
pub struct ChatActionUploadingPhoto {
    #[doc = "Upload progress, as a percentage"]
    pub progress: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is uploading a document "]
pub struct ChatActionUploadingDocument {
    #[doc = "Upload progress, as a percentage"]
    pub progress: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is picking a location or venue to send"]
pub struct ChatActionChoosingLocation {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is picking a contact to send"]
pub struct ChatActionChoosingContact {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user has started to play a game"]
pub struct ChatActionStartPlayingGame {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is recording a video note"]
pub struct ChatActionRecordingVideoNote {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is uploading a video note "]
pub struct ChatActionUploadingVideoNote {
    #[doc = "Upload progress, as a percentage"]
    pub progress: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user has cancelled the previous action"]
pub struct ChatActionCancel {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user status was never changed"]
pub struct UserStatusEmpty {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is online "]
pub struct UserStatusOnline {
    #[doc = "Point in time (Unix timestamp) when the user's online status will expire"]
    pub expires: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is offline "]
pub struct UserStatusOffline {
    #[doc = "Point in time (Unix timestamp) when the user was last online"]
    pub was_online: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user was online recently"]
pub struct UserStatusRecently {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is offline, but was online last week"]
pub struct UserStatusLastWeek {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user is offline, but was online last month"]
pub struct UserStatusLastMonth {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a list of stickers "]
pub struct Stickers {
    #[doc = "List of stickers"]
    pub stickers: Vec<Sticker>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a list of emoji "]
pub struct Emojis {
    #[doc = "List of emojis"]
    pub emojis: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a sticker set"]
pub struct StickerSet {
    #[doc = "Identifier of the sticker set "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "Title of the sticker set "]
    pub title: String,
    #[doc = "Name of the sticker set "]
    pub name: String,
    #[doc = "Sticker set thumbnail in WEBP format with width and height 100; may be null. The file can be downloaded only before the thumbnail is changed"]
    pub thumbnail: Option<PhotoSize>,
    #[doc = "True, if the sticker set has been installed by the current user "]
    #[serde(default)]
    pub is_installed: bool,
    #[doc = "True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously"]
    #[serde(default)]
    pub is_archived: bool,
    #[doc = "True, if the sticker set is official "]
    #[serde(default)]
    pub is_official: bool,
    #[doc = "True, is the stickers in the set are animated "]
    #[serde(default)]
    pub is_animated: bool,
    #[doc = "True, if the stickers in the set are masks "]
    #[serde(default)]
    pub is_masks: bool,
    #[doc = "True for already viewed trending sticker sets"]
    #[serde(default)]
    pub is_viewed: bool,
    #[doc = "List of stickers in this set "]
    pub stickers: Vec<Sticker>,
    #[doc = "A list of emoji corresponding to the stickers in the same order. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object"]
    pub emojis: Vec<Emojis>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents short information about a sticker set"]
pub struct StickerSetInfo {
    #[doc = "Identifier of the sticker set "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "Title of the sticker set "]
    pub title: String,
    #[doc = "Name of the sticker set "]
    pub name: String,
    #[doc = "Sticker set thumbnail in WEBP format with width and height 100; may be null"]
    pub thumbnail: Option<PhotoSize>,
    #[doc = "True, if the sticker set has been installed by current user "]
    #[serde(default)]
    pub is_installed: bool,
    #[doc = "True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously"]
    #[serde(default)]
    pub is_archived: bool,
    #[doc = "True, if the sticker set is official "]
    #[serde(default)]
    pub is_official: bool,
    #[doc = "True, is the stickers in the set are animated "]
    #[serde(default)]
    pub is_animated: bool,
    #[doc = "True, if the stickers in the set are masks "]
    #[serde(default)]
    pub is_masks: bool,
    #[doc = "True for already viewed trending sticker sets"]
    #[serde(default)]
    pub is_viewed: bool,
    #[doc = "Total number of stickers in the set "]
    pub size: i32,
    #[doc = "Contains up to the first 5 stickers from the set, depending on the context. If the client needs more stickers the full set should be requested"]
    pub covers: Vec<Sticker>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a list of sticker sets "]
pub struct StickerSets {
    #[doc = "Approximate total number of sticker sets found "]
    pub total_count: i32,
    #[doc = "List of sticker sets"]
    pub sets: Vec<StickerSetInfo>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The call wasn't discarded, or the reason is unknown"]
pub struct CallDiscardReasonEmpty {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The call was ended before the conversation started. It was cancelled by the caller or missed by the other party"]
pub struct CallDiscardReasonMissed {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The call was ended before the conversation started. It was declined by the other party"]
pub struct CallDiscardReasonDeclined {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The call was ended during the conversation because the users were disconnected"]
pub struct CallDiscardReasonDisconnected {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The call was ended because one of the parties hung up"]
pub struct CallDiscardReasonHungUp {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Specifies the supported call protocols"]
pub struct CallProtocol {
    #[doc = "True, if UDP peer-to-peer connections are supported"]
    #[serde(default)]
    pub udp_p2p: bool,
    #[doc = "True, if connection through UDP reflectors is supported"]
    #[serde(default)]
    pub udp_reflector: bool,
    #[doc = "The minimum supported API layer; use 65"]
    pub min_layer: i32,
    #[doc = "The maximum supported API layer; use 65"]
    pub max_layer: i32,
    #[doc = "List of supported libtgvoip versions"]
    pub library_versions: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes the address of UDP reflectors "]
pub struct CallConnection {
    #[doc = "Reflector identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "IPv4 reflector address "]
    pub ip: String,
    #[doc = "IPv6 reflector address "]
    pub ipv6: String,
    #[doc = "Reflector port number "]
    pub port: i32,
    #[doc = "Connection peer tag"]
    pub peer_tag: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains the call identifier "]
pub struct CallId {
    #[doc = "Call identifier"]
    pub id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The call is pending, waiting to be accepted by a user "]
pub struct CallStatePending {
    #[doc = "True, if the call has already been created by the server "]
    #[serde(default)]
    pub is_created: bool,
    #[doc = "True, if the call has already been received by the other party"]
    #[serde(default)]
    pub is_received: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The call has been answered and encryption keys are being exchanged"]
pub struct CallStateExchangingKeys {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The call is ready to use "]
pub struct CallStateReady {
    #[doc = "Call protocols supported by the peer "]
    pub protocol: CallProtocol,
    #[doc = "Available UDP reflectors "]
    pub connections: Vec<CallConnection>,
    #[doc = "A JSON-encoded call config "]
    pub config: String,
    #[doc = "Call encryption key "]
    pub encryption_key: String,
    #[doc = "Encryption key emojis fingerprint "]
    pub emojis: Vec<String>,
    #[doc = "True, if peer-to-peer connection is allowed by users privacy settings"]
    #[serde(default)]
    pub allow_p2p: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The call is hanging up after discardCall has been called"]
pub struct CallStateHangingUp {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The call has ended successfully "]
pub struct CallStateDiscarded {
    #[doc = "The reason, why the call has ended "]
    pub reason: CallDiscardReason,
    #[doc = "True, if the call rating should be sent to the server "]
    #[serde(default)]
    pub need_rating: bool,
    #[doc = "True, if the call debug information should be sent to the server"]
    #[serde(default)]
    pub need_debug_information: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The call has ended with an error "]
pub struct CallStateError {
    #[doc = "Error. An error with the code 4005000 will be returned if an outgoing call is missed because of an expired timeout"]
    pub error: Error,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user heard their own voice"]
pub struct CallProblemEcho {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user heard background noise"]
pub struct CallProblemNoise {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The other side kept disappearing"]
pub struct CallProblemInterruptions {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The speech was distorted"]
pub struct CallProblemDistortedSpeech {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user couldn't hear the other side"]
pub struct CallProblemSilentLocal {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The other side couldn't hear the user"]
pub struct CallProblemSilentRemote {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The call ended unexpectedly"]
pub struct CallProblemDropped {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a call "]
pub struct Call {
    #[doc = "Call identifier, not persistent "]
    pub id: i32,
    #[doc = "Peer user identifier "]
    pub user_id: i32,
    #[doc = "True, if the call is outgoing "]
    #[serde(default)]
    pub is_outgoing: bool,
    #[doc = "Call state"]
    pub state: CallState,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains settings for the authentication of the user's phone number"]
pub struct PhoneNumberAuthenticationSettings {
    #[doc = "Pass true if the authentication code may be sent via flash call to the specified phone number"]
    #[serde(default)]
    pub allow_flash_call: bool,
    #[doc = "Pass true if the authenticated phone number is used on the current device"]
    #[serde(default)]
    pub is_current_phone_number: bool,
    #[doc = "For official applications only. True, if the app can use Android SMS Retriever API (requires Google Play Services >= 10.2) to automatically receive the authentication code from the SMS. See https://developers.google.com/identity/sms-retriever/ for more details"]
    #[serde(default)]
    pub allow_sms_retriever_api: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a list of animations "]
pub struct Animations {
    #[doc = "List of animations"]
    pub animations: Vec<Animation>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents the result of an ImportContacts request "]
pub struct ImportedContacts {
    #[doc = "User identifiers of the imported contacts in the same order as they were specified in the request; 0 if the contact is not yet a registered user"]
    pub user_ids: Vec<i32>,
    #[doc = "The number of users that imported the corresponding contact; 0 for already registered users or if unavailable"]
    pub importer_count: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains an HTTP URL "]
pub struct HttpUrl {
    #[doc = "The URL"]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a link to an animated GIF "]
pub struct InputInlineQueryResultAnimatedGif {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Title of the query result "]
    pub title: String,
    #[doc = "URL of the static result thumbnail (JPEG or GIF), if it exists"]
    pub thumbnail_url: String,
    #[doc = "The URL of the GIF-file (file size must not exceed 1MB) "]
    pub gif_url: String,
    #[doc = "Duration of the GIF, in seconds "]
    pub gif_duration: i32,
    #[doc = "Width of the GIF "]
    pub gif_width: i32,
    #[doc = "Height of the GIF"]
    pub gif_height: i32,
    #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageAnimation, InputMessageLocation, InputMessageVenue or InputMessageContact"]
    pub input_message_content: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a link to an animated (i.e. without sound) H.264/MPEG-4 AVC video "]
pub struct InputInlineQueryResultAnimatedMpeg4 {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Title of the result "]
    pub title: String,
    #[doc = "URL of the static result thumbnail (JPEG or GIF), if it exists"]
    pub thumbnail_url: String,
    #[doc = "The URL of the MPEG4-file (file size must not exceed 1MB) "]
    pub mpeg4_url: String,
    #[doc = "Duration of the video, in seconds "]
    pub mpeg4_duration: i32,
    #[doc = "Width of the video "]
    pub mpeg4_width: i32,
    #[doc = "Height of the video"]
    pub mpeg4_height: i32,
    #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageAnimation, InputMessageLocation, InputMessageVenue or InputMessageContact"]
    pub input_message_content: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a link to an article or web page "]
pub struct InputInlineQueryResultArticle {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "URL of the result, if it exists "]
    pub url: String,
    #[doc = "True, if the URL must be not shown "]
    #[serde(default)]
    pub hide_url: bool,
    #[doc = "Title of the result"]
    pub title: String,
    #[doc = "A short description of the result "]
    pub description: String,
    #[doc = "URL of the result thumbnail, if it exists "]
    pub thumbnail_url: String,
    #[doc = "Thumbnail width, if known "]
    pub thumbnail_width: i32,
    #[doc = "Thumbnail height, if known"]
    pub thumbnail_height: i32,
    #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact"]
    pub input_message_content: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a link to an MP3 audio file "]
pub struct InputInlineQueryResultAudio {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Title of the audio file "]
    pub title: String,
    #[doc = "Performer of the audio file"]
    pub performer: String,
    #[doc = "The URL of the audio file "]
    pub audio_url: String,
    #[doc = "Audio file duration, in seconds"]
    pub audio_duration: i32,
    #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageAudio, InputMessageLocation, InputMessageVenue or InputMessageContact"]
    pub input_message_content: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a user contact "]
pub struct InputInlineQueryResultContact {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "User contact "]
    pub contact: Contact,
    #[doc = "URL of the result thumbnail, if it exists "]
    pub thumbnail_url: String,
    #[doc = "Thumbnail width, if known "]
    pub thumbnail_width: i32,
    #[doc = "Thumbnail height, if known"]
    pub thumbnail_height: i32,
    #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact"]
    pub input_message_content: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a link to a file "]
pub struct InputInlineQueryResultDocument {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Title of the resulting file "]
    pub title: String,
    #[doc = "Short description of the result, if known "]
    pub description: String,
    #[doc = "URL of the file "]
    pub document_url: String,
    #[doc = "MIME type of the file content; only \"application/pdf\" and \"application/zip\" are currently allowed"]
    pub mime_type: String,
    #[doc = "The URL of the file thumbnail, if it exists "]
    pub thumbnail_url: String,
    #[doc = "Width of the thumbnail "]
    pub thumbnail_width: i32,
    #[doc = "Height of the thumbnail"]
    pub thumbnail_height: i32,
    #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageDocument, InputMessageLocation, InputMessageVenue or InputMessageContact"]
    pub input_message_content: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a game "]
pub struct InputInlineQueryResultGame {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Short name of the game "]
    pub game_short_name: String,
    #[doc = "Message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a point on the map "]
pub struct InputInlineQueryResultLocation {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Location result "]
    pub location: Location,
    #[doc = "Amount of time relative to the message sent time until the location can be updated, in seconds "]
    pub live_period: i32,
    #[doc = "Title of the result "]
    pub title: String,
    #[doc = "URL of the result thumbnail, if it exists "]
    pub thumbnail_url: String,
    #[doc = "Thumbnail width, if known "]
    pub thumbnail_width: i32,
    #[doc = "Thumbnail height, if known"]
    pub thumbnail_height: i32,
    #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact"]
    pub input_message_content: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents link to a JPEG image "]
pub struct InputInlineQueryResultPhoto {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Title of the result, if known "]
    pub title: String,
    #[doc = "A short description of the result, if known "]
    pub description: String,
    #[doc = "URL of the photo thumbnail, if it exists"]
    pub thumbnail_url: String,
    #[doc = "The URL of the JPEG photo (photo size must not exceed 5MB) "]
    pub photo_url: String,
    #[doc = "Width of the photo "]
    pub photo_width: i32,
    #[doc = "Height of the photo"]
    pub photo_height: i32,
    #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessagePhoto, InputMessageLocation, InputMessageVenue or InputMessageContact"]
    pub input_message_content: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a link to a WEBP or TGS sticker "]
pub struct InputInlineQueryResultSticker {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "URL of the sticker thumbnail, if it exists"]
    pub thumbnail_url: String,
    #[doc = "The URL of the WEBP or TGS sticker (sticker file size must not exceed 5MB) "]
    pub sticker_url: String,
    #[doc = "Width of the sticker "]
    pub sticker_width: i32,
    #[doc = "Height of the sticker"]
    pub sticker_height: i32,
    #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, inputMessageSticker, InputMessageLocation, InputMessageVenue or InputMessageContact"]
    pub input_message_content: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents information about a venue "]
pub struct InputInlineQueryResultVenue {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Venue result "]
    pub venue: Venue,
    #[doc = "URL of the result thumbnail, if it exists "]
    pub thumbnail_url: String,
    #[doc = "Thumbnail width, if known "]
    pub thumbnail_width: i32,
    #[doc = "Thumbnail height, if known"]
    pub thumbnail_height: i32,
    #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact"]
    pub input_message_content: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a link to a page containing an embedded video player or a video file "]
pub struct InputInlineQueryResultVideo {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Title of the result "]
    pub title: String,
    #[doc = "A short description of the result, if known"]
    pub description: String,
    #[doc = "The URL of the video thumbnail (JPEG), if it exists "]
    pub thumbnail_url: String,
    #[doc = "URL of the embedded video player or video file "]
    pub video_url: String,
    #[doc = "MIME type of the content of the video URL, only \"text/html\" or \"video/mp4\" are currently supported"]
    pub mime_type: String,
    #[doc = "Width of the video "]
    pub video_width: i32,
    #[doc = "Height of the video "]
    pub video_height: i32,
    #[doc = "Video duration, in seconds"]
    pub video_duration: i32,
    #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageVideo, InputMessageLocation, InputMessageVenue or InputMessageContact"]
    pub input_message_content: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a link to an opus-encoded audio file within an OGG container, single channel audio "]
pub struct InputInlineQueryResultVoiceNote {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Title of the voice note"]
    pub title: String,
    #[doc = "The URL of the voice note file "]
    pub voice_note_url: String,
    #[doc = "Duration of the voice note, in seconds"]
    pub voice_note_duration: i32,
    #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageVoiceNote, InputMessageLocation, InputMessageVenue or InputMessageContact"]
    pub input_message_content: InputMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a link to an article or web page "]
pub struct InlineQueryResultArticle {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "URL of the result, if it exists "]
    pub url: String,
    #[doc = "True, if the URL must be not shown "]
    #[serde(default)]
    pub hide_url: bool,
    #[doc = "Title of the result"]
    pub title: String,
    #[doc = "A short description of the result "]
    pub description: String,
    #[doc = "Result thumbnail; may be null"]
    pub thumbnail: Option<PhotoSize>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a user contact "]
pub struct InlineQueryResultContact {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "A user contact "]
    pub contact: Contact,
    #[doc = "Result thumbnail; may be null"]
    pub thumbnail: Option<PhotoSize>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a point on the map "]
pub struct InlineQueryResultLocation {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Location result "]
    pub location: Location,
    #[doc = "Title of the result "]
    pub title: String,
    #[doc = "Result thumbnail; may be null"]
    pub thumbnail: Option<PhotoSize>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents information about a venue "]
pub struct InlineQueryResultVenue {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Venue result "]
    pub venue: Venue,
    #[doc = "Result thumbnail; may be null"]
    pub thumbnail: Option<PhotoSize>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents information about a game "]
pub struct InlineQueryResultGame {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Game result"]
    pub game: Game,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents an animation file "]
pub struct InlineQueryResultAnimation {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Animation file "]
    pub animation: Animation,
    #[doc = "Animation title"]
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents an audio file "]
pub struct InlineQueryResultAudio {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Audio file"]
    pub audio: Audio,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a document "]
pub struct InlineQueryResultDocument {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Document "]
    pub document: Document,
    #[doc = "Document title "]
    pub title: String,
    #[doc = "Document description"]
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a photo "]
pub struct InlineQueryResultPhoto {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Photo "]
    pub photo: Photo,
    #[doc = "Title of the result, if known "]
    pub title: String,
    #[doc = "A short description of the result, if known"]
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a sticker "]
pub struct InlineQueryResultSticker {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Sticker"]
    pub sticker: Sticker,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a video "]
pub struct InlineQueryResultVideo {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Video "]
    pub video: Video,
    #[doc = "Title of the video "]
    pub title: String,
    #[doc = "Description of the video"]
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a voice note "]
pub struct InlineQueryResultVoiceNote {
    #[doc = "Unique identifier of the query result "]
    pub id: String,
    #[doc = "Voice note "]
    pub voice_note: VoiceNote,
    #[doc = "Title of the voice note"]
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query "]
pub struct InlineQueryResults {
    #[doc = "Unique identifier of the inline query "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub inline_query_id: i64,
    #[doc = "The offset for the next request. If empty, there are no more results "]
    pub next_offset: String,
    #[doc = "Results of the query"]
    pub results: Vec<InlineQueryResult>,
    #[doc = "If non-empty, this text should be shown on the button, which opens a private chat with the bot and sends the bot a start message with the switch_pm_parameter "]
    pub switch_pm_text: String,
    #[doc = "Parameter for the bot start message"]
    pub switch_pm_parameter: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The payload from a general callback button "]
pub struct CallbackQueryPayloadData {
    #[doc = "Data that was attached to the callback button"]
    pub data: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The payload from a game callback button "]
pub struct CallbackQueryPayloadGame {
    #[doc = "A short name of the game that was attached to the callback button"]
    pub game_short_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a bot's answer to a callback query "]
pub struct CallbackQueryAnswer {
    #[doc = "Text of the answer "]
    pub text: String,
    #[doc = "True, if an alert should be shown to the user instead of a toast notification "]
    #[serde(default)]
    pub show_alert: bool,
    #[doc = "URL to be opened"]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains the result of a custom request "]
pub struct CustomRequestResult {
    #[doc = "A JSON-serialized result"]
    pub result: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains one row of the game high score table "]
pub struct GameHighScore {
    #[doc = "Position in the high score table "]
    pub position: i32,
    #[doc = "User identifier "]
    pub user_id: i32,
    #[doc = "User score"]
    pub score: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of game high scores "]
pub struct GameHighScores {
    #[doc = "A list of game high scores"]
    pub scores: Vec<GameHighScore>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message was edited "]
pub struct ChatEventMessageEdited {
    #[doc = "The original message before the edit "]
    pub old_message: Message,
    #[doc = "The message after it was edited"]
    pub new_message: Message,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message was deleted "]
pub struct ChatEventMessageDeleted {
    #[doc = "Deleted message"]
    pub message: Message,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A poll in a message was stopped "]
pub struct ChatEventPollStopped {
    #[doc = "The message with the poll"]
    pub message: Message,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message was pinned "]
pub struct ChatEventMessagePinned {
    #[doc = "Pinned message"]
    pub message: Message,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message was unpinned"]
pub struct ChatEventMessageUnpinned {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new member joined the chat"]
pub struct ChatEventMemberJoined {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A member left the chat"]
pub struct ChatEventMemberLeft {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new chat member was invited "]
pub struct ChatEventMemberInvited {
    #[doc = "New member user identifier "]
    pub user_id: i32,
    #[doc = "New member status"]
    pub status: ChatMemberStatus,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat member has gained/lost administrator status, or the list of their administrator privileges has changed "]
pub struct ChatEventMemberPromoted {
    #[doc = "Chat member user identifier "]
    pub user_id: i32,
    #[doc = "Previous status of the chat member "]
    pub old_status: ChatMemberStatus,
    #[doc = "New status of the chat member"]
    pub new_status: ChatMemberStatus,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat member was restricted/unrestricted or banned/unbanned, or the list of their restrictions has changed "]
pub struct ChatEventMemberRestricted {
    #[doc = "Chat member user identifier "]
    pub user_id: i32,
    #[doc = "Previous status of the chat member "]
    pub old_status: ChatMemberStatus,
    #[doc = "New status of the chat member"]
    pub new_status: ChatMemberStatus,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat title was changed "]
pub struct ChatEventTitleChanged {
    #[doc = "Previous chat title "]
    pub old_title: String,
    #[doc = "New chat title"]
    pub new_title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat permissions was changed "]
pub struct ChatEventPermissionsChanged {
    #[doc = "Previous chat permissions "]
    pub old_permissions: ChatPermissions,
    #[doc = "New chat permissions"]
    pub new_permissions: ChatPermissions,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat description was changed "]
pub struct ChatEventDescriptionChanged {
    #[doc = "Previous chat description "]
    pub old_description: String,
    #[doc = "New chat description"]
    pub new_description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat username was changed "]
pub struct ChatEventUsernameChanged {
    #[doc = "Previous chat username "]
    pub old_username: String,
    #[doc = "New chat username"]
    pub new_username: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat photo was changed "]
pub struct ChatEventPhotoChanged {
    #[doc = "Previous chat photo value; may be null "]
    pub old_photo: Option<Photo>,
    #[doc = "New chat photo value; may be null"]
    pub new_photo: Option<Photo>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The can_invite_users permission of a supergroup chat was toggled "]
pub struct ChatEventInvitesToggled {
    #[doc = "New value of can_invite_users permission"]
    #[serde(default)]
    pub can_invite_users: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The linked chat of a supergroup was changed "]
pub struct ChatEventLinkedChatChanged {
    #[doc = "Previous supergroup linked chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub old_linked_chat_id: i64,
    #[doc = "New supergroup linked chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub new_linked_chat_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The slow_mode_delay setting of a supergroup was changed "]
pub struct ChatEventSlowModeDelayChanged {
    #[doc = "Previous value of slow_mode_delay "]
    pub old_slow_mode_delay: i32,
    #[doc = "New value of slow_mode_delay"]
    pub new_slow_mode_delay: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The sign_messages setting of a channel was toggled "]
pub struct ChatEventSignMessagesToggled {
    #[doc = "New value of sign_messages"]
    #[serde(default)]
    pub sign_messages: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The supergroup sticker set was changed "]
pub struct ChatEventStickerSetChanged {
    #[doc = "Previous identifier of the chat sticker set; 0 if none "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub old_sticker_set_id: i64,
    #[doc = "New identifier of the chat sticker set; 0 if none"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub new_sticker_set_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The supergroup location was changed "]
pub struct ChatEventLocationChanged {
    #[doc = "Previous location; may be null "]
    pub old_location: Option<ChatLocation>,
    #[doc = "New location; may be null"]
    pub new_location: Option<ChatLocation>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The is_all_history_available setting of a supergroup was toggled "]
pub struct ChatEventIsAllHistoryAvailableToggled {
    #[doc = "New value of is_all_history_available"]
    #[serde(default)]
    pub is_all_history_available: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a chat event "]
pub struct ChatEvent {
    #[doc = "Chat event identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "Point in time (Unix timestamp) when the event happened "]
    pub date: i32,
    #[doc = "Identifier of the user who performed the action that triggered the event "]
    pub user_id: i32,
    #[doc = "Action performed by the user"]
    pub action: ChatEventAction,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of chat events "]
pub struct ChatEvents {
    #[doc = "List of events"]
    pub events: Vec<ChatEvent>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a set of filters used to obtain a chat event log"]
pub struct ChatEventLogFilters {
    #[doc = "True, if message edits should be returned"]
    #[serde(default)]
    pub message_edits: bool,
    #[doc = "True, if message deletions should be returned"]
    #[serde(default)]
    pub message_deletions: bool,
    #[doc = "True, if pin/unpin events should be returned"]
    #[serde(default)]
    pub message_pins: bool,
    #[doc = "True, if members joining events should be returned"]
    #[serde(default)]
    pub member_joins: bool,
    #[doc = "True, if members leaving events should be returned"]
    #[serde(default)]
    pub member_leaves: bool,
    #[doc = "True, if invited member events should be returned"]
    #[serde(default)]
    pub member_invites: bool,
    #[doc = "True, if member promotion/demotion events should be returned"]
    #[serde(default)]
    pub member_promotions: bool,
    #[doc = "True, if member restricted/unrestricted/banned/unbanned events should be returned"]
    #[serde(default)]
    pub member_restrictions: bool,
    #[doc = "True, if changes in chat information should be returned"]
    #[serde(default)]
    pub info_changes: bool,
    #[doc = "True, if changes in chat settings should be returned"]
    #[serde(default)]
    pub setting_changes: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An ordinary language pack string "]
pub struct LanguagePackStringValueOrdinary {
    #[doc = "String value"]
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A language pack string which has different forms based on the number of some object it mentions. See https://www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more info"]
pub struct LanguagePackStringValuePluralized {
    #[doc = "Value for zero objects "]
    pub zero_value: String,
    #[doc = "Value for one object "]
    pub one_value: String,
    #[doc = "Value for two objects"]
    pub two_value: String,
    #[doc = "Value for few objects "]
    pub few_value: String,
    #[doc = "Value for many objects "]
    pub many_value: String,
    #[doc = "Default value"]
    pub other_value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A deleted language pack string, the value should be taken from the built-in english language pack"]
pub struct LanguagePackStringValueDeleted {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents one language pack string "]
pub struct LanguagePackString {
    #[doc = "String key "]
    pub key: String,
    #[doc = "String value"]
    pub value: LanguagePackStringValue,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of language pack strings "]
pub struct LanguagePackStrings {
    #[doc = "A list of language pack strings"]
    pub strings: Vec<LanguagePackString>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a language pack "]
pub struct LanguagePackInfo {
    #[doc = "Unique language pack identifier"]
    pub id: String,
    #[doc = "Identifier of a base language pack; may be empty. If a string is missed in the language pack, then it should be fetched from base language pack. Unsupported in custom language packs"]
    pub base_language_pack_id: String,
    #[doc = "Language name "]
    pub name: String,
    #[doc = "Name of the language in that language"]
    pub native_name: String,
    #[doc = "A language code to be used to apply plural forms. See https://www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more info"]
    pub plural_code: String,
    #[doc = "True, if the language pack is official "]
    #[serde(default)]
    pub is_official: bool,
    #[doc = "True, if the language pack strings are RTL "]
    #[serde(default)]
    pub is_rtl: bool,
    #[doc = "True, if the language pack is a beta language pack"]
    #[serde(default)]
    pub is_beta: bool,
    #[doc = "True, if the language pack is installed by the current user"]
    #[serde(default)]
    pub is_installed: bool,
    #[doc = "Total number of non-deleted strings from the language pack "]
    pub total_string_count: i32,
    #[doc = "Total number of translated strings from the language pack"]
    pub translated_string_count: i32,
    #[doc = "Total number of non-deleted strings from the language pack available locally "]
    pub local_string_count: i32,
    #[doc = "Link to language translation interface; empty for custom local language packs"]
    pub translation_url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about the current localization target "]
pub struct LocalizationTargetInfo {
    #[doc = "List of available language packs for this application"]
    pub language_packs: Vec<LanguagePackInfo>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A token for Firebase Cloud Messaging "]
pub struct DeviceTokenFirebaseCloudMessaging {
    #[doc = "Device registration token; may be empty to de-register a device "]
    pub token: String,
    #[doc = "True, if push notifications should be additionally encrypted"]
    #[serde(default)]
    pub encrypt: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A token for Apple Push Notification service "]
pub struct DeviceTokenApplePush {
    #[doc = "Device token; may be empty to de-register a device "]
    pub device_token: String,
    #[doc = "True, if App Sandbox is enabled"]
    #[serde(default)]
    pub is_app_sandbox: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A token for Apple Push Notification service VoIP notifications "]
pub struct DeviceTokenApplePushVoIP {
    #[doc = "Device token; may be empty to de-register a device "]
    pub device_token: String,
    #[doc = "True, if App Sandbox is enabled "]
    #[serde(default)]
    pub is_app_sandbox: bool,
    #[doc = "True, if push notifications should be additionally encrypted"]
    #[serde(default)]
    pub encrypt: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A token for Windows Push Notification Services "]
pub struct DeviceTokenWindowsPush {
    #[doc = "The access token that will be used to send notifications; may be empty to de-register a device"]
    pub access_token: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A token for Microsoft Push Notification Service "]
pub struct DeviceTokenMicrosoftPush {
    #[doc = "Push notification channel URI; may be empty to de-register a device"]
    pub channel_uri: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A token for Microsoft Push Notification Service VoIP channel "]
pub struct DeviceTokenMicrosoftPushVoIP {
    #[doc = "Push notification channel URI; may be empty to de-register a device"]
    pub channel_uri: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A token for web Push API "]
pub struct DeviceTokenWebPush {
    #[doc = "Absolute URL exposed by the push service where the application server can send push messages; may be empty to de-register a device"]
    pub endpoint: String,
    #[doc = "Base64url-encoded P-256 elliptic curve Diffie-Hellman public key "]
    pub p256dh_base64url: String,
    #[doc = "Base64url-encoded authentication secret"]
    pub auth_base64url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A token for Simple Push API for Firefox OS "]
pub struct DeviceTokenSimplePush {
    #[doc = "Absolute URL exposed by the push service where the application server can send push messages; may be empty to de-register a device"]
    pub endpoint: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A token for Ubuntu Push Client service "]
pub struct DeviceTokenUbuntuPush {
    #[doc = "Token; may be empty to de-register a device"]
    pub token: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A token for BlackBerry Push Service "]
pub struct DeviceTokenBlackBerryPush {
    #[doc = "Token; may be empty to de-register a device"]
    pub token: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A token for Tizen Push Service "]
pub struct DeviceTokenTizenPush {
    #[doc = "Push service registration identifier; may be empty to de-register a device"]
    pub reg_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification "]
pub struct PushReceiverId {
    #[doc = "The globally unique identifier of push notification subscription"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a solid fill of a background "]
pub struct BackgroundFillSolid {
    #[doc = "A color of the background in the RGB24 format"]
    pub color: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a gradient fill of a background "]
pub struct BackgroundFillGradient {
    #[doc = "A top color of the background in the RGB24 format "]
    pub top_color: i32,
    #[doc = "A bottom color of the background in the RGB24 format"]
    pub bottom_color: i32,
    #[doc = "Clockwise rotation angle of the gradient, in degrees; 0-359. Should be always divisible by 45"]
    pub rotation_angle: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A wallpaper in JPEG format"]
pub struct BackgroundTypeWallpaper {
    #[doc = "True, if the wallpaper must be downscaled to fit in 450x450 square and then box-blurred with radius 12"]
    #[serde(default)]
    pub is_blurred: bool,
    #[doc = "True, if the background needs to be slightly moved when device is tilted"]
    #[serde(default)]
    pub is_moving: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A PNG or TGV (gzipped subset of SVG with MIME type \"application/x-tgwallpattern\") pattern to be combined with the background fill chosen by the user"]
pub struct BackgroundTypePattern {
    #[doc = "Description of the background fill"]
    pub fill: BackgroundFill,
    #[doc = "Intensity of the pattern when it is shown above the filled background, 0-100"]
    pub intensity: i32,
    #[doc = "True, if the background needs to be slightly moved when device is tilted"]
    #[serde(default)]
    pub is_moving: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A filled background "]
pub struct BackgroundTypeFill {
    #[doc = "Description of the background fill"]
    pub fill: BackgroundFill,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a chat background"]
pub struct Background {
    #[doc = "Unique background identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "True, if this is one of default backgrounds"]
    #[serde(default)]
    pub is_default: bool,
    #[doc = "True, if the background is dark and is recommended to be used with dark theme"]
    #[serde(default)]
    pub is_dark: bool,
    #[doc = "Unique background name"]
    pub name: String,
    #[doc = "Document with the background; may be null. Null only for filled backgrounds"]
    pub document: Option<Document>,
    #[serde(rename = "type")]
    #[doc = "Type of the background"]
    pub type_: BackgroundType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of backgrounds "]
pub struct Backgrounds {
    #[doc = "A list of backgrounds"]
    pub backgrounds: Vec<Background>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A background from a local file"]
pub struct InputBackgroundLocal {
    #[doc = "Background file to use. Only inputFileLocal and inputFileGenerated are supported. The file must be in JPEG format for wallpapers and in PNG format for patterns"]
    pub background: InputFile,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A background from the server "]
pub struct InputBackgroundRemote {
    #[doc = "The background identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub background_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of hashtags "]
pub struct Hashtags {
    #[doc = "A list of hashtags"]
    pub hashtags: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The session can be used"]
pub struct CanTransferOwnershipResultOk {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The 2-step verification needs to be enabled first"]
pub struct CanTransferOwnershipResultPasswordNeeded {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The 2-step verification was enabled recently, user needs to wait "]
pub struct CanTransferOwnershipResultPasswordTooFresh {
    #[doc = "Time left before the session can be used to transfer ownership of a chat, in seconds"]
    pub retry_after: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The session was created recently, user needs to wait "]
pub struct CanTransferOwnershipResultSessionTooFresh {
    #[doc = "Time left before the session can be used to transfer ownership of a chat, in seconds"]
    pub retry_after: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The username can be set"]
pub struct CheckChatUsernameResultOk {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The username is invalid"]
pub struct CheckChatUsernameResultUsernameInvalid {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The username is occupied"]
pub struct CheckChatUsernameResultUsernameOccupied {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user has too much chats with username, one of them should be made private first"]
pub struct CheckChatUsernameResultPublicChatsTooMuch {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user can't be a member of a public supergroup"]
pub struct CheckChatUsernameResultPublicGroupsUnavailable {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A general message with hidden content "]
pub struct PushMessageContentHidden {
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An animation message (GIF-style). "]
pub struct PushMessageContentAnimation {
    #[doc = "Message content; may be null "]
    pub animation: Option<Animation>,
    #[doc = "Animation caption "]
    pub caption: String,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An audio message "]
pub struct PushMessageContentAudio {
    #[doc = "Message content; may be null "]
    pub audio: Option<Audio>,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a user contact "]
pub struct PushMessageContentContact {
    #[doc = "Contact's name "]
    pub name: String,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A contact has registered with Telegram"]
pub struct PushMessageContentContactRegistered {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A document message (a general file) "]
pub struct PushMessageContentDocument {
    #[doc = "Message content; may be null "]
    pub document: Option<Document>,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a game "]
pub struct PushMessageContentGame {
    #[doc = "Game title, empty for pinned game message "]
    pub title: String,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new high score was achieved in a game "]
pub struct PushMessageContentGameScore {
    #[doc = "Game title, empty for pinned message "]
    pub title: String,
    #[doc = "New score, 0 for pinned message "]
    pub score: i32,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with an invoice from a bot "]
pub struct PushMessageContentInvoice {
    #[doc = "Product price "]
    pub price: String,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a location "]
pub struct PushMessageContentLocation {
    #[doc = "True, if the location is live "]
    #[serde(default)]
    pub is_live: bool,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A photo message "]
pub struct PushMessageContentPhoto {
    #[doc = "Message content; may be null "]
    pub photo: Option<Photo>,
    #[doc = "Photo caption "]
    pub caption: String,
    #[doc = "True, if the photo is secret "]
    #[serde(default)]
    pub is_secret: bool,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a poll "]
pub struct PushMessageContentPoll {
    #[doc = "Poll question "]
    pub question: String,
    #[doc = "True, if the poll is regular and not in quiz mode "]
    #[serde(default)]
    pub is_regular: bool,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A screenshot of a message in the chat has been taken"]
pub struct PushMessageContentScreenshotTaken {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a sticker "]
pub struct PushMessageContentSticker {
    #[doc = "Message content; may be null "]
    pub sticker: Option<Sticker>,
    #[doc = "Emoji corresponding to the sticker; may be empty "]
    pub emoji: String,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A text message "]
pub struct PushMessageContentText {
    #[doc = "Message text "]
    pub text: String,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A video message "]
pub struct PushMessageContentVideo {
    #[doc = "Message content; may be null "]
    pub video: Option<Video>,
    #[doc = "Video caption "]
    pub caption: String,
    #[doc = "True, if the video is secret "]
    #[serde(default)]
    pub is_secret: bool,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A video note message "]
pub struct PushMessageContentVideoNote {
    #[doc = "Message content; may be null "]
    pub video_note: Option<VideoNote>,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A voice note message "]
pub struct PushMessageContentVoiceNote {
    #[doc = "Message content; may be null "]
    pub voice_note: Option<VoiceNote>,
    #[doc = "True, if the message is a pinned message with the specified content"]
    #[serde(default)]
    pub is_pinned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A newly created basic group"]
pub struct PushMessageContentBasicGroupChatCreate {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "New chat members were invited to a group "]
pub struct PushMessageContentChatAddMembers {
    #[doc = "Name of the added member "]
    pub member_name: String,
    #[doc = "True, if the current user was added to the group"]
    #[serde(default)]
    pub is_current_user: bool,
    #[doc = "True, if the user has returned to the group themself"]
    #[serde(default)]
    pub is_returned: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat photo was edited"]
pub struct PushMessageContentChatChangePhoto {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat title was edited "]
pub struct PushMessageContentChatChangeTitle {
    #[doc = "New chat title"]
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat member was deleted "]
pub struct PushMessageContentChatDeleteMember {
    #[doc = "Name of the deleted member "]
    pub member_name: String,
    #[doc = "True, if the current user was deleted from the group"]
    #[serde(default)]
    pub is_current_user: bool,
    #[doc = "True, if the user has left the group themself"]
    #[serde(default)]
    pub is_left: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new member joined the chat by invite link"]
pub struct PushMessageContentChatJoinByLink {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A forwarded messages "]
pub struct PushMessageContentMessageForwards {
    #[doc = "Number of forwarded messages"]
    pub total_count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A media album "]
pub struct PushMessageContentMediaAlbum {
    #[doc = "Number of messages in the album "]
    pub total_count: i32,
    #[doc = "True, if the album has at least one photo "]
    #[serde(default)]
    pub has_photos: bool,
    #[doc = "True, if the album has at least one video"]
    #[serde(default)]
    pub has_videos: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "New message was received "]
pub struct NotificationTypeNewMessage {
    #[doc = "The message"]
    pub message: Message,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "New secret chat was created"]
pub struct NotificationTypeNewSecretChat {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "New call was received "]
pub struct NotificationTypeNewCall {
    #[doc = "Call identifier"]
    pub call_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "New message was received through a push notification"]
pub struct NotificationTypeNewPushMessage {
    #[doc = "The message identifier. The message will not be available in the chat history, but the ID can be used in viewMessages and as reply_to_message_id"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "Sender of the message; 0 if unknown. Corresponding user may be inaccessible"]
    pub sender_user_id: i32,
    #[doc = "Name of the sender; can be different from the name of the sender user"]
    pub sender_name: String,
    #[doc = "True, if the message is outgoing"]
    #[serde(default)]
    pub is_outgoing: bool,
    #[doc = "Push message content"]
    pub content: PushMessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with ordinary unread messages"]
pub struct NotificationGroupTypeMessages {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with unread mentions of the current user, replies to their messages, or a pinned message"]
pub struct NotificationGroupTypeMentions {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A group containing a notification of type notificationTypeNewSecretChat"]
pub struct NotificationGroupTypeSecretChat {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A group containing notifications of type notificationTypeNewCall"]
pub struct NotificationGroupTypeCalls {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a notification "]
pub struct Notification {
    #[doc = "Unique persistent identifier of this notification "]
    pub id: i32,
    #[doc = "Notification date"]
    pub date: i32,
    #[doc = "True, if the notification was initially silent "]
    #[serde(default)]
    pub is_silent: bool,
    #[serde(rename = "type")]
    #[doc = "Notification type"]
    pub type_: NotificationType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes a group of notifications "]
pub struct NotificationGroup {
    #[doc = "Unique persistent auto-incremented from 1 identifier of the notification group "]
    pub id: i32,
    #[serde(rename = "type")]
    #[doc = "Type of the group"]
    pub type_: NotificationGroupType,
    #[doc = "Identifier of a chat to which all notifications in the group belong"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Total number of active notifications in the group "]
    pub total_count: i32,
    #[doc = "The list of active notifications"]
    pub notifications: Vec<Notification>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a boolean option "]
pub struct OptionValueBoolean {
    #[doc = "The value of the option"]
    #[serde(default)]
    pub value: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents an unknown option or an option which has a default value"]
pub struct OptionValueEmpty {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents an integer option "]
pub struct OptionValueInteger {
    #[doc = "The value of the option"]
    pub value: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a string option "]
pub struct OptionValueString {
    #[doc = "The value of the option"]
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents one member of a JSON object "]
pub struct JsonObjectMember {
    #[doc = "Member's key "]
    pub key: String,
    #[doc = "Member's value"]
    pub value: JsonValue,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a null JSON value"]
pub struct JsonValueNull {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a boolean JSON value "]
pub struct JsonValueBoolean {
    #[doc = "The value"]
    #[serde(default)]
    pub value: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a numeric JSON value "]
pub struct JsonValueNumber {
    #[doc = "The value"]
    pub value: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a string JSON value "]
pub struct JsonValueString {
    #[doc = "The value"]
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a JSON array "]
pub struct JsonValueArray {
    #[doc = "The list of array elements"]
    pub values: Vec<JsonValue>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a JSON object "]
pub struct JsonValueObject {
    #[doc = "The list of object members"]
    pub members: Vec<JsonObjectMember>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A rule to allow all users to do something"]
pub struct UserPrivacySettingRuleAllowAll {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A rule to allow all of a user's contacts to do something"]
pub struct UserPrivacySettingRuleAllowContacts {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A rule to allow certain specified users to do something "]
pub struct UserPrivacySettingRuleAllowUsers {
    #[doc = "The user identifiers, total number of users in all rules must not exceed 1000"]
    pub user_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A rule to allow all members of certain specified basic groups and supergroups to doing something "]
pub struct UserPrivacySettingRuleAllowChatMembers {
    #[doc = "The chat identifiers, total number of chats in all rules must not exceed 20"]
    pub chat_ids: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A rule to restrict all users from doing something"]
pub struct UserPrivacySettingRuleRestrictAll {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A rule to restrict all contacts of a user from doing something"]
pub struct UserPrivacySettingRuleRestrictContacts {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A rule to restrict all specified users from doing something "]
pub struct UserPrivacySettingRuleRestrictUsers {
    #[doc = "The user identifiers, total number of users in all rules must not exceed 1000"]
    pub user_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A rule to restrict all members of specified basic groups and supergroups from doing something "]
pub struct UserPrivacySettingRuleRestrictChatMembers {
    #[doc = "The chat identifiers, total number of chats in all rules must not exceed 20"]
    pub chat_ids: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed "]
pub struct UserPrivacySettingRules {
    #[doc = "A list of rules"]
    pub rules: Vec<UserPrivacySettingRule>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A privacy setting for managing whether the user's online status is visible"]
pub struct UserPrivacySettingShowStatus {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A privacy setting for managing whether the user's profile photo is visible"]
pub struct UserPrivacySettingShowProfilePhoto {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A privacy setting for managing whether a link to the user's account is included in forwarded messages"]
pub struct UserPrivacySettingShowLinkInForwardedMessages {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A privacy setting for managing whether the user's phone number is visible"]
pub struct UserPrivacySettingShowPhoneNumber {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A privacy setting for managing whether the user can be invited to chats"]
pub struct UserPrivacySettingAllowChatInvites {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A privacy setting for managing whether the user can be called"]
pub struct UserPrivacySettingAllowCalls {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A privacy setting for managing whether peer-to-peer connections can be used for calls"]
pub struct UserPrivacySettingAllowPeerToPeerCalls {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A privacy setting for managing whether the user can be found by their phone number. Checked only if the phone number is not known to the other user. Can be set only to \"Allow contacts\" or \"Allow all\""]
pub struct UserPrivacySettingAllowFindingByPhoneNumber {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about the period of inactivity after which the current user's account will automatically be deleted "]
pub struct AccountTtl {
    #[doc = "Number of days of inactivity before the account will be flagged for deletion; should range from 30-366 days"]
    pub days: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about one session in a Telegram application used by the current user. Sessions should be shown to the user in the returned order"]
pub struct Session {
    #[doc = "Session identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "True, if this session is the current session"]
    #[serde(default)]
    pub is_current: bool,
    #[doc = "True, if a password is needed to complete authorization of the session"]
    #[serde(default)]
    pub is_password_pending: bool,
    #[doc = "Telegram API identifier, as provided by the application "]
    pub api_id: i32,
    #[doc = "Name of the application, as provided by the application"]
    pub application_name: String,
    #[doc = "The version of the application, as provided by the application "]
    pub application_version: String,
    #[doc = "True, if the application is an official application or uses the api_id of an official application"]
    #[serde(default)]
    pub is_official_application: bool,
    #[doc = "Model of the device the application has been run or is running on, as provided by the application "]
    pub device_model: String,
    #[doc = "Operating system the application has been run or is running on, as provided by the application"]
    pub platform: String,
    #[doc = "Version of the operating system the application has been run or is running on, as provided by the application "]
    pub system_version: String,
    #[doc = "Point in time (Unix timestamp) when the user has logged in"]
    pub log_in_date: i32,
    #[doc = "Point in time (Unix timestamp) when the session was last used "]
    pub last_active_date: i32,
    #[doc = "IP address from which the session was created, in human-readable format"]
    pub ip: String,
    #[doc = "A two-letter country code for the country from which the session was created, based on the IP address "]
    pub country: String,
    #[doc = "Region code from which the session was created, based on the IP address"]
    pub region: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of sessions "]
pub struct Sessions {
    #[doc = "List of sessions"]
    pub sessions: Vec<Session>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about one website the current user is logged in with Telegram"]
pub struct ConnectedWebsite {
    #[doc = "Website identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "The domain name of the website"]
    pub domain_name: String,
    #[doc = "User identifier of a bot linked with the website"]
    pub bot_user_id: i32,
    #[doc = "The version of a browser used to log in"]
    pub browser: String,
    #[doc = "Operating system the browser is running on"]
    pub platform: String,
    #[doc = "Point in time (Unix timestamp) when the user was logged in"]
    pub log_in_date: i32,
    #[doc = "Point in time (Unix timestamp) when obtained authorization was last used"]
    pub last_active_date: i32,
    #[doc = "IP address from which the user was logged in, in human-readable format"]
    pub ip: String,
    #[doc = "Human-readable description of a country and a region, from which the user was logged in, based on the IP address"]
    pub location: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of websites the current user is logged in with Telegram "]
pub struct ConnectedWebsites {
    #[doc = "List of connected websites"]
    pub websites: Vec<ConnectedWebsite>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat contains spam messages"]
pub struct ChatReportReasonSpam {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat promotes violence"]
pub struct ChatReportReasonViolence {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat contains pornographic messages"]
pub struct ChatReportReasonPornography {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat has child abuse related content"]
pub struct ChatReportReasonChildAbuse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat contains copyrighted content"]
pub struct ChatReportReasonCopyright {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The location-based chat is unrelated to its stated location"]
pub struct ChatReportReasonUnrelatedLocation {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A custom reason provided by the user "]
pub struct ChatReportReasonCustom {
    #[doc = "Report text"]
    pub text: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a public HTTPS link to a message in a supergroup or channel with a username "]
pub struct PublicMessageLink {
    #[doc = "Message link "]
    pub link: String,
    #[doc = "HTML-code for embedding the message"]
    pub html: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a link to a message in a chat"]
pub struct MessageLinkInfo {
    #[doc = "True, if the link is a public link for a message in a chat"]
    #[serde(default)]
    pub is_public: bool,
    #[doc = "If found, identifier of the chat to which the message belongs, 0 otherwise"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "If found, the linked message; may be null"]
    pub message: Option<Message>,
    #[doc = "True, if the whole media album to which the message belongs is linked"]
    #[serde(default)]
    pub for_album: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a part of a file "]
pub struct FilePart {
    #[doc = "File bytes"]
    pub data: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The data is not a file"]
pub struct FileTypeNone {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is an animation"]
pub struct FileTypeAnimation {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is an audio file"]
pub struct FileTypeAudio {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is a document"]
pub struct FileTypeDocument {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is a photo"]
pub struct FileTypePhoto {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is a profile photo"]
pub struct FileTypeProfilePhoto {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file was sent to a secret chat (the file type is not known to the server)"]
pub struct FileTypeSecret {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is a thumbnail of a file from a secret chat"]
pub struct FileTypeSecretThumbnail {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is a file from Secure storage used for storing Telegram Passport files"]
pub struct FileTypeSecure {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is a sticker"]
pub struct FileTypeSticker {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is a thumbnail of another file"]
pub struct FileTypeThumbnail {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file type is not yet known"]
pub struct FileTypeUnknown {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is a video"]
pub struct FileTypeVideo {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is a video note"]
pub struct FileTypeVideoNote {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is a voice note"]
pub struct FileTypeVoiceNote {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file is a wallpaper or a background pattern"]
pub struct FileTypeWallpaper {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains the storage usage statistics for a specific file type "]
pub struct StorageStatisticsByFileType {
    #[doc = "File type "]
    pub file_type: FileType,
    #[doc = "Total size of the files "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub size: i64,
    #[doc = "Total number of files"]
    pub count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains the storage usage statistics for a specific chat "]
pub struct StorageStatisticsByChat {
    #[doc = "Chat identifier; 0 if none "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Total size of the files in the chat "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub size: i64,
    #[doc = "Total number of files in the chat "]
    pub count: i32,
    #[doc = "Statistics split by file types"]
    pub by_file_type: Vec<StorageStatisticsByFileType>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains the exact storage usage statistics split by chats and file type "]
pub struct StorageStatistics {
    #[doc = "Total size of files "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub size: i64,
    #[doc = "Total number of files "]
    pub count: i32,
    #[doc = "Statistics split by chats"]
    pub by_chat: Vec<StorageStatisticsByChat>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains approximate storage usage statistics, excluding files of unknown file type "]
pub struct StorageStatisticsFast {
    #[doc = "Approximate total size of files "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub files_size: i64,
    #[doc = "Approximate number of files"]
    pub file_count: i32,
    #[doc = "Size of the database "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub database_size: i64,
    #[doc = "Size of the language pack database "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub language_pack_database_size: i64,
    #[doc = "Size of the TDLib internal log"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub log_size: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains database statistics"]
pub struct DatabaseStatistics {
    #[doc = "Database statistics in an unspecified human-readable format"]
    pub statistics: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The network is not available"]
pub struct NetworkTypeNone {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A mobile network"]
pub struct NetworkTypeMobile {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A mobile roaming network"]
pub struct NetworkTypeMobileRoaming {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A Wi-Fi network"]
pub struct NetworkTypeWiFi {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A different network type (e.g., Ethernet network)"]
pub struct NetworkTypeOther {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about the total amount of data that was used to send and receive files "]
pub struct NetworkStatisticsEntryFile {
    #[doc = "Type of the file the data is part of "]
    pub file_type: FileType,
    #[doc = "Type of the network the data was sent through. Call setNetworkType to maintain the actual network type"]
    pub network_type: NetworkType,
    #[doc = "Total number of bytes sent "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub sent_bytes: i64,
    #[doc = "Total number of bytes received"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub received_bytes: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about the total amount of data that was used for calls "]
pub struct NetworkStatisticsEntryCall {
    #[doc = "Type of the network the data was sent through. Call setNetworkType to maintain the actual network type"]
    pub network_type: NetworkType,
    #[doc = "Total number of bytes sent "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub sent_bytes: i64,
    #[doc = "Total number of bytes received "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub received_bytes: i64,
    #[doc = "Total call duration, in seconds"]
    pub duration: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A full list of available network statistic entries "]
pub struct NetworkStatistics {
    #[doc = "Point in time (Unix timestamp) when the app began collecting statistics "]
    pub since_date: i32,
    #[doc = "Network statistics entries"]
    pub entries: Vec<NetworkStatisticsEntry>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains auto-download settings"]
pub struct AutoDownloadSettings {
    #[doc = "True, if the auto-download is enabled"]
    #[serde(default)]
    pub is_auto_download_enabled: bool,
    #[doc = "The maximum size of a photo file to be auto-downloaded"]
    pub max_photo_file_size: i32,
    #[doc = "The maximum size of a video file to be auto-downloaded"]
    pub max_video_file_size: i32,
    #[doc = "The maximum size of other file types to be auto-downloaded"]
    pub max_other_file_size: i32,
    #[doc = "The maximum suggested bitrate for uploaded videos"]
    pub video_upload_bitrate: i32,
    #[doc = "True, if the beginning of videos needs to be preloaded for instant playback"]
    #[serde(default)]
    pub preload_large_videos: bool,
    #[doc = "True, if the next audio track needs to be preloaded while the user is listening to an audio file"]
    #[serde(default)]
    pub preload_next_audio: bool,
    #[doc = "True, if \"use less data for calls\" option needs to be enabled"]
    #[serde(default)]
    pub use_less_data_for_calls: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains auto-download settings presets for the user"]
pub struct AutoDownloadSettingsPresets {
    #[doc = "Preset with lowest settings; supposed to be used by default when roaming"]
    pub low: AutoDownloadSettings,
    #[doc = "Preset with medium settings; supposed to be used by default when using mobile data"]
    pub medium: AutoDownloadSettings,
    #[doc = "Preset with highest settings; supposed to be used by default when connected on Wi-Fi"]
    pub high: AutoDownloadSettings,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Currently waiting for the network to become available. Use setNetworkType to change the available network type"]
pub struct ConnectionStateWaitingForNetwork {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Currently establishing a connection with a proxy server"]
pub struct ConnectionStateConnectingToProxy {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Currently establishing a connection to the Telegram servers"]
pub struct ConnectionStateConnecting {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Downloading data received while the client was offline"]
pub struct ConnectionStateUpdating {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "There is a working connection to the Telegram servers"]
pub struct ConnectionStateReady {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A category containing frequently used private chats with non-bot users"]
pub struct TopChatCategoryUsers {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A category containing frequently used private chats with bot users"]
pub struct TopChatCategoryBots {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A category containing frequently used basic groups and supergroups"]
pub struct TopChatCategoryGroups {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A category containing frequently used channels"]
pub struct TopChatCategoryChannels {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A category containing frequently used chats with inline bots sorted by their usage in inline mode"]
pub struct TopChatCategoryInlineBots {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A category containing frequently used chats used for calls"]
pub struct TopChatCategoryCalls {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A category containing frequently used chats used to forward messages"]
pub struct TopChatCategoryForwardChats {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A URL linking to a user "]
pub struct TMeUrlTypeUser {
    #[doc = "Identifier of the user"]
    pub user_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A URL linking to a public supergroup or channel "]
pub struct TMeUrlTypeSupergroup {
    #[doc = "Identifier of the supergroup or channel"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub supergroup_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat invite link "]
pub struct TMeUrlTypeChatInvite {
    #[doc = "Chat invite link info"]
    pub info: ChatInviteLinkInfo,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A URL linking to a sticker set "]
pub struct TMeUrlTypeStickerSet {
    #[doc = "Identifier of the sticker set"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub sticker_set_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a URL linking to an internal Telegram entity "]
pub struct TMeUrl {
    #[doc = "URL "]
    pub url: String,
    #[serde(rename = "type")]
    #[doc = "Type of the URL"]
    pub type_: TMeUrlType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of t.me URLs "]
pub struct TMeUrls {
    #[doc = "List of URLs"]
    pub urls: Vec<TMeUrl>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a counter "]
pub struct Count {
    #[doc = "Count"]
    pub count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains some text "]
pub struct Text {
    #[doc = "Text"]
    pub text: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a value representing a number of seconds "]
pub struct Seconds {
    #[doc = "Number of seconds"]
    pub seconds: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a tg:// deep link "]
pub struct DeepLinkInfo {
    #[doc = "Text to be shown to the user "]
    pub text: FormattedText,
    #[doc = "True, if user should be asked to update the application"]
    #[serde(default)]
    pub need_update_application: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The text uses Markdown-style formatting"]
pub struct TextParseModeMarkdown {
    #[doc = "Version of the parser: 0 or 1 - Telegram Bot API \"Markdown\" parse mode, 2 - Telegram Bot API \"MarkdownV2\" parse mode"]
    pub version: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The text uses HTML-style formatting. The same as Telegram Bot API \"HTML\" parse mode"]
pub struct TextParseModeHTML {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A SOCKS5 proxy server "]
pub struct ProxyTypeSocks5 {
    #[doc = "Username for logging in; may be empty "]
    pub username: String,
    #[doc = "Password for logging in; may be empty"]
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A HTTP transparent proxy server "]
pub struct ProxyTypeHttp {
    #[doc = "Username for logging in; may be empty "]
    pub username: String,
    #[doc = "Password for logging in; may be empty "]
    pub password: String,
    #[doc = "Pass true if the proxy supports only HTTP requests and doesn't support transparent TCP connections via HTTP CONNECT method"]
    #[serde(default)]
    pub http_only: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An MTProto proxy server "]
pub struct ProxyTypeMtproto {
    #[doc = "The proxy's secret in hexadecimal encoding"]
    pub secret: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains information about a proxy server "]
pub struct Proxy {
    #[doc = "Unique identifier of the proxy "]
    pub id: i32,
    #[doc = "Proxy server IP address "]
    pub server: String,
    #[doc = "Proxy server port "]
    pub port: i32,
    #[doc = "Point in time (Unix timestamp) when the proxy was last used; 0 if never "]
    pub last_used_date: i32,
    #[doc = "True, if the proxy is enabled now "]
    #[serde(default)]
    pub is_enabled: bool,
    #[serde(rename = "type")]
    #[doc = "Type of the proxy"]
    pub type_: ProxyType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a list of proxy servers "]
pub struct Proxies {
    #[doc = "List of proxy servers"]
    pub proxies: Vec<Proxy>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A static sticker in PNG format, which will be converted to WEBP server-side"]
pub struct InputStickerStatic {
    #[doc = "PNG image with the sticker; must be up to 512 KB in size and fit in a 512x512 square"]
    pub sticker: InputFile,
    #[doc = "Emojis corresponding to the sticker"]
    pub emojis: String,
    #[doc = "For masks, position where the mask should be placed; may be null"]
    pub mask_position: Option<MaskPosition>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An animated sticker in TGS format"]
pub struct InputStickerAnimated {
    #[doc = "File with the animated sticker. Only local or uploaded within a week files are supported. See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements"]
    pub sticker: InputFile,
    #[doc = "Emojis corresponding to the sticker"]
    pub emojis: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Represents a date range "]
pub struct DateRange {
    #[doc = "Point in time (Unix timestamp) at which the date range begins "]
    pub start_date: i32,
    #[doc = "Point in time (Unix timestamp) at which the date range ends"]
    pub end_date: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A statistics value "]
pub struct StatisticsValue {
    #[doc = "The value "]
    pub value: f64,
    #[doc = "The value for the previous day "]
    pub previous_value: f64,
    #[doc = "The growth rate of the value, as a percentage"]
    pub growth_rate_percentage: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A graph data "]
pub struct StatisticsGraphData {
    #[doc = "Graph data in JSON format "]
    pub json_data: String,
    #[doc = "If non-empty, a token which can be used to receive a zoomed in graph"]
    pub zoom_token: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The graph data to be asynchronously loaded through getChatStatisticsGraph "]
pub struct StatisticsGraphAsync {
    #[doc = "The token to use for data loading"]
    pub token: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An error message to be shown to the user instead of the graph "]
pub struct StatisticsGraphError {
    #[doc = "The error message"]
    pub error_message: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains statistics about interactions with a message"]
pub struct ChatStatisticsMessageInteractionCounters {
    #[doc = "Message identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "Number of times the message was viewed"]
    pub view_count: i32,
    #[doc = "Number of times the message was forwarded"]
    pub forward_count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A detailed statistics about a chat"]
pub struct ChatStatistics {
    #[doc = "A period to which the statistics applies"]
    pub period: DateRange,
    #[doc = "Number of members in the chat"]
    pub member_count: StatisticsValue,
    #[doc = "Mean number of times the recently sent messages was viewed"]
    pub mean_view_count: StatisticsValue,
    #[doc = "Mean number of times the recently sent messages was shared"]
    pub mean_share_count: StatisticsValue,
    #[doc = "A percentage of users with enabled notifications for the chat"]
    pub enabled_notifications_percentage: f64,
    #[doc = "A graph containing number of members in the chat"]
    pub member_count_graph: StatisticsGraph,
    #[doc = "A graph containing number of members joined and left the chat"]
    pub join_graph: StatisticsGraph,
    #[doc = "A graph containing number of members muted and unmuted the chat"]
    pub mute_graph: StatisticsGraph,
    #[doc = "A graph containing number of message views in a given hour in the last two weeks"]
    pub view_count_by_hour_graph: StatisticsGraph,
    #[doc = "A graph containing number of message views per source"]
    pub view_count_by_source_graph: StatisticsGraph,
    #[doc = "A graph containing number of new member joins per source"]
    pub join_by_source_graph: StatisticsGraph,
    #[doc = "A graph containing number of users viewed chat messages per language"]
    pub language_graph: StatisticsGraph,
    #[doc = "A graph containing number of chat message views and shares"]
    pub message_interaction_graph: StatisticsGraph,
    #[doc = "A graph containing number of views of associated with the chat instant views"]
    pub instant_view_interaction_graph: StatisticsGraph,
    #[doc = "Detailed statistics about number of views and shares of recently sent messages"]
    pub recent_message_interactions: Vec<ChatStatisticsMessageInteractionCounters>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user authorization state has changed "]
pub struct UpdateAuthorizationState {
    #[doc = "New authorization state"]
    pub authorization_state: AuthorizationState,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new message was received; can also be an outgoing message "]
pub struct UpdateNewMessage {
    #[doc = "The new message"]
    pub message: Message,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option \"use_quick_ack\" is set to true. This update may be sent multiple times for the same message"]
pub struct UpdateMessageSendAcknowledged {
    #[doc = "The chat identifier of the sent message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "A temporary message identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message has been successfully sent "]
pub struct UpdateMessageSendSucceeded {
    #[doc = "Information about the sent message. Usually only the message identifier, date, and content are changed, but almost all other fields can also change "]
    pub message: Message,
    #[doc = "The previous temporary message identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub old_message_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update"]
pub struct UpdateMessageSendFailed {
    #[doc = "Contains information about the message which failed to send "]
    pub message: Message,
    #[doc = "The previous temporary message identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub old_message_id: i64,
    #[doc = "An error code "]
    pub error_code: i32,
    #[doc = "Error message"]
    pub error_message: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The message content has changed "]
pub struct UpdateMessageContent {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Message identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "New message content"]
    pub new_content: MessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message was edited. Changes in the message content will come in a separate updateMessageContent "]
pub struct UpdateMessageEdited {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Message identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "Point in time (Unix timestamp) when the message was edited "]
    pub edit_date: i32,
    #[doc = "New message reply markup; may be null"]
    pub reply_markup: Option<ReplyMarkup>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The view count of the message has changed "]
pub struct UpdateMessageViews {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Message identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "New value of the view count"]
    pub views: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The message content was opened. Updates voice note messages to \"listened\", video note messages to \"viewed\" and starts the TTL timer for self-destructing messages "]
pub struct UpdateMessageContentOpened {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Message identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with an unread mention was read "]
pub struct UpdateMessageMentionRead {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Message identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "The new number of unread mention messages left in the chat"]
    pub unread_mention_count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A message with a live location was viewed. When the update is received, the client is supposed to update the live location"]
pub struct UpdateMessageLiveLocationViewed {
    #[doc = "Identifier of the chat with the live location message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message with live location"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the client. The chat field changes will be reported through separate updates "]
pub struct UpdateNewChat {
    #[doc = "The chat"]
    pub chat: Chat,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The list to which the chat belongs was changed. This update is guaranteed to be sent only when chat.order == 0 and the current or the new chat list is null "]
pub struct UpdateChatChatList {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The new chat's chat list; may be null"]
    pub chat_list: Option<ChatList>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The title of a chat was changed "]
pub struct UpdateChatTitle {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The new chat title"]
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat photo was changed "]
pub struct UpdateChatPhoto {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The new chat photo; may be null"]
    pub photo: Option<ChatPhoto>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Chat permissions was changed "]
pub struct UpdateChatPermissions {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The new chat permissions"]
    pub permissions: ChatPermissions,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The last message of a chat was changed. If last_message is null, then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case "]
pub struct UpdateChatLastMessage {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The new last message in the chat; may be null "]
    pub last_message: Option<Message>,
    #[doc = "New value of the chat order"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub order: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The order of the chat in the chat list has changed. Instead of this update updateChatLastMessage, updateChatIsPinned, updateChatDraftMessage, or updateChatSource might be sent "]
pub struct UpdateChatOrder {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New value of the order"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub order: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat was pinned or unpinned "]
pub struct UpdateChatIsPinned {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New value of is_pinned "]
    #[serde(default)]
    pub is_pinned: bool,
    #[doc = "New value of the chat order"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub order: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat was marked as unread or was read "]
pub struct UpdateChatIsMarkedAsUnread {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New value of is_marked_as_unread"]
    #[serde(default)]
    pub is_marked_as_unread: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat's source in the chat list has changed "]
pub struct UpdateChatSource {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New chat's source; may be null "]
    pub source: Option<ChatSource>,
    #[doc = "New value of chat order"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub order: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat's has_scheduled_messages field has changed "]
pub struct UpdateChatHasScheduledMessages {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New value of has_scheduled_messages"]
    #[serde(default)]
    pub has_scheduled_messages: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The value of the default disable_notification parameter, used when a message is sent to the chat, was changed "]
pub struct UpdateChatDefaultDisableNotification {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The new default_disable_notification value"]
    #[serde(default)]
    pub default_disable_notification: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Incoming messages were read or number of unread messages has been changed "]
pub struct UpdateChatReadInbox {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the last read incoming message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub last_read_inbox_message_id: i64,
    #[doc = "The number of unread messages left in the chat"]
    pub unread_count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Outgoing messages were read "]
pub struct UpdateChatReadOutbox {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of last read outgoing message"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub last_read_outbox_message_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat unread_mention_count has changed "]
pub struct UpdateChatUnreadMentionCount {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The number of unread mention messages left in the chat"]
    pub unread_mention_count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Notification settings for a chat were changed "]
pub struct UpdateChatNotificationSettings {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The new notification settings"]
    pub notification_settings: ChatNotificationSettings,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Notification settings for some type of chats were updated "]
pub struct UpdateScopeNotificationSettings {
    #[doc = "Types of chats for which notification settings were updated "]
    pub scope: NotificationSettingsScope,
    #[doc = "The new notification settings"]
    pub notification_settings: ScopeNotificationSettings,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat action bar was changed "]
pub struct UpdateChatActionBar {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The new value of the action bar; may be null"]
    pub action_bar: Option<ChatActionBar>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The chat pinned message was changed "]
pub struct UpdateChatPinnedMessage {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The new identifier of the pinned message; 0 if there is no pinned message in the chat"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub pinned_message_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user"]
pub struct UpdateChatReplyMarkup {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message from which reply markup needs to be used; 0 if there is no default custom reply markup in the chat"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub reply_markup_message_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update shouldn't be applied "]
pub struct UpdateChatDraftMessage {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The new draft message; may be null "]
    pub draft_message: Option<DraftMessage>,
    #[doc = "New value of the chat order"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub order: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The number of online group members has changed. This update with non-zero count is sent only for currently opened chats. There is no guarantee that it will be sent just after the count has changed "]
pub struct UpdateChatOnlineMemberCount {
    #[doc = "Identifier of the chat "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New number of online members in the chat, or 0 if unknown"]
    pub online_member_count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A notification was changed "]
pub struct UpdateNotification {
    #[doc = "Unique notification group identifier "]
    pub notification_group_id: i32,
    #[doc = "Changed notification"]
    pub notification: Notification,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A list of active notifications in a notification group has changed"]
pub struct UpdateNotificationGroup {
    #[doc = "Unique notification group identifier"]
    pub notification_group_id: i32,
    #[serde(rename = "type")]
    #[doc = "New type of the notification group"]
    pub type_: NotificationGroupType,
    #[doc = "Identifier of a chat to which all notifications in the group belong"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Chat identifier, which notification settings must be applied to the added notifications"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub notification_settings_chat_id: i64,
    #[doc = "True, if the notifications should be shown without sound"]
    #[serde(default)]
    pub is_silent: bool,
    #[doc = "Total number of unread notifications in the group, can be bigger than number of active notifications"]
    pub total_count: i32,
    #[doc = "List of added group notifications, sorted by notification ID "]
    pub added_notifications: Vec<Notification>,
    #[doc = "Identifiers of removed group notifications, sorted by notification ID"]
    pub removed_notification_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains active notifications that was shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update "]
pub struct UpdateActiveNotifications {
    #[doc = "Lists of active notification groups"]
    pub groups: Vec<NotificationGroup>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications"]
pub struct UpdateHavePendingNotifications {
    #[doc = "True, if there are some delayed notification updates, which will be sent soon"]
    #[serde(default)]
    pub have_delayed_notifications: bool,
    #[doc = "True, if there can be some yet unreceived notifications, which are being fetched from the server"]
    #[serde(default)]
    pub have_unreceived_notifications: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Some messages were deleted "]
pub struct UpdateDeleteMessages {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifiers of the deleted messages"]
    pub message_ids: Vec<i64>,
    #[doc = "True, if the messages are permanently deleted by a user (as opposed to just becoming inaccessible)"]
    #[serde(default)]
    pub is_permanent: bool,
    #[doc = "True, if the messages are deleted only from the cache and can possibly be retrieved again in the future"]
    #[serde(default)]
    pub from_cache: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "User activity in the chat has changed "]
pub struct UpdateUserChatAction {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of a user performing an action "]
    pub user_id: i32,
    #[doc = "The action description"]
    pub action: ChatAction,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user went online or offline "]
pub struct UpdateUserStatus {
    #[doc = "User identifier "]
    pub user_id: i32,
    #[doc = "New status of the user"]
    pub status: UserStatus,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the client "]
pub struct UpdateUser {
    #[doc = "New data about the user"]
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the client "]
pub struct UpdateBasicGroup {
    #[doc = "New data about the group"]
    pub basic_group: BasicGroup,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the client "]
pub struct UpdateSupergroup {
    #[doc = "New data about the supergroup"]
    pub supergroup: Supergroup,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the client "]
pub struct UpdateSecretChat {
    #[doc = "New data about the secret chat"]
    pub secret_chat: SecretChat,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Some data from userFullInfo has been changed "]
pub struct UpdateUserFullInfo {
    #[doc = "User identifier "]
    pub user_id: i32,
    #[doc = "New full information about the user"]
    pub user_full_info: UserFullInfo,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Some data from basicGroupFullInfo has been changed "]
pub struct UpdateBasicGroupFullInfo {
    #[doc = "Identifier of a basic group "]
    pub basic_group_id: i32,
    #[doc = "New full information about the group"]
    pub basic_group_full_info: BasicGroupFullInfo,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Some data from supergroupFullInfo has been changed "]
pub struct UpdateSupergroupFullInfo {
    #[doc = "Identifier of the supergroup or channel "]
    pub supergroup_id: i32,
    #[doc = "New full information about the supergroup"]
    pub supergroup_full_info: SupergroupFullInfo,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Service notification from the server. Upon receiving this the client must show a popup with the content of the notification"]
pub struct UpdateServiceNotification {
    #[serde(rename = "type")]
    #[doc = "Notification type. If type begins with \"AUTH_KEY_DROP_\", then two buttons \"Cancel\" and \"Log out\" should be shown under notification; if user presses the second, all local data should be destroyed using Destroy method"]
    pub type_: String,
    #[doc = "Notification content"]
    pub content: MessageContent,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Information about a file was updated "]
pub struct UpdateFile {
    #[doc = "New data about the file"]
    pub file: File,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The file generation process needs to be started by the client"]
pub struct UpdateFileGenerationStart {
    #[doc = "Unique identifier for the generation process"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub generation_id: i64,
    #[doc = "The path to a file from which a new file is generated; may be empty"]
    pub original_path: String,
    #[doc = "The path to a file that should be created and where the new file should be generated"]
    pub destination_path: String,
    #[doc = "String specifying the conversion applied to the original file. If conversion is \"#url#\" than original_path contains an HTTP/HTTPS URL of a file, which should be downloaded by the client"]
    pub conversion: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "File generation is no longer needed "]
pub struct UpdateFileGenerationStop {
    #[doc = "Unique identifier for the generation process"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub generation_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "New call was created or information about a call was updated "]
pub struct UpdateCall {
    #[doc = "New data about a call"]
    pub call: Call,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Some privacy setting rules have been changed "]
pub struct UpdateUserPrivacySettingRules {
    #[doc = "The privacy setting "]
    pub setting: UserPrivacySetting,
    #[doc = "New privacy rules"]
    pub rules: UserPrivacySettingRules,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Number of unread messages in a chat list has changed. This update is sent only if the message database is used "]
pub struct UpdateUnreadMessageCount {
    #[doc = "The chat list with changed number of unread messages"]
    pub chat_list: ChatList,
    #[doc = "Total number of unread messages "]
    pub unread_count: i32,
    #[doc = "Total number of unread messages in unmuted chats"]
    pub unread_unmuted_count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used"]
pub struct UpdateUnreadChatCount {
    #[doc = "The chat list with changed number of unread messages"]
    pub chat_list: ChatList,
    #[doc = "Approximate total number of chats in the chat list"]
    pub total_count: i32,
    #[doc = "Total number of unread chats "]
    pub unread_count: i32,
    #[doc = "Total number of unread unmuted chats"]
    pub unread_unmuted_count: i32,
    #[doc = "Total number of chats marked as unread "]
    pub marked_as_unread_count: i32,
    #[doc = "Total number of unmuted chats marked as unread"]
    pub marked_as_unread_unmuted_count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "An option changed its value "]
pub struct UpdateOption {
    #[doc = "The option name "]
    pub name: String,
    #[doc = "The new option value"]
    pub value: OptionValue,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A sticker set has changed "]
pub struct UpdateStickerSet {
    #[doc = "The sticker set"]
    pub sticker_set: StickerSet,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The list of installed sticker sets was updated "]
pub struct UpdateInstalledStickerSets {
    #[doc = "True, if the list of installed mask sticker sets was updated "]
    #[serde(default)]
    pub is_masks: bool,
    #[doc = "The new list of installed ordinary sticker sets"]
    pub sticker_set_ids: Vec<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The list of trending sticker sets was updated or some of them were viewed "]
pub struct UpdateTrendingStickerSets {
    #[doc = "The prefix of the list of trending sticker sets with the newest trending sticker sets"]
    pub sticker_sets: StickerSets,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The list of recently used stickers was updated "]
pub struct UpdateRecentStickers {
    #[doc = "True, if the list of stickers attached to photo or video files was updated, otherwise the list of sent stickers is updated "]
    #[serde(default)]
    pub is_attached: bool,
    #[doc = "The new list of file identifiers of recently used stickers"]
    pub sticker_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The list of favorite stickers was updated "]
pub struct UpdateFavoriteStickers {
    #[doc = "The new list of file identifiers of favorite stickers"]
    pub sticker_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The list of saved animations was updated "]
pub struct UpdateSavedAnimations {
    #[doc = "The new list of file identifiers of saved animations"]
    pub animation_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The selected background has changed "]
pub struct UpdateSelectedBackground {
    #[doc = "True, if background for dark theme has changed "]
    #[serde(default)]
    pub for_dark_theme: bool,
    #[doc = "The new selected background; may be null"]
    pub background: Option<Background>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Some language pack strings have been updated "]
pub struct UpdateLanguagePackStrings {
    #[doc = "Localization target to which the language pack belongs "]
    pub localization_target: String,
    #[doc = "Identifier of the updated language pack "]
    pub language_pack_id: String,
    #[doc = "List of changed language pack strings"]
    pub strings: Vec<LanguagePackString>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The connection state has changed "]
pub struct UpdateConnectionState {
    #[doc = "The new connection state"]
    pub state: ConnectionState,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method should be called with the reason \"Decline ToS update\" "]
pub struct UpdateTermsOfService {
    #[doc = "Identifier of the terms of service "]
    pub terms_of_service_id: String,
    #[doc = "The new terms of service"]
    pub terms_of_service: TermsOfService,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The list of users nearby has changed. The update is sent only 60 seconds after a successful searchChatsNearby request "]
pub struct UpdateUsersNearby {
    #[doc = "The new list of users nearby"]
    pub users_nearby: Vec<ChatNearby>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The list of supported dice emojis has changed "]
pub struct UpdateDiceEmojis {
    #[doc = "The new list of supported dice emojis"]
    pub emojis: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new incoming inline query; for bots only "]
pub struct UpdateNewInlineQuery {
    #[doc = "Unique query identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "Identifier of the user who sent the query "]
    pub sender_user_id: i32,
    #[doc = "User location, provided by the client; may be null"]
    pub user_location: Option<Location>,
    #[doc = "Text of the query "]
    pub query: String,
    #[doc = "Offset of the first entry to return"]
    pub offset: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The user has chosen a result of an inline query; for bots only "]
pub struct UpdateNewChosenInlineResult {
    #[doc = "Identifier of the user who sent the query "]
    pub sender_user_id: i32,
    #[doc = "User location, provided by the client; may be null"]
    pub user_location: Option<Location>,
    #[doc = "Text of the query "]
    pub query: String,
    #[doc = "Identifier of the chosen result "]
    pub result_id: String,
    #[doc = "Identifier of the sent inline message, if known"]
    pub inline_message_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new incoming callback query; for bots only "]
pub struct UpdateNewCallbackQuery {
    #[doc = "Unique query identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "Identifier of the user who sent the query"]
    pub sender_user_id: i32,
    #[doc = "Identifier of the chat where the query was sent "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message, from which the query originated"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "Identifier that uniquely corresponds to the chat to which the message was sent "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_instance: i64,
    #[doc = "Query payload"]
    pub payload: CallbackQueryPayload,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new incoming callback query from a message sent via a bot; for bots only "]
pub struct UpdateNewInlineCallbackQuery {
    #[doc = "Unique query identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "Identifier of the user who sent the query "]
    pub sender_user_id: i32,
    #[doc = "Identifier of the inline message, from which the query originated"]
    pub inline_message_id: String,
    #[doc = "An identifier uniquely corresponding to the chat a message was sent to "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_instance: i64,
    #[doc = "Query payload"]
    pub payload: CallbackQueryPayload,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new incoming shipping query; for bots only. Only for invoices with flexible price "]
pub struct UpdateNewShippingQuery {
    #[doc = "Unique query identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "Identifier of the user who sent the query "]
    pub sender_user_id: i32,
    #[doc = "Invoice payload "]
    pub invoice_payload: String,
    #[doc = "User shipping address"]
    pub shipping_address: Address,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new incoming pre-checkout query; for bots only. Contains full information about a checkout "]
pub struct UpdateNewPreCheckoutQuery {
    #[doc = "Unique query identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "Identifier of the user who sent the query "]
    pub sender_user_id: i32,
    #[doc = "Currency for the product price "]
    pub currency: String,
    #[doc = "Total price for the product, in the minimal quantity of the currency"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub total_amount: i64,
    #[doc = "Invoice payload "]
    pub invoice_payload: String,
    #[doc = "Identifier of a shipping option chosen by the user; may be empty if not applicable "]
    pub shipping_option_id: String,
    #[doc = "Information about the order; may be null"]
    pub order_info: Option<OrderInfo>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new incoming event; for bots only "]
pub struct UpdateNewCustomEvent {
    #[doc = "A JSON-serialized event"]
    pub event: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A new incoming query; for bots only "]
pub struct UpdateNewCustomQuery {
    #[doc = "The query identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: i64,
    #[doc = "JSON-serialized query data "]
    pub data: String,
    #[doc = "Query timeout"]
    pub timeout: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A poll was updated; for bots only "]
pub struct UpdatePoll {
    #[doc = "New data about the poll"]
    pub poll: Poll,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A user changed the answer to a poll; for bots only "]
pub struct UpdatePollAnswer {
    #[doc = "Unique poll identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub poll_id: i64,
    #[doc = "The user, who changed the answer to the poll "]
    pub user_id: i32,
    #[doc = "0-based identifiers of answer options, chosen by the user"]
    pub option_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of updates "]
pub struct Updates {
    #[doc = "List of updates"]
    pub updates: Vec<Update>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The log is written to stderr or an OS specific log"]
pub struct LogStreamDefault {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The log is written to a file "]
pub struct LogStreamFile {
    #[doc = "Path to the file to where the internal TDLib log will be written "]
    pub path: String,
    #[doc = "The maximum size of the file to where the internal TDLib log is written before the file will be auto-rotated"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub max_file_size: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "The log is written nowhere"]
pub struct LogStreamEmpty {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a TDLib internal log verbosity level "]
pub struct LogVerbosityLevel {
    #[doc = "Log verbosity level"]
    pub verbosity_level: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Contains a list of available TDLib internal log tags "]
pub struct LogTags {
    #[doc = "List of log tags"]
    pub tags: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A simple object containing a number; for testing only "]
pub struct TestInt {
    #[doc = "Number"]
    pub value: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A simple object containing a string; for testing only "]
pub struct TestString {
    #[doc = "String"]
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A simple object containing a sequence of bytes; for testing only "]
pub struct TestBytes {
    #[doc = "Bytes"]
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A simple object containing a vector of numbers; for testing only "]
pub struct TestVectorInt {
    #[doc = "Vector of numbers"]
    pub value: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A simple object containing a vector of objects that hold a number; for testing only "]
pub struct TestVectorIntObject {
    #[doc = "Vector of objects"]
    pub value: Vec<TestInt>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A simple object containing a vector of strings; for testing only "]
pub struct TestVectorString {
    #[doc = "Vector of strings"]
    pub value: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "A simple object containing a vector of objects that hold a string; for testing only "]
pub struct TestVectorStringObject {
    #[doc = "Vector of objects"]
    pub value: Vec<TestString>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum MaskPoint {
    MaskPointForehead(MaskPointForehead),
    MaskPointEyes(MaskPointEyes),
    MaskPointMouth(MaskPointMouth),
    MaskPointChin(MaskPointChin),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum FileType {
    FileTypeNone(FileTypeNone),
    FileTypeAnimation(FileTypeAnimation),
    FileTypeAudio(FileTypeAudio),
    FileTypeDocument(FileTypeDocument),
    FileTypePhoto(FileTypePhoto),
    FileTypeProfilePhoto(FileTypeProfilePhoto),
    FileTypeSecret(FileTypeSecret),
    FileTypeSecretThumbnail(FileTypeSecretThumbnail),
    FileTypeSecure(FileTypeSecure),
    FileTypeSticker(FileTypeSticker),
    FileTypeThumbnail(FileTypeThumbnail),
    FileTypeUnknown(FileTypeUnknown),
    FileTypeVideo(FileTypeVideo),
    FileTypeVideoNote(FileTypeVideoNote),
    FileTypeVoiceNote(FileTypeVoiceNote),
    FileTypeWallpaper(FileTypeWallpaper),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum ProxyType {
    ProxyTypeSocks5(ProxyTypeSocks5),
    ProxyTypeHttp(ProxyTypeHttp),
    ProxyTypeMtproto(ProxyTypeMtproto),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum LoginUrlInfo {
    LoginUrlInfoOpen(LoginUrlInfoOpen),
    LoginUrlInfoRequestConfirmation(LoginUrlInfoRequestConfirmation),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum ChatMembersFilter {
    ChatMembersFilterContacts(ChatMembersFilterContacts),
    ChatMembersFilterAdministrators(ChatMembersFilterAdministrators),
    ChatMembersFilterMembers(ChatMembersFilterMembers),
    ChatMembersFilterRestricted(ChatMembersFilterRestricted),
    ChatMembersFilterBanned(ChatMembersFilterBanned),
    ChatMembersFilterBots(ChatMembersFilterBots),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum ChatEventAction {
    ChatEventMessageEdited(ChatEventMessageEdited),
    ChatEventMessageDeleted(ChatEventMessageDeleted),
    ChatEventPollStopped(ChatEventPollStopped),
    ChatEventMessagePinned(ChatEventMessagePinned),
    ChatEventMessageUnpinned(ChatEventMessageUnpinned),
    ChatEventMemberJoined(ChatEventMemberJoined),
    ChatEventMemberLeft(ChatEventMemberLeft),
    ChatEventMemberInvited(ChatEventMemberInvited),
    ChatEventMemberPromoted(ChatEventMemberPromoted),
    ChatEventMemberRestricted(ChatEventMemberRestricted),
    ChatEventTitleChanged(ChatEventTitleChanged),
    ChatEventPermissionsChanged(ChatEventPermissionsChanged),
    ChatEventDescriptionChanged(ChatEventDescriptionChanged),
    ChatEventUsernameChanged(ChatEventUsernameChanged),
    ChatEventPhotoChanged(ChatEventPhotoChanged),
    ChatEventInvitesToggled(ChatEventInvitesToggled),
    ChatEventLinkedChatChanged(ChatEventLinkedChatChanged),
    ChatEventSlowModeDelayChanged(ChatEventSlowModeDelayChanged),
    ChatEventSignMessagesToggled(ChatEventSignMessagesToggled),
    ChatEventStickerSetChanged(ChatEventStickerSetChanged),
    ChatEventLocationChanged(ChatEventLocationChanged),
    ChatEventIsAllHistoryAvailableToggled(ChatEventIsAllHistoryAvailableToggled),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum InputSticker {
    InputStickerStatic(InputStickerStatic),
    InputStickerAnimated(InputStickerAnimated),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum ChatSource {
    ChatSourceMtprotoProxy(ChatSourceMtprotoProxy),
    ChatSourcePublicServiceAnnouncement(ChatSourcePublicServiceAnnouncement),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum TextEntityType {
    TextEntityTypeMention(TextEntityTypeMention),
    TextEntityTypeHashtag(TextEntityTypeHashtag),
    TextEntityTypeCashtag(TextEntityTypeCashtag),
    TextEntityTypeBotCommand(TextEntityTypeBotCommand),
    TextEntityTypeUrl(TextEntityTypeUrl),
    TextEntityTypeEmailAddress(TextEntityTypeEmailAddress),
    TextEntityTypePhoneNumber(TextEntityTypePhoneNumber),
    TextEntityTypeBankCardNumber(TextEntityTypeBankCardNumber),
    TextEntityTypeBold(TextEntityTypeBold),
    TextEntityTypeItalic(TextEntityTypeItalic),
    TextEntityTypeUnderline(TextEntityTypeUnderline),
    TextEntityTypeStrikethrough(TextEntityTypeStrikethrough),
    TextEntityTypeCode(TextEntityTypeCode),
    TextEntityTypePre(TextEntityTypePre),
    TextEntityTypePreCode(TextEntityTypePreCode),
    TextEntityTypeTextUrl(TextEntityTypeTextUrl),
    TextEntityTypeMentionName(TextEntityTypeMentionName),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum NetworkType {
    NetworkTypeNone(NetworkTypeNone),
    NetworkTypeMobile(NetworkTypeMobile),
    NetworkTypeMobileRoaming(NetworkTypeMobileRoaming),
    NetworkTypeWiFi(NetworkTypeWiFi),
    NetworkTypeOther(NetworkTypeOther),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum PassportElement {
    PassportElementPersonalDetails(PassportElementPersonalDetails),
    PassportElementPassport(PassportElementPassport),
    PassportElementDriverLicense(PassportElementDriverLicense),
    PassportElementIdentityCard(PassportElementIdentityCard),
    PassportElementInternalPassport(PassportElementInternalPassport),
    PassportElementAddress(PassportElementAddress),
    PassportElementUtilityBill(PassportElementUtilityBill),
    PassportElementBankStatement(PassportElementBankStatement),
    PassportElementRentalAgreement(PassportElementRentalAgreement),
    PassportElementPassportRegistration(PassportElementPassportRegistration),
    PassportElementTemporaryRegistration(PassportElementTemporaryRegistration),
    PassportElementPhoneNumber(PassportElementPhoneNumber),
    PassportElementEmailAddress(PassportElementEmailAddress),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum MessageContent {
    MessageText(MessageText),
    MessageAnimation(MessageAnimation),
    MessageAudio(MessageAudio),
    MessageDocument(MessageDocument),
    MessagePhoto(MessagePhoto),
    MessageExpiredPhoto(MessageExpiredPhoto),
    MessageSticker(MessageSticker),
    MessageVideo(MessageVideo),
    MessageExpiredVideo(MessageExpiredVideo),
    MessageVideoNote(MessageVideoNote),
    MessageVoiceNote(MessageVoiceNote),
    MessageLocation(MessageLocation),
    MessageVenue(MessageVenue),
    MessageContact(MessageContact),
    MessageDice(MessageDice),
    MessageGame(MessageGame),
    MessagePoll(MessagePoll),
    MessageInvoice(MessageInvoice),
    MessageCall(MessageCall),
    MessageBasicGroupChatCreate(MessageBasicGroupChatCreate),
    MessageSupergroupChatCreate(MessageSupergroupChatCreate),
    MessageChatChangeTitle(MessageChatChangeTitle),
    MessageChatChangePhoto(MessageChatChangePhoto),
    MessageChatDeletePhoto(MessageChatDeletePhoto),
    MessageChatAddMembers(MessageChatAddMembers),
    MessageChatJoinByLink(MessageChatJoinByLink),
    MessageChatDeleteMember(MessageChatDeleteMember),
    MessageChatUpgradeTo(MessageChatUpgradeTo),
    MessageChatUpgradeFrom(MessageChatUpgradeFrom),
    MessagePinMessage(MessagePinMessage),
    MessageScreenshotTaken(MessageScreenshotTaken),
    MessageChatSetTtl(MessageChatSetTtl),
    MessageCustomServiceAction(MessageCustomServiceAction),
    MessageGameScore(MessageGameScore),
    MessagePaymentSuccessful(MessagePaymentSuccessful),
    MessagePaymentSuccessfulBot(MessagePaymentSuccessfulBot),
    MessageContactRegistered(MessageContactRegistered),
    MessageWebsiteConnected(MessageWebsiteConnected),
    MessagePassportDataSent(MessagePassportDataSent),
    MessagePassportDataReceived(MessagePassportDataReceived),
    MessageUnsupported(MessageUnsupported),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum LanguagePackStringValue {
    LanguagePackStringValueOrdinary(LanguagePackStringValueOrdinary),
    LanguagePackStringValuePluralized(LanguagePackStringValuePluralized),
    LanguagePackStringValueDeleted(LanguagePackStringValueDeleted),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum CallState {
    CallStatePending(CallStatePending),
    CallStateExchangingKeys(CallStateExchangingKeys),
    CallStateReady(CallStateReady),
    CallStateHangingUp(CallStateHangingUp),
    CallStateDiscarded(CallStateDiscarded),
    CallStateError(CallStateError),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum KeyboardButtonType {
    KeyboardButtonTypeText(KeyboardButtonTypeText),
    KeyboardButtonTypeRequestPhoneNumber(KeyboardButtonTypeRequestPhoneNumber),
    KeyboardButtonTypeRequestLocation(KeyboardButtonTypeRequestLocation),
    KeyboardButtonTypeRequestPoll(KeyboardButtonTypeRequestPoll),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum RichText {
    RichTextPlain(RichTextPlain),
    RichTextBold(RichTextBold),
    RichTextItalic(RichTextItalic),
    RichTextUnderline(RichTextUnderline),
    RichTextStrikethrough(RichTextStrikethrough),
    RichTextFixed(RichTextFixed),
    RichTextUrl(RichTextUrl),
    RichTextEmailAddress(RichTextEmailAddress),
    RichTextSubscript(RichTextSubscript),
    RichTextSuperscript(RichTextSuperscript),
    RichTextMarked(RichTextMarked),
    RichTextPhoneNumber(RichTextPhoneNumber),
    RichTextIcon(RichTextIcon),
    RichTextReference(RichTextReference),
    RichTextAnchor(RichTextAnchor),
    RichTextAnchorLink(RichTextAnchorLink),
    RichTexts(RichTexts),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum PageBlock {
    PageBlockTitle(PageBlockTitle),
    PageBlockSubtitle(PageBlockSubtitle),
    PageBlockAuthorDate(PageBlockAuthorDate),
    PageBlockHeader(PageBlockHeader),
    PageBlockSubheader(PageBlockSubheader),
    PageBlockKicker(PageBlockKicker),
    PageBlockParagraph(PageBlockParagraph),
    PageBlockPreformatted(PageBlockPreformatted),
    PageBlockFooter(PageBlockFooter),
    PageBlockDivider(PageBlockDivider),
    PageBlockAnchor(PageBlockAnchor),
    PageBlockList(PageBlockList),
    PageBlockBlockQuote(PageBlockBlockQuote),
    PageBlockPullQuote(PageBlockPullQuote),
    PageBlockAnimation(PageBlockAnimation),
    PageBlockAudio(PageBlockAudio),
    PageBlockPhoto(PageBlockPhoto),
    PageBlockVideo(PageBlockVideo),
    PageBlockVoiceNote(PageBlockVoiceNote),
    PageBlockCover(PageBlockCover),
    PageBlockEmbedded(PageBlockEmbedded),
    PageBlockEmbeddedPost(PageBlockEmbeddedPost),
    PageBlockCollage(PageBlockCollage),
    PageBlockSlideshow(PageBlockSlideshow),
    PageBlockChatLink(PageBlockChatLink),
    PageBlockTable(PageBlockTable),
    PageBlockDetails(PageBlockDetails),
    PageBlockRelatedArticles(PageBlockRelatedArticles),
    PageBlockMap(PageBlockMap),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum InputBackground {
    InputBackgroundLocal(InputBackgroundLocal),
    InputBackgroundRemote(InputBackgroundRemote),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum InputPassportElementErrorSource {
    InputPassportElementErrorSourceUnspecified(InputPassportElementErrorSourceUnspecified),
    InputPassportElementErrorSourceDataField(InputPassportElementErrorSourceDataField),
    InputPassportElementErrorSourceFrontSide(InputPassportElementErrorSourceFrontSide),
    InputPassportElementErrorSourceReverseSide(InputPassportElementErrorSourceReverseSide),
    InputPassportElementErrorSourceSelfie(InputPassportElementErrorSourceSelfie),
    InputPassportElementErrorSourceTranslationFile(InputPassportElementErrorSourceTranslationFile),
    InputPassportElementErrorSourceTranslationFiles(
        InputPassportElementErrorSourceTranslationFiles,
    ),
    InputPassportElementErrorSourceFile(InputPassportElementErrorSourceFile),
    InputPassportElementErrorSourceFiles(InputPassportElementErrorSourceFiles),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum PushMessageContent {
    PushMessageContentHidden(PushMessageContentHidden),
    PushMessageContentAnimation(PushMessageContentAnimation),
    PushMessageContentAudio(PushMessageContentAudio),
    PushMessageContentContact(PushMessageContentContact),
    PushMessageContentContactRegistered(PushMessageContentContactRegistered),
    PushMessageContentDocument(PushMessageContentDocument),
    PushMessageContentGame(PushMessageContentGame),
    PushMessageContentGameScore(PushMessageContentGameScore),
    PushMessageContentInvoice(PushMessageContentInvoice),
    PushMessageContentLocation(PushMessageContentLocation),
    PushMessageContentPhoto(PushMessageContentPhoto),
    PushMessageContentPoll(PushMessageContentPoll),
    PushMessageContentScreenshotTaken(PushMessageContentScreenshotTaken),
    PushMessageContentSticker(PushMessageContentSticker),
    PushMessageContentText(PushMessageContentText),
    PushMessageContentVideo(PushMessageContentVideo),
    PushMessageContentVideoNote(PushMessageContentVideoNote),
    PushMessageContentVoiceNote(PushMessageContentVoiceNote),
    PushMessageContentBasicGroupChatCreate(PushMessageContentBasicGroupChatCreate),
    PushMessageContentChatAddMembers(PushMessageContentChatAddMembers),
    PushMessageContentChatChangePhoto(PushMessageContentChatChangePhoto),
    PushMessageContentChatChangeTitle(PushMessageContentChatChangeTitle),
    PushMessageContentChatDeleteMember(PushMessageContentChatDeleteMember),
    PushMessageContentChatJoinByLink(PushMessageContentChatJoinByLink),
    PushMessageContentMessageForwards(PushMessageContentMessageForwards),
    PushMessageContentMediaAlbum(PushMessageContentMediaAlbum),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum AuthenticationCodeType {
    AuthenticationCodeTypeTelegramMessage(AuthenticationCodeTypeTelegramMessage),
    AuthenticationCodeTypeSms(AuthenticationCodeTypeSms),
    AuthenticationCodeTypeCall(AuthenticationCodeTypeCall),
    AuthenticationCodeTypeFlashCall(AuthenticationCodeTypeFlashCall),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum ChatMemberStatus {
    ChatMemberStatusCreator(ChatMemberStatusCreator),
    ChatMemberStatusAdministrator(ChatMemberStatusAdministrator),
    ChatMemberStatusMember(ChatMemberStatusMember),
    ChatMemberStatusRestricted(ChatMemberStatusRestricted),
    ChatMemberStatusLeft(ChatMemberStatusLeft),
    ChatMemberStatusBanned(ChatMemberStatusBanned),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum UserPrivacySettingRule {
    UserPrivacySettingRuleAllowAll(UserPrivacySettingRuleAllowAll),
    UserPrivacySettingRuleAllowContacts(UserPrivacySettingRuleAllowContacts),
    UserPrivacySettingRuleAllowUsers(UserPrivacySettingRuleAllowUsers),
    UserPrivacySettingRuleAllowChatMembers(UserPrivacySettingRuleAllowChatMembers),
    UserPrivacySettingRuleRestrictAll(UserPrivacySettingRuleRestrictAll),
    UserPrivacySettingRuleRestrictContacts(UserPrivacySettingRuleRestrictContacts),
    UserPrivacySettingRuleRestrictUsers(UserPrivacySettingRuleRestrictUsers),
    UserPrivacySettingRuleRestrictChatMembers(UserPrivacySettingRuleRestrictChatMembers),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum AuthorizationState {
    AuthorizationStateWaitTdlibParameters(AuthorizationStateWaitTdlibParameters),
    AuthorizationStateWaitEncryptionKey(AuthorizationStateWaitEncryptionKey),
    AuthorizationStateWaitPhoneNumber(AuthorizationStateWaitPhoneNumber),
    AuthorizationStateWaitCode(AuthorizationStateWaitCode),
    AuthorizationStateWaitOtherDeviceConfirmation(AuthorizationStateWaitOtherDeviceConfirmation),
    AuthorizationStateWaitRegistration(AuthorizationStateWaitRegistration),
    AuthorizationStateWaitPassword(AuthorizationStateWaitPassword),
    AuthorizationStateReady(AuthorizationStateReady),
    AuthorizationStateLoggingOut(AuthorizationStateLoggingOut),
    AuthorizationStateClosing(AuthorizationStateClosing),
    AuthorizationStateClosed(AuthorizationStateClosed),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum OptionValue {
    OptionValueBoolean(OptionValueBoolean),
    OptionValueEmpty(OptionValueEmpty),
    OptionValueInteger(OptionValueInteger),
    OptionValueString(OptionValueString),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum ChatType {
    ChatTypePrivate(ChatTypePrivate),
    ChatTypeBasicGroup(ChatTypeBasicGroup),
    ChatTypeSupergroup(ChatTypeSupergroup),
    ChatTypeSecret(ChatTypeSecret),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum UserStatus {
    UserStatusEmpty(UserStatusEmpty),
    UserStatusOnline(UserStatusOnline),
    UserStatusOffline(UserStatusOffline),
    UserStatusRecently(UserStatusRecently),
    UserStatusLastWeek(UserStatusLastWeek),
    UserStatusLastMonth(UserStatusLastMonth),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum MessageSendingState {
    MessageSendingStatePending(MessageSendingStatePending),
    MessageSendingStateFailed(MessageSendingStateFailed),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum MessageForwardOrigin {
    MessageForwardOriginUser(MessageForwardOriginUser),
    MessageForwardOriginHiddenUser(MessageForwardOriginHiddenUser),
    MessageForwardOriginChannel(MessageForwardOriginChannel),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum PageBlockVerticalAlignment {
    PageBlockVerticalAlignmentTop(PageBlockVerticalAlignmentTop),
    PageBlockVerticalAlignmentMiddle(PageBlockVerticalAlignmentMiddle),
    PageBlockVerticalAlignmentBottom(PageBlockVerticalAlignmentBottom),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum CanTransferOwnershipResult {
    CanTransferOwnershipResultOk(CanTransferOwnershipResultOk),
    CanTransferOwnershipResultPasswordNeeded(CanTransferOwnershipResultPasswordNeeded),
    CanTransferOwnershipResultPasswordTooFresh(CanTransferOwnershipResultPasswordTooFresh),
    CanTransferOwnershipResultSessionTooFresh(CanTransferOwnershipResultSessionTooFresh),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum PageBlockHorizontalAlignment {
    PageBlockHorizontalAlignmentLeft(PageBlockHorizontalAlignmentLeft),
    PageBlockHorizontalAlignmentCenter(PageBlockHorizontalAlignmentCenter),
    PageBlockHorizontalAlignmentRight(PageBlockHorizontalAlignmentRight),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum SearchMessagesFilter {
    SearchMessagesFilterEmpty(SearchMessagesFilterEmpty),
    SearchMessagesFilterAnimation(SearchMessagesFilterAnimation),
    SearchMessagesFilterAudio(SearchMessagesFilterAudio),
    SearchMessagesFilterDocument(SearchMessagesFilterDocument),
    SearchMessagesFilterPhoto(SearchMessagesFilterPhoto),
    SearchMessagesFilterVideo(SearchMessagesFilterVideo),
    SearchMessagesFilterVoiceNote(SearchMessagesFilterVoiceNote),
    SearchMessagesFilterPhotoAndVideo(SearchMessagesFilterPhotoAndVideo),
    SearchMessagesFilterUrl(SearchMessagesFilterUrl),
    SearchMessagesFilterChatPhoto(SearchMessagesFilterChatPhoto),
    SearchMessagesFilterCall(SearchMessagesFilterCall),
    SearchMessagesFilterMissedCall(SearchMessagesFilterMissedCall),
    SearchMessagesFilterVideoNote(SearchMessagesFilterVideoNote),
    SearchMessagesFilterVoiceAndVideoNote(SearchMessagesFilterVoiceAndVideoNote),
    SearchMessagesFilterMention(SearchMessagesFilterMention),
    SearchMessagesFilterUnreadMention(SearchMessagesFilterUnreadMention),
    SearchMessagesFilterFailedToSend(SearchMessagesFilterFailedToSend),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum ReplyMarkup {
    ReplyMarkupRemoveKeyboard(ReplyMarkupRemoveKeyboard),
    ReplyMarkupForceReply(ReplyMarkupForceReply),
    ReplyMarkupShowKeyboard(ReplyMarkupShowKeyboard),
    ReplyMarkupInlineKeyboard(ReplyMarkupInlineKeyboard),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum InputInlineQueryResult {
    InputInlineQueryResultAnimatedGif(InputInlineQueryResultAnimatedGif),
    InputInlineQueryResultAnimatedMpeg4(InputInlineQueryResultAnimatedMpeg4),
    InputInlineQueryResultArticle(InputInlineQueryResultArticle),
    InputInlineQueryResultAudio(InputInlineQueryResultAudio),
    InputInlineQueryResultContact(InputInlineQueryResultContact),
    InputInlineQueryResultDocument(InputInlineQueryResultDocument),
    InputInlineQueryResultGame(InputInlineQueryResultGame),
    InputInlineQueryResultLocation(InputInlineQueryResultLocation),
    InputInlineQueryResultPhoto(InputInlineQueryResultPhoto),
    InputInlineQueryResultSticker(InputInlineQueryResultSticker),
    InputInlineQueryResultVenue(InputInlineQueryResultVenue),
    InputInlineQueryResultVideo(InputInlineQueryResultVideo),
    InputInlineQueryResultVoiceNote(InputInlineQueryResultVoiceNote),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum TMeUrlType {
    TMeUrlTypeUser(TMeUrlTypeUser),
    TMeUrlTypeSupergroup(TMeUrlTypeSupergroup),
    TMeUrlTypeChatInvite(TMeUrlTypeChatInvite),
    TMeUrlTypeStickerSet(TMeUrlTypeStickerSet),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum ChatAction {
    ChatActionTyping(ChatActionTyping),
    ChatActionRecordingVideo(ChatActionRecordingVideo),
    ChatActionUploadingVideo(ChatActionUploadingVideo),
    ChatActionRecordingVoiceNote(ChatActionRecordingVoiceNote),
    ChatActionUploadingVoiceNote(ChatActionUploadingVoiceNote),
    ChatActionUploadingPhoto(ChatActionUploadingPhoto),
    ChatActionUploadingDocument(ChatActionUploadingDocument),
    ChatActionChoosingLocation(ChatActionChoosingLocation),
    ChatActionChoosingContact(ChatActionChoosingContact),
    ChatActionStartPlayingGame(ChatActionStartPlayingGame),
    ChatActionRecordingVideoNote(ChatActionRecordingVideoNote),
    ChatActionUploadingVideoNote(ChatActionUploadingVideoNote),
    ChatActionCancel(ChatActionCancel),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum UserPrivacySetting {
    UserPrivacySettingShowStatus(UserPrivacySettingShowStatus),
    UserPrivacySettingShowProfilePhoto(UserPrivacySettingShowProfilePhoto),
    UserPrivacySettingShowLinkInForwardedMessages(UserPrivacySettingShowLinkInForwardedMessages),
    UserPrivacySettingShowPhoneNumber(UserPrivacySettingShowPhoneNumber),
    UserPrivacySettingAllowChatInvites(UserPrivacySettingAllowChatInvites),
    UserPrivacySettingAllowCalls(UserPrivacySettingAllowCalls),
    UserPrivacySettingAllowPeerToPeerCalls(UserPrivacySettingAllowPeerToPeerCalls),
    UserPrivacySettingAllowFindingByPhoneNumber(UserPrivacySettingAllowFindingByPhoneNumber),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum ChatList {
    ChatListMain(ChatListMain),
    ChatListArchive(ChatListArchive),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum UserType {
    UserTypeRegular(UserTypeRegular),
    UserTypeDeleted(UserTypeDeleted),
    UserTypeBot(UserTypeBot),
    UserTypeUnknown(UserTypeUnknown),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum NotificationSettingsScope {
    NotificationSettingsScopePrivateChats(NotificationSettingsScopePrivateChats),
    NotificationSettingsScopeGroupChats(NotificationSettingsScopeGroupChats),
    NotificationSettingsScopeChannelChats(NotificationSettingsScopeChannelChats),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum TopChatCategory {
    TopChatCategoryUsers(TopChatCategoryUsers),
    TopChatCategoryBots(TopChatCategoryBots),
    TopChatCategoryGroups(TopChatCategoryGroups),
    TopChatCategoryChannels(TopChatCategoryChannels),
    TopChatCategoryInlineBots(TopChatCategoryInlineBots),
    TopChatCategoryCalls(TopChatCategoryCalls),
    TopChatCategoryForwardChats(TopChatCategoryForwardChats),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum PassportElementErrorSource {
    PassportElementErrorSourceUnspecified(PassportElementErrorSourceUnspecified),
    PassportElementErrorSourceDataField(PassportElementErrorSourceDataField),
    PassportElementErrorSourceFrontSide(PassportElementErrorSourceFrontSide),
    PassportElementErrorSourceReverseSide(PassportElementErrorSourceReverseSide),
    PassportElementErrorSourceSelfie(PassportElementErrorSourceSelfie),
    PassportElementErrorSourceTranslationFile(PassportElementErrorSourceTranslationFile),
    PassportElementErrorSourceTranslationFiles(PassportElementErrorSourceTranslationFiles),
    PassportElementErrorSourceFile(PassportElementErrorSourceFile),
    PassportElementErrorSourceFiles(PassportElementErrorSourceFiles),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum SecretChatState {
    SecretChatStatePending(SecretChatStatePending),
    SecretChatStateReady(SecretChatStateReady),
    SecretChatStateClosed(SecretChatStateClosed),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum PublicChatType {
    PublicChatTypeHasUsername(PublicChatTypeHasUsername),
    PublicChatTypeIsLocationBased(PublicChatTypeIsLocationBased),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum InputMessageContent {
    InputMessageText(InputMessageText),
    InputMessageAnimation(InputMessageAnimation),
    InputMessageAudio(InputMessageAudio),
    InputMessageDocument(InputMessageDocument),
    InputMessagePhoto(InputMessagePhoto),
    InputMessageSticker(InputMessageSticker),
    InputMessageVideo(InputMessageVideo),
    InputMessageVideoNote(InputMessageVideoNote),
    InputMessageVoiceNote(InputMessageVoiceNote),
    InputMessageLocation(InputMessageLocation),
    InputMessageVenue(InputMessageVenue),
    InputMessageContact(InputMessageContact),
    InputMessageDice(InputMessageDice),
    InputMessageGame(InputMessageGame),
    InputMessageInvoice(InputMessageInvoice),
    InputMessagePoll(InputMessagePoll),
    InputMessageForwarded(InputMessageForwarded),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum InlineQueryResult {
    InlineQueryResultArticle(InlineQueryResultArticle),
    InlineQueryResultContact(InlineQueryResultContact),
    InlineQueryResultLocation(InlineQueryResultLocation),
    InlineQueryResultVenue(InlineQueryResultVenue),
    InlineQueryResultGame(InlineQueryResultGame),
    InlineQueryResultAnimation(InlineQueryResultAnimation),
    InlineQueryResultAudio(InlineQueryResultAudio),
    InlineQueryResultDocument(InlineQueryResultDocument),
    InlineQueryResultPhoto(InlineQueryResultPhoto),
    InlineQueryResultSticker(InlineQueryResultSticker),
    InlineQueryResultVideo(InlineQueryResultVideo),
    InlineQueryResultVoiceNote(InlineQueryResultVoiceNote),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum InputPassportElement {
    InputPassportElementPersonalDetails(InputPassportElementPersonalDetails),
    InputPassportElementPassport(InputPassportElementPassport),
    InputPassportElementDriverLicense(InputPassportElementDriverLicense),
    InputPassportElementIdentityCard(InputPassportElementIdentityCard),
    InputPassportElementInternalPassport(InputPassportElementInternalPassport),
    InputPassportElementAddress(InputPassportElementAddress),
    InputPassportElementUtilityBill(InputPassportElementUtilityBill),
    InputPassportElementBankStatement(InputPassportElementBankStatement),
    InputPassportElementRentalAgreement(InputPassportElementRentalAgreement),
    InputPassportElementPassportRegistration(InputPassportElementPassportRegistration),
    InputPassportElementTemporaryRegistration(InputPassportElementTemporaryRegistration),
    InputPassportElementPhoneNumber(InputPassportElementPhoneNumber),
    InputPassportElementEmailAddress(InputPassportElementEmailAddress),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum SupergroupMembersFilter {
    SupergroupMembersFilterRecent(SupergroupMembersFilterRecent),
    SupergroupMembersFilterContacts(SupergroupMembersFilterContacts),
    SupergroupMembersFilterAdministrators(SupergroupMembersFilterAdministrators),
    SupergroupMembersFilterSearch(SupergroupMembersFilterSearch),
    SupergroupMembersFilterRestricted(SupergroupMembersFilterRestricted),
    SupergroupMembersFilterBanned(SupergroupMembersFilterBanned),
    SupergroupMembersFilterBots(SupergroupMembersFilterBots),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum JsonValue {
    JsonValueNull(JsonValueNull),
    JsonValueBoolean(JsonValueBoolean),
    JsonValueNumber(JsonValueNumber),
    JsonValueString(JsonValueString),
    JsonValueArray(JsonValueArray),
    JsonValueObject(JsonValueObject),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum InputFile {
    InputFileId(InputFileId),
    InputFileRemote(InputFileRemote),
    InputFileLocal(InputFileLocal),
    InputFileGenerated(InputFileGenerated),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum BackgroundFill {
    BackgroundFillSolid(BackgroundFillSolid),
    BackgroundFillGradient(BackgroundFillGradient),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum InlineKeyboardButtonType {
    InlineKeyboardButtonTypeUrl(InlineKeyboardButtonTypeUrl),
    InlineKeyboardButtonTypeLoginUrl(InlineKeyboardButtonTypeLoginUrl),
    InlineKeyboardButtonTypeCallback(InlineKeyboardButtonTypeCallback),
    InlineKeyboardButtonTypeCallbackGame(InlineKeyboardButtonTypeCallbackGame),
    InlineKeyboardButtonTypeSwitchInline(InlineKeyboardButtonTypeSwitchInline),
    InlineKeyboardButtonTypeBuy(InlineKeyboardButtonTypeBuy),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum NetworkStatisticsEntry {
    NetworkStatisticsEntryFile(NetworkStatisticsEntryFile),
    NetworkStatisticsEntryCall(NetworkStatisticsEntryCall),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum PollType {
    PollTypeRegular(PollTypeRegular),
    PollTypeQuiz(PollTypeQuiz),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum ChatActionBar {
    ChatActionBarReportSpam(ChatActionBarReportSpam),
    ChatActionBarReportUnrelatedLocation(ChatActionBarReportUnrelatedLocation),
    ChatActionBarReportAddBlock(ChatActionBarReportAddBlock),
    ChatActionBarAddContact(ChatActionBarAddContact),
    ChatActionBarSharePhoneNumber(ChatActionBarSharePhoneNumber),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum BackgroundType {
    BackgroundTypeWallpaper(BackgroundTypeWallpaper),
    BackgroundTypePattern(BackgroundTypePattern),
    BackgroundTypeFill(BackgroundTypeFill),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum PassportElementType {
    PassportElementTypePersonalDetails(PassportElementTypePersonalDetails),
    PassportElementTypePassport(PassportElementTypePassport),
    PassportElementTypeDriverLicense(PassportElementTypeDriverLicense),
    PassportElementTypeIdentityCard(PassportElementTypeIdentityCard),
    PassportElementTypeInternalPassport(PassportElementTypeInternalPassport),
    PassportElementTypeAddress(PassportElementTypeAddress),
    PassportElementTypeUtilityBill(PassportElementTypeUtilityBill),
    PassportElementTypeBankStatement(PassportElementTypeBankStatement),
    PassportElementTypeRentalAgreement(PassportElementTypeRentalAgreement),
    PassportElementTypePassportRegistration(PassportElementTypePassportRegistration),
    PassportElementTypeTemporaryRegistration(PassportElementTypeTemporaryRegistration),
    PassportElementTypePhoneNumber(PassportElementTypePhoneNumber),
    PassportElementTypeEmailAddress(PassportElementTypeEmailAddress),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum DeviceToken {
    DeviceTokenFirebaseCloudMessaging(DeviceTokenFirebaseCloudMessaging),
    DeviceTokenApplePush(DeviceTokenApplePush),
    DeviceTokenApplePushVoIP(DeviceTokenApplePushVoIP),
    DeviceTokenWindowsPush(DeviceTokenWindowsPush),
    DeviceTokenMicrosoftPush(DeviceTokenMicrosoftPush),
    DeviceTokenMicrosoftPushVoIP(DeviceTokenMicrosoftPushVoIP),
    DeviceTokenWebPush(DeviceTokenWebPush),
    DeviceTokenSimplePush(DeviceTokenSimplePush),
    DeviceTokenUbuntuPush(DeviceTokenUbuntuPush),
    DeviceTokenBlackBerryPush(DeviceTokenBlackBerryPush),
    DeviceTokenTizenPush(DeviceTokenTizenPush),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum CheckChatUsernameResult {
    CheckChatUsernameResultOk(CheckChatUsernameResultOk),
    CheckChatUsernameResultUsernameInvalid(CheckChatUsernameResultUsernameInvalid),
    CheckChatUsernameResultUsernameOccupied(CheckChatUsernameResultUsernameOccupied),
    CheckChatUsernameResultPublicChatsTooMuch(CheckChatUsernameResultPublicChatsTooMuch),
    CheckChatUsernameResultPublicGroupsUnavailable(CheckChatUsernameResultPublicGroupsUnavailable),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum MessageSchedulingState {
    MessageSchedulingStateSendAtDate(MessageSchedulingStateSendAtDate),
    MessageSchedulingStateSendWhenOnline(MessageSchedulingStateSendWhenOnline),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum StatisticsGraph {
    StatisticsGraphData(StatisticsGraphData),
    StatisticsGraphAsync(StatisticsGraphAsync),
    StatisticsGraphError(StatisticsGraphError),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum TextParseMode {
    TextParseModeMarkdown(TextParseModeMarkdown),
    TextParseModeHTML(TextParseModeHTML),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum NotificationType {
    NotificationTypeNewMessage(NotificationTypeNewMessage),
    NotificationTypeNewSecretChat(NotificationTypeNewSecretChat),
    NotificationTypeNewCall(NotificationTypeNewCall),
    NotificationTypeNewPushMessage(NotificationTypeNewPushMessage),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum ConnectionState {
    ConnectionStateWaitingForNetwork(ConnectionStateWaitingForNetwork),
    ConnectionStateConnectingToProxy(ConnectionStateConnectingToProxy),
    ConnectionStateConnecting(ConnectionStateConnecting),
    ConnectionStateUpdating(ConnectionStateUpdating),
    ConnectionStateReady(ConnectionStateReady),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum CallProblem {
    CallProblemEcho(CallProblemEcho),
    CallProblemNoise(CallProblemNoise),
    CallProblemInterruptions(CallProblemInterruptions),
    CallProblemDistortedSpeech(CallProblemDistortedSpeech),
    CallProblemSilentLocal(CallProblemSilentLocal),
    CallProblemSilentRemote(CallProblemSilentRemote),
    CallProblemDropped(CallProblemDropped),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum InputCredentials {
    InputCredentialsSaved(InputCredentialsSaved),
    InputCredentialsNew(InputCredentialsNew),
    InputCredentialsAndroidPay(InputCredentialsAndroidPay),
    InputCredentialsApplePay(InputCredentialsApplePay),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum NotificationGroupType {
    NotificationGroupTypeMessages(NotificationGroupTypeMessages),
    NotificationGroupTypeMentions(NotificationGroupTypeMentions),
    NotificationGroupTypeSecretChat(NotificationGroupTypeSecretChat),
    NotificationGroupTypeCalls(NotificationGroupTypeCalls),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum LogStream {
    LogStreamDefault(LogStreamDefault),
    LogStreamFile(LogStreamFile),
    LogStreamEmpty(LogStreamEmpty),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum ChatReportReason {
    ChatReportReasonSpam(ChatReportReasonSpam),
    ChatReportReasonViolence(ChatReportReasonViolence),
    ChatReportReasonPornography(ChatReportReasonPornography),
    ChatReportReasonChildAbuse(ChatReportReasonChildAbuse),
    ChatReportReasonCopyright(ChatReportReasonCopyright),
    ChatReportReasonUnrelatedLocation(ChatReportReasonUnrelatedLocation),
    ChatReportReasonCustom(ChatReportReasonCustom),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum Update {
    UpdateAuthorizationState(UpdateAuthorizationState),
    UpdateNewMessage(UpdateNewMessage),
    UpdateMessageSendAcknowledged(UpdateMessageSendAcknowledged),
    UpdateMessageSendSucceeded(UpdateMessageSendSucceeded),
    UpdateMessageSendFailed(UpdateMessageSendFailed),
    UpdateMessageContent(UpdateMessageContent),
    UpdateMessageEdited(UpdateMessageEdited),
    UpdateMessageViews(UpdateMessageViews),
    UpdateMessageContentOpened(UpdateMessageContentOpened),
    UpdateMessageMentionRead(UpdateMessageMentionRead),
    UpdateMessageLiveLocationViewed(UpdateMessageLiveLocationViewed),
    UpdateNewChat(UpdateNewChat),
    UpdateChatChatList(UpdateChatChatList),
    UpdateChatTitle(UpdateChatTitle),
    UpdateChatPhoto(UpdateChatPhoto),
    UpdateChatPermissions(UpdateChatPermissions),
    UpdateChatLastMessage(UpdateChatLastMessage),
    UpdateChatOrder(UpdateChatOrder),
    UpdateChatIsPinned(UpdateChatIsPinned),
    UpdateChatIsMarkedAsUnread(UpdateChatIsMarkedAsUnread),
    UpdateChatSource(UpdateChatSource),
    UpdateChatHasScheduledMessages(UpdateChatHasScheduledMessages),
    UpdateChatDefaultDisableNotification(UpdateChatDefaultDisableNotification),
    UpdateChatReadInbox(UpdateChatReadInbox),
    UpdateChatReadOutbox(UpdateChatReadOutbox),
    UpdateChatUnreadMentionCount(UpdateChatUnreadMentionCount),
    UpdateChatNotificationSettings(UpdateChatNotificationSettings),
    UpdateScopeNotificationSettings(UpdateScopeNotificationSettings),
    UpdateChatActionBar(UpdateChatActionBar),
    UpdateChatPinnedMessage(UpdateChatPinnedMessage),
    UpdateChatReplyMarkup(UpdateChatReplyMarkup),
    UpdateChatDraftMessage(UpdateChatDraftMessage),
    UpdateChatOnlineMemberCount(UpdateChatOnlineMemberCount),
    UpdateNotification(UpdateNotification),
    UpdateNotificationGroup(UpdateNotificationGroup),
    UpdateActiveNotifications(UpdateActiveNotifications),
    UpdateHavePendingNotifications(UpdateHavePendingNotifications),
    UpdateDeleteMessages(UpdateDeleteMessages),
    UpdateUserChatAction(UpdateUserChatAction),
    UpdateUserStatus(UpdateUserStatus),
    UpdateUser(UpdateUser),
    UpdateBasicGroup(UpdateBasicGroup),
    UpdateSupergroup(UpdateSupergroup),
    UpdateSecretChat(UpdateSecretChat),
    UpdateUserFullInfo(UpdateUserFullInfo),
    UpdateBasicGroupFullInfo(UpdateBasicGroupFullInfo),
    UpdateSupergroupFullInfo(UpdateSupergroupFullInfo),
    UpdateServiceNotification(UpdateServiceNotification),
    UpdateFile(UpdateFile),
    UpdateFileGenerationStart(UpdateFileGenerationStart),
    UpdateFileGenerationStop(UpdateFileGenerationStop),
    UpdateCall(UpdateCall),
    UpdateUserPrivacySettingRules(UpdateUserPrivacySettingRules),
    UpdateUnreadMessageCount(UpdateUnreadMessageCount),
    UpdateUnreadChatCount(UpdateUnreadChatCount),
    UpdateOption(UpdateOption),
    UpdateStickerSet(UpdateStickerSet),
    UpdateInstalledStickerSets(UpdateInstalledStickerSets),
    UpdateTrendingStickerSets(UpdateTrendingStickerSets),
    UpdateRecentStickers(UpdateRecentStickers),
    UpdateFavoriteStickers(UpdateFavoriteStickers),
    UpdateSavedAnimations(UpdateSavedAnimations),
    UpdateSelectedBackground(UpdateSelectedBackground),
    UpdateLanguagePackStrings(UpdateLanguagePackStrings),
    UpdateConnectionState(UpdateConnectionState),
    UpdateTermsOfService(UpdateTermsOfService),
    UpdateUsersNearby(UpdateUsersNearby),
    UpdateDiceEmojis(UpdateDiceEmojis),
    UpdateNewInlineQuery(UpdateNewInlineQuery),
    UpdateNewChosenInlineResult(UpdateNewChosenInlineResult),
    UpdateNewCallbackQuery(UpdateNewCallbackQuery),
    UpdateNewInlineCallbackQuery(UpdateNewInlineCallbackQuery),
    UpdateNewShippingQuery(UpdateNewShippingQuery),
    UpdateNewPreCheckoutQuery(UpdateNewPreCheckoutQuery),
    UpdateNewCustomEvent(UpdateNewCustomEvent),
    UpdateNewCustomQuery(UpdateNewCustomQuery),
    UpdatePoll(UpdatePoll),
    UpdatePollAnswer(UpdatePollAnswer),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum CallDiscardReason {
    CallDiscardReasonEmpty(CallDiscardReasonEmpty),
    CallDiscardReasonMissed(CallDiscardReasonMissed),
    CallDiscardReasonDeclined(CallDiscardReasonDeclined),
    CallDiscardReasonDisconnected(CallDiscardReasonDisconnected),
    CallDiscardReasonHungUp(CallDiscardReasonHungUp),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
#[doc = ""]
pub enum CallbackQueryPayload {
    CallbackQueryPayloadData(CallbackQueryPayloadData),
    CallbackQueryPayloadGame(CallbackQueryPayloadGame),
}
