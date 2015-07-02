// mod definitions;
// mod defs;
use std::ffi::CStr;
use std::ffi::CString;
use libc::*;
use std;

use defsenum::*;
use defsother::*;
use errors::Error;

pub struct TS3Functions {
	getClientLibVersion:						extern fn(*mut *mut c_char) -> c_uint,
	getClientLibVersionNumber: 					extern fn(*mut c_ulong) -> c_uint,
	spawnNewServerConnectionHandler:			extern fn(c_int, *mut c_ulong) -> c_uint,
	destroyServerConnectionHandler:				extern fn(c_ulong) -> c_uint,

	// Error handling
	getErrorMessage:							extern fn(c_uint, *mut *mut c_char) -> c_uint,

	// Memory management
	freeMemory:									extern fn(*mut c_void) -> c_uint,

	// Logging
	logMessage:									extern fn(*const c_char, LogLevel, *const c_char, c_ulong) -> c_uint,

	// sound
	getPlayBackDeviceList:						extern fn(*const c_char, *mut *mut *mut *mut c_char) -> c_uint,
	getPlaybackModeList:						extern fn(*mut *mut *mut c_char) -> c_uint,
	getCaptureDeviceList:						extern fn(*const c_char, *mut *mut *mut *mut c_char) -> c_uint,
	getCaptureModeList:							extern fn(*mut *mut *mut c_char) -> c_uint,
	getDefaultPlaybackDevice:					extern fn(*const c_char, *mut *mut *mut c_char) -> c_uint,
	getDefaultCaptureMode:						extern fn(*mut *mut c_char) -> c_uint,
	openPlaybackDevice:							extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	openCaptureDevice:							extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	getCurrentPlaybackDeviceName:				extern fn(c_ulong, *mut *mut c_char, *mut c_int) -> c_uint,
	getCurrentPlayBackMode:						extern fn(c_ulong, *mut *mut c_char) -> c_uint,
	getCurrentCaptureDeviceName:				extern fn(c_ulong, *mut *mut c_char) -> c_uint,
	getCurrentCaptureMode:						extern fn(c_ulong, *mut *mut c_char) -> c_uint,
	initiateGracefulPlaybackShutdown:			extern fn(c_ulong) -> c_uint,
	closePlaybackDevice:						extern fn(c_ulong) -> c_uint,
	closeCaptureDevice:							extern fn(c_ulong) -> c_uint,
	activateCaptureDevice:						extern fn(c_ulong) -> c_uint,
	playWaveFileHandle:							extern fn(c_ulong, *const c_char, *mut c_ulong) -> c_uint,
	pauseWaveFileHandle:						extern fn(c_ulong, c_ulong, c_int) -> c_uint,
	closeWaveFileHandle:						extern fn(c_ulong, c_ulong) -> c_uint,
	registerCustomDevice:						extern fn(*const c_char, *const c_char, c_int, c_int, c_int, c_int) -> c_uint,
	unregisterCustomDevice:						extern fn(*const c_char) -> c_uint,
	processCustomCaptureData:					extern fn(*const c_char, *const c_short, c_int) -> c_uint,
	acquireCustomPlaybackData:					extern fn(*const c_char, *mut c_short, c_int) -> c_uint,

	// preprocessor
	getPreProcessorInfoValueFloat:				extern fn(c_ulong, *const c_char, *mut c_float) -> c_uint,
	getPreProcessorConfigValue:					extern fn(c_ulong, *const c_char, *mut *mut c_char) -> c_uint,
	setPreProcessorConfigValue:					extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,

	// encoder
	getEncodeConfigValue:						extern fn(c_ulong, *const c_char, *mut *mut c_char) -> c_uint,

	// playback
	getPlaybackConfigValueAsFloat:				extern fn(c_ulong, *const c_char, *mut c_float) -> c_uint,
	setPlaybackConfigValue:						extern fn(c_ulong, *const c_char, *const c_char) -> c_uint, 
	setClientVolumeModifier:					extern fn(c_ulong, c_ushort, c_float) -> c_uint,

	// recording
	startVoiceRecording:						extern fn(c_ulong) -> c_uint,
	stopVoiceRecording:							extern fn(c_ulong) -> c_uint,

