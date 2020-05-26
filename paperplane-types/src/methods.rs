use crate::types::*;
use serde::de::DeserializeOwned;
use serde::{ Serialize, Deserialize };
use std::fmt::Debug;

pub trait Method: Serialize + Clone {
    const TYPE: &'static str;
    type Response: DeserializeOwned + Debug;

    fn tag(self) -> MethodType<Self>
    where
        Self: Sized,
    {
        MethodType {
            type_: Self::TYPE,
            payload: self,
        }
    }
}
#[derive(Serialize, Debug, Clone)]
pub struct MethodType<T: Method> {
    #[serde(rename = "@type")]
    pub type_: &'static str,
    #[serde(flatten)]
    pub payload: T,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the current authorization state; this is an offline request. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state"]
pub struct GetAuthorizationState {}
impl Method for GetAuthorizationState {
    const TYPE: &'static str = "getAuthorizationState";
    type Response = AuthorizationState;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets the parameters for TDLib initialization. Works only when the current authorization state is authorizationStateWaitTdlibParameters "]
pub struct SetTdlibParameters {
    #[doc = "Parameters"]
    pub parameters: TdlibParameters,
}
impl Method for SetTdlibParameters {
    const TYPE: &'static str = "setTdlibParameters";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks the database encryption key for correctness. Works only when the current authorization state is authorizationStateWaitEncryptionKey "]
pub struct CheckDatabaseEncryptionKey {
    #[doc = "Encryption key to check or set up"]
    pub encryption_key: String,
}
impl Method for CheckDatabaseEncryptionKey {
    const TYPE: &'static str = "checkDatabaseEncryptionKey";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets the phone number of the user and sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitPhoneNumber,\n or if there is no pending authentication query and the current authorization state is authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword"]
pub struct SetAuthenticationPhoneNumber {
    #[doc = "The phone number of the user, in international format "]
    pub phone_number: String,
    #[doc = "Settings for the authentication of the user's phone number"]
    pub settings: PhoneNumberAuthenticationSettings,
}
impl Method for SetAuthenticationPhoneNumber {
    const TYPE: &'static str = "setAuthenticationPhoneNumber";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Re-sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitCode and the next_code_type of the result is not null"]
pub struct ResendAuthenticationCode {}
impl Method for ResendAuthenticationCode {
    const TYPE: &'static str = "resendAuthenticationCode";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks the authentication code. Works only when the current authorization state is authorizationStateWaitCode "]
pub struct CheckAuthenticationCode {
    #[doc = "The verification code received via SMS, Telegram message, phone call, or flash call"]
    pub code: String,
}
impl Method for CheckAuthenticationCode {
    const TYPE: &'static str = "checkAuthenticationCode";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Requests QR code authentication by scanning a QR code on another logged in device. Works only when the current authorization state is authorizationStateWaitPhoneNumber "]
pub struct RequestQrCodeAuthentication {
    #[doc = "List of user identifiers of other users currently using the client"]
    pub other_user_ids: Vec<i32>,
}
impl Method for RequestQrCodeAuthentication {
    const TYPE: &'static str = "requestQrCodeAuthentication";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Finishes user registration. Works only when the current authorization state is authorizationStateWaitRegistration"]
pub struct RegisterUser {
    #[doc = "The first name of the user; 1-64 characters "]
    pub first_name: String,
    #[doc = "The last name of the user; 0-64 characters"]
    pub last_name: String,
}
impl Method for RegisterUser {
    const TYPE: &'static str = "registerUser";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks the authentication password for correctness. Works only when the current authorization state is authorizationStateWaitPassword "]
pub struct CheckAuthenticationPassword {
    #[doc = "The password to check"]
    pub password: String,
}
impl Method for CheckAuthenticationPassword {
    const TYPE: &'static str = "checkAuthenticationPassword";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Requests to send a password recovery code to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword"]
pub struct RequestAuthenticationPasswordRecovery {}
impl Method for RequestAuthenticationPasswordRecovery {
    const TYPE: &'static str = "requestAuthenticationPasswordRecovery";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Recovers the password with a password recovery code sent to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword "]
pub struct RecoverAuthenticationPassword {
    #[doc = "Recovery code to check"]
    pub recovery_code: String,
}
impl Method for RecoverAuthenticationPassword {
    const TYPE: &'static str = "recoverAuthenticationPassword";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks the authentication token of a bot; to log in as a bot. Works only when the current authorization state is authorizationStateWaitPhoneNumber. Can be used instead of setAuthenticationPhoneNumber and checkAuthenticationCode to log in "]
pub struct CheckAuthenticationBotToken {
    #[doc = "The bot token"]
    pub token: String,
}
impl Method for CheckAuthenticationBotToken {
    const TYPE: &'static str = "checkAuthenticationBotToken";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Closes the TDLib instance after a proper logout. Requires an available network connection. All local data will be destroyed. After the logout completes, updateAuthorizationState with authorizationStateClosed will be sent"]
pub struct LogOut {}
impl Method for LogOut {
    const TYPE: &'static str = "logOut";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Closes the TDLib instance. All databases will be flushed to disk and properly closed. After the close completes, updateAuthorizationState with authorizationStateClosed will be sent"]
pub struct Close {}
impl Method for Close {
    const TYPE: &'static str = "close";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Closes the TDLib instance, destroying all local data without a proper logout. The current user session will remain in the list of all active sessions. All local data will be destroyed. After the destruction completes updateAuthorizationState with authorizationStateClosed will be sent"]
pub struct Destroy {}
impl Method for Destroy {
    const TYPE: &'static str = "destroy";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Confirms QR code authentication on another device. Returns created session on success "]
pub struct ConfirmQrCodeAuthentication {
    #[doc = "A link from a QR code. The link must be scanned by the in-app camera"]
    pub link: String,
}
impl Method for ConfirmQrCodeAuthentication {
    const TYPE: &'static str = "confirmQrCodeAuthentication";
    type Response = Session;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns all updates needed to restore current TDLib state, i.e. all actual UpdateAuthorizationState/UpdateUser/UpdateNewChat and others. This is especially useful if TDLib is run in a separate process. This is an offline method. Can be called before authorization"]
pub struct GetCurrentState {}
impl Method for GetCurrentState {
    const TYPE: &'static str = "getCurrentState";
    type Response = Updates;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the database encryption key. Usually the encryption key is never changed and is stored in some OS keychain "]
pub struct SetDatabaseEncryptionKey {
    #[doc = "New encryption key"]
    pub new_encryption_key: String,
}
impl Method for SetDatabaseEncryptionKey {
    const TYPE: &'static str = "setDatabaseEncryptionKey";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the current state of 2-step verification"]
pub struct GetPasswordState {}
impl Method for GetPasswordState {
    const TYPE: &'static str = "getPasswordState";
    type Response = PasswordState;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the password for the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed"]
pub struct SetPassword {
    #[doc = "Previous password of the user "]
    pub old_password: String,
    #[doc = "New password of the user; may be empty to remove the password "]
    pub new_password: String,
    #[doc = "New password hint; may be empty "]
    pub new_hint: String,
    #[doc = "Pass true if the recovery email address should be changed "]
    #[serde(default)]
    pub set_recovery_email_address: bool,
    #[doc = "New recovery email address; may be empty"]
    pub new_recovery_email_address: String,
}
impl Method for SetPassword {
    const TYPE: &'static str = "setPassword";
    type Response = PasswordState;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a 2-step verification recovery email address that was previously set up. This method can be used to verify a password provided by the user "]
pub struct GetRecoveryEmailAddress {
    #[doc = "The password for the current user"]
    pub password: String,
}
impl Method for GetRecoveryEmailAddress {
    const TYPE: &'static str = "getRecoveryEmailAddress";
    type Response = RecoveryEmailAddress;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the 2-step verification recovery email address of the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed.\n If new_recovery_email_address is the same as the email address that is currently set up, this call succeeds immediately and aborts all other requests waiting for an email confirmation "]
pub struct SetRecoveryEmailAddress {
    #[doc = "Password of the current user "]
    pub password: String,
    #[doc = "New recovery email address"]
    pub new_recovery_email_address: String,
}
impl Method for SetRecoveryEmailAddress {
    const TYPE: &'static str = "setRecoveryEmailAddress";
    type Response = PasswordState;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks the 2-step verification recovery email address verification code "]
pub struct CheckRecoveryEmailAddressCode {
    #[doc = "Verification code"]
    pub code: String,
}
impl Method for CheckRecoveryEmailAddressCode {
    const TYPE: &'static str = "checkRecoveryEmailAddressCode";
    type Response = PasswordState;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Resends the 2-step verification recovery email address verification code"]
pub struct ResendRecoveryEmailAddressCode {}
impl Method for ResendRecoveryEmailAddressCode {
    const TYPE: &'static str = "resendRecoveryEmailAddressCode";
    type Response = PasswordState;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Requests to send a password recovery code to an email address that was previously set up"]
pub struct RequestPasswordRecovery {}
impl Method for RequestPasswordRecovery {
    const TYPE: &'static str = "requestPasswordRecovery";
    type Response = EmailAddressAuthenticationCodeInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Recovers the password using a recovery code sent to an email address that was previously set up "]
pub struct RecoverPassword {
    #[doc = "Recovery code to check"]
    pub recovery_code: String,
}
impl Method for RecoverPassword {
    const TYPE: &'static str = "recoverPassword";
    type Response = PasswordState;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Creates a new temporary password for processing payments "]
pub struct CreateTemporaryPassword {
    #[doc = "Persistent user password "]
    pub password: String,
    #[doc = "Time during which the temporary password will be valid, in seconds; should be between 60 and 86400"]
    pub valid_for: i32,
}
impl Method for CreateTemporaryPassword {
    const TYPE: &'static str = "createTemporaryPassword";
    type Response = TemporaryPasswordState;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about the current temporary password"]
pub struct GetTemporaryPasswordState {}
impl Method for GetTemporaryPasswordState {
    const TYPE: &'static str = "getTemporaryPasswordState";
    type Response = TemporaryPasswordState;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the current user"]
pub struct GetMe {}
impl Method for GetMe {
    const TYPE: &'static str = "getMe";
    type Response = User;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a user by their identifier. This is an offline request if the current user is not a bot "]
pub struct GetUser {
    #[doc = "User identifier"]
    pub user_id: i32,
}
impl Method for GetUser {
    const TYPE: &'static str = "getUser";
    type Response = User;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns full information about a user by their identifier "]
pub struct GetUserFullInfo {
    #[doc = "User identifier"]
    pub user_id: i32,
}
impl Method for GetUserFullInfo {
    const TYPE: &'static str = "getUserFullInfo";
    type Response = UserFullInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a basic group by its identifier. This is an offline request if the current user is not a bot "]
pub struct GetBasicGroup {
    #[doc = "Basic group identifier"]
    pub basic_group_id: i32,
}
impl Method for GetBasicGroup {
    const TYPE: &'static str = "getBasicGroup";
    type Response = BasicGroup;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns full information about a basic group by its identifier "]
pub struct GetBasicGroupFullInfo {
    #[doc = "Basic group identifier"]
    pub basic_group_id: i32,
}
impl Method for GetBasicGroupFullInfo {
    const TYPE: &'static str = "getBasicGroupFullInfo";
    type Response = BasicGroupFullInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a supergroup or a channel by its identifier. This is an offline request if the current user is not a bot "]
pub struct GetSupergroup {
    #[doc = "Supergroup or channel identifier"]
    pub supergroup_id: i32,
}
impl Method for GetSupergroup {
    const TYPE: &'static str = "getSupergroup";
    type Response = Supergroup;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns full information about a supergroup or a channel by its identifier, cached for up to 1 minute "]
pub struct GetSupergroupFullInfo {
    #[doc = "Supergroup or channel identifier"]
    pub supergroup_id: i32,
}
impl Method for GetSupergroupFullInfo {
    const TYPE: &'static str = "getSupergroupFullInfo";
    type Response = SupergroupFullInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a secret chat by its identifier. This is an offline request "]
pub struct GetSecretChat {
    #[doc = "Secret chat identifier"]
    pub secret_chat_id: i32,
}
impl Method for GetSecretChat {
    const TYPE: &'static str = "getSecretChat";
    type Response = SecretChat;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a chat by its identifier, this is an offline request if the current user is not a bot "]
pub struct GetChat {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for GetChat {
    const TYPE: &'static str = "getChat";
    type Response = Chat;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a message "]
pub struct GetMessage {
    #[doc = "Identifier of the chat the message belongs to "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message to get"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
}
impl Method for GetMessage {
    const TYPE: &'static str = "getMessage";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a message, if it is available locally without sending network request. This is an offline request "]
pub struct GetMessageLocally {
    #[doc = "Identifier of the chat the message belongs to "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message to get"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
}
impl Method for GetMessageLocally {
    const TYPE: &'static str = "getMessageLocally";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a message that is replied by given message "]
pub struct GetRepliedMessage {
    #[doc = "Identifier of the chat the message belongs to "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message reply to which get"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
}
impl Method for GetRepliedMessage {
    const TYPE: &'static str = "getRepliedMessage";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a pinned chat message "]
pub struct GetChatPinnedMessage {
    #[doc = "Identifier of the chat the message belongs to"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for GetChatPinnedMessage {
    const TYPE: &'static str = "getChatPinnedMessage";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about messages. If a message is not found, returns null on the corresponding position of the result "]
pub struct GetMessages {
    #[doc = "Identifier of the chat the messages belong to "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifiers of the messages to get"]
    pub message_ids: Vec<i64>,
}
impl Method for GetMessages {
    const TYPE: &'static str = "getMessages";
    type Response = Messages;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a file; this is an offline request "]
pub struct GetFile {
    #[doc = "Identifier of the file to get"]
    pub file_id: i32,
}
impl Method for GetFile {
    const TYPE: &'static str = "getFile";
    type Response = File;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a file by its remote ID; this is an offline request. Can be used to register a URL as a file for further uploading, or sending as a message. Even the request succeeds, the file can be used only if it is still accessible to the user.\n For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the client"]
pub struct GetRemoteFile {
    #[doc = "Remote identifier of the file to get "]
    pub remote_file_id: String,
    #[doc = "File type, if known"]
    pub file_type: FileType,
}
impl Method for GetRemoteFile {
    const TYPE: &'static str = "getRemoteFile";
    type Response = File;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns an ordered list of chats in a chat list. Chats are sorted by the pair (order, chat_id) in decreasing order. (For example, to get a list of chats from the beginning, the offset_order should be equal to a biggest signed 64-bit number 9223372036854775807 == 2^63 - 1).\n For optimal performance the number of returned chats is chosen by the library"]
pub struct GetChats {
    #[doc = "The chat list in which to return chats"]
    pub chat_list: ChatList,
    #[doc = "Chat order to return chats from "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub offset_order: i64,
    #[doc = "Chat identifier to return chats from"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub offset_chat_id: i64,
    #[doc = "The maximum number of chats to be returned. It is possible that fewer chats than the limit are returned even if the end of the list is not reached"]
    pub limit: i32,
}
impl Method for GetChats {
    const TYPE: &'static str = "getChats";
    type Response = Chats;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches a public chat by its username. Currently only private chats, supergroups and channels can be public. Returns the chat if found; otherwise an error is returned "]
pub struct SearchPublicChat {
    #[doc = "Username to be resolved"]
    pub username: String,
}
impl Method for SearchPublicChat {
    const TYPE: &'static str = "searchPublicChat";
    type Response = Chat;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches public chats by looking for specified query in their username and title. Currently only private chats, supergroups and channels can be public. Returns a meaningful number of results. Returns nothing if the length of the searched username prefix is less than 5. Excludes private chats with contacts and chats from the chat list from the results "]
pub struct SearchPublicChats {
    #[doc = "Query to search for"]
    pub query: String,
}
impl Method for SearchPublicChats {
    const TYPE: &'static str = "searchPublicChats";
    type Response = Chats;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for the specified query in the title and username of already known chats, this is an offline request. Returns chats in the order seen in the chat list "]
pub struct SearchChats {
    #[doc = "Query to search for. If the query is empty, returns up to 20 recently found chats "]
    pub query: String,
    #[doc = "The maximum number of chats to be returned"]
    pub limit: i32,
}
impl Method for SearchChats {
    const TYPE: &'static str = "searchChats";
    type Response = Chats;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for the specified query in the title and username of already known chats via request to the server. Returns chats in the order seen in the chat list "]
pub struct SearchChatsOnServer {
    #[doc = "Query to search for "]
    pub query: String,
    #[doc = "The maximum number of chats to be returned"]
    pub limit: i32,
}
impl Method for SearchChatsOnServer {
    const TYPE: &'static str = "searchChatsOnServer";
    type Response = Chats;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of users and location-based supergroups nearby. The list of users nearby will be updated for 60 seconds after the request by the updates updateUsersNearby. The request should be sent again every 25 seconds with adjusted location to not miss new chats "]
pub struct SearchChatsNearby {
    #[doc = "Current user location"]
    pub location: Location,
}
impl Method for SearchChatsNearby {
    const TYPE: &'static str = "searchChatsNearby";
    type Response = ChatsNearby;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of frequently used chats. Supported only if the chat info database is enabled "]
pub struct GetTopChats {
    #[doc = "Category of chats to be returned "]
    pub category: TopChatCategory,
    #[doc = "The maximum number of chats to be returned; up to 30"]
    pub limit: i32,
}
impl Method for GetTopChats {
    const TYPE: &'static str = "getTopChats";
    type Response = Chats;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes a chat from the list of frequently used chats. Supported only if the chat info database is enabled "]
pub struct RemoveTopChat {
    #[doc = "Category of frequently used chats "]
    pub category: TopChatCategory,
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for RemoveTopChat {
    const TYPE: &'static str = "removeTopChat";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds a chat to the list of recently found chats. The chat is added to the beginning of the list. If the chat is already in the list, it will be removed from the list first "]
pub struct AddRecentlyFoundChat {
    #[doc = "Identifier of the chat to add"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for AddRecentlyFoundChat {
    const TYPE: &'static str = "addRecentlyFoundChat";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes a chat from the list of recently found chats "]
pub struct RemoveRecentlyFoundChat {
    #[doc = "Identifier of the chat to be removed"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for RemoveRecentlyFoundChat {
    const TYPE: &'static str = "removeRecentlyFoundChat";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Clears the list of recently found chats"]
pub struct ClearRecentlyFoundChats {}
impl Method for ClearRecentlyFoundChats {
    const TYPE: &'static str = "clearRecentlyFoundChats";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks whether a username can be set for a chat "]
pub struct CheckChatUsername {
    #[doc = "Chat identifier; should be identifier of a supergroup chat, or a channel chat, or a private chat with self, or zero if chat is being created "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Username to be checked"]
    pub username: String,
}
impl Method for CheckChatUsername {
    const TYPE: &'static str = "checkChatUsername";
    type Response = CheckChatUsernameResult;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of public chats of the specified type, owned by the user "]
pub struct GetCreatedPublicChats {
    #[serde(rename = "type")]
    #[doc = "Type of the public chats to return"]
    pub type_: PublicChatType,
}
impl Method for GetCreatedPublicChats {
    const TYPE: &'static str = "getCreatedPublicChats";
    type Response = Chats;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks whether the maximum number of owned public chats has been reached. Returns corresponding error if the limit was reached "]
pub struct CheckCreatedPublicChatsLimit {
    #[serde(rename = "type")]
    #[doc = "Type of the public chats, for which to check the limit"]
    pub type_: PublicChatType,
}
impl Method for CheckCreatedPublicChatsLimit {
    const TYPE: &'static str = "checkCreatedPublicChatsLimit";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of basic group and supergroup chats, which can be used as a discussion group for a channel. Basic group chats need to be first upgraded to supergroups before they can be set as a discussion group"]
pub struct GetSuitableDiscussionChats {}
impl Method for GetSuitableDiscussionChats {
    const TYPE: &'static str = "getSuitableDiscussionChats";
    type Response = Chats;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of recently inactive supergroups and channels. Can be used when user reaches limit on the number of joined supergroups and channels and receives CHANNELS_TOO_MUCH error"]
pub struct GetInactiveSupergroupChats {}
impl Method for GetInactiveSupergroupChats {
    const TYPE: &'static str = "getInactiveSupergroupChats";
    type Response = Chats;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of common group chats with a given user. Chats are sorted by their type and creation date "]
pub struct GetGroupsInCommon {
    #[doc = "User identifier "]
    pub user_id: i32,
    #[doc = "Chat identifier starting from which to return chats; use 0 for the first request "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub offset_chat_id: i64,
    #[doc = "The maximum number of chats to be returned; up to 100"]
    pub limit: i32,
}
impl Method for GetGroupsInCommon {
    const TYPE: &'static str = "getGroupsInCommon";
    type Response = Chats;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id).\n For optimal performance the number of returned messages is chosen by the library. This is an offline request if only_local is true"]
pub struct GetChatHistory {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message starting from which history must be fetched; use 0 to get results from the last message"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub from_message_id: i64,
    #[doc = "Specify 0 to get results from exactly the from_message_id or a negative offset up to 99 to get additionally some newer messages"]
    pub offset: i32,
    #[doc = "The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater or equal to -offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached"]
    pub limit: i32,
    #[doc = "If true, returns only messages that are available locally without sending network requests"]
    #[serde(default)]
    pub only_local: bool,
}
impl Method for GetChatHistory {
    const TYPE: &'static str = "getChatHistory";
    type Response = Messages;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Deletes all messages in the chat. Use Chat.can_be_deleted_only_for_self and Chat.can_be_deleted_for_all_users fields to find whether and how the method can be applied to the chat"]
pub struct DeleteChatHistory {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Pass true if the chat should be removed from the chat list "]
    #[serde(default)]
    pub remove_from_chat_list: bool,
    #[doc = "Pass true to try to delete chat history for all users"]
    #[serde(default)]
    pub revoke: bool,
}
impl Method for DeleteChatHistory {
    const TYPE: &'static str = "deleteChatHistory";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query\n (searchSecretMessages should be used instead), or without an enabled message database. For optimal performance the number of returned messages is chosen by the library"]
pub struct SearchChatMessages {
    #[doc = "Identifier of the chat in which to search messages"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Query to search for"]
    pub query: String,
    #[doc = "If not 0, only messages sent by the specified user will be returned. Not supported in secret chats"]
    pub sender_user_id: i32,
    #[doc = "Identifier of the message starting from which history must be fetched; use 0 to get results from the last message"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub from_message_id: i64,
    #[doc = "Specify 0 to get results from exactly the from_message_id or a negative offset to get the specified message and some newer messages"]
    pub offset: i32,
    #[doc = "The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater than -offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached"]
    pub limit: i32,
    #[doc = "Filter for message content in the search results"]
    pub filter: SearchMessagesFilter,
}
impl Method for SearchChatMessages {
    const TYPE: &'static str = "searchChatMessages";
    type Response = Messages;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for messages in all chats except secret chats. Returns the results in reverse chronological order (i.e., in order of decreasing (date, chat_id, message_id)).\n For optimal performance the number of returned messages is chosen by the library"]
pub struct SearchMessages {
    #[doc = "Chat list in which to search messages; pass null to search in all chats regardless of their chat list"]
    pub chat_list: ChatList,
    #[doc = "Query to search for"]
    pub query: String,
    #[doc = "The date of the message starting from which the results should be fetched. Use 0 or any date in the future to get results from the last message"]
    pub offset_date: i32,
    #[doc = "The chat identifier of the last found message, or 0 for the first request"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub offset_chat_id: i64,
    #[doc = "The message identifier of the last found message, or 0 for the first request"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub offset_message_id: i64,
    #[doc = "The maximum number of messages to be returned, up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached"]
    pub limit: i32,
}
impl Method for SearchMessages {
    const TYPE: &'static str = "searchMessages";
    type Response = Messages;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for messages in secret chats. Returns the results in reverse chronological order. For optimal performance the number of returned messages is chosen by the library"]
pub struct SearchSecretMessages {
    #[doc = "Identifier of the chat in which to search. Specify 0 to search in all secret chats "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Query to search for. If empty, searchChatMessages should be used instead"]
    pub query: String,
    #[doc = "The identifier from the result of a previous request, use 0 to get results from the last message"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub from_search_id: i64,
    #[doc = "The maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached"]
    pub limit: i32,
    #[doc = "A filter for the content of messages in the search results"]
    pub filter: SearchMessagesFilter,
}
impl Method for SearchSecretMessages {
    const TYPE: &'static str = "searchSecretMessages";
    type Response = FoundMessages;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for call messages. Returns the results in reverse chronological order (i. e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library"]
pub struct SearchCallMessages {
    #[doc = "Identifier of the message from which to search; use 0 to get results from the last message"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub from_message_id: i64,
    #[doc = "The maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached "]
    pub limit: i32,
    #[doc = "If true, returns only messages with missed calls"]
    #[serde(default)]
    pub only_missed: bool,
}
impl Method for SearchCallMessages {
    const TYPE: &'static str = "searchCallMessages";
    type Response = Messages;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about the recent locations of chat members that were sent to the chat. Returns up to 1 location message per user "]
pub struct SearchChatRecentLocationMessages {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The maximum number of messages to be returned"]
    pub limit: i32,
}
impl Method for SearchChatRecentLocationMessages {
    const TYPE: &'static str = "searchChatRecentLocationMessages";
    type Response = Messages;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns all active live locations that should be updated by the client. The list is persistent across application restarts only if the message database is used"]
pub struct GetActiveLiveLocationMessages {}
impl Method for GetActiveLiveLocationMessages {
    const TYPE: &'static str = "getActiveLiveLocationMessages";
    type Response = Messages;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the last message sent in a chat no later than the specified date "]
pub struct GetChatMessageByDate {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Point in time (Unix timestamp) relative to which to search for messages"]
    pub date: i32,
}
impl Method for GetChatMessageByDate {
    const TYPE: &'static str = "getChatMessageByDate";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns approximate number of messages of the specified type in the chat "]
pub struct GetChatMessageCount {
    #[doc = "Identifier of the chat in which to count messages "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Filter for message content; searchMessagesFilterEmpty is unsupported in this function "]
    pub filter: SearchMessagesFilter,
    #[doc = "If true, returns count that is available locally without sending network requests, returning -1 if the number of messages is unknown"]
    #[serde(default)]
    pub return_local: bool,
}
impl Method for GetChatMessageCount {
    const TYPE: &'static str = "getChatMessageCount";
    type Response = Count;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns all scheduled messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id) "]
pub struct GetChatScheduledMessages {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for GetChatScheduledMessages {
    const TYPE: &'static str = "getChatScheduledMessages";
    type Response = Messages;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes an active notification from notification list. Needs to be called only if the notification is removed by the current user "]
pub struct RemoveNotification {
    #[doc = "Identifier of notification group to which the notification belongs "]
    pub notification_group_id: i32,
    #[doc = "Identifier of removed notification"]
    pub notification_id: i32,
}
impl Method for RemoveNotification {
    const TYPE: &'static str = "removeNotification";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes a group of active notifications. Needs to be called only if the notification group is removed by the current user "]
pub struct RemoveNotificationGroup {
    #[doc = "Notification group identifier "]
    pub notification_group_id: i32,
    #[doc = "The maximum identifier of removed notifications"]
    pub max_notification_id: i32,
}
impl Method for RemoveNotificationGroup {
    const TYPE: &'static str = "removeNotificationGroup";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a public HTTPS link to a message. Available only for messages in supergroups and channels with a username"]
pub struct GetPublicMessageLink {
    #[doc = "Identifier of the chat to which the message belongs"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "Pass true if a link for a whole media album should be returned"]
    #[serde(default)]
    pub for_album: bool,
}
impl Method for GetPublicMessageLink {
    const TYPE: &'static str = "getPublicMessageLink";
    type Response = PublicMessageLink;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a private HTTPS link to a message in a chat. Available only for already sent messages in supergroups and channels. The link will work only for members of the chat"]
pub struct GetMessageLink {
    #[doc = "Identifier of the chat to which the message belongs"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
}
impl Method for GetMessageLink {
    const TYPE: &'static str = "getMessageLink";
    type Response = HttpUrl;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a public or private message link "]
pub struct GetMessageLinkInfo {
    #[doc = "The message link in the format \"https://t.me/c/...\", or \"tg://privatepost?...\", or \"https://t.me/username/...\", or \"tg://resolve?...\""]
    pub url: String,
}
impl Method for GetMessageLinkInfo {
    const TYPE: &'static str = "getMessageLinkInfo";
    type Response = MessageLinkInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends a message. Returns the sent message"]
pub struct SendMessage {
    #[doc = "Target chat "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message to reply to or 0"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub reply_to_message_id: i64,
    #[doc = "Options to be used to send the message"]
    pub options: SendMessageOptions,
    #[doc = "Markup for replying to the message; for bots only "]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "The content of the message to be sent"]
    pub input_message_content: InputMessageContent,
}
impl Method for SendMessage {
    const TYPE: &'static str = "sendMessage";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends messages grouped together into an album. Currently only photo and video messages can be grouped into an album. Returns sent messages"]
pub struct SendMessageAlbum {
    #[doc = "Target chat "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of a message to reply to or 0"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub reply_to_message_id: i64,
    #[doc = "Options to be used to send the messages"]
    pub options: SendMessageOptions,
    #[doc = "Contents of messages to be sent"]
    pub input_message_contents: Vec<InputMessageContent>,
}
impl Method for SendMessageAlbum {
    const TYPE: &'static str = "sendMessageAlbum";
    type Response = Messages;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Invites a bot to a chat (if it is not yet a member) and sends it the /start command. Bots can't be invited to a private chat other than the chat with the bot. Bots can't be invited to channels (although they can be added as admins) and secret chats. Returns the sent message"]
pub struct SendBotStartMessage {
    #[doc = "Identifier of the bot "]
    pub bot_user_id: i32,
    #[doc = "Identifier of the target chat "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "A hidden parameter sent to the bot for deep linking purposes (https://core.telegram.org/bots#deep-linking)"]
    pub parameter: String,
}
impl Method for SendBotStartMessage {
    const TYPE: &'static str = "sendBotStartMessage";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends the result of an inline query as a message. Returns the sent message. Always clears a chat draft message"]
pub struct SendInlineQueryResultMessage {
    #[doc = "Target chat "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of a message to reply to or 0"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub reply_to_message_id: i64,
    #[doc = "Options to be used to send the message"]
    pub options: SendMessageOptions,
    #[doc = "Identifier of the inline query "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub query_id: i64,
    #[doc = "Identifier of the inline result"]
    pub result_id: String,
    #[doc = "If true, there will be no mention of a bot, via which the message is sent. Can be used only for bots GetOption(\"animation_search_bot_username\"), GetOption(\"photo_search_bot_username\") and GetOption(\"venue_search_bot_username\")"]
    #[serde(default)]
    pub hide_via_bot: bool,
}
impl Method for SendInlineQueryResultMessage {
    const TYPE: &'static str = "sendInlineQueryResultMessage";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Forwards previously sent messages. Returns the forwarded messages in the same order as the message identifiers passed in message_ids. If a message can't be forwarded, null will be returned instead of the message"]
pub struct ForwardMessages {
    #[doc = "Identifier of the chat to which to forward messages "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the chat from which to forward messages "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub from_chat_id: i64,
    #[doc = "Identifiers of the messages to forward"]
    pub message_ids: Vec<i64>,
    #[doc = "Options to be used to send the messages"]
    pub options: SendMessageOptions,
    #[doc = "True, if the messages should be grouped into an album after forwarding. For this to work, no more than 10 messages may be forwarded, and all of them must be photo or video messages"]
    #[serde(default)]
    pub as_album: bool,
    #[doc = "True, if content of the messages needs to be copied without links to the original messages. Always true if the messages are forwarded to a secret chat"]
    #[serde(default)]
    pub send_copy: bool,
    #[doc = "True, if media captions of message copies needs to be removed. Ignored if send_copy is false"]
    #[serde(default)]
    pub remove_caption: bool,
}
impl Method for ForwardMessages {
    const TYPE: &'static str = "forwardMessages";
    type Response = Messages;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Resends messages which failed to send. Can be called only for messages for which messageSendingStateFailed.can_retry is true and after specified in messageSendingStateFailed.retry_after time passed.\n If a message is re-sent, the corresponding failed to send message is deleted. Returns the sent messages in the same order as the message identifiers passed in message_ids. If a message can't be re-sent, null will be returned instead of the message"]
pub struct ResendMessages {
    #[doc = "Identifier of the chat to send messages "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifiers of the messages to resend. Message identifiers must be in a strictly increasing order"]
    pub message_ids: Vec<i64>,
}
impl Method for ResendMessages {
    const TYPE: &'static str = "resendMessages";
    type Response = Messages;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the current TTL setting (sets a new self-destruct timer) in a secret chat and sends the corresponding message "]
pub struct SendChatSetTtlMessage {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New TTL value, in seconds"]
    pub ttl: i32,
}
impl Method for SendChatSetTtlMessage {
    const TYPE: &'static str = "sendChatSetTtlMessage";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends a notification about a screenshot taken in a chat. Supported only in private and secret chats "]
pub struct SendChatScreenshotTakenNotification {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for SendChatScreenshotTakenNotification {
    const TYPE: &'static str = "sendChatScreenshotTakenNotification";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds a local message to a chat. The message is persistent across application restarts only if the message database is used. Returns the added message "]
pub struct AddLocalMessage {
    #[doc = "Target chat "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the user who will be shown as the sender of the message; may be 0 for channel posts"]
    pub sender_user_id: i32,
    #[doc = "Identifier of the message to reply to or 0 "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub reply_to_message_id: i64,
    #[doc = "Pass true to disable notification for the message "]
    #[serde(default)]
    pub disable_notification: bool,
    #[doc = "The content of the message to be added"]
    pub input_message_content: InputMessageContent,
}
impl Method for AddLocalMessage {
    const TYPE: &'static str = "addLocalMessage";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Deletes messages "]
pub struct DeleteMessages {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifiers of the messages to be deleted "]
    pub message_ids: Vec<i64>,
    #[doc = "Pass true to try to delete messages for all chat members. Always true for supergroups, channels and secret chats"]
    #[serde(default)]
    pub revoke: bool,
}
impl Method for DeleteMessages {
    const TYPE: &'static str = "deleteMessages";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Deletes all messages sent by the specified user to a chat. Supported only for supergroups; requires can_delete_messages administrator privileges "]
pub struct DeleteChatMessagesFromUser {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "User identifier"]
    pub user_id: i32,
}
impl Method for DeleteChatMessagesFromUser {
    const TYPE: &'static str = "deleteChatMessagesFromUser";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits the text of a message (or a text of a game message). Returns the edited message after the edit is completed on the server side"]
pub struct EditMessageText {
    #[doc = "The chat the message belongs to "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "The new message reply markup; for bots only "]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "New text content of the message. Should be of type InputMessageText"]
    pub input_message_content: InputMessageContent,
}
impl Method for EditMessageText {
    const TYPE: &'static str = "editMessageText";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits the message content of a live location. Messages can be edited for a limited period of time specified in the live location. Returns the edited message after the edit is completed on the server side"]
pub struct EditMessageLiveLocation {
    #[doc = "The chat the message belongs to "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "The new message reply markup; for bots only "]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "New location content of the message; may be null. Pass null to stop sharing the live location"]
    pub location: Option<Location>,
}
impl Method for EditMessageLiveLocation {
    const TYPE: &'static str = "editMessageLiveLocation";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits the content of a message with an animation, an audio, a document, a photo or a video. The media in the message can't be replaced if the message was set to self-destruct. Media can't be replaced by self-destructing media. Media in an album can be edited only to contain a photo or a video. Returns the edited message after the edit is completed on the server side"]
pub struct EditMessageMedia {
    #[doc = "The chat the message belongs to "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "The new message reply markup; for bots only "]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "New content of the message. Must be one of the following types: InputMessageAnimation, InputMessageAudio, InputMessageDocument, InputMessagePhoto or InputMessageVideo"]
    pub input_message_content: InputMessageContent,
}
impl Method for EditMessageMedia {
    const TYPE: &'static str = "editMessageMedia";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits the message content caption. Returns the edited message after the edit is completed on the server side"]
pub struct EditMessageCaption {
    #[doc = "The chat the message belongs to "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "The new message reply markup; for bots only "]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "New message content caption; 0-GetOption(\"message_caption_length_max\") characters"]
    pub caption: FormattedText,
}
impl Method for EditMessageCaption {
    const TYPE: &'static str = "editMessageCaption";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits the message reply markup; for bots only. Returns the edited message after the edit is completed on the server side"]
pub struct EditMessageReplyMarkup {
    #[doc = "The chat the message belongs to "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "The new message reply markup"]
    pub reply_markup: ReplyMarkup,
}
impl Method for EditMessageReplyMarkup {
    const TYPE: &'static str = "editMessageReplyMarkup";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits the text of an inline text or game message sent via a bot; for bots only "]
pub struct EditInlineMessageText {
    #[doc = "Inline message identifier "]
    pub inline_message_id: String,
    #[doc = "The new message reply markup "]
    pub reply_markup: ReplyMarkup,
    #[doc = "New text content of the message. Should be of type InputMessageText"]
    pub input_message_content: InputMessageContent,
}
impl Method for EditInlineMessageText {
    const TYPE: &'static str = "editInlineMessageText";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits the content of a live location in an inline message sent via a bot; for bots only "]
pub struct EditInlineMessageLiveLocation {
    #[doc = "Inline message identifier "]
    pub inline_message_id: String,
    #[doc = "The new message reply markup "]
    pub reply_markup: ReplyMarkup,
    #[doc = "New location content of the message; may be null. Pass null to stop sharing the live location"]
    pub location: Option<Location>,
}
impl Method for EditInlineMessageLiveLocation {
    const TYPE: &'static str = "editInlineMessageLiveLocation";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits the content of a message with an animation, an audio, a document, a photo or a video in an inline message sent via a bot; for bots only "]
pub struct EditInlineMessageMedia {
    #[doc = "Inline message identifier"]
    pub inline_message_id: String,
    #[doc = "The new message reply markup; for bots only "]
    pub reply_markup: Option<ReplyMarkup>,
    #[doc = "New content of the message. Must be one of the following types: InputMessageAnimation, InputMessageAudio, InputMessageDocument, InputMessagePhoto or InputMessageVideo"]
    pub input_message_content: InputMessageContent,
}
impl Method for EditInlineMessageMedia {
    const TYPE: &'static str = "editInlineMessageMedia";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits the caption of an inline message sent via a bot; for bots only "]
pub struct EditInlineMessageCaption {
    #[doc = "Inline message identifier "]
    pub inline_message_id: String,
    #[doc = "The new message reply markup "]
    pub reply_markup: ReplyMarkup,
    #[doc = "New message content caption; 0-GetOption(\"message_caption_length_max\") characters"]
    pub caption: FormattedText,
}
impl Method for EditInlineMessageCaption {
    const TYPE: &'static str = "editInlineMessageCaption";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits the reply markup of an inline message sent via a bot; for bots only "]
pub struct EditInlineMessageReplyMarkup {
    #[doc = "Inline message identifier "]
    pub inline_message_id: String,
    #[doc = "The new message reply markup"]
    pub reply_markup: ReplyMarkup,
}
impl Method for EditInlineMessageReplyMarkup {
    const TYPE: &'static str = "editInlineMessageReplyMarkup";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits the time when a scheduled message will be sent. Scheduling state of all messages in the same album or forwarded together with the message will be also changed "]
pub struct EditMessageSchedulingState {
    #[doc = "The chat the message belongs to "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "The new message scheduling state. Pass null to send the message immediately"]
    pub scheduling_state: MessageSchedulingState,
}
impl Method for EditMessageSchedulingState {
    const TYPE: &'static str = "editMessageSchedulingState";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns all entities (mentions, hashtags, cashtags, bot commands, bank card numbers, URLs, and email addresses) contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously "]
pub struct GetTextEntities {
    #[doc = "The text in which to look for entites"]
    pub text: String,
}
impl Method for GetTextEntities {
    const TYPE: &'static str = "getTextEntities";
    type Response = TextEntities;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Parses Bold, Italic, Underline, Strikethrough, Code, Pre, PreCode, TextUrl and MentionName entities contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously "]
pub struct ParseTextEntities {
    #[doc = "The text to parse "]
    pub text: String,
    #[doc = "Text parse mode"]
    pub parse_mode: TextParseMode,
}
impl Method for ParseTextEntities {
    const TYPE: &'static str = "parseTextEntities";
    type Response = FormattedText;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Parses Markdown entities in a human-friendly format, ignoring mark up errors. This is an offline method. Can be called before authorization. Can be called synchronously"]
pub struct ParseMarkdown {
    #[doc = "The text to parse. For example, \"__italic__ ~~strikethrough~~ **bold** `code` ```pre``` __[italic__ text_url](telegram.org) __italic**bold italic__bold**\""]
    pub text: FormattedText,
}
impl Method for ParseMarkdown {
    const TYPE: &'static str = "parseMarkdown";
    type Response = FormattedText;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Replaces text entities with Markdown formatting in a human-friendly format. Entities that can't be represented in Markdown unambiguously are kept as is. This is an offline method. Can be called before authorization. Can be called synchronously "]
pub struct GetMarkdownText {
    #[doc = "The text"]
    pub text: FormattedText,
}
impl Method for GetMarkdownText {
    const TYPE: &'static str = "getMarkdownText";
    type Response = FormattedText;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the MIME type of a file, guessed by its extension. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously "]
pub struct GetFileMimeType {
    #[doc = "The name of the file or path to the file"]
    pub file_name: String,
}
impl Method for GetFileMimeType {
    const TYPE: &'static str = "getFileMimeType";
    type Response = Text;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the extension of a file, guessed by its MIME type. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously "]
pub struct GetFileExtension {
    #[doc = "The MIME type of the file"]
    pub mime_type: String,
}
impl Method for GetFileExtension {
    const TYPE: &'static str = "getFileExtension";
    type Response = Text;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes potentially dangerous characters from the name of a file. The encoding of the file name is supposed to be UTF-8. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously "]
pub struct CleanFileName {
    #[doc = "File name or path to the file"]
    pub file_name: String,
}
impl Method for CleanFileName {
    const TYPE: &'static str = "cleanFileName";
    type Response = Text;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. This is an offline method. Can be called before authorization. Can be called synchronously"]
pub struct GetLanguagePackString {
    #[doc = "Path to the language pack database in which strings are stored "]
    pub language_pack_database_path: String,
    #[doc = "Localization target to which the language pack belongs "]
    pub localization_target: String,
    #[doc = "Language pack identifier "]
    pub language_pack_id: String,
    #[doc = "Language pack key of the string to be returned"]
    pub key: String,
}
impl Method for GetLanguagePackString {
    const TYPE: &'static str = "getLanguagePackString";
    type Response = LanguagePackStringValue;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Converts a JSON-serialized string to corresponding JsonValue object. This is an offline method. Can be called before authorization. Can be called synchronously "]
pub struct GetJsonValue {
    #[doc = "The JSON-serialized string"]
    pub json: String,
}
impl Method for GetJsonValue {
    const TYPE: &'static str = "getJsonValue";
    type Response = JsonValue;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Converts a JsonValue object to corresponding JSON-serialized string. This is an offline method. Can be called before authorization. Can be called synchronously "]
pub struct GetJsonString {
    #[doc = "The JsonValue object"]
    pub json_value: JsonValue,
}
impl Method for GetJsonString {
    const TYPE: &'static str = "getJsonString";
    type Response = Text;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the user answer to a poll. A poll in quiz mode can be answered only once"]
pub struct SetPollAnswer {
    #[doc = "Identifier of the chat to which the poll belongs "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message containing the poll"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "0-based identifiers of answer options, chosen by the user. User can choose more than 1 answer option only is the poll allows multiple answers"]
    pub option_ids: Vec<i32>,
}
impl Method for SetPollAnswer {
    const TYPE: &'static str = "setPollAnswer";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns users voted for the specified option in a non-anonymous polls. For the optimal performance the number of returned users is chosen by the library"]
pub struct GetPollVoters {
    #[doc = "Identifier of the chat to which the poll belongs "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message containing the poll"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "0-based identifier of the answer option"]
    pub option_id: i32,
    #[doc = "Number of users to skip in the result; must be non-negative"]
    pub offset: i32,
    #[doc = "The maximum number of users to be returned; must be positive and can't be greater than 50. Fewer users may be returned than specified by the limit, even if the end of the voter list has not been reached"]
    pub limit: i32,
}
impl Method for GetPollVoters {
    const TYPE: &'static str = "getPollVoters";
    type Response = Users;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Stops a poll. A poll in a message can be stopped when the message has can_be_edited flag set"]
pub struct StopPoll {
    #[doc = "Identifier of the chat to which the poll belongs "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message containing the poll "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "The new message reply markup; for bots only"]
    pub reply_markup: Option<ReplyMarkup>,
}
impl Method for StopPoll {
    const TYPE: &'static str = "stopPoll";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a button of type inlineKeyboardButtonTypeLoginUrl. The method needs to be called when the user presses the button"]
pub struct GetLoginUrlInfo {
    #[doc = "Chat identifier of the message with the button "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Message identifier of the message with the button "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "Button identifier"]
    pub button_id: i32,
}
impl Method for GetLoginUrlInfo {
    const TYPE: &'static str = "getLoginUrlInfo";
    type Response = LoginUrlInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns an HTTP URL which can be used to automatically authorize the user on a website after clicking an inline button of type inlineKeyboardButtonTypeLoginUrl.\n Use the method getLoginUrlInfo to find whether a prior user confirmation is needed. If an error is returned, then the button must be handled as an ordinary URL button"]
pub struct GetLoginUrl {
    #[doc = "Chat identifier of the message with the button "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Message identifier of the message with the button "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "Button identifier"]
    pub button_id: i32,
    #[doc = "True, if the user allowed the bot to send them messages"]
    #[serde(default)]
    pub allow_write_access: bool,
}
impl Method for GetLoginUrl {
    const TYPE: &'static str = "getLoginUrl";
    type Response = HttpUrl;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends an inline query to a bot and returns its results. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires "]
pub struct GetInlineQueryResults {
    #[doc = "The identifier of the target bot"]
    pub bot_user_id: i32,
    #[doc = "Identifier of the chat where the query was sent "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Location of the user, only if needed "]
    pub user_location: Location,
    #[doc = "Text of the query "]
    pub query: String,
    #[doc = "Offset of the first entry to return"]
    pub offset: String,
}
impl Method for GetInlineQueryResults {
    const TYPE: &'static str = "getInlineQueryResults";
    type Response = InlineQueryResults;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets the result of an inline query; for bots only "]
pub struct AnswerInlineQuery {
    #[doc = "Identifier of the inline query "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub inline_query_id: i64,
    #[doc = "True, if the result of the query can be cached for the specified user"]
    #[serde(default)]
    pub is_personal: bool,
    #[doc = "The results of the query "]
    pub results: Vec<InputInlineQueryResult>,
    #[doc = "Allowed time to cache the results of the query, in seconds "]
    pub cache_time: i32,
    #[doc = "Offset for the next inline query; pass an empty string if there are no more results"]
    pub next_offset: String,
    #[doc = "If non-empty, this text should be shown on the button that opens a private chat with the bot and sends a start message to the bot with the parameter switch_pm_parameter "]
    pub switch_pm_text: String,
    #[doc = "The parameter for the bot start message"]
    pub switch_pm_parameter: String,
}
impl Method for AnswerInlineQuery {
    const TYPE: &'static str = "answerInlineQuery";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends a callback query to a bot and returns an answer. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires "]
pub struct GetCallbackQueryAnswer {
    #[doc = "Identifier of the chat with the message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message from which the query originated "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "Query payload"]
    pub payload: CallbackQueryPayload,
}
impl Method for GetCallbackQueryAnswer {
    const TYPE: &'static str = "getCallbackQueryAnswer";
    type Response = CallbackQueryAnswer;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets the result of a callback query; for bots only "]
pub struct AnswerCallbackQuery {
    #[doc = "Identifier of the callback query "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub callback_query_id: i64,
    #[doc = "Text of the answer "]
    pub text: String,
    #[doc = "If true, an alert should be shown to the user instead of a toast notification "]
    #[serde(default)]
    pub show_alert: bool,
    #[doc = "URL to be opened "]
    pub url: String,
    #[doc = "Time during which the result of the query can be cached, in seconds"]
    pub cache_time: i32,
}
impl Method for AnswerCallbackQuery {
    const TYPE: &'static str = "answerCallbackQuery";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets the result of a shipping query; for bots only "]
pub struct AnswerShippingQuery {
    #[doc = "Identifier of the shipping query "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub shipping_query_id: i64,
    #[doc = "Available shipping options "]
    pub shipping_options: Vec<ShippingOption>,
    #[doc = "An error message, empty on success"]
    pub error_message: String,
}
impl Method for AnswerShippingQuery {
    const TYPE: &'static str = "answerShippingQuery";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets the result of a pre-checkout query; for bots only "]
pub struct AnswerPreCheckoutQuery {
    #[doc = "Identifier of the pre-checkout query "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub pre_checkout_query_id: i64,
    #[doc = "An error message, empty on success"]
    pub error_message: String,
}
impl Method for AnswerPreCheckoutQuery {
    const TYPE: &'static str = "answerPreCheckoutQuery";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Updates the game score of the specified user in the game; for bots only "]
pub struct SetGameScore {
    #[doc = "The chat to which the message with the game belongs "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "True, if the message should be edited "]
    #[serde(default)]
    pub edit_message: bool,
    #[doc = "User identifier "]
    pub user_id: i32,
    #[doc = "The new score"]
    pub score: i32,
    #[doc = "Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table"]
    #[serde(default)]
    pub force: bool,
}
impl Method for SetGameScore {
    const TYPE: &'static str = "setGameScore";
    type Response = Message;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Updates the game score of the specified user in a game; for bots only "]
pub struct SetInlineGameScore {
    #[doc = "Inline message identifier "]
    pub inline_message_id: String,
    #[doc = "True, if the message should be edited "]
    #[serde(default)]
    pub edit_message: bool,
    #[doc = "User identifier "]
    pub user_id: i32,
    #[doc = "The new score"]
    pub score: i32,
    #[doc = "Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table"]
    #[serde(default)]
    pub force: bool,
}
impl Method for SetInlineGameScore {
    const TYPE: &'static str = "setInlineGameScore";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the high scores for a game and some part of the high score table in the range of the specified user; for bots only "]
pub struct GetGameHighScores {
    #[doc = "The chat that contains the message with the game "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "User identifier"]
    pub user_id: i32,
}
impl Method for GetGameHighScores {
    const TYPE: &'static str = "getGameHighScores";
    type Response = GameHighScores;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns game high scores and some part of the high score table in the range of the specified user; for bots only "]
pub struct GetInlineGameHighScores {
    #[doc = "Inline message identifier "]
    pub inline_message_id: String,
    #[doc = "User identifier"]
    pub user_id: i32,
}
impl Method for GetInlineGameHighScores {
    const TYPE: &'static str = "getInlineGameHighScores";
    type Response = GameHighScores;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Deletes the default reply markup from a chat. Must be called after a one-time keyboard or a ForceReply reply markup has been used. UpdateChatReplyMarkup will be sent if the reply markup will be changed "]
pub struct DeleteChatReplyMarkup {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The message identifier of the used keyboard"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
}
impl Method for DeleteChatReplyMarkup {
    const TYPE: &'static str = "deleteChatReplyMarkup";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends a notification about user activity in a chat "]
pub struct SendChatAction {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The action description"]
    pub action: ChatAction,
}
impl Method for SendChatAction {
    const TYPE: &'static str = "sendChatAction";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Informs TDLib that the chat is opened by the user. Many useful activities depend on the chat being opened or closed (e.g., in supergroups and channels all updates are received only for opened chats) "]
pub struct OpenChat {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for OpenChat {
    const TYPE: &'static str = "openChat";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Informs TDLib that the chat is closed by the user. Many useful activities depend on the chat being opened or closed "]
pub struct CloseChat {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for CloseChat {
    const TYPE: &'static str = "closeChat";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Informs TDLib that messages are being viewed by the user. Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels) "]
pub struct ViewMessages {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The identifiers of the messages being viewed"]
    pub message_ids: Vec<i64>,
    #[doc = "True, if messages in closed chats should be marked as read"]
    #[serde(default)]
    pub force_read: bool,
}
impl Method for ViewMessages {
    const TYPE: &'static str = "viewMessages";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Informs TDLib that the message content has been opened (e.g., the user has opened a photo, video, document, location or venue, or has listened to an audio file or voice note message). An updateMessageContentOpened update will be generated if something has changed "]
pub struct OpenMessageContent {
    #[doc = "Chat identifier of the message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the message with the opened content"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
}
impl Method for OpenMessageContent {
    const TYPE: &'static str = "openMessageContent";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Marks all mentions in a chat as read "]
pub struct ReadAllChatMentions {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for ReadAllChatMentions {
    const TYPE: &'static str = "readAllChatMentions";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns an existing chat corresponding to a given user "]
pub struct CreatePrivateChat {
    #[doc = "User identifier "]
    pub user_id: i32,
    #[doc = "If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect"]
    #[serde(default)]
    pub force: bool,
}
impl Method for CreatePrivateChat {
    const TYPE: &'static str = "createPrivateChat";
    type Response = Chat;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns an existing chat corresponding to a known basic group "]
pub struct CreateBasicGroupChat {
    #[doc = "Basic group identifier "]
    pub basic_group_id: i32,
    #[doc = "If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect"]
    #[serde(default)]
    pub force: bool,
}
impl Method for CreateBasicGroupChat {
    const TYPE: &'static str = "createBasicGroupChat";
    type Response = Chat;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns an existing chat corresponding to a known supergroup or channel "]
pub struct CreateSupergroupChat {
    #[doc = "Supergroup or channel identifier "]
    pub supergroup_id: i32,
    #[doc = "If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect"]
    #[serde(default)]
    pub force: bool,
}
impl Method for CreateSupergroupChat {
    const TYPE: &'static str = "createSupergroupChat";
    type Response = Chat;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns an existing chat corresponding to a known secret chat "]
pub struct CreateSecretChat {
    #[doc = "Secret chat identifier"]
    pub secret_chat_id: i32,
}
impl Method for CreateSecretChat {
    const TYPE: &'static str = "createSecretChat";
    type Response = Chat;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Creates a new basic group and sends a corresponding messageBasicGroupChatCreate. Returns the newly created chat "]
pub struct CreateNewBasicGroupChat {
    #[doc = "Identifiers of users to be added to the basic group "]
    pub user_ids: Vec<i32>,
    #[doc = "Title of the new basic group; 1-128 characters"]
    pub title: String,
}
impl Method for CreateNewBasicGroupChat {
    const TYPE: &'static str = "createNewBasicGroupChat";
    type Response = Chat;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat "]
pub struct CreateNewSupergroupChat {
    #[doc = "Title of the new chat; 1-128 characters "]
    pub title: String,
    #[doc = "True, if a channel chat should be created "]
    #[serde(default)]
    pub is_channel: bool,
    #[doc = "Chat description; 0-255 characters "]
    pub description: String,
    #[doc = "Chat location if a location-based supergroup is being created"]
    pub location: ChatLocation,
}
impl Method for CreateNewSupergroupChat {
    const TYPE: &'static str = "createNewSupergroupChat";
    type Response = Chat;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Creates a new secret chat. Returns the newly created chat "]
pub struct CreateNewSecretChat {
    #[doc = "Identifier of the target user"]
    pub user_id: i32,
}
impl Method for CreateNewSecretChat {
    const TYPE: &'static str = "createNewSecretChat";
    type Response = Chat;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Creates a new supergroup from an existing basic group and sends a corresponding messageChatUpgradeTo and messageChatUpgradeFrom; requires creator privileges. Deactivates the original basic group "]
pub struct UpgradeBasicGroupChatToSupergroupChat {
    #[doc = "Identifier of the chat to upgrade"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for UpgradeBasicGroupChatToSupergroupChat {
    const TYPE: &'static str = "upgradeBasicGroupChatToSupergroupChat";
    type Response = Chat;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Moves a chat to a different chat list. Current chat list of the chat must ne non-null "]
pub struct SetChatChatList {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New chat list of the chat. The chat with the current user (Saved Messages) and the chat 777000 (Telegram) can't be moved to the Archive chat list"]
    pub chat_list: ChatList,
}
impl Method for SetChatChatList {
    const TYPE: &'static str = "setChatChatList";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the chat title. Supported only for basic groups, supergroups and channels. Requires can_change_info rights. The title will not be changed until the request to the server has been completed"]
pub struct SetChatTitle {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New title of the chat; 1-128 characters"]
    pub title: String,
}
impl Method for SetChatTitle {
    const TYPE: &'static str = "setChatTitle";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the photo of a chat. Supported only for basic groups, supergroups and channels. Requires can_change_info rights. The photo will not be changed before request to the server has been completed"]
pub struct SetChatPhoto {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New chat photo. You can use a zero InputFileId to delete the chat photo. Files that are accessible only by HTTP URL are not acceptable"]
    pub photo: InputFile,
}
impl Method for SetChatPhoto {
    const TYPE: &'static str = "setChatPhoto";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the chat members permissions. Supported only for basic groups and supergroups. Requires can_restrict_members administrator right"]
pub struct SetChatPermissions {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New non-administrator members permissions in the chat"]
    pub permissions: ChatPermissions,
}
impl Method for SetChatPermissions {
    const TYPE: &'static str = "setChatPermissions";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the draft message in a chat "]
pub struct SetChatDraftMessage {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New draft message; may be null"]
    pub draft_message: Option<DraftMessage>,
}
impl Method for SetChatDraftMessage {
    const TYPE: &'static str = "setChatDraftMessage";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the notification settings of a chat. Notification settings of a chat with the current user (Saved Messages) can't be changed"]
pub struct SetChatNotificationSettings {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New notification settings for the chat. If the chat is muted for more than 1 week, it is considered to be muted forever"]
    pub notification_settings: ChatNotificationSettings,
}
impl Method for SetChatNotificationSettings {
    const TYPE: &'static str = "setChatNotificationSettings";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the pinned state of a chat. You can pin up to GetOption(\"pinned_chat_count_max\")/GetOption(\"pinned_archived_chat_count_max\") non-secret chats and the same number of secret chats in the main/archive chat list "]
pub struct ToggleChatIsPinned {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New value of is_pinned"]
    #[serde(default)]
    pub is_pinned: bool,
}
impl Method for ToggleChatIsPinned {
    const TYPE: &'static str = "toggleChatIsPinned";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the marked as unread state of a chat "]
pub struct ToggleChatIsMarkedAsUnread {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New value of is_marked_as_unread"]
    #[serde(default)]
    pub is_marked_as_unread: bool,
}
impl Method for ToggleChatIsMarkedAsUnread {
    const TYPE: &'static str = "toggleChatIsMarkedAsUnread";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the value of the default disable_notification parameter, used when a message is sent to a chat "]
pub struct ToggleChatDefaultDisableNotification {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New value of default_disable_notification"]
    #[serde(default)]
    pub default_disable_notification: bool,
}
impl Method for ToggleChatDefaultDisableNotification {
    const TYPE: &'static str = "toggleChatDefaultDisableNotification";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes client data associated with a chat "]
pub struct SetChatClientData {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New value of client_data"]
    pub client_data: String,
}
impl Method for SetChatClientData {
    const TYPE: &'static str = "setChatClientData";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes information about a chat. Available for basic groups, supergroups, and channels. Requires can_change_info rights "]
pub struct SetChatDescription {
    #[doc = "Identifier of the chat "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New chat description; 0-255 characters"]
    pub description: String,
}
impl Method for SetChatDescription {
    const TYPE: &'static str = "setChatDescription";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the discussion group of a channel chat; requires can_change_info rights in the channel if it is specified "]
pub struct SetChatDiscussionGroup {
    #[doc = "Identifier of the channel chat. Pass 0 to remove a link from the supergroup passed in the second argument to a linked channel chat (requires can_pin_messages rights in the supergroup) "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of a new channel's discussion group. Use 0 to remove the discussion group.\n Use the method getSuitableDiscussionChats to find all suitable groups. Basic group chats needs to be first upgraded to supergroup chats. If new chat members don't have access to old messages in the supergroup, then toggleSupergroupIsAllHistoryAvailable needs to be used first to change that"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub discussion_chat_id: i64,
}
impl Method for SetChatDiscussionGroup {
    const TYPE: &'static str = "setChatDiscussionGroup";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the location of a chat. Available only for some location-based supergroups, use supergroupFullInfo.can_set_location to check whether the method is allowed to use "]
pub struct SetChatLocation {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New location for the chat; must be valid and not null"]
    pub location: ChatLocation,
}
impl Method for SetChatLocation {
    const TYPE: &'static str = "setChatLocation";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the slow mode delay of a chat. Available only for supergroups; requires can_restrict_members rights "]
pub struct SetChatSlowModeDelay {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "New slow mode delay for the chat; must be one of 0, 10, 30, 60, 300, 900, 3600"]
    pub slow_mode_delay: i32,
}
impl Method for SetChatSlowModeDelay {
    const TYPE: &'static str = "setChatSlowModeDelay";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Pins a message in a chat; requires can_pin_messages rights "]
pub struct PinChatMessage {
    #[doc = "Identifier of the chat "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the new pinned message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "True, if there should be no notification about the pinned message"]
    #[serde(default)]
    pub disable_notification: bool,
}
impl Method for PinChatMessage {
    const TYPE: &'static str = "pinChatMessage";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes the pinned message from a chat; requires can_pin_messages rights in the group or channel "]
pub struct UnpinChatMessage {
    #[doc = "Identifier of the chat"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for UnpinChatMessage {
    const TYPE: &'static str = "unpinChatMessage";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds current user as a new member to a chat. Private and secret chats can't be joined using this method "]
pub struct JoinChat {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for JoinChat {
    const TYPE: &'static str = "joinChat";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes current user from chat members. Private and secret chats can't be left using this method "]
pub struct LeaveChat {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for LeaveChat {
    const TYPE: &'static str = "leaveChat";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds a new member to a chat. Members can't be added to private or secret chats. Members will not be added until the chat state has been synchronized with the server"]
pub struct AddChatMember {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the user "]
    pub user_id: i32,
    #[doc = "The number of earlier messages from the chat to be forwarded to the new member; up to 100. Ignored for supergroups and channels"]
    pub forward_limit: i32,
}
impl Method for AddChatMember {
    const TYPE: &'static str = "addChatMember";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds multiple new members to a chat. Currently this option is only available for supergroups and channels. This option can't be used to join a chat. Members can't be added to a channel if it has more than 200 members. Members will not be added until the chat state has been synchronized with the server"]
pub struct AddChatMembers {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifiers of the users to be added to the chat"]
    pub user_ids: Vec<i32>,
}
impl Method for AddChatMembers {
    const TYPE: &'static str = "addChatMembers";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the status of a chat member, needs appropriate privileges. This function is currently not suitable for adding new members to the chat and transferring chat ownership; instead, use addChatMember or transferChatOwnership. The chat member status will not be changed until it has been synchronized with the server"]
pub struct SetChatMemberStatus {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "User identifier "]
    pub user_id: i32,
    #[doc = "The new status of the member in the chat"]
    pub status: ChatMemberStatus,
}
impl Method for SetChatMemberStatus {
    const TYPE: &'static str = "setChatMemberStatus";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks whether the current session can be used to transfer a chat ownership to another user"]
pub struct CanTransferOwnership {}
impl Method for CanTransferOwnership {
    const TYPE: &'static str = "canTransferOwnership";
    type Response = CanTransferOwnershipResult;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the owner of a chat. The current user must be a current owner of the chat. Use the method canTransferOwnership to check whether the ownership can be transferred from the current session. Available only for supergroups and channel chats"]
pub struct TransferChatOwnership {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Identifier of the user to which transfer the ownership. The ownership can't be transferred to a bot or to a deleted user "]
    pub user_id: i32,
    #[doc = "The password of the current user"]
    pub password: String,
}
impl Method for TransferChatOwnership {
    const TYPE: &'static str = "transferChatOwnership";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a single member of a chat "]
pub struct GetChatMember {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "User identifier"]
    pub user_id: i32,
}
impl Method for GetChatMember {
    const TYPE: &'static str = "getChatMember";
    type Response = ChatMember;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for a specified query in the first name, last name and username of the members of a specified chat. Requires administrator rights in channels "]
pub struct SearchChatMembers {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Query to search for "]
    pub query: String,
    #[doc = "The maximum number of users to be returned "]
    pub limit: i32,
    #[doc = "The type of users to return. By default, chatMembersFilterMembers"]
    pub filter: ChatMembersFilter,
}
impl Method for SearchChatMembers {
    const TYPE: &'static str = "searchChatMembers";
    type Response = ChatMembers;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of administrators of the chat with their custom titles "]
pub struct GetChatAdministrators {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for GetChatAdministrators {
    const TYPE: &'static str = "getChatAdministrators";
    type Response = ChatAdministrators;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Clears draft messages in all chats "]
pub struct ClearAllDraftMessages {
    #[doc = "If true, local draft messages in secret chats will not be cleared"]
    #[serde(default)]
    pub exclude_secret_chats: bool,
}
impl Method for ClearAllDraftMessages {
    const TYPE: &'static str = "clearAllDraftMessages";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns list of chats with non-default notification settings "]
pub struct GetChatNotificationSettingsExceptions {
    #[doc = "If specified, only chats from the specified scope will be returned "]
    pub scope: NotificationSettingsScope,
    #[doc = "If true, also chats with non-default sound will be returned"]
    #[serde(default)]
    pub compare_sound: bool,
}
impl Method for GetChatNotificationSettingsExceptions {
    const TYPE: &'static str = "getChatNotificationSettingsExceptions";
    type Response = Chats;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the notification settings for chats of a given type "]
pub struct GetScopeNotificationSettings {
    #[doc = "Types of chats for which to return the notification settings information"]
    pub scope: NotificationSettingsScope,
}
impl Method for GetScopeNotificationSettings {
    const TYPE: &'static str = "getScopeNotificationSettings";
    type Response = ScopeNotificationSettings;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes notification settings for chats of a given type "]
pub struct SetScopeNotificationSettings {
    #[doc = "Types of chats for which to change the notification settings "]
    pub scope: NotificationSettingsScope,
    #[doc = "The new notification settings for the given scope"]
    pub notification_settings: ScopeNotificationSettings,
}
impl Method for SetScopeNotificationSettings {
    const TYPE: &'static str = "setScopeNotificationSettings";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Resets all notification settings to their default values. By default, all chats are unmuted, the sound is set to \"default\" and message previews are shown"]
pub struct ResetAllNotificationSettings {}
impl Method for ResetAllNotificationSettings {
    const TYPE: &'static str = "resetAllNotificationSettings";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the order of pinned chats "]
pub struct SetPinnedChats {
    #[doc = "Chat list in which to change the order of pinned chats "]
    pub chat_list: ChatList,
    #[doc = "The new list of pinned chats"]
    pub chat_ids: Vec<i64>,
}
impl Method for SetPinnedChats {
    const TYPE: &'static str = "setPinnedChats";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Downloads a file from the cloud. Download progress and completion of the download will be notified through updateFile updates"]
pub struct DownloadFile {
    #[doc = "Identifier of the file to download"]
    pub file_id: i32,
    #[doc = "Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which downloadFile was called will be downloaded first"]
    pub priority: i32,
    #[doc = "The starting position from which the file should be downloaded"]
    pub offset: i32,
    #[doc = "Number of bytes which should be downloaded starting from the \"offset\" position before the download will be automatically cancelled; use 0 to download without a limit"]
    pub limit: i32,
    #[doc = "If false, this request returns file state just after the download has been started. If true, this request returns file state only after\n the download has succeeded, has failed, has been cancelled or a new downloadFile request with different offset/limit parameters was sent"]
    #[serde(default)]
    pub synchronous: bool,
}
impl Method for DownloadFile {
    const TYPE: &'static str = "downloadFile";
    type Response = File;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns file downloaded prefix size from a given offset "]
pub struct GetFileDownloadedPrefixSize {
    #[doc = "Identifier of the file "]
    pub file_id: i32,
    #[doc = "Offset from which downloaded prefix size should be calculated"]
    pub offset: i32,
}
impl Method for GetFileDownloadedPrefixSize {
    const TYPE: &'static str = "getFileDownloadedPrefixSize";
    type Response = Count;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Stops the downloading of a file. If a file has already been downloaded, does nothing "]
pub struct CancelDownloadFile {
    #[doc = "Identifier of a file to stop downloading "]
    pub file_id: i32,
    #[doc = "Pass true to stop downloading only if it hasn't been started, i.e. request hasn't been sent to server"]
    #[serde(default)]
    pub only_if_pending: bool,
}
impl Method for CancelDownloadFile {
    const TYPE: &'static str = "cancelDownloadFile";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Asynchronously uploads a file to the cloud without sending it in a message. updateFile will be used to notify about upload progress and successful completion of the upload. The file will not have a persistent remote identifier until it will be sent in a message "]
pub struct UploadFile {
    #[doc = "File to upload "]
    pub file: InputFile,
    #[doc = "File type"]
    pub file_type: FileType,
    #[doc = "Priority of the upload (1-32). The higher the priority, the earlier the file will be uploaded. If the priorities of two files are equal, then the first one for which uploadFile was called will be uploaded first"]
    pub priority: i32,
}
impl Method for UploadFile {
    const TYPE: &'static str = "uploadFile";
    type Response = File;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Stops the uploading of a file. Supported only for files uploaded by using uploadFile. For other files the behavior is undefined "]
pub struct CancelUploadFile {
    #[doc = "Identifier of the file to stop uploading"]
    pub file_id: i32,
}
impl Method for CancelUploadFile {
    const TYPE: &'static str = "cancelUploadFile";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Writes a part of a generated file. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct write to the destination file"]
pub struct WriteGeneratedFilePart {
    #[doc = "The identifier of the generation process "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub generation_id: i64,
    #[doc = "The offset from which to write the data to the file "]
    pub offset: i32,
    #[doc = "The data to write"]
    pub data: String,
}
impl Method for WriteGeneratedFilePart {
    const TYPE: &'static str = "writeGeneratedFilePart";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Informs TDLib on a file generation progress"]
pub struct SetFileGenerationProgress {
    #[doc = "The identifier of the generation process"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub generation_id: i64,
    #[doc = "Expected size of the generated file, in bytes; 0 if unknown"]
    pub expected_size: i32,
    #[doc = "The number of bytes already generated"]
    pub local_prefix_size: i32,
}
impl Method for SetFileGenerationProgress {
    const TYPE: &'static str = "setFileGenerationProgress";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Finishes the file generation"]
pub struct FinishFileGeneration {
    #[doc = "The identifier of the generation process"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub generation_id: i64,
    #[doc = "If set, means that file generation has failed and should be terminated"]
    pub error: Error,
}
impl Method for FinishFileGeneration {
    const TYPE: &'static str = "finishFileGeneration";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Reads a part of a file from the TDLib file cache and returns read bytes. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct read from the file"]
pub struct ReadFilePart {
    #[doc = "Identifier of the file. The file must be located in the TDLib file cache"]
    pub file_id: i32,
    #[doc = "The offset from which to read the file"]
    pub offset: i32,
    #[doc = "Number of bytes to read. An error will be returned if there are not enough bytes available in the file from the specified position. Pass 0 to read all available data from the specified position"]
    pub count: i32,
}
impl Method for ReadFilePart {
    const TYPE: &'static str = "readFilePart";
    type Response = FilePart;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Deletes a file from the TDLib file cache "]
pub struct DeleteFile {
    #[doc = "Identifier of the file to delete"]
    pub file_id: i32,
}
impl Method for DeleteFile {
    const TYPE: &'static str = "deleteFile";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Generates a new invite link for a chat; the previously generated link is revoked. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right "]
pub struct GenerateChatInviteLink {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for GenerateChatInviteLink {
    const TYPE: &'static str = "generateChatInviteLink";
    type Response = ChatInviteLink;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks the validity of an invite link for a chat and returns information about the corresponding chat "]
pub struct CheckChatInviteLink {
    #[doc = "Invite link to be checked; should begin with \"https://t.me/joinchat/\", \"https://telegram.me/joinchat/\", or \"https://telegram.dog/joinchat/\""]
    pub invite_link: String,
}
impl Method for CheckChatInviteLink {
    const TYPE: &'static str = "checkChatInviteLink";
    type Response = ChatInviteLinkInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Uses an invite link to add the current user to the chat if possible. The new member will not be added until the chat state has been synchronized with the server"]
pub struct JoinChatByInviteLink {
    #[doc = "Invite link to import; should begin with \"https://t.me/joinchat/\", \"https://telegram.me/joinchat/\", or \"https://telegram.dog/joinchat/\""]
    pub invite_link: String,
}
impl Method for JoinChatByInviteLink {
    const TYPE: &'static str = "joinChatByInviteLink";
    type Response = Chat;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Creates a new call "]
pub struct CreateCall {
    #[doc = "Identifier of the user to be called "]
    pub user_id: i32,
    #[doc = "Description of the call protocols supported by the client"]
    pub protocol: CallProtocol,
}
impl Method for CreateCall {
    const TYPE: &'static str = "createCall";
    type Response = CallId;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Accepts an incoming call "]
pub struct AcceptCall {
    #[doc = "Call identifier "]
    pub call_id: i32,
    #[doc = "Description of the call protocols supported by the client"]
    pub protocol: CallProtocol,
}
impl Method for AcceptCall {
    const TYPE: &'static str = "acceptCall";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Discards a call "]
pub struct DiscardCall {
    #[doc = "Call identifier "]
    pub call_id: i32,
    #[doc = "True, if the user was disconnected "]
    #[serde(default)]
    pub is_disconnected: bool,
    #[doc = "The call duration, in seconds "]
    pub duration: i32,
    #[doc = "Identifier of the connection used during the call"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub connection_id: i64,
}
impl Method for DiscardCall {
    const TYPE: &'static str = "discardCall";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends a call rating "]
pub struct SendCallRating {
    #[doc = "Call identifier "]
    pub call_id: i32,
    #[doc = "Call rating; 1-5 "]
    pub rating: i32,
    #[doc = "An optional user comment if the rating is less than 5 "]
    pub comment: String,
    #[doc = "List of the exact types of problems with the call, specified by the user"]
    pub problems: Vec<CallProblem>,
}
impl Method for SendCallRating {
    const TYPE: &'static str = "sendCallRating";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends debug information for a call "]
pub struct SendCallDebugInformation {
    #[doc = "Call identifier "]
    pub call_id: i32,
    #[doc = "Debug information in application-specific format"]
    pub debug_information: String,
}
impl Method for SendCallDebugInformation {
    const TYPE: &'static str = "sendCallDebugInformation";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds a user to the blacklist "]
pub struct BlockUser {
    #[doc = "User identifier"]
    pub user_id: i32,
}
impl Method for BlockUser {
    const TYPE: &'static str = "blockUser";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes a user from the blacklist "]
pub struct UnblockUser {
    #[doc = "User identifier"]
    pub user_id: i32,
}
impl Method for UnblockUser {
    const TYPE: &'static str = "unblockUser";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns users that were blocked by the current user "]
pub struct GetBlockedUsers {
    #[doc = "Number of users to skip in the result; must be non-negative "]
    pub offset: i32,
    #[doc = "The maximum number of users to return; up to 100"]
    pub limit: i32,
}
impl Method for GetBlockedUsers {
    const TYPE: &'static str = "getBlockedUsers";
    type Response = Users;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds a user to the contact list or edits an existing contact by their user identifier "]
pub struct AddContact {
    #[doc = "The contact to add or edit; phone number can be empty and needs to be specified only if known, vCard is ignored"]
    pub contact: Contact,
    #[doc = "True, if the new contact needs to be allowed to see current user's phone number. A corresponding rule to userPrivacySettingShowPhoneNumber will be added if needed. Use the field UserFullInfo.need_phone_number_privacy_exception to check whether the current user needs to be asked to share their phone number"]
    #[serde(default)]
    pub share_phone_number: bool,
}
impl Method for AddContact {
    const TYPE: &'static str = "addContact";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds new contacts or edits existing contacts by their phone numbers; contacts' user identifiers are ignored "]
pub struct ImportContacts {
    #[doc = "The list of contacts to import or edit; contacts' vCard are ignored and are not imported"]
    pub contacts: Vec<Contact>,
}
impl Method for ImportContacts {
    const TYPE: &'static str = "importContacts";
    type Response = ImportedContacts;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns all user contacts"]
pub struct GetContacts {}
impl Method for GetContacts {
    const TYPE: &'static str = "getContacts";
    type Response = Users;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for the specified query in the first names, last names and usernames of the known user contacts "]
pub struct SearchContacts {
    #[doc = "Query to search for; may be empty to return all contacts "]
    pub query: String,
    #[doc = "The maximum number of users to be returned"]
    pub limit: i32,
}
impl Method for SearchContacts {
    const TYPE: &'static str = "searchContacts";
    type Response = Users;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes users from the contact list "]
pub struct RemoveContacts {
    #[doc = "Identifiers of users to be deleted"]
    pub user_ids: Vec<i32>,
}
impl Method for RemoveContacts {
    const TYPE: &'static str = "removeContacts";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the total number of imported contacts"]
pub struct GetImportedContactCount {}
impl Method for GetImportedContactCount {
    const TYPE: &'static str = "getImportedContactCount";
    type Response = Count;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes imported contacts using the list of current user contacts saved on the device. Imports newly added contacts and, if at least the file database is enabled, deletes recently deleted contacts.\n Query result depends on the result of the previous query, so only one query is possible at the same time "]
pub struct ChangeImportedContacts {
    #[doc = "The new list of contacts, contact's vCard are ignored and are not imported"]
    pub contacts: Vec<Contact>,
}
impl Method for ChangeImportedContacts {
    const TYPE: &'static str = "changeImportedContacts";
    type Response = ImportedContacts;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Clears all imported contacts, contact list remains unchanged"]
pub struct ClearImportedContacts {}
impl Method for ClearImportedContacts {
    const TYPE: &'static str = "clearImportedContacts";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Shares the phone number of the current user with a mutual contact. Supposed to be called when the user clicks on chatActionBarSharePhoneNumber "]
pub struct SharePhoneNumber {
    #[doc = "Identifier of the user with whom to share the phone number. The user must be a mutual contact"]
    pub user_id: i32,
}
impl Method for SharePhoneNumber {
    const TYPE: &'static str = "sharePhoneNumber";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the profile photos of a user. The result of this query may be outdated: some photos might have been deleted already "]
pub struct GetUserProfilePhotos {
    #[doc = "User identifier "]
    pub user_id: i32,
    #[doc = "The number of photos to skip; must be non-negative "]
    pub offset: i32,
    #[doc = "The maximum number of photos to be returned; up to 100"]
    pub limit: i32,
}
impl Method for GetUserProfilePhotos {
    const TYPE: &'static str = "getUserProfilePhotos";
    type Response = UserProfilePhotos;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns stickers from the installed sticker sets that correspond to a given emoji. If the emoji is not empty, favorite and recently used stickers may also be returned "]
pub struct GetStickers {
    #[doc = "String representation of emoji. If empty, returns all known installed stickers "]
    pub emoji: String,
    #[doc = "The maximum number of stickers to be returned"]
    pub limit: i32,
}
impl Method for GetStickers {
    const TYPE: &'static str = "getStickers";
    type Response = Stickers;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for stickers from public sticker sets that correspond to a given emoji "]
pub struct SearchStickers {
    #[doc = "String representation of emoji; must be non-empty "]
    pub emoji: String,
    #[doc = "The maximum number of stickers to be returned"]
    pub limit: i32,
}
impl Method for SearchStickers {
    const TYPE: &'static str = "searchStickers";
    type Response = Stickers;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of installed sticker sets "]
pub struct GetInstalledStickerSets {
    #[doc = "Pass true to return mask sticker sets; pass false to return ordinary sticker sets"]
    #[serde(default)]
    pub is_masks: bool,
}
impl Method for GetInstalledStickerSets {
    const TYPE: &'static str = "getInstalledStickerSets";
    type Response = StickerSets;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of archived sticker sets "]
pub struct GetArchivedStickerSets {
    #[doc = "Pass true to return mask stickers sets; pass false to return ordinary sticker sets "]
    #[serde(default)]
    pub is_masks: bool,
    #[doc = "Identifier of the sticker set from which to return the result "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub offset_sticker_set_id: i64,
    #[doc = "The maximum number of sticker sets to return"]
    pub limit: i32,
}
impl Method for GetArchivedStickerSets {
    const TYPE: &'static str = "getArchivedStickerSets";
    type Response = StickerSets;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of trending sticker sets. For the optimal performance the number of returned sticker sets is chosen by the library"]
pub struct GetTrendingStickerSets {
    #[doc = "The offset from which to return the sticker sets; must be non-negative"]
    pub offset: i32,
    #[doc = "The maximum number of sticker sets to be returned; must be non-negative. Fewer sticker sets may be returned than specified by the limit, even if the end of the list has not been reached"]
    pub limit: i32,
}
impl Method for GetTrendingStickerSets {
    const TYPE: &'static str = "getTrendingStickerSets";
    type Response = StickerSets;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of sticker sets attached to a file. Currently only photos and videos can have attached sticker sets "]
pub struct GetAttachedStickerSets {
    #[doc = "File identifier"]
    pub file_id: i32,
}
impl Method for GetAttachedStickerSets {
    const TYPE: &'static str = "getAttachedStickerSets";
    type Response = StickerSets;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a sticker set by its identifier "]
pub struct GetStickerSet {
    #[doc = "Identifier of the sticker set"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub set_id: i64,
}
impl Method for GetStickerSet {
    const TYPE: &'static str = "getStickerSet";
    type Response = StickerSet;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for a sticker set by its name "]
pub struct SearchStickerSet {
    #[doc = "Name of the sticker set"]
    pub name: String,
}
impl Method for SearchStickerSet {
    const TYPE: &'static str = "searchStickerSet";
    type Response = StickerSet;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for installed sticker sets by looking for specified query in their title and name "]
pub struct SearchInstalledStickerSets {
    #[doc = "Pass true to return mask sticker sets; pass false to return ordinary sticker sets "]
    #[serde(default)]
    pub is_masks: bool,
    #[doc = "Query to search for "]
    pub query: String,
    #[doc = "The maximum number of sticker sets to return"]
    pub limit: i32,
}
impl Method for SearchInstalledStickerSets {
    const TYPE: &'static str = "searchInstalledStickerSets";
    type Response = StickerSets;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for ordinary sticker sets by looking for specified query in their title and name. Excludes installed sticker sets from the results "]
pub struct SearchStickerSets {
    #[doc = "Query to search for"]
    pub query: String,
}
impl Method for SearchStickerSets {
    const TYPE: &'static str = "searchStickerSets";
    type Response = StickerSets;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Installs/uninstalls or activates/archives a sticker set "]
pub struct ChangeStickerSet {
    #[doc = "Identifier of the sticker set "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub set_id: i64,
    #[doc = "The new value of is_installed "]
    #[serde(default)]
    pub is_installed: bool,
    #[doc = "The new value of is_archived. A sticker set can't be installed and archived simultaneously"]
    #[serde(default)]
    pub is_archived: bool,
}
impl Method for ChangeStickerSet {
    const TYPE: &'static str = "changeStickerSet";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Informs the server that some trending sticker sets have been viewed by the user "]
pub struct ViewTrendingStickerSets {
    #[doc = "Identifiers of viewed trending sticker sets"]
    pub sticker_set_ids: Vec<i64>,
}
impl Method for ViewTrendingStickerSets {
    const TYPE: &'static str = "viewTrendingStickerSets";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the order of installed sticker sets "]
pub struct ReorderInstalledStickerSets {
    #[doc = "Pass true to change the order of mask sticker sets; pass false to change the order of ordinary sticker sets "]
    #[serde(default)]
    pub is_masks: bool,
    #[doc = "Identifiers of installed sticker sets in the new correct order"]
    pub sticker_set_ids: Vec<i64>,
}
impl Method for ReorderInstalledStickerSets {
    const TYPE: &'static str = "reorderInstalledStickerSets";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of recently used stickers "]
pub struct GetRecentStickers {
    #[doc = "Pass true to return stickers and masks that were recently attached to photos or video files; pass false to return recently sent stickers"]
    #[serde(default)]
    pub is_attached: bool,
}
impl Method for GetRecentStickers {
    const TYPE: &'static str = "getRecentStickers";
    type Response = Stickers;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Manually adds a new sticker to the list of recently used stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list"]
pub struct AddRecentSticker {
    #[doc = "Pass true to add the sticker to the list of stickers recently attached to photo or video files; pass false to add the sticker to the list of recently sent stickers "]
    #[serde(default)]
    pub is_attached: bool,
    #[doc = "Sticker file to add"]
    pub sticker: InputFile,
}
impl Method for AddRecentSticker {
    const TYPE: &'static str = "addRecentSticker";
    type Response = Stickers;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes a sticker from the list of recently used stickers "]
pub struct RemoveRecentSticker {
    #[doc = "Pass true to remove the sticker from the list of stickers recently attached to photo or video files; pass false to remove the sticker from the list of recently sent stickers "]
    #[serde(default)]
    pub is_attached: bool,
    #[doc = "Sticker file to delete"]
    pub sticker: InputFile,
}
impl Method for RemoveRecentSticker {
    const TYPE: &'static str = "removeRecentSticker";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Clears the list of recently used stickers "]
pub struct ClearRecentStickers {
    #[doc = "Pass true to clear the list of stickers recently attached to photo or video files; pass false to clear the list of recently sent stickers"]
    #[serde(default)]
    pub is_attached: bool,
}
impl Method for ClearRecentStickers {
    const TYPE: &'static str = "clearRecentStickers";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns favorite stickers"]
pub struct GetFavoriteStickers {}
impl Method for GetFavoriteStickers {
    const TYPE: &'static str = "getFavoriteStickers";
    type Response = Stickers;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds a new sticker to the list of favorite stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list"]
pub struct AddFavoriteSticker {
    #[doc = "Sticker file to add"]
    pub sticker: InputFile,
}
impl Method for AddFavoriteSticker {
    const TYPE: &'static str = "addFavoriteSticker";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes a sticker from the list of favorite stickers "]
pub struct RemoveFavoriteSticker {
    #[doc = "Sticker file to delete from the list"]
    pub sticker: InputFile,
}
impl Method for RemoveFavoriteSticker {
    const TYPE: &'static str = "removeFavoriteSticker";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns emoji corresponding to a sticker. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object "]
pub struct GetStickerEmojis {
    #[doc = "Sticker file identifier"]
    pub sticker: InputFile,
}
impl Method for GetStickerEmojis {
    const TYPE: &'static str = "getStickerEmojis";
    type Response = Emojis;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for emojis by keywords. Supported only if the file database is enabled "]
pub struct SearchEmojis {
    #[doc = "Text to search for "]
    pub text: String,
    #[doc = "True, if only emojis, which exactly match text needs to be returned "]
    #[serde(default)]
    pub exact_match: bool,
    #[doc = "List of possible IETF language tags of the user's input language; may be empty if unknown"]
    pub input_language_codes: Vec<String>,
}
impl Method for SearchEmojis {
    const TYPE: &'static str = "searchEmojis";
    type Response = Emojis;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns an HTTP URL which can be used to automatically log in to the translation platform and suggest new emoji replacements. The URL will be valid for 30 seconds after generation "]
pub struct GetEmojiSuggestionsUrl {
    #[doc = "Language code for which the emoji replacements will be suggested"]
    pub language_code: String,
}
impl Method for GetEmojiSuggestionsUrl {
    const TYPE: &'static str = "getEmojiSuggestionsUrl";
    type Response = HttpUrl;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns saved animations"]
pub struct GetSavedAnimations {}
impl Method for GetSavedAnimations {
    const TYPE: &'static str = "getSavedAnimations";
    type Response = Animations;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Manually adds a new animation to the list of saved animations. The new animation is added to the beginning of the list. If the animation was already in the list, it is removed first. Only non-secret video animations with MIME type \"video/mp4\" can be added to the list"]
pub struct AddSavedAnimation {
    #[doc = "The animation file to be added. Only animations known to the server (i.e. successfully sent via a message) can be added to the list"]
    pub animation: InputFile,
}
impl Method for AddSavedAnimation {
    const TYPE: &'static str = "addSavedAnimation";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes an animation from the list of saved animations "]
pub struct RemoveSavedAnimation {
    #[doc = "Animation file to be removed"]
    pub animation: InputFile,
}
impl Method for RemoveSavedAnimation {
    const TYPE: &'static str = "removeSavedAnimation";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns up to 20 recently used inline bots in the order of their last usage"]
pub struct GetRecentInlineBots {}
impl Method for GetRecentInlineBots {
    const TYPE: &'static str = "getRecentInlineBots";
    type Response = Users;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for recently used hashtags by their prefix "]
pub struct SearchHashtags {
    #[doc = "Hashtag prefix to search for "]
    pub prefix: String,
    #[doc = "The maximum number of hashtags to be returned"]
    pub limit: i32,
}
impl Method for SearchHashtags {
    const TYPE: &'static str = "searchHashtags";
    type Response = Hashtags;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes a hashtag from the list of recently used hashtags "]
pub struct RemoveRecentHashtag {
    #[doc = "Hashtag to delete"]
    pub hashtag: String,
}
impl Method for RemoveRecentHashtag {
    const TYPE: &'static str = "removeRecentHashtag";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a web page preview by the text of the message. Do not call this function too often. Returns a 404 error if the web page has no preview "]
pub struct GetWebPagePreview {
    #[doc = "Message text with formatting"]
    pub text: FormattedText,
}
impl Method for GetWebPagePreview {
    const TYPE: &'static str = "getWebPagePreview";
    type Response = WebPage;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns an instant view version of a web page if available. Returns a 404 error if the web page has no instant view page "]
pub struct GetWebPageInstantView {
    #[doc = "The web page URL "]
    pub url: String,
    #[doc = "If true, the full instant view for the web page will be returned"]
    #[serde(default)]
    pub force_full: bool,
}
impl Method for GetWebPageInstantView {
    const TYPE: &'static str = "getWebPageInstantView";
    type Response = WebPageInstantView;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Uploads a new profile photo for the current user. If something changes, updateUser will be sent "]
pub struct SetProfilePhoto {
    #[doc = "Profile photo to set. inputFileId and inputFileRemote may still be unsupported"]
    pub photo: InputFile,
}
impl Method for SetProfilePhoto {
    const TYPE: &'static str = "setProfilePhoto";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Deletes a profile photo. If something changes, updateUser will be sent "]
pub struct DeleteProfilePhoto {
    #[doc = "Identifier of the profile photo to delete"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub profile_photo_id: i64,
}
impl Method for DeleteProfilePhoto {
    const TYPE: &'static str = "deleteProfilePhoto";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the first and last name of the current user. If something changes, updateUser will be sent "]
pub struct SetName {
    #[doc = "The new value of the first name for the user; 1-64 characters "]
    pub first_name: String,
    #[doc = "The new value of the optional last name for the user; 0-64 characters"]
    pub last_name: String,
}
impl Method for SetName {
    const TYPE: &'static str = "setName";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the bio of the current user "]
pub struct SetBio {
    #[doc = "The new value of the user bio; 0-70 characters without line feeds"]
    pub bio: String,
}
impl Method for SetBio {
    const TYPE: &'static str = "setBio";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the username of the current user. If something changes, updateUser will be sent "]
pub struct SetUsername {
    #[doc = "The new value of the username. Use an empty string to remove the username"]
    pub username: String,
}
impl Method for SetUsername {
    const TYPE: &'static str = "setUsername";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the location of the current user. Needs to be called if GetOption(\"is_location_visible\") is true and location changes for more than 1 kilometer "]
pub struct SetLocation {
    #[doc = "The new location of the user"]
    pub location: Location,
}
impl Method for SetLocation {
    const TYPE: &'static str = "setLocation";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the phone number of the user and sends an authentication code to the user's new phone number. On success, returns information about the sent code"]
pub struct ChangePhoneNumber {
    #[doc = "The new phone number of the user in international format "]
    pub phone_number: String,
    #[doc = "Settings for the authentication of the user's phone number"]
    pub settings: PhoneNumberAuthenticationSettings,
}
impl Method for ChangePhoneNumber {
    const TYPE: &'static str = "changePhoneNumber";
    type Response = AuthenticationCodeInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Re-sends the authentication code sent to confirm a new phone number for the user. Works only if the previously received authenticationCodeInfo next_code_type was not null"]
pub struct ResendChangePhoneNumberCode {}
impl Method for ResendChangePhoneNumberCode {
    const TYPE: &'static str = "resendChangePhoneNumberCode";
    type Response = AuthenticationCodeInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks the authentication code sent to confirm a new phone number of the user "]
pub struct CheckChangePhoneNumberCode {
    #[doc = "Verification code received by SMS, phone call or flash call"]
    pub code: String,
}
impl Method for CheckChangePhoneNumberCode {
    const TYPE: &'static str = "checkChangePhoneNumberCode";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets the list of commands supported by the bot; for bots only "]
pub struct SetCommands {
    #[doc = "List of the bot's commands"]
    pub commands: Vec<BotCommand>,
}
impl Method for SetCommands {
    const TYPE: &'static str = "setCommands";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns all active sessions of the current user"]
pub struct GetActiveSessions {}
impl Method for GetActiveSessions {
    const TYPE: &'static str = "getActiveSessions";
    type Response = Sessions;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Terminates a session of the current user "]
pub struct TerminateSession {
    #[doc = "Session identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub session_id: i64,
}
impl Method for TerminateSession {
    const TYPE: &'static str = "terminateSession";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Terminates all other sessions of the current user"]
pub struct TerminateAllOtherSessions {}
impl Method for TerminateAllOtherSessions {
    const TYPE: &'static str = "terminateAllOtherSessions";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns all website where the current user used Telegram to log in"]
pub struct GetConnectedWebsites {}
impl Method for GetConnectedWebsites {
    const TYPE: &'static str = "getConnectedWebsites";
    type Response = ConnectedWebsites;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Disconnects website from the current user's Telegram account "]
pub struct DisconnectWebsite {
    #[doc = "Website identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub website_id: i64,
}
impl Method for DisconnectWebsite {
    const TYPE: &'static str = "disconnectWebsite";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Disconnects all websites from the current user's Telegram account"]
pub struct DisconnectAllWebsites {}
impl Method for DisconnectAllWebsites {
    const TYPE: &'static str = "disconnectAllWebsites";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the username of a supergroup or channel, requires owner privileges in the supergroup or channel "]
pub struct SetSupergroupUsername {
    #[doc = "Identifier of the supergroup or channel "]
    pub supergroup_id: i32,
    #[doc = "New value of the username. Use an empty string to remove the username"]
    pub username: String,
}
impl Method for SetSupergroupUsername {
    const TYPE: &'static str = "setSupergroupUsername";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the sticker set of a supergroup; requires can_change_info rights "]
pub struct SetSupergroupStickerSet {
    #[doc = "Identifier of the supergroup "]
    pub supergroup_id: i32,
    #[doc = "New value of the supergroup sticker set identifier. Use 0 to remove the supergroup sticker set"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub sticker_set_id: i64,
}
impl Method for SetSupergroupStickerSet {
    const TYPE: &'static str = "setSupergroupStickerSet";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Toggles sender signatures messages sent in a channel; requires can_change_info rights "]
pub struct ToggleSupergroupSignMessages {
    #[doc = "Identifier of the channel "]
    pub supergroup_id: i32,
    #[doc = "New value of sign_messages"]
    #[serde(default)]
    pub sign_messages: bool,
}
impl Method for ToggleSupergroupSignMessages {
    const TYPE: &'static str = "toggleSupergroupSignMessages";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Toggles whether the message history of a supergroup is available to new members; requires can_change_info rights "]
pub struct ToggleSupergroupIsAllHistoryAvailable {
    #[doc = "The identifier of the supergroup "]
    pub supergroup_id: i32,
    #[doc = "The new value of is_all_history_available"]
    #[serde(default)]
    pub is_all_history_available: bool,
}
impl Method for ToggleSupergroupIsAllHistoryAvailable {
    const TYPE: &'static str = "toggleSupergroupIsAllHistoryAvailable";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Reports some messages from a user in a supergroup as spam; requires administrator rights in the supergroup "]
pub struct ReportSupergroupSpam {
    #[doc = "Supergroup identifier "]
    pub supergroup_id: i32,
    #[doc = "User identifier "]
    pub user_id: i32,
    #[doc = "Identifiers of messages sent in the supergroup by the user. This list must be non-empty"]
    pub message_ids: Vec<i64>,
}
impl Method for ReportSupergroupSpam {
    const TYPE: &'static str = "reportSupergroupSpam";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about members or banned users in a supergroup or channel. Can be used only if SupergroupFullInfo.can_get_members == true; additionally, administrator privileges may be required for some filters "]
pub struct GetSupergroupMembers {
    #[doc = "Identifier of the supergroup or channel"]
    pub supergroup_id: i32,
    #[doc = "The type of users to return. By default, supergroupMembersRecent "]
    pub filter: SupergroupMembersFilter,
    #[doc = "Number of users to skip "]
    pub offset: i32,
    #[doc = "The maximum number of users be returned; up to 200"]
    pub limit: i32,
}
impl Method for GetSupergroupMembers {
    const TYPE: &'static str = "getSupergroupMembers";
    type Response = ChatMembers;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Deletes a supergroup or channel along with all messages in the corresponding chat. This will release the supergroup or channel username and remove all members; requires owner privileges in the supergroup or channel. Chats with more than 1000 members can't be deleted using this method "]
pub struct DeleteSupergroup {
    #[doc = "Identifier of the supergroup or channel"]
    pub supergroup_id: i32,
}
impl Method for DeleteSupergroup {
    const TYPE: &'static str = "deleteSupergroup";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Closes a secret chat, effectively transferring its state to secretChatStateClosed "]
pub struct CloseSecretChat {
    #[doc = "Secret chat identifier"]
    pub secret_chat_id: i32,
}
impl Method for CloseSecretChat {
    const TYPE: &'static str = "closeSecretChat";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a list of service actions taken by chat members and administrators in the last 48 hours. Available only for supergroups and channels. Requires administrator rights. Returns results in reverse chronological order (i. e., in order of decreasing event_id)"]
pub struct GetChatEventLog {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Search query by which to filter events "]
    pub query: String,
    #[doc = "Identifier of an event from which to return results. Use 0 to get results from the latest events "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub from_event_id: i64,
    #[doc = "The maximum number of events to return; up to 100"]
    pub limit: i32,
    #[doc = "The types of events to return. By default, all types will be returned "]
    pub filters: ChatEventLogFilters,
    #[doc = "User identifiers by which to filter events. By default, events relating to all users will be returned"]
    pub user_ids: Vec<i32>,
}
impl Method for GetChatEventLog {
    const TYPE: &'static str = "getChatEventLog";
    type Response = ChatEvents;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns an invoice payment form. This method should be called when the user presses inlineKeyboardButtonBuy "]
pub struct GetPaymentForm {
    #[doc = "Chat identifier of the Invoice message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Message identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
}
impl Method for GetPaymentForm {
    const TYPE: &'static str = "getPaymentForm";
    type Response = PaymentForm;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Validates the order information provided by a user and returns the available shipping options for a flexible invoice "]
pub struct ValidateOrderInfo {
    #[doc = "Chat identifier of the Invoice message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Message identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "The order information, provided by the user "]
    pub order_info: OrderInfo,
    #[doc = "True, if the order information can be saved"]
    #[serde(default)]
    pub allow_save: bool,
}
impl Method for ValidateOrderInfo {
    const TYPE: &'static str = "validateOrderInfo";
    type Response = ValidatedOrderInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends a filled-out payment form to the bot for final verification "]
pub struct SendPaymentForm {
    #[doc = "Chat identifier of the Invoice message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Message identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
    #[doc = "Identifier returned by ValidateOrderInfo, or an empty string "]
    pub order_info_id: String,
    #[doc = "Identifier of a chosen shipping option, if applicable"]
    pub shipping_option_id: String,
    #[doc = "The credentials chosen by user for payment"]
    pub credentials: InputCredentials,
}
impl Method for SendPaymentForm {
    const TYPE: &'static str = "sendPaymentForm";
    type Response = PaymentResult;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a successful payment "]
pub struct GetPaymentReceipt {
    #[doc = "Chat identifier of the PaymentSuccessful message "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Message identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub message_id: i64,
}
impl Method for GetPaymentReceipt {
    const TYPE: &'static str = "getPaymentReceipt";
    type Response = PaymentReceipt;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns saved order info, if any"]
pub struct GetSavedOrderInfo {}
impl Method for GetSavedOrderInfo {
    const TYPE: &'static str = "getSavedOrderInfo";
    type Response = OrderInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Deletes saved order info"]
pub struct DeleteSavedOrderInfo {}
impl Method for DeleteSavedOrderInfo {
    const TYPE: &'static str = "deleteSavedOrderInfo";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Deletes saved credentials for all payment provider bots"]
pub struct DeleteSavedCredentials {}
impl Method for DeleteSavedCredentials {
    const TYPE: &'static str = "deleteSavedCredentials";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a user that can be contacted to get support"]
pub struct GetSupportUser {}
impl Method for GetSupportUser {
    const TYPE: &'static str = "getSupportUser";
    type Response = User;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns backgrounds installed by the user "]
pub struct GetBackgrounds {
    #[doc = "True, if the backgrounds needs to be ordered for dark theme"]
    #[serde(default)]
    pub for_dark_theme: bool,
}
impl Method for GetBackgrounds {
    const TYPE: &'static str = "getBackgrounds";
    type Response = Backgrounds;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Constructs a persistent HTTP URL for a background "]
pub struct GetBackgroundUrl {
    #[doc = "Background name "]
    pub name: String,
    #[serde(rename = "type")]
    #[doc = "Background type"]
    pub type_: BackgroundType,
}
impl Method for GetBackgroundUrl {
    const TYPE: &'static str = "getBackgroundUrl";
    type Response = HttpUrl;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Searches for a background by its name "]
pub struct SearchBackground {
    #[doc = "The name of the background"]
    pub name: String,
}
impl Method for SearchBackground {
    const TYPE: &'static str = "searchBackground";
    type Response = Background;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the background selected by the user; adds background to the list of installed backgrounds"]
pub struct SetBackground {
    #[doc = "The input background to use, null for filled backgrounds"]
    pub background: InputBackground,
    #[serde(rename = "type")]
    #[doc = "Background type; null for default background. The method will return error 404 if type is null"]
    pub type_: BackgroundType,
    #[doc = "True, if the background is chosen for dark theme"]
    #[serde(default)]
    pub for_dark_theme: bool,
}
impl Method for SetBackground {
    const TYPE: &'static str = "setBackground";
    type Response = Background;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes background from the list of installed backgrounds "]
pub struct RemoveBackground {
    #[doc = "The background identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub background_id: i64,
}
impl Method for RemoveBackground {
    const TYPE: &'static str = "removeBackground";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Resets list of installed backgrounds to its default value"]
pub struct ResetBackgrounds {}
impl Method for ResetBackgrounds {
    const TYPE: &'static str = "resetBackgrounds";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about the current localization target. This is an offline request if only_local is true. Can be called before authorization "]
pub struct GetLocalizationTargetInfo {
    #[doc = "If true, returns only locally available information without sending network requests"]
    #[serde(default)]
    pub only_local: bool,
}
impl Method for GetLocalizationTargetInfo {
    const TYPE: &'static str = "getLocalizationTargetInfo";
    type Response = LocalizationTargetInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a language pack. Returned language pack identifier may be different from a provided one. Can be called before authorization "]
pub struct GetLanguagePackInfo {
    #[doc = "Language pack identifier"]
    pub language_pack_id: String,
}
impl Method for GetLanguagePackInfo {
    const TYPE: &'static str = "getLanguagePackInfo";
    type Response = LanguagePackInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns strings from a language pack in the current localization target by their keys. Can be called before authorization "]
pub struct GetLanguagePackStrings {
    #[doc = "Language pack identifier of the strings to be returned "]
    pub language_pack_id: String,
    #[doc = "Language pack keys of the strings to be returned; leave empty to request all available strings"]
    pub keys: Vec<String>,
}
impl Method for GetLanguagePackStrings {
    const TYPE: &'static str = "getLanguagePackStrings";
    type Response = LanguagePackStrings;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Fetches the latest versions of all strings from a language pack in the current localization target from the server. This method doesn't need to be called explicitly for the current used/base language packs. Can be called before authorization "]
pub struct SynchronizeLanguagePack {
    #[doc = "Language pack identifier"]
    pub language_pack_id: String,
}
impl Method for SynchronizeLanguagePack {
    const TYPE: &'static str = "synchronizeLanguagePack";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds a custom server language pack to the list of installed language packs in current localization target. Can be called before authorization "]
pub struct AddCustomServerLanguagePack {
    #[doc = "Identifier of a language pack to be added; may be different from a name that is used in an \"https://t.me/setlanguage/\" link"]
    pub language_pack_id: String,
}
impl Method for AddCustomServerLanguagePack {
    const TYPE: &'static str = "addCustomServerLanguagePack";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds or changes a custom local language pack to the current localization target "]
pub struct SetCustomLanguagePack {
    #[doc = "Information about the language pack. Language pack ID must start with 'X', consist only of English letters, digits and hyphens, and must not exceed 64 characters. Can be called before authorization "]
    pub info: LanguagePackInfo,
    #[doc = "Strings of the new language pack"]
    pub strings: Vec<LanguagePackString>,
}
impl Method for SetCustomLanguagePack {
    const TYPE: &'static str = "setCustomLanguagePack";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits information about a custom local language pack in the current localization target. Can be called before authorization "]
pub struct EditCustomLanguagePackInfo {
    #[doc = "New information about the custom local language pack"]
    pub info: LanguagePackInfo,
}
impl Method for EditCustomLanguagePackInfo {
    const TYPE: &'static str = "editCustomLanguagePackInfo";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds, edits or deletes a string in a custom local language pack. Can be called before authorization "]
pub struct SetCustomLanguagePackString {
    #[doc = "Identifier of a previously added custom local language pack in the current localization target "]
    pub language_pack_id: String,
    #[doc = "New language pack string"]
    pub new_string: LanguagePackString,
}
impl Method for SetCustomLanguagePackString {
    const TYPE: &'static str = "setCustomLanguagePackString";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Deletes all information about a language pack in the current localization target. The language pack which is currently in use (including base language pack) or is being synchronized can't be deleted. Can be called before authorization "]
pub struct DeleteLanguagePack {
    #[doc = "Identifier of the language pack to delete"]
    pub language_pack_id: String,
}
impl Method for DeleteLanguagePack {
    const TYPE: &'static str = "deleteLanguagePack";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Registers the currently used device for receiving push notifications. Returns a globally unique identifier of the push notification subscription "]
pub struct RegisterDevice {
    #[doc = "Device token "]
    pub device_token: DeviceToken,
    #[doc = "List of user identifiers of other users currently using the client"]
    pub other_user_ids: Vec<i32>,
}
impl Method for RegisterDevice {
    const TYPE: &'static str = "registerDevice";
    type Response = PushReceiverId;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Handles a push notification. Returns error with code 406 if the push notification is not supported and connection to the server is required to fetch new data. Can be called before authorization"]
pub struct ProcessPushNotification {
    #[doc = "JSON-encoded push notification payload with all fields sent by the server, and \"google.sent_time\" and \"google.notification.sound\" fields added"]
    pub payload: String,
}
impl Method for ProcessPushNotification {
    const TYPE: &'static str = "processPushNotification";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a globally unique push notification subscription identifier for identification of an account, which has received a push notification. This is an offline method. Can be called before authorization. Can be called synchronously "]
pub struct GetPushReceiverId {
    #[doc = "JSON-encoded push notification payload"]
    pub payload: String,
}
impl Method for GetPushReceiverId {
    const TYPE: &'static str = "getPushReceiverId";
    type Response = PushReceiverId;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns t.me URLs recently visited by a newly registered user "]
pub struct GetRecentlyVisitedTMeUrls {
    #[doc = "Google Play referrer to identify the user"]
    pub referrer: String,
}
impl Method for GetRecentlyVisitedTMeUrls {
    const TYPE: &'static str = "getRecentlyVisitedTMeUrls";
    type Response = TMeUrls;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes user privacy settings "]
pub struct SetUserPrivacySettingRules {
    #[doc = "The privacy setting "]
    pub setting: UserPrivacySetting,
    #[doc = "The new privacy rules"]
    pub rules: UserPrivacySettingRules,
}
impl Method for SetUserPrivacySettingRules {
    const TYPE: &'static str = "setUserPrivacySettingRules";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the current privacy settings "]
pub struct GetUserPrivacySettingRules {
    #[doc = "The privacy setting"]
    pub setting: UserPrivacySetting,
}
impl Method for GetUserPrivacySettingRules {
    const TYPE: &'static str = "getUserPrivacySettingRules";
    type Response = UserPrivacySettingRules;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the value of an option by its name. (Check the list of available options on https://core.telegram.org/tdlib/options.) Can be called before authorization"]
pub struct GetOption {
    #[doc = "The name of the option"]
    pub name: String,
}
impl Method for GetOption {
    const TYPE: &'static str = "getOption";
    type Response = OptionValue;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets the value of an option. (Check the list of available options on https://core.telegram.org/tdlib/options.) Only writable options can be set. Can be called before authorization"]
pub struct SetOption {
    #[doc = "The name of the option "]
    pub name: String,
    #[doc = "The new value of the option"]
    pub value: OptionValue,
}
impl Method for SetOption {
    const TYPE: &'static str = "setOption";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the period of inactivity after which the account of the current user will automatically be deleted "]
pub struct SetAccountTtl {
    #[doc = "New account TTL"]
    pub ttl: AccountTtl,
}
impl Method for SetAccountTtl {
    const TYPE: &'static str = "setAccountTtl";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the period of inactivity after which the account of the current user will automatically be deleted"]
pub struct GetAccountTtl {}
impl Method for GetAccountTtl {
    const TYPE: &'static str = "getAccountTtl";
    type Response = AccountTtl;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Deletes the account of the current user, deleting all information associated with the user from the server. The phone number of the account can be used to create a new account. Can be called before authorization when the current authorization state is authorizationStateWaitPassword "]
pub struct DeleteAccount {
    #[doc = "The reason why the account was deleted; optional"]
    pub reason: String,
}
impl Method for DeleteAccount {
    const TYPE: &'static str = "deleteAccount";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes a chat action bar without any other action "]
pub struct RemoveChatActionBar {
    #[doc = "Chat identifier"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for RemoveChatActionBar {
    const TYPE: &'static str = "removeChatActionBar";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Reports a chat to the Telegram moderators. A chat can be reported only from the chat action bar, or if this is a private chats with a bot, a private chat with a user sharing their location, a supergroup, or a channel, since other chats can't be checked by moderators "]
pub struct ReportChat {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The reason for reporting the chat "]
    pub reason: ChatReportReason,
    #[doc = "Identifiers of reported messages, if any"]
    pub message_ids: Vec<i64>,
}
impl Method for ReportChat {
    const TYPE: &'static str = "reportChat";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns an HTTP URL with the chat statistics. Currently this method of getting the statistics is disabled and can be deleted in the future "]
pub struct GetChatStatisticsUrl {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Parameters from \"tg://statsrefresh?params=******\" link "]
    pub parameters: String,
    #[doc = "Pass true if a URL with the dark theme must be returned"]
    #[serde(default)]
    pub is_dark: bool,
}
impl Method for GetChatStatisticsUrl {
    const TYPE: &'static str = "getChatStatisticsUrl";
    type Response = HttpUrl;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns detailed statistics about a chat. Currently this method can be used only for channels. Requires administrator rights in the channel "]
pub struct GetChatStatistics {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "Pass true if a dark theme is used by the app"]
    #[serde(default)]
    pub is_dark: bool,
}
impl Method for GetChatStatistics {
    const TYPE: &'static str = "getChatStatistics";
    type Response = ChatStatistics;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Loads asynchronous or zoomed in chat statistics graph "]
pub struct GetChatStatisticsGraph {
    #[doc = "Chat identifier "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The token for graph loading "]
    pub token: String,
    #[doc = "X-value for zoomed in graph or 0 otherwise"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub x: i64,
}
impl Method for GetChatStatisticsGraph {
    const TYPE: &'static str = "getChatStatisticsGraph";
    type Response = StatisticsGraph;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns storage usage statistics. Can be called before authorization "]
pub struct GetStorageStatistics {
    #[doc = "The maximum number of chats with the largest storage usage for which separate statistics should be returned. All other chats will be grouped in entries with chat_id == 0. If the chat info database is not used, the chat_limit is ignored and is always set to 0"]
    pub chat_limit: i32,
}
impl Method for GetStorageStatistics {
    const TYPE: &'static str = "getStorageStatistics";
    type Response = StorageStatistics;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Quickly returns approximate storage usage statistics. Can be called before authorization"]
pub struct GetStorageStatisticsFast {}
impl Method for GetStorageStatisticsFast {
    const TYPE: &'static str = "getStorageStatisticsFast";
    type Response = StorageStatisticsFast;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns database statistics"]
pub struct GetDatabaseStatistics {}
impl Method for GetDatabaseStatistics {
    const TYPE: &'static str = "getDatabaseStatistics";
    type Response = DatabaseStatistics;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Optimizes storage usage, i.e. deletes some files and returns new storage usage statistics. Secret thumbnails can't be deleted"]
pub struct OptimizeStorage {
    #[doc = "Limit on the total size of files after deletion. Pass -1 to use the default limit"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub size: i64,
    #[doc = "Limit on the time that has passed since the last time a file was accessed (or creation time for some filesystems). Pass -1 to use the default limit"]
    pub ttl: i32,
    #[doc = "Limit on the total count of files after deletion. Pass -1 to use the default limit"]
    pub count: i32,
    #[doc = "The amount of time after the creation of a file during which it can't be deleted, in seconds. Pass -1 to use the default value"]
    pub immunity_delay: i32,
    #[doc = "If not empty, only files with the given type(s) are considered. By default, all types except thumbnails, profile photos, stickers and wallpapers are deleted"]
    pub file_types: Vec<FileType>,
    #[doc = "If not empty, only files from the given chats are considered. Use 0 as chat identifier to delete files not belonging to any chat (e.g., profile photos)"]
    pub chat_ids: Vec<i64>,
    #[doc = "If not empty, files from the given chats are excluded. Use 0 as chat identifier to exclude all files not belonging to any chat (e.g., profile photos)"]
    pub exclude_chat_ids: Vec<i64>,
    #[doc = "Pass true if deleted file statistics needs to be returned instead of the whole storage usage statistics. Affects only returned statistics"]
    #[serde(default)]
    pub return_deleted_file_statistics: bool,
    #[doc = "Same as in getStorageStatistics. Affects only returned statistics"]
    pub chat_limit: i32,
}
impl Method for OptimizeStorage {
    const TYPE: &'static str = "optimizeStorage";
    type Response = StorageStatistics;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets the current network type. Can be called before authorization. Calling this method forces all network connections to reopen, mitigating the delay in switching between different networks, so it should be called whenever the network is changed, even if the network type remains the same.\n Network type is used to check whether the library can use the network at all and also for collecting detailed network data usage statistics "]
pub struct SetNetworkType {
    #[serde(rename = "type")]
    #[doc = "The new network type. By default, networkTypeOther"]
    pub type_: NetworkType,
}
impl Method for SetNetworkType {
    const TYPE: &'static str = "setNetworkType";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns network data usage statistics. Can be called before authorization "]
pub struct GetNetworkStatistics {
    #[doc = "If true, returns only data for the current library launch"]
    #[serde(default)]
    pub only_current: bool,
}
impl Method for GetNetworkStatistics {
    const TYPE: &'static str = "getNetworkStatistics";
    type Response = NetworkStatistics;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds the specified data to data usage statistics. Can be called before authorization "]
pub struct AddNetworkStatistics {
    #[doc = "The network statistics entry with the data to be added to statistics"]
    pub entry: NetworkStatisticsEntry,
}
impl Method for AddNetworkStatistics {
    const TYPE: &'static str = "addNetworkStatistics";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Resets all network data usage statistics to zero. Can be called before authorization"]
pub struct ResetNetworkStatistics {}
impl Method for ResetNetworkStatistics {
    const TYPE: &'static str = "resetNetworkStatistics";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns auto-download settings presets for the current user"]
pub struct GetAutoDownloadSettingsPresets {}
impl Method for GetAutoDownloadSettingsPresets {
    const TYPE: &'static str = "getAutoDownloadSettingsPresets";
    type Response = AutoDownloadSettingsPresets;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets auto-download settings "]
pub struct SetAutoDownloadSettings {
    #[doc = "New user auto-download settings "]
    pub settings: AutoDownloadSettings,
    #[serde(rename = "type")]
    #[doc = "Type of the network for which the new settings are applied"]
    pub type_: NetworkType,
}
impl Method for SetAutoDownloadSettings {
    const TYPE: &'static str = "setAutoDownloadSettings";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a bank card "]
pub struct GetBankCardInfo {
    #[doc = "The bank card number"]
    pub bank_card_number: String,
}
impl Method for GetBankCardInfo {
    const TYPE: &'static str = "getBankCardInfo";
    type Response = BankCardInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns one of the available Telegram Passport elements "]
pub struct GetPassportElement {
    #[serde(rename = "type")]
    #[doc = "Telegram Passport element type "]
    pub type_: PassportElementType,
    #[doc = "Password of the current user"]
    pub password: String,
}
impl Method for GetPassportElement {
    const TYPE: &'static str = "getPassportElement";
    type Response = PassportElement;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns all available Telegram Passport elements "]
pub struct GetAllPassportElements {
    #[doc = "Password of the current user"]
    pub password: String,
}
impl Method for GetAllPassportElements {
    const TYPE: &'static str = "getAllPassportElements";
    type Response = PassportElements;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds an element to the user's Telegram Passport. May return an error with a message \"PHONE_VERIFICATION_NEEDED\" or \"EMAIL_VERIFICATION_NEEDED\" if the chosen phone number or the chosen email address must be verified first "]
pub struct SetPassportElement {
    #[doc = "Input Telegram Passport element "]
    pub element: InputPassportElement,
    #[doc = "Password of the current user"]
    pub password: String,
}
impl Method for SetPassportElement {
    const TYPE: &'static str = "setPassportElement";
    type Response = PassportElement;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Deletes a Telegram Passport element "]
pub struct DeletePassportElement {
    #[serde(rename = "type")]
    #[doc = "Element type"]
    pub type_: PassportElementType,
}
impl Method for DeletePassportElement {
    const TYPE: &'static str = "deletePassportElement";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Informs the user that some of the elements in their Telegram Passport contain errors; for bots only. The user will not be able to resend the elements, until the errors are fixed "]
pub struct SetPassportElementErrors {
    #[doc = "User identifier "]
    pub user_id: i32,
    #[doc = "The errors"]
    pub errors: Vec<InputPassportElementError>,
}
impl Method for SetPassportElementErrors {
    const TYPE: &'static str = "setPassportElementErrors";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns an IETF language tag of the language preferred in the country, which should be used to fill native fields in Telegram Passport personal details. Returns a 404 error if unknown "]
pub struct GetPreferredCountryLanguage {
    #[doc = "A two-letter ISO 3166-1 alpha-2 country code"]
    pub country_code: String,
}
impl Method for GetPreferredCountryLanguage {
    const TYPE: &'static str = "getPreferredCountryLanguage";
    type Response = Text;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends a code to verify a phone number to be added to a user's Telegram Passport"]
pub struct SendPhoneNumberVerificationCode {
    #[doc = "The phone number of the user, in international format "]
    pub phone_number: String,
    #[doc = "Settings for the authentication of the user's phone number"]
    pub settings: PhoneNumberAuthenticationSettings,
}
impl Method for SendPhoneNumberVerificationCode {
    const TYPE: &'static str = "sendPhoneNumberVerificationCode";
    type Response = AuthenticationCodeInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Re-sends the code to verify a phone number to be added to a user's Telegram Passport"]
pub struct ResendPhoneNumberVerificationCode {}
impl Method for ResendPhoneNumberVerificationCode {
    const TYPE: &'static str = "resendPhoneNumberVerificationCode";
    type Response = AuthenticationCodeInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks the phone number verification code for Telegram Passport "]
pub struct CheckPhoneNumberVerificationCode {
    #[doc = "Verification code"]
    pub code: String,
}
impl Method for CheckPhoneNumberVerificationCode {
    const TYPE: &'static str = "checkPhoneNumberVerificationCode";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends a code to verify an email address to be added to a user's Telegram Passport "]
pub struct SendEmailAddressVerificationCode {
    #[doc = "Email address"]
    pub email_address: String,
}
impl Method for SendEmailAddressVerificationCode {
    const TYPE: &'static str = "sendEmailAddressVerificationCode";
    type Response = EmailAddressAuthenticationCodeInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Re-sends the code to verify an email address to be added to a user's Telegram Passport"]
pub struct ResendEmailAddressVerificationCode {}
impl Method for ResendEmailAddressVerificationCode {
    const TYPE: &'static str = "resendEmailAddressVerificationCode";
    type Response = EmailAddressAuthenticationCodeInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks the email address verification code for Telegram Passport "]
pub struct CheckEmailAddressVerificationCode {
    #[doc = "Verification code"]
    pub code: String,
}
impl Method for CheckEmailAddressVerificationCode {
    const TYPE: &'static str = "checkEmailAddressVerificationCode";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns a Telegram Passport authorization form for sharing data with a service "]
pub struct GetPassportAuthorizationForm {
    #[doc = "User identifier of the service's bot "]
    pub bot_user_id: i32,
    #[doc = "Telegram Passport element types requested by the service "]
    pub scope: String,
    #[doc = "Service's public_key "]
    pub public_key: String,
    #[doc = "Authorization form nonce provided by the service"]
    pub nonce: String,
}
impl Method for GetPassportAuthorizationForm {
    const TYPE: &'static str = "getPassportAuthorizationForm";
    type Response = PassportAuthorizationForm;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns already available Telegram Passport elements suitable for completing a Telegram Passport authorization form. Result can be received only once for each authorization form "]
pub struct GetPassportAuthorizationFormAvailableElements {
    #[doc = "Authorization form identifier "]
    pub autorization_form_id: i32,
    #[doc = "Password of the current user"]
    pub password: String,
}
impl Method for GetPassportAuthorizationFormAvailableElements {
    const TYPE: &'static str = "getPassportAuthorizationFormAvailableElements";
    type Response = PassportElementsWithErrors;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends a Telegram Passport authorization form, effectively sharing data with the service. This method must be called after getPassportAuthorizationFormAvailableElements if some previously available elements need to be used"]
pub struct SendPassportAuthorizationForm {
    #[doc = "Authorization form identifier "]
    pub autorization_form_id: i32,
    #[doc = "Types of Telegram Passport elements chosen by user to complete the authorization form"]
    pub types: Vec<PassportElementType>,
}
impl Method for SendPassportAuthorizationForm {
    const TYPE: &'static str = "sendPassportAuthorizationForm";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends phone number confirmation code. Should be called when user presses \"https://t.me/confirmphone?phone=*******&hash=**********\" or \"tg://confirmphone?phone=*******&hash=**********\" link "]
pub struct SendPhoneNumberConfirmationCode {
    #[doc = "Value of the \"hash\" parameter from the link"]
    pub hash: String,
    #[doc = "Value of the \"phone\" parameter from the link "]
    pub phone_number: String,
    #[doc = "Settings for the authentication of the user's phone number"]
    pub settings: PhoneNumberAuthenticationSettings,
}
impl Method for SendPhoneNumberConfirmationCode {
    const TYPE: &'static str = "sendPhoneNumberConfirmationCode";
    type Response = AuthenticationCodeInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Resends phone number confirmation code"]
pub struct ResendPhoneNumberConfirmationCode {}
impl Method for ResendPhoneNumberConfirmationCode {
    const TYPE: &'static str = "resendPhoneNumberConfirmationCode";
    type Response = AuthenticationCodeInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Checks phone number confirmation code "]
pub struct CheckPhoneNumberConfirmationCode {
    #[doc = "The phone number confirmation code"]
    pub code: String,
}
impl Method for CheckPhoneNumberConfirmationCode {
    const TYPE: &'static str = "checkPhoneNumberConfirmationCode";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Informs the server about the number of pending bot updates if they haven't been processed for a long time; for bots only "]
pub struct SetBotUpdatesStatus {
    #[doc = "The number of pending updates "]
    pub pending_update_count: i32,
    #[doc = "The last error message"]
    pub error_message: String,
}
impl Method for SetBotUpdatesStatus {
    const TYPE: &'static str = "setBotUpdatesStatus";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Uploads a PNG image with a sticker; for bots only; returns the uploaded file"]
pub struct UploadStickerFile {
    #[doc = "Sticker file owner "]
    pub user_id: i32,
    #[doc = "PNG image with the sticker; must be up to 512 KB in size and fit in 512x512 square"]
    pub png_sticker: InputFile,
}
impl Method for UploadStickerFile {
    const TYPE: &'static str = "uploadStickerFile";
    type Response = File;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Creates a new sticker set; for bots only. Returns the newly created sticker set"]
pub struct CreateNewStickerSet {
    #[doc = "Sticker set owner"]
    pub user_id: i32,
    #[doc = "Sticker set title; 1-64 characters"]
    pub title: String,
    #[doc = "Sticker set name. Can contain only English letters, digits and underscores. Must end with *\"_by_<bot username>\"* (*<bot_username>* is case insensitive); 1-64 characters"]
    pub name: String,
    #[doc = "True, if stickers are masks. Animated stickers can't be masks"]
    #[serde(default)]
    pub is_masks: bool,
    #[doc = "List of stickers to be added to the set; must be non-empty. All stickers must be of the same type"]
    pub stickers: Vec<InputSticker>,
}
impl Method for CreateNewStickerSet {
    const TYPE: &'static str = "createNewStickerSet";
    type Response = StickerSet;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds a new sticker to a set; for bots only. Returns the sticker set"]
pub struct AddStickerToSet {
    #[doc = "Sticker set owner "]
    pub user_id: i32,
    #[doc = "Sticker set name "]
    pub name: String,
    #[doc = "Sticker to add to the set"]
    pub sticker: InputSticker,
}
impl Method for AddStickerToSet {
    const TYPE: &'static str = "addStickerToSet";
    type Response = StickerSet;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets a sticker set thumbnail; for bots only. Returns the sticker set"]
pub struct SetStickerSetThumbnail {
    #[doc = "Sticker set owner "]
    pub user_id: i32,
    #[doc = "Sticker set name"]
    pub name: String,
    #[doc = "Thumbnail to set in PNG or TGS format. Animated thumbnail must be set for animated sticker sets and only for them. You can use a zero InputFileId to delete the thumbnail"]
    pub thumbnail: InputFile,
}
impl Method for SetStickerSetThumbnail {
    const TYPE: &'static str = "setStickerSetThumbnail";
    type Response = StickerSet;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Changes the position of a sticker in the set to which it belongs; for bots only. The sticker set must have been created by the bot"]
pub struct SetStickerPositionInSet {
    #[doc = "Sticker "]
    pub sticker: InputFile,
    #[doc = "New position of the sticker in the set, zero-based"]
    pub position: i32,
}
impl Method for SetStickerPositionInSet {
    const TYPE: &'static str = "setStickerPositionInSet";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes a sticker from the set to which it belongs; for bots only. The sticker set must have been created by the bot "]
pub struct RemoveStickerFromSet {
    #[doc = "Sticker"]
    pub sticker: InputFile,
}
impl Method for RemoveStickerFromSet {
    const TYPE: &'static str = "removeStickerFromSet";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a file with a map thumbnail in PNG format. Only map thumbnail files with size less than 1MB can be downloaded "]
pub struct GetMapThumbnailFile {
    #[doc = "Location of the map center "]
    pub location: Location,
    #[doc = "Map zoom level; 13-20 "]
    pub zoom: i32,
    #[doc = "Map width in pixels before applying scale; 16-1024 "]
    pub width: i32,
    #[doc = "Map height in pixels before applying scale; 16-1024 "]
    pub height: i32,
    #[doc = "Map scale; 1-3 "]
    pub scale: i32,
    #[doc = "Identifier of a chat, in which the thumbnail will be shown. Use 0 if unknown"]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
}
impl Method for GetMapThumbnailFile {
    const TYPE: &'static str = "getMapThumbnailFile";
    type Response = File;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Accepts Telegram terms of services "]
pub struct AcceptTermsOfService {
    #[doc = "Terms of service identifier"]
    pub terms_of_service_id: String,
}
impl Method for AcceptTermsOfService {
    const TYPE: &'static str = "acceptTermsOfService";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends a custom request; for bots only "]
pub struct SendCustomRequest {
    #[doc = "The method name "]
    pub method: String,
    #[doc = "JSON-serialized method parameters"]
    pub parameters: String,
}
impl Method for SendCustomRequest {
    const TYPE: &'static str = "sendCustomRequest";
    type Response = CustomRequestResult;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Answers a custom query; for bots only "]
pub struct AnswerCustomQuery {
    #[doc = "Identifier of a custom query "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub custom_query_id: i64,
    #[doc = "JSON-serialized answer to the query"]
    pub data: String,
}
impl Method for AnswerCustomQuery {
    const TYPE: &'static str = "answerCustomQuery";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Succeeds after a specified amount of time has passed. Can be called before authorization. Can be called before initialization "]
pub struct SetAlarm {
    #[doc = "Number of seconds before the function returns"]
    pub seconds: f64,
}
impl Method for SetAlarm {
    const TYPE: &'static str = "setAlarm";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Uses current user IP to found their country. Returns two-letter ISO 3166-1 alpha-2 country code. Can be called before authorization"]
pub struct GetCountryCode {}
impl Method for GetCountryCode {
    const TYPE: &'static str = "getCountryCode";
    type Response = Text;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the default text for invitation messages to be used as a placeholder when the current user invites friends to Telegram"]
pub struct GetInviteText {}
impl Method for GetInviteText {
    const TYPE: &'static str = "getInviteText";
    type Response = Text;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about a tg:// deep link. Use \"tg://need_update_for_some_feature\" or \"tg:some_unsupported_feature\" for testing. Returns a 404 error for unknown links. Can be called before authorization "]
pub struct GetDeepLinkInfo {
    #[doc = "The link"]
    pub link: String,
}
impl Method for GetDeepLinkInfo {
    const TYPE: &'static str = "getDeepLinkInfo";
    type Response = DeepLinkInfo;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns application config, provided by the server. Can be called before authorization"]
pub struct GetApplicationConfig {}
impl Method for GetApplicationConfig {
    const TYPE: &'static str = "getApplicationConfig";
    type Response = JsonValue;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Saves application log event on the server. Can be called before authorization "]
pub struct SaveApplicationLogEvent {
    #[serde(rename = "type")]
    #[doc = "Event type "]
    pub type_: String,
    #[doc = "Optional chat identifier, associated with the event "]
    #[serde(deserialize_with = "::serde_aux::field_attributes::deserialize_number_from_string")]
    pub chat_id: i64,
    #[doc = "The log event data"]
    pub data: JsonValue,
}
impl Method for SaveApplicationLogEvent {
    const TYPE: &'static str = "saveApplicationLogEvent";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds a proxy server for network requests. Can be called before authorization "]
pub struct AddProxy {
    #[doc = "Proxy server IP address "]
    pub server: String,
    #[doc = "Proxy server port "]
    pub port: i32,
    #[doc = "True, if the proxy should be enabled "]
    #[serde(default)]
    pub enable: bool,
    #[serde(rename = "type")]
    #[doc = "Proxy type"]
    pub type_: ProxyType,
}
impl Method for AddProxy {
    const TYPE: &'static str = "addProxy";
    type Response = Proxy;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Edits an existing proxy server for network requests. Can be called before authorization "]
pub struct EditProxy {
    #[doc = "Proxy identifier "]
    pub proxy_id: i32,
    #[doc = "Proxy server IP address "]
    pub server: String,
    #[doc = "Proxy server port "]
    pub port: i32,
    #[doc = "True, if the proxy should be enabled "]
    #[serde(default)]
    pub enable: bool,
    #[serde(rename = "type")]
    #[doc = "Proxy type"]
    pub type_: ProxyType,
}
impl Method for EditProxy {
    const TYPE: &'static str = "editProxy";
    type Response = Proxy;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Enables a proxy. Only one proxy can be enabled at a time. Can be called before authorization "]
pub struct EnableProxy {
    #[doc = "Proxy identifier"]
    pub proxy_id: i32,
}
impl Method for EnableProxy {
    const TYPE: &'static str = "enableProxy";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Disables the currently enabled proxy. Can be called before authorization"]
pub struct DisableProxy {}
impl Method for DisableProxy {
    const TYPE: &'static str = "disableProxy";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Removes a proxy server. Can be called before authorization "]
pub struct RemoveProxy {
    #[doc = "Proxy identifier"]
    pub proxy_id: i32,
}
impl Method for RemoveProxy {
    const TYPE: &'static str = "removeProxy";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns list of proxies that are currently set up. Can be called before authorization"]
pub struct GetProxies {}
impl Method for GetProxies {
    const TYPE: &'static str = "getProxies";
    type Response = Proxies;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns an HTTPS link, which can be used to add a proxy. Available only for SOCKS5 and MTProto proxies. Can be called before authorization "]
pub struct GetProxyLink {
    #[doc = "Proxy identifier"]
    pub proxy_id: i32,
}
impl Method for GetProxyLink {
    const TYPE: &'static str = "getProxyLink";
    type Response = Text;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Computes time needed to receive a response from a Telegram server through a proxy. Can be called before authorization "]
pub struct PingProxy {
    #[doc = "Proxy identifier. Use 0 to ping a Telegram server without a proxy"]
    pub proxy_id: i32,
}
impl Method for PingProxy {
    const TYPE: &'static str = "pingProxy";
    type Response = Seconds;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets new log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously "]
pub struct SetLogStream {
    #[doc = "New log stream"]
    pub log_stream: LogStream,
}
impl Method for SetLogStream {
    const TYPE: &'static str = "setLogStream";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns information about currently used log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously"]
pub struct GetLogStream {}
impl Method for GetLogStream {
    const TYPE: &'static str = "getLogStream";
    type Response = LogStream;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets the verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously"]
pub struct SetLogVerbosityLevel {
    #[doc = "New value of the verbosity level for logging. Value 0 corresponds to fatal errors, value 1 corresponds to errors, value 2 corresponds to warnings and debug warnings, value 3 corresponds to informational, value 4 corresponds to debug, value 5 corresponds to verbose debug, value greater than 5 and up to 1023 can be used to enable even more logging"]
    pub new_verbosity_level: i32,
}
impl Method for SetLogVerbosityLevel {
    const TYPE: &'static str = "setLogVerbosityLevel";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns current verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously"]
pub struct GetLogVerbosityLevel {}
impl Method for GetLogVerbosityLevel {
    const TYPE: &'static str = "getLogVerbosityLevel";
    type Response = LogVerbosityLevel;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns list of available TDLib internal log tags, for example, [\"actor\", \"binlog\", \"connections\", \"notifications\", \"proxy\"]. This is an offline method. Can be called before authorization. Can be called synchronously"]
pub struct GetLogTags {}
impl Method for GetLogTags {
    const TYPE: &'static str = "getLogTags";
    type Response = LogTags;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sets the verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously"]
pub struct SetLogTagVerbosityLevel {
    #[doc = "Logging tag to change verbosity level "]
    pub tag: String,
    #[doc = "New verbosity level; 1-1024"]
    pub new_verbosity_level: i32,
}
impl Method for SetLogTagVerbosityLevel {
    const TYPE: &'static str = "setLogTagVerbosityLevel";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns current verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously "]
pub struct GetLogTagVerbosityLevel {
    #[doc = "Logging tag to change verbosity level"]
    pub tag: String,
}
impl Method for GetLogTagVerbosityLevel {
    const TYPE: &'static str = "getLogTagVerbosityLevel";
    type Response = LogVerbosityLevel;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Adds a message to TDLib internal log. This is an offline method. Can be called before authorization. Can be called synchronously"]
pub struct AddLogMessage {
    #[doc = "The minimum verbosity level needed for the message to be logged, 0-1023 "]
    pub verbosity_level: i32,
    #[doc = "Text of a message to log"]
    pub text: String,
}
impl Method for AddLogMessage {
    const TYPE: &'static str = "addLogMessage";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Does nothing; for testing only. This is an offline method. Can be called before authorization"]
pub struct TestCallEmpty {}
impl Method for TestCallEmpty {
    const TYPE: &'static str = "testCallEmpty";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the received string; for testing only. This is an offline method. Can be called before authorization "]
pub struct TestCallString {
    #[doc = "String to return"]
    pub x: String,
}
impl Method for TestCallString {
    const TYPE: &'static str = "testCallString";
    type Response = TestString;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the received bytes; for testing only. This is an offline method. Can be called before authorization "]
pub struct TestCallBytes {
    #[doc = "Bytes to return"]
    pub x: String,
}
impl Method for TestCallBytes {
    const TYPE: &'static str = "testCallBytes";
    type Response = TestBytes;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the received vector of numbers; for testing only. This is an offline method. Can be called before authorization "]
pub struct TestCallVectorInt {
    #[doc = "Vector of numbers to return"]
    pub x: Vec<i32>,
}
impl Method for TestCallVectorInt {
    const TYPE: &'static str = "testCallVectorInt";
    type Response = TestVectorInt;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the received vector of objects containing a number; for testing only. This is an offline method. Can be called before authorization "]
pub struct TestCallVectorIntObject {
    #[doc = "Vector of objects to return"]
    pub x: Vec<TestInt>,
}
impl Method for TestCallVectorIntObject {
    const TYPE: &'static str = "testCallVectorIntObject";
    type Response = TestVectorIntObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the received vector of strings; for testing only. This is an offline method. Can be called before authorization "]
pub struct TestCallVectorString {
    #[doc = "Vector of strings to return"]
    pub x: Vec<String>,
}
impl Method for TestCallVectorString {
    const TYPE: &'static str = "testCallVectorString";
    type Response = TestVectorString;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the received vector of objects containing a string; for testing only. This is an offline method. Can be called before authorization "]
pub struct TestCallVectorStringObject {
    #[doc = "Vector of objects to return"]
    pub x: Vec<TestString>,
}
impl Method for TestCallVectorStringObject {
    const TYPE: &'static str = "testCallVectorStringObject";
    type Response = TestVectorStringObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the squared received number; for testing only. This is an offline method. Can be called before authorization "]
pub struct TestSquareInt {
    #[doc = "Number to square"]
    pub x: i32,
}
impl Method for TestSquareInt {
    const TYPE: &'static str = "testSquareInt";
    type Response = TestInt;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends a simple network request to the Telegram servers; for testing only. Can be called before authorization"]
pub struct TestNetwork {}
impl Method for TestNetwork {
    const TYPE: &'static str = "testNetwork";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Sends a simple network request to the Telegram servers via proxy; for testing only. Can be called before authorization "]
pub struct TestProxy {
    #[doc = "Proxy server IP address "]
    pub server: String,
    #[doc = "Proxy server port "]
    pub port: i32,
    #[serde(rename = "type")]
    #[doc = "Proxy type"]
    pub type_: ProxyType,
    #[doc = "Identifier of a datacenter, with which to test connection "]
    pub dc_id: i32,
    #[doc = "The maximum overall timeout for the request"]
    pub timeout: f64,
}
impl Method for TestProxy {
    const TYPE: &'static str = "testProxy";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Forces an updates.getDifference call to the Telegram servers; for testing only"]
pub struct TestGetDifference {}
impl Method for TestGetDifference {
    const TYPE: &'static str = "testGetDifference";
    type Response = Ok;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization"]
pub struct TestUseUpdate {}
impl Method for TestUseUpdate {
    const TYPE: &'static str = "testUseUpdate";
    type Response = Update;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Returns the specified error and ensures that the Error object is used; for testing only. This is an offline method. Can be called before authorization. Can be called synchronously "]
pub struct TestReturnError {
    #[doc = "The error to be returned"]
    pub error: Error,
}
impl Method for TestReturnError {
    const TYPE: &'static str = "testReturnError";
    type Response = Error;
}
