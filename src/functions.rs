// mod definitions;
// mod defs;
use std::ffi::CStr;
use std::ffi::CString;
use libc::*;
use std::vec::Vec;

use definitions::*;
use errors::Error;


// low level function-pointers
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
	startConnection:							extern fn(c_ulong, *const c_char, *const c_char, c_uint, *const c_char, *const *const c_char, *const c_char, *const c_char) -> c_uint,
	stopConnection:								extern fn(c_ulong, *const c_char) -> c_uint,
	requestClientMove:							extern fn(c_ulong, c_ushort, c_ulong, *const c_char, *const c_char) -> c_uint, 
	requestClientVariables:						extern fn(c_ulong, c_ushort, *const c_char) -> c_uint,
	requestClientKickFromChannel:				extern fn(c_ulong, c_ushort, *const c_char, *const c_char) -> c_uint,
	requestClientKickFromServer:				extern fn(c_ulong, c_ushort, *const c_char, *const c_char) -> c_uint,
	requestChannelDelete:						extern fn(c_ulong, c_ulong, c_int, *const c_char) -> c_uint,
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

// threading markers
unsafe impl ::std::marker::Sync for TS3Functions { }

// higher level stuff ^-^
impl TS3Functions {
	pub unsafe fn get_client_lib_version(&self) -> Result<String, Error> {
		let mut foo: *mut c_char = ::std::ptr::null_mut();
		let err = (self.getClientLibVersion)(&mut foo);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			let cstr = CStr::from_ptr(foo);
			let string = ::std::str::from_utf8(cstr.to_bytes()).unwrap().to_owned();
			(self.freeMemory)(foo as *mut c_void);
			Ok(string)
		} else {
			Err(err)
		}
	}

	pub unsafe fn get_client_lib_version_number(&self) -> Result<u64, Error> {
		let mut version: c_ulong = 0;
		let err = (self.getClientLibVersionNumber)(&mut version);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(version as u64)
		} else {
			Err(err)
		}
	}

	pub unsafe fn spawn_new_server_connection_handler(&self, port: i32) -> Result<ServerConnectionHandler, Error> {
		let mut handler: c_ulong = 0;
		let err = (self.spawnNewServerConnectionHandler)(port, &mut handler);
		let err = Error::from(err);

		if err == Error::ERROR_ok {
			Ok(ServerConnectionHandler(handler as u64))
		} else {
			Err(err)
		}
	}

	pub unsafe fn destroy_server_connection_handler(&self, handler: ServerConnectionHandler) -> Result<(), Error> {
		let err = (self.destroyServerConnectionHandler)(handler.into());
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	// Error handling
	pub unsafe fn get_error_message(&self, errorCode: u32) -> Result<String, Error> {
		let mut foo: *mut c_char = ::std::ptr::null_mut();
		let err = (self.getErrorMessage)(errorCode as c_uint, &mut foo);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			let cstr = CStr::from_ptr(foo);
			let string = ::std::str::from_utf8(cstr.to_bytes()).unwrap().to_owned();
			(self.freeMemory)(foo as *mut c_void);
			Ok(string)
		} else {
			Err(err)
		}
	}

	// memory management being omitted, we won't use it in the abstract level

	// Logging
	pub unsafe fn log_message(&self, logMessage: String, severity: LogLevel, channel: String, logId: u64) -> Result<(), Error> {
		let message_ptr = CString::new(logMessage).unwrap();
		let channel_ptr = CString::new(channel).unwrap();
		let err = (self.logMessage)(message_ptr.as_ptr(), severity, channel_ptr.as_ptr(), logId as c_ulong);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	// TODO: sound

	// preprocesor
	pub unsafe fn get_preprocessor_info_value_float(&self, handler: ServerConnectionHandler, identifier: String) -> Result<f32, Error> {
		let ServerConnectionHandler(h) = handler;
		let mut result: f32 = 0.0;
		let ident_cstr = CString::new(identifier).unwrap();
		let err = (self.getPreProcessorInfoValueFloat)(h as c_ulong, ident_cstr.as_ptr(), &mut result);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(result)
		} else {
			Err(err)
		}
	}

	pub unsafe fn get_preprocessor_config_value(&self, handler: ServerConnectionHandler, identifier: String) -> Result<String, Error> {
		let ServerConnectionHandler(h) = handler;
		let mut foo: *mut c_char = ::std::ptr::null_mut();
		let ident_cstr = CString::new(identifier).unwrap();
		let err = (self.getPreProcessorConfigValue)(h as c_ulong, ident_cstr.as_ptr(), &mut foo);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			let cstr = CStr::from_ptr(foo);
			let string = ::std::str::from_utf8(cstr.to_bytes()).unwrap().to_owned();
			(self.freeMemory)(foo as *mut c_void);
			Ok(string)
		} else {
			Err(err)
		}
	}

	pub unsafe fn set_preprocessor_config_value(&self, handler: ServerConnectionHandler, identifer: String, value: String) -> Result<(), Error> {
		let ServerConnectionHandler(h) = handler;
		let ident_ptr = CString::new(identifer).unwrap();
		let value_ptr = CString::new(value).unwrap();
		let err = (self.setPreProcessorConfigValue)(h as c_ulong, ident_ptr.as_ptr(), value_ptr.as_ptr());
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	// encoder
	pub unsafe fn get_encode_config_value(&self, handler: ServerConnectionHandler, identifier: String) -> Result<String, Error> {
		let ServerConnectionHandler(h) = handler;
		let mut foo: *mut c_char = ::std::ptr::null_mut();
		let ident_cstr = CString::new(identifier).unwrap();
		let err = (self.getEncodeConfigValue)(h as c_ulong, ident_cstr.as_ptr(), &mut foo);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			let result = ::std::str::from_utf8(CStr::from_ptr(foo).to_bytes()).unwrap().to_owned();
			(self.freeMemory)(foo as *mut c_void);
			Ok(result)
		} else {
			Err(err)
		}
	}

	// playback
	pub unsafe fn get_playback_config_value_as_float(&self, handler: ServerConnectionHandler, identifier: String) -> Result<f32, Error> {
		let ServerConnectionHandler(h) = handler;
		let mut result: f32 = 0.0;
		let ident_cstr = CString::new(identifier).unwrap();
		let err = (self.getPlaybackConfigValueAsFloat)(h as c_ulong, ident_cstr.as_ptr(), &mut result);
		let err = Error::from(err);

		if err == Error::ERROR_ok {
			Ok(result)
		} else {
			Err(err)
		}
	}

	pub unsafe fn set_playback_config_value(&self, handler: ServerConnectionHandler, identifier: String, value: String) -> Result<(), Error> {
		let ServerConnectionHandler(h) = handler;
		let ident_cstr = CString::new(identifier).unwrap();
		let value_cstr = CString::new(value).unwrap();
		let err = (self.setPlaybackConfigValue)(h as c_ulong, ident_cstr.as_ptr(), value_cstr.as_ptr());
		let err = Error::from(err);

		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	pub unsafe fn set_client_volume_modifier(&self, handler: ServerConnectionHandler, client_id: u16, modifier: f32) -> Result<(), Error> {
		let ServerConnectionHandler(h) = handler;
		let err = (self.setClientVolumeModifier)(h as c_ulong, client_id as c_ushort, modifier as c_float);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	// recording
	pub unsafe fn start_voice_recording(&self, handler: ServerConnectionHandler) -> Result<(), Error> {
		let ServerConnectionHandler(h) = handler;
		let err = (self.startVoiceRecording)(h as c_ulong);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	pub unsafe fn stop_voice_recording(&self, handler: ServerConnectionHandler) -> Result<(), Error> {
		let ServerConnectionHandler(h) = handler;
		let err = (self.stopVoiceRecording)(h as c_ulong);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	// interaction with the server
	pub unsafe fn start_connection(
				&self, 
				handler: ServerConnectionHandler, 
				ident: String, 
				ip: String, 
				port: u32, 
				nickname: String, 
				defaultChannels: &mut Vec<String>, 
				defaultChannelPw: String, 
				serverPw: String) 
					-> Result<(), Error> {

		let ServerConnectionHandler(h) = handler;
		if defaultChannels.last().map(|s| !s.is_empty()).unwrap_or(false) {
			defaultChannels.push(String::new());
		};
		let channels: Vec<CString> = defaultChannels.into_iter().map(|s| CString::new(&**s).unwrap()).collect();
		let channel_ptrs: Vec<*const c_char> = channels.iter().map(|s| s.as_ptr()).collect();
		let ptr_channels: *const *const c_char = if channel_ptrs.is_empty() {
			::std::ptr::null()
		} else {
			channel_ptrs.as_ptr()
		};
		let ident_cstr = CString::new(ident).unwrap();
		let ip_cstr = CString::new(ip).unwrap();
		let nick_cstr = CString::new(nickname).unwrap();
		let defChanPw_cstr = CString::new(defaultChannelPw).unwrap();
		let defServPw_cstr = CString::new(serverPw).unwrap();

		let err = (self.startConnection)(
			h as c_ulong, 
			ident_cstr.as_ptr(), 
			ip_cstr.as_ptr(),
			port, 
			nick_cstr.as_ptr(), 
			ptr_channels, 
			defChanPw_cstr.as_ptr(), 
			defServPw_cstr.as_ptr());

		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	pub unsafe fn stop_connection(&self, handler: ServerConnectionHandler, quit_message: String) -> Result<(), Error> {
		let ServerConnectionHandler(h) = handler;
		let message_cstr = CString::new(quit_message).unwrap();
		let err = (self.stopConnection)(h as c_ulong, message_cstr.as_ptr());
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	pub unsafe fn request_client_move(&self, handler: ServerConnectionHandler, clientId: u16, newChannelId: u64, password: String, returnCode: String) -> Result<(), Error> {
		let ServerConnectionHandler(h) = handler;
		let password_cstr = CString::new(password).unwrap();
		let return_empty = returnCode.is_empty();
		let return_code_cstr = CString::new(returnCode).unwrap();
		let reutrn_code_ptr: *const c_char = if return_empty {
			::std::ptr::null()
		} else {
			return_code_cstr.as_ptr()
		};

		let err = (self.requestClientMove)(h as c_ulong, clientId as c_ushort, newChannelId as c_ulong, password_cstr.as_ptr(), reutrn_code_ptr);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	pub unsafe fn request_client_variables(&self, handler: ServerConnectionHandler, clientId: u16, returnCode: String) -> Result<(), Error> {
		let ServerConnectionHandler(h) = handler;
		let returnCode_cstr = CString::new(returnCode).unwrap();
		let err = (self.requestClientVariables)(h as c_ulong, clientId as c_ushort, returnCode_cstr.as_ptr());
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}	

	pub unsafe fn request_client_kick_from_channel(&self, handler: ServerConnectionHandler, clientId: u16, reason: String, returnCode: String) -> Result<(), Error> {
		let reason_cstr = CString::new(reason).unwrap();
		let ret_is_null = returnCode.is_empty();
		let returnCode_cstr = CString::new(returnCode).unwrap();
		let returnCode_ptr = if ret_is_null {
			::std::ptr::null()
		} else {
			returnCode_cstr.as_ptr()
		};
		let ServerConnectionHandler(h) = handler;
		let err = (self.requestClientKickFromChannel)(h as c_ulong, clientId as c_ushort, reason_cstr.as_ptr(), returnCode_ptr);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	pub unsafe fn request_client_kick_from_server(&self, handler: ServerConnectionHandler, clientId: u16, reason: String, returnCode: String) -> Result<(), Error> {
		let ServerConnectionHandler(h) = handler;
		let reason_cstr = CString::new(reason).unwrap();
		let ret_empty = returnCode.is_empty();
		let returnCode_cstr = CString::new(returnCode).unwrap();
		let returnCode_ptr = if ret_empty {
			::std::ptr::null()
		} else {
			returnCode_cstr.as_ptr()
		};

		let err = (self.requestClientKickFromServer)(h as c_ulong, clientId as c_ushort, reason_cstr.as_ptr(), returnCode_ptr);
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	pub unsafe fn request_channel_delete(&self, handler: ServerConnectionHandler, channel_id: u64, force: i32, return_code: String) -> Result<(), Error> {
		let return_code_cstr = CString::new(return_code).unwrap();
		let err = (self.requestChannelDelete)(handler.into(), channel_id as c_ulong, force as c_int, return_code_cstr.as_ptr());
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	pub unsafe fn request_send_private_text_msg(&self, handler: ServerConnectionHandler, message: String, client_id: u16, return_code: String) -> Result<(), Error> {
		let message_cstr = CString::new(message).unwrap();
		let return_code_cstr = CString::new(return_code).unwrap();

		let err = (self.requestSendPrivateTextMsg)(handler.into(), message_cstr.as_ptr(), client_id as c_short, return_code_cstr.as_ptr());
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	pub unsafe fn request_send_channel_text_msg(&self, handler: ServerConnectionHandler, message: String, target_channel_id: u16, return_code: String) -> Result<(), Error> {
		let message_cstr = CString::new(message).unwrap();
		let return_code_cstr = CString::new(return_code).unwrap();

		let err = (self.requestSendChannelTextMsg)(handler.into(), message_cstr.as_ptr(), target_channel_id as c_uint, return_code_cstr.as_ptr());
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	pub unsafe fn request_send_server_text_msg(&self, handler: ServerConnectionHandler, message: String, return_code: String) -> Result<(), Error> {
		let message_cstr = CString::new(message).unwrap();
		let return_code_cstr = CString::new(return_code).unwrap();

		let err = (self.requestSendServerTextMsg)(handler.into(), message_cstr.as_ptr(), return_code_cstr.as_ptr());
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	pub unsafe fn request_connection_info(&self, handler: ServerConnectionHandler, client_id: u16, return_code: String) -> Result<(), Error> {
		let return_code_cstr = CString::new(return_code).unwrap();

		let err = (self.requestConnectionInfo)(handler.into(), client_id as c_short, return_code_cstr.as_ptr());
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}

	pub unsafe fn request_client_set_whisper_list(&self, handler: ServerConnectionHandler, client_id: u16, target_channel_ids: Vec<u64>, target_client_ids: Vec<u16>, return_code: String) -> Result<(), Error> {
		let target_chan_ids_c: Vec<c_ulong> = target_channel_ids.into_iter().map(|id| id as c_ulong).collect();
		let target_client_ids_c: Vec<c_short> = target_client_ids.into_iter().map(|id| id as c_short).collect();
		let return_code_cstr = CString::new(return_code).unwrap();

		let err = (self.requestClientSetWhisperList)(handler.into(), client_id as c_short, target_chan_ids_c.as_ptr(), target_client_ids_c.as_ptr(), return_code_cstr.as_ptr());
		let err = Error::from(err);
		if err == Error::ERROR_ok {
			Ok(())
		} else {
			Err(err)
		}
	}
}