	// interaction with the server
	startConnection:							extern fn(c_ulong, *const c_char, *const c_char, c_uint, *const c_char, *mut *const c_char, *const c_char, *const c_char) -> c_uint,
	stopConnection:								extern fn(c_ulong, *const c_char) -> c_uint,
	requestClientMove:							extern fn(c_ulong, c_ushort, c_ulong, *const c_char, *const c_char) -> c_uint, 
	requestClientVariables:						extern fn(c_ulong, c_ushort, c_ulong, *const c_char) -> c_uint,
	requestClientKickFromChannel:				extern fn(c_ulong, c_ushort, *const c_char, *const c_char) -> c_uint,
	requestClientKickFromServer:				extern fn(c_ulong, c_ushort, *const c_char, *const c_char) -> c_uint,
	requestChannelDelete:						extern fn(c_ulong, c_ulong, c_int, *const c_char) -> c_uint,
	requsetChannelMove:							extern fn(c_ulong, c_ulong, c_ulong, c_ulong, *const c_char) -> c_uint,
	requestSendPrivateTextMsg:					extern fn(c_ulong, *const c_char, c_short, *const c_char) -> c_uint,
	requestSendChannelTextMsg:					extern fn(c_ulong, *const c_char, c_ulong, *const c_char) -> c_uint,
	requestSendServerTextMsg:					extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	requestConnectionInfo:						extern fn(c_ulong, c_short, *const c_char) -> c_uint,
	requestClientSetWhisperList:				extern fn(c_ulong, c_short, *const c_ulong, *const c_short, *const c_char) -> c_uint,
	requestChannelSubscribe:					extern fn(c_ulong, *const c_ulong, *const c_char) -> c_uint,
	requestChannelSubscribeAll:					extern fn(c_ulong, *const c_char) -> c_uint,
	requestChannelUnsubscribe:					extern fn(c_ulong, *const c_ulong, *const c_char) -> c_uint,
	requestChannelUnsubscribeAll:				extern fn(c_ulong, *const c_char) -> c_uint,
	requestChannelDescription:					extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestMuteClients:							extern fn(c_ulong, *const c_short, *const c_char) -> c_uint,
	requsetUnmuteClients:						extern fn(c_ulong, *const c_short, *const c_char) -> c_uint,
	requestClientPoke:							extern fn(c_ulong, c_short, *const c_char, *const c_char) -> c_uint,
	requestClientIDs:							extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	clientChatClosed:							extern fn(c_ulong, *const c_char, c_short, *const c_char) -> c_uint,
	clientChatComposing:						extern fn(c_ulong, c_short, *const c_char) -> c_uint,
	requestServerTemporaryPasswordAdd:			extern fn(c_ulong, *const c_char, *const c_char, c_ulong, c_ulong, *const c_char, *const c_char) -> c_uint,
	requestServerTemporaryPasswordDel:			extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	requestServerTemporaryPasswordList:			extern fn(c_ulong, *const c_char) -> c_uint,

	// access clientlib information

	// query own client id
	getClientId:								extern fn(c_ulong, *mut c_short) -> c_uint,

	// client info
	getClientSelfVariableAsInt:					extern fn(c_ulong, size_t, *mut c_int) -> c_uint,
	getClientSelfVariableAsString:				extern fn(c_ulong, size_t, *mut *mut c_char) -> c_uint,
	setClientSelfVariableAsInt:					extern fn(c_ulong, size_t, c_int) -> c_uint,
	setClientSelfVariableAsString:				extern fn(c_ulong, size_t, *const c_char) -> c_uint,
	flushClientSelfUpdates:						extern fn(c_ulong, *const c_char) -> c_uint,
	getClientVariableAsInt:						extern fn(c_ulong, c_short, size_t, *mut c_int) -> c_uint,
	getClientVariableAsUInt64:					extern fn(c_ulong, c_short, size_t, *mut c_ulong) -> c_uint,
	getClientVariableAsString:					extern fn(c_ulong, c_short, size_t, *mut *mut c_char) -> c_uint,
	getClientList:								extern fn(c_ulong, *mut *mut c_short) -> c_uint,
	getChannelOfClient:							extern fn(c_ulong, c_short, *mut c_ulong) -> c_uint,

	// channel info
	getChannelVariableAsInt:					extern fn(c_ulong, c_ulong, size_t, *mut c_int) -> c_uint,
	getChannelVariableAsUInt64:					extern fn(c_ulong, c_ulong, size_t, *mut c_ulong) -> c_uint,
	getChannelVariableAsString:					extern fn(c_ulong, c_ulong, size_t, *mut *mut c_char) -> c_uint,
	getChannelIDFromChannelNames:				extern fn(c_ulong, *mut *mut c_char, *mut c_ulong) -> c_uint,
	setChannelVariableAsInt:					extern fn(c_ulong, c_ulong, size_t, c_int) -> c_uint,
	setChannelVariableAsUInt64:					extern fn(c_ulong, c_ulong, size_t, c_ulong) -> c_uint,
	setChannelVariableAsString:					extern fn(c_ulong, c_ulong, size_t, *const c_char) -> c_uint,
	flushChannelUpdates:						extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	flushChannelCreation:						extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	getChannelList:								extern fn(c_ulong, *mut *mut c_ulong) -> c_uint,
	getChannelClientList:						extern fn(c_ulong, c_ulong, *mut *mut c_short) -> c_uint,
	getParentChannelOfChannel:					extern fn(c_ulong, c_ulong, *mut c_ulong) -> c_uint,

	// server info
	getServerConnectionHandlerList:				extern fn(*mut *mut c_ulong) -> c_uint,
	getServerVariableAsInt:						extern fn(c_ulong, size_t, *mut c_int) -> c_uint,
	getServerVariableAsUInt64:					extern fn(c_ulong, size_t, *mut c_ulong) -> c_uint,
	getServerVariableAsString:					extern fn(c_ulong, size_t, *mut *mut c_char) -> c_uint,
	requestServerVariables:						extern fn(c_ulong) -> c_uint,

	// connection info
	getConnectionStatus:						extern fn(c_ulong, *mut c_int) -> c_uint,
	getConnectionVariableAsUInt64:				extern fn(c_ulong, c_short, size_t, *mut c_ulong) -> c_uint,
	getConnectionVariableAsDouble:				extern fn(c_ulong, c_short, size_t, *mut c_double) -> c_uint,
	getConnectionVariableAsString:				extern fn(c_ulong, c_short, size_t, *mut *mut c_char) -> c_uint,
	cleanUpConnectionInfo:						extern fn(c_ulong, c_short) -> c_uint,

	// client related
	requestClietnDBIDfromUID:					extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	requestClientNamefromUID:					extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	requestClientNamefromDBID:					extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestClientEditDescription:				extern fn(c_ulong, c_short, *const c_char, *const c_char) -> c_uint,
	requestClientSetIsTalker:					extern fn(c_ulong, c_int, *const c_char, *const c_char) -> c_uint,

	// plugin related
	requestSendClientQueryCommand:				extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,

	// Filetransfer
	getTransferFileName:						extern fn(c_short, *mut *mut c_char) -> c_uint,
	getTransferFilePath:						extern fn(c_short, *mut *mut c_char) -> c_uint,
	getTransferFileSize:						extern fn(c_short, *mut c_ulong) -> c_uint,
	getTransferFileSizeDone:					extern fn(c_short, *mut c_ulong) -> c_uint,
	isTransferSender:							extern fn(c_short, *mut c_int) -> c_uint,
	getTransferStatus:							extern fn(c_short, *mut c_int) -> c_uint,
	getCurrentTransferSpeed:					extern fn(c_short, *mut c_float) -> c_uint,
	getAverageTransferSpeed:					extern fn(c_short, *mut c_float) -> c_uint,
	getTransferRunTime:							extern fn(c_short, *mut c_ulong) -> c_uint,
	sendFile:									extern fn(c_ulong, c_ulong, *const c_char, *const c_char, c_int, c_int, *const c_char, *mut c_short, *const c_char) -> c_uint,
	requestFile:								extern fn(c_ulong, c_ulong, *const c_char, *const c_char, c_int, c_int, *const c_char, *mut c_short, *const c_char) -> c_uint,
	haltTransfer:								extern fn(c_ulong, c_short, c_int, *const c_char) -> c_uint,
	requestFileList:							extern fn(c_ulong, c_ulong, *const c_char, *const c_char, *const c_char) -> c_uint,
	requestFileInfo:							extern fn(c_ulong, c_ulong, *const c_char, *const c_char, *const c_char) -> c_uint,
	requestDeleteFile:							extern fn(c_ulong, c_ulong, *const c_char, *mut *const c_char, *const c_char) -> c_uint,
	requestCreateDirectory:						extern fn(c_ulong, c_ulong, *const c_char, *const char, *const char) -> c_uint,
	requsetRenameFile:							extern fn(c_ulong, c_ulong, *const c_char, c_ulong, *const c_char, *const c_char, *const c_char, *const c_char) -> c_uint,

	// Offline message management
	requestMessageAdd:							extern fn(c_ulong, *const c_char, *const c_char, *const c_char, *const c_char) -> c_uint,
	requestMessageDel:							extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestMessageGet:							extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestMessageList:							extern fn(c_ulong, *const c_char) -> c_uint,
	requestMessageUpdateFlag:					extern fn(c_ulong, c_ulong, c_int, *const c_char) -> c_uint,

	// Interacting with the server - confirming passwords
	verifyServerPassword:						extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	verifyChannelPassword:						extern fn(c_ulong, c_ulong, *const c_char, *const c_char) -> c_uint,

	// Interacting with the server - banning
	banclient:									extern fn(c_ulong, c_short, c_ulong, *const c_char, *const c_char) -> c_uint,
	banadd:										extern fn(c_ulong, *const c_char, *const c_char, *const c_char, c_ulong, *const c_char, *const c_char) -> c_uint,
	banclientdbid:								extern fn(c_ulong, c_ulong, c_ulong, *const c_char, *const c_char) -> c_uint,
	bandel:										extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	bandelall:									extern fn(c_ulong, *const c_char) -> c_uint,
	requestBanList:								extern fn(c_ulong, *const c_char) -> c_uint,

	// Interacting with the server - complain
	requestComplainAdd:							extern fn(c_ulong, c_ulong, *const c_char, *const c_char) -> c_uint,
	requestComplainDel:							extern fn(c_ulong, c_ulong, c_ulong, *const c_char) -> c_uint,
	requestComplainDelAll:						extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestComplainList:						extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,

	// Permissions
	requestServerGroupList:						extern fn(c_ulong, *const c_char) -> c_uint,
	requestServerGroupAdd:						extern fn(c_ulong, *const c_char, c_int, *const c_char) -> c_uint,
	requestServerGroupDel:						extern fn(c_ulong, c_ulong, c_int, *const c_char) -> c_uint,
	requestServerGroupAddClient:				extern fn(c_ulong, c_ulong, c_ulong, *const c_char) -> c_uint,
	requestServerGroupDelClient:				extern fn(c_ulong, c_ulong, c_ulong, *const c_char) -> c_uint,
	requestServerGroupsByClientID:				extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestServerGroupAddPerm:					extern fn(c_ulong, c_ulong, c_int, *const c_uint, *const c_int, *const c_int, *const c_int, c_int, *const c_char) -> c_uint,
	requestServerGroupDelPerm:					extern fn(c_ulong, c_ulong, c_int, *const c_uint, c_int, *const c_char) -> c_uint,
	requestServerGroupPermList:					extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestServerGroupClientList:				extern fn(c_ulong, c_ulong, c_int, *const c_char) -> c_uint,
	requestChannelGroupList:					extern fn(c_ulong, *const c_char) -> c_uint,
	requestChannelGroupAdd:						extern fn(c_ulong, *const c_char, c_int, *const c_char) -> c_uint,
	requestChannelGroupDel:						extern fn(c_ulong, c_ulong, c_int, *const c_char) -> c_uint,
	requestChanneLGroupAddPerm:					extern fn(c_ulong, c_ulong, c_int, c_int, *const c_uint, *const c_int, c_int, *const c_char) -> c_uint,
	requestChannelGroupDelPerm:					extern fn(c_ulong, c_ulong, c_int, *const c_uint, c_int, *const c_char) -> c_uint,
	requestChannelGroupPermList:				extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestSetClientChannelGroup:				extern fn(c_ulong, *const c_ulong, *const c_ulong, *const c_ulong, c_int, *const c_char) -> c_uint,
	requestChannelAddPerm:						extern fn(c_ulong, c_ulong, *const c_uint, *const c_int, c_int, *const c_char) -> c_uint,
	requestChannelDelPerm:						extern fn(c_ulong, c_ulong, *const c_uint, c_int, *const c_char) -> c_uint,
	requestChannelPermList:						extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestClientAddPerm:						extern fn(c_ulong, c_ulong, *const c_uint, *const c_int, *const c_int, c_int, *const c_char) -> c_uint,
	requestClientDelPerm:						extern fn(c_ulong, c_ulong, *const c_uint, c_int, *const c_char) -> c_uint,
	requestClientPermList:						extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestChannelClientAddPerm:				extern fn(c_ulong, c_ulong, c_ulong, *const c_uint, *const c_int, c_int, *const c_char) -> c_uint,
	requestChannelClientDelPerm:				extern fn(c_ulong, c_ulong, c_ulong, *const c_uint, c_int, *const c_char) -> c_uint,
	requestChannelClientPermList:				extern fn(c_ulong, c_ulong, c_ulong, *const c_char) -> c_uint,
	priviledgeKeyUse:							extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	requestPermissionList:						extern fn(c_ulong, *const c_char) -> c_uint,
	requestPermissionOverview:					extern fn(c_ulong, c_ulong, c_ulong, *const c_char) -> c_uint,

	// Helper functions
	clientPropertyStringToFlag:					extern fn(*const c_char, *mut size_t) -> c_uint,
	channelPropertyStringToFlag:				extern fn(*const c_char, *mut size_t) -> c_uint,
	serverPropertyStringToFlag:					extern fn(*const c_char, *mut size_t) -> c_uint,

	// Client functions
	getAppPath:									extern fn(*mut c_char, size_t),
	getResourcesPath:							extern fn(*mut c_char, size_t),
	getConfigPath:								extern fn(*mut c_char, size_t),
	getPluginPath:								extern fn(*mut c_char, size_t),
	getCurrentServerConnectionHandlerID: 		extern fn() -> c_ulong,
	printMessage:								extern fn(c_ulong, *const c_char, PluginMessageTarget),
	urlsToBB:									extern fn(*const c_char, *mut c_char, size_t),
	sendPluginCommand:							extern fn(c_ulong, *const c_char, *const c_char, c_int, *const c_short, *const c_char),
	getDirectories:								extern fn(*const c_char, *mut c_char, size_t),
	getServerConnectInfo:						extern fn(c_ulong, *mut c_char, *mut c_ushort, *mut c_char, size_t) -> c_uint,
	getChannelConnectionInfo:					extern fn(c_ulong, c_ulong, *mut c_char, *mut c_char, size_t) -> c_uint,
	createReturnCode:							extern fn(*const c_char, *mut c_char, size_t),
	requestInfoUpdate:							extern fn(c_ulong, PluginItemType, c_ulong) -> c_uint,
	getServerVersion:							extern fn(c_ulong) -> c_ulong,
	isWhispering:								extern fn(c_ulong, c_short, *mut c_int) -> c_uint,
	isReceivingWhisper:							extern fn(c_ulong, c_short, *mut c_int) -> c_uint,
	getAvatar:									extern fn(c_ulong, c_short, *mut c_char, size_t) -> c_uint,
	setPluginMenuEnabled:						extern fn(*const c_char, c_int, c_int),
	showHotkeySetup:							extern fn(),
	requestHotkeyInputDialog:					extern fn(*const c_char, *const c_char, c_int, *mut c_void),
	getHotkeyFromKeyword:						extern fn(*const c_char, *mut *const c_char, *mut *mut c_char, size_t, size_t) -> c_uint,
	getClientDisplayName:						extern fn(c_ulong, c_short, *mut c_char, size_t) -> c_uint,
	getBookmarkList:							extern fn(*mut *mut PluginBookmarkList) -> c_uint,
	getProfileList:								extern fn(PluginGuiProfile, *mut c_int, *mut *mut *mut c_char) -> c_uint,
	guiConnect:									extern fn(PluginConnectTab, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *mut c_ulong) -> c_uint,
	guiConnectBookmark:							extern fn(PluginConnectTab, *const c_char, *mut c_ulong) -> c_uint,
	createBookmark:								extern fn(*const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char) -> c_uint,
	getPermissionIDByName:						extern fn(c_ulong, *const c_char, *mut c_uint) -> c_uint,
	getClientNeededPermission:					extern fn(c_ulong, *const c_char, *mut c_int) -> c_uint
}

// higher level stuff ^-^
impl TS3Functions {
	pub unsafe fn get_client_lib_version(&self) -> Result<String, Error> {
		let mut foo: *mut c_char = std::ptr::null_mut();
		let err = (self.getClientLibVersion)(&mut foo);
		let err = Error::from_u32(err);
		if err == Error::ERROR_ok {
			let cstr = CStr::from_ptr(foo);
			let string = std::str::from_utf8(cstr.to_bytes()).unwrap().to_owned();
			(self.freeMemory)(foo as *mut c_void);
			Ok(string)
		} else {
			Err(err)
		}
	}

	pub unsafe fn get_client_lib_version_number(&self) -> Result<u64, Error> {
		let mut version: c_ulong = 0;
		let err = (self.getClientLibVersionNumber)(&mut version);
		let err = Error::from_u32(err);
		if err == Error::ERROR_ok {
			Ok(version as u64)
		} else {
			Err(err)
		}
	}

	pub unsafe fn spawn_new_server_connection_handler(&self, port: i32) -> Result<u64, Error> {
		let mut handler: c_ulong = 0;
		let err = (self.spawnNewServerConnectionHandler)(port, &mut handler);
		let err = Error::from_u32(err);

		if err == Error::ERROR_ok {
			Ok(handler as u64)
		} else {
			Err(err)
		}
	}

	pub unsafe fn destroy_server_connection_handler(&self, handler: u64) -> Result<(), Error> {
		let err = (self.destroyServerConnectionHandler)(handler as c_ulong);
		let err = Error::from_u32(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	// Error handling
	pub unsafe fn get_error_message(&self, errorCode: u32) -> Result<String, Error> {
		let mut foo: *mut c_char = std::ptr::null_mut();
		let err = (self.getErrorMessage)(errorCode as c_uint, &mut foo);
		let err = Error::from_u32(err);
		if err == Error::ERROR_ok {
			let cstr = CStr::from_ptr(foo);
			let string = std::str::from_utf8(cstr.to_bytes()).unwrap().to_owned();
			(self.freeMemory)(foo as *mut c_void);
			Ok(string)
		} else {
			Err(err)
		}
	}

	// memory management being omitted, we won't use it in the abstract level

	// Logging
	pub unsafe fn log_message(&self, logMessage: String, severity: LogLevel, channel: String, logId: u64) -> Result<(), Error> {
		let message_ptr = CString::new(logMessage).unwrap().as_ptr();
		let channel_ptr = CString::new(channel).unwrap().as_ptr();
		let err = (self.logMessage)(message_ptr, severity, channel_ptr, logId as c_ulong);
		let err = Error::from_u32(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	// TODO: sound

	// preprocesor
	pub unsafe fn get_preprocessor_info_value_float(&self, handler: u64, identifier: String) -> Result<f32, Error> {
		let mut result: f32 = 0.0;
		let err = (self.getPreProcessorInfoValueFloat)(handler as c_ulong, CString::new(identifier).unwrap().as_ptr(), &mut result);
		let err = Error::from_u32(err);
		if err == Error::ERROR_ok {
			Ok(result)
		} else {
			Err(err)
		}
	}

	pub unsafe fn get_preprocessor_config_value(&self, handler: u64, identifier: String) -> Result<String, Error> {
		let mut foo: *mut c_char = std::ptr::null_mut();
		let err = (self.getPreProcessorConfigValue)(handler as c_ulong, CString::new(identifier).unwrap().as_ptr(), &mut foo);
		let err = Error::from_u32(err);
		if err == Error::ERROR_ok {
			let cstr = CStr::from_ptr(foo);
			let string = std::str::from_utf8(cstr.to_bytes()).unwrap().to_owned();
			(self.freeMemory)(foo as *mut c_void);
			Ok(string)
		} else {
			Err(err)
		}
	}

	pub unsafe fn set_preprocessor_config_value(&self, handler: u64, identifer: String, value: String) -> Result<(), Error> {
		let ident_ptr = CString::new(identifer).unwrap().as_ptr();
		let value_ptr = CString::new(value).unwrap().as_ptr();
		let err = (self.setPreProcessorConfigValue)(handler as c_ulong, ident_ptr, value_ptr);
		let err = Error::from_u32(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	// encoder
	pub unsafe fn get_encode_config_value(&self, handler: u64, identifier: String) -> Result<String, Error> {
		let mut foo: *mut c_char = std::ptr::null_mut();
		let ident_ptr = CString::new(identifier).unwrap().as_ptr();
		let err = (self.getEncodeConfigValue)(handler as c_ulong, ident_ptr, &mut foo);
		let err = Error::from_u32(err);
		if err == Error::ERROR_ok {
			let result = std::str::from_utf8(CStr::from_ptr(foo).to_bytes()).unwrap().to_owned();
			(self.freeMemory)(foo as *mut c_void);
			Ok(result)
		} else {
			Err(err)
		}
	}
}