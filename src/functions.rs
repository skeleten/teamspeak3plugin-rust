// as in ts3_functions.h

use std::ffi::CStr;
use std::ffi::CString;
use libc::*;


mod definitions2;
use definitions::LogLevel;

struct TS3Functions {
	getClientLibVersion:				extern fn(CStr*) -> c_uint,
	getClientLibVersionNumber: 			extern fn(c_ulong*) -> c_uint,
	spawnNewServerConnectionHandler:	extern fn(c_int, c_ulong*) -> c_uint,
	destroyServerConnectionHandler:		extern fn(c_ulong) -> c_uint,

	// Error handling
	getErrorMessage:					extern fn(c_uint, CStr*) -> c_uint,

	// Memory management
	freeMemory:							extern fn(c_void*) -> c_uint,

	// Logging
	logMessage:							extern fn(*const c_char, LogLevel, *const c_char, c_ulong) -> c_uint,

	// sound
	getPlayBackDeviceList:				extern fn(*const c_char, ****c_char) -> c_uint,
	getPlaybackModeList:				extern fn(***c_char) -> c_uint,
	getCaptureDeviceList:				extern fn(*const c_char, ****c_char) -> c_uint,
	getCaptureModeList:					extern fn(***c_char) -> c_uint,
	getDefaultPlaybackDevice:			extern fn(*const c_char, ***char) -> c_uint,
	getDefaultCaptureMode:				extern fn(**c_char) -> c_uint,
	openPlaybackDevice:					extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	openCaptureDevice:					extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	getCurrentPlaybackDeviceName:		extern fn(c_ulong, **c_char, *c_int) -> c_uint,
	getCurrentPlayBackMode:				extern fn(c_ulong, **c_char) -> c_uint,
	getCurrentCaptureDeviceName:		extern fn(c_ulong, **c_char) -> c_uint,
	getCurrentCaptureMode:				extern fn(c_ulong, **c_char) -> c_uint,
	initiateGracefulPlaybackShutdown:	extern fn(c_ulong) -> c_uint,
	closePlaybackDevice:				extern fn(c_ulong) -> c_uint,
	closeCaptureDevice:					extern fn(c_ulong) -> c_uint,
	activateCaptureDevice:				extern fn(c_ulong) -> c_uint,
	playWaveFileHandle:					extern fn(c_ulong, *const c_char, *c_ulong) -> c_uint,
	pauseWaveFileHandle:				extern fn(c_ulong, c_ulong, c_int) -> c_uint,
	closeWaveFileHandle:				extern fn(c_ulong, c_ulong) -> c_uint,
	registerCustomDevice:				extern fn(*const c_char, *const c_char, c_int, c_int, c_int, c_int) -> c_uint,
	unregisterCustomDevice:				extern fn(*const c_char) -> c_uint,
	processCustomCaptureData:			extern fn(*const c_char, *const c_short, c_int) -> c_uint,
	acquireCustomPlaybackData:			extern fn(*const c_char, *c_short, c_int) -> c_uint,

	// preprocessor
	getPreProcessorInfoValueFloat:		extern fn(c_ulong, *const c_char, *c_float) -> c_uint,
	getPreProcessorConfigValue:			extern fn(c_ulong, *const c_char, **c_char) -> c_uint,
	setPreProcessorConfigValue:			extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,

	// encoder
	getEncodeConfigValue:				extern fn(c_ulong, *const c_char, **c_char) -> c_uint,

	// playback
	getPlaybackConfigValueAsFloat:		extern fn(c_ulong, *const c_char, *c_float) -> c_uint,
	setPlaybackConfigValue:				extern fn(c_ulong, *const c_char, *const c_char) -> c_uint, 
	setClientVolumeModifier:			extern fn(c_ulong, c_ushort, c_float) -> c_uint,

	// recording
	startVoiceRecording:				extern fn(c_ulong) -> c_uint,
	stopVoiceRecording:					extern fn(c_ulong) -> c_uint,

	// interaction with the server
	startConnection:					extern fn(c_ulong, *const c_char, *const c_char, c_uint, *const c_char, **const c_char, *const c_char, *const c_char) -> c_uint,
	stopConnection:						extern fn(c_ulong, *const c_char) -> c_uint,
	requestClientMove:					extern fn(c_ulong, c_ushort, c_ulong, *const c_char, *const c_char) -> c_uint, 
	requestClientVariables:				extern fn(c_ulong, c_ushort, c_ulong, *const c_char) -> c_uint,
	requestClientKickFromChannel:		extern fn(c_ulong, c_ushort, *const c_char, *const c_char) -> c_uint,
	requestClientKickFromServer:		extern fn(c_ulong, c_ushort, *const c_char, *const c_char) -> c_uint,
	requestChannelDelete:				extern fn(c_ulong, c_ulong, c_int, *const c_char) -> c_uint,
	requsetChannelMove:					extern fn(c_ulong, c_ulong, c_ulong, c_ulong, *const c_char) -> c_uint,
	requestSendPrivateTextMsg:			extern fn(c_ulong, *const c_char, c_short, *const c_char) -> c_uint,
	requestSendChannelTextMsg:			extern fn(c_ulong, *const c_char, c_ulong, *const c_char) -> c_uint,
	requestSendServerTextMsg:			extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	requestConnectionInfo:				extern fn(c_ulong, c_short, *const c_char) -> c_uint,
	requestClientSetWhisperList:		extern fn(c_ulong, c_short, *const c_ulong, *const c_short, *const c_char) -> c_uint,
	requestChannelSubscribe:			extern fn(c_ulong, *const c_ulong, *const c_char) -> c_uint,
	requestChannelSubscribeAll:			extern fn(c_ulong, *const c_char) -> c_uint,
	requestChannelUnsubscribe:			extern fn(c_ulong, *const c_ulong, *const c_char) -> c_uint,
	requestChannelUnsubscribeAll:		extern fn(c_ulong, *const c_char) -> c_uint,
	requestChannelDescription:			extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestMuteClients:					extern fn(c_ulong, *const c_short, *const c_char) -> c_uint,
	requsetUnmuteClients:				extern fn(c_ulong, *const c_short, *const c_char) -> c_uint,
	requestClientPoke:					extern fn(c_ulong, c_short, *const c_char, *const c_char) -> c_uint,
	requestClientIDs:					extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	clientChatClosed:					extern fn(c_ulong, *const c_char, c_short, *const c_char) -> c_uint,
	clientChatComposing:				extern fn(c_ulong, c_short, *const c_char) -> c_uint,
	requestServerTemporaryPasswordAdd:	extern fn(c_ulong, *const c_char, *const c_char, c_ulong, c_ulong, *const c_char, *const c_char) -> c_uint,
	requestServerTemporaryPasswordDel:	extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	requestServerTemporaryPasswordList:	extern fn(c_ulong, *const c_char) -> c_uint,

	// access clientlib information

	// query own client id
	getClientId:						extern fn(c_ulong, *c_short) -> c_uint,

	// client info
	getClientSelfVariableAsInt:			extern fn(c_ulong, size_t, *c_int) -> c_uint,
	getClientSelfVariableAsString:		extern fn(c_ulong, size_t, **c_char) -> c_uint,
	setClientSelfVariableAsInt:			extern fn(c_ulong, size_t, c_int) -> c_uint,
	setClientSelfVariableAsString:		extern fn(c_ulong, size_t, *const c_char) -> c_uint,
	flushClientSelfUpdates:				extern fn(c_ulong, *const c_char) -> c_uint,
	getClientVariableAsInt:				extern fn(c_ulong, c_short, size_t, *c_int) -> c_uint,
	getClientVariableAsUInt64:			extern fn(c_ulong, c_short, size_t, *c_ulong) -> c_uint,
	getClientVariableAsString:			extern fn(c_ulong, c_short, size_t, **c_char) -> c_uint,
	getClientList:						extern fn(c_ulong, **c_short) -> c_uint,
	getChannelOfClient:					extern fn(c_ulong, c_short, *c_ulong) -> c_uint,

	// channel info
	getChannelVariableAsInt:			extern fn(c_ulong, c_ulong, size_t, *c_int) -> c_uint,
	getChannelVariableAsUInt64:			extern fn(c_ulong, c_ulong, size_t, *c_ulong) -> c_uint,
	getChannelVariableAsString:			extern fn(c_ulong, c_ulong, size_t, **c_char) -> c_uint,
	getChannelIDFromChannelNames:		extern fn(c_ulong, **c_char, *c_ulong) -> c_uint,
	setChannelVariableAsInt:			extern fn(c_ulong, c_ulong, size_t, c_int) -> c_uint,
	setChannelVariableAsUInt64:			extern fn(c_ulong, c_ulong, size_t, c_ulong) -> c_uint,
	setChannelVariableAsString:			extern fn(c_ulong, c_ulong, size_t, *const c_char) -> c_uint,
	flushChannelUpdates:				extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	flushChannelCreation:				extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	getChannelList:						extern fn(c_ulong, **c_ulong) -> c_uint,
	getChannelClientList:				extern fn(c_ulong, c_ulong, **c_short) -> c_uint,
	getParentChannelOfChannel:			extern fn(c_ulong, c_ulong, *c_ulong) -> c_uint,

	// server info
	getServerConnectionHandlerList:		extern fn(**c_ulong) -> c_uint,
	getServerVariableAsInt:				extern fn(c_ulong, size_t, *c_int) -> c_uint,
	getServerVariableAsUInt64:			extern fn(c_ulong, size_t, *c_ulong) -> c_uint,
	getServerVariableAsString:			extern fn(c_ulong, size_t, **c_char) -> c_uint,
	requestServerVariables:				extern fn(c_ulong) -> c_uint,

	// connection info
	getConnectionStatus:				extern fn(c_ulong, *c_int) -> c_uint,
	getConnectionVariableAsUInt64:		extern fn(c_ulong, c_short, size_t, *c_ulong) -> c_uint,
	getConnectionVariableAsDouble:		extern fn(c_ulong, c_short, size_t, *c_double) -> c_uint,
	getConnectionVariableAsString:		extern fn(c_ulong, c_short, size_t, **c_char) -> c_uint,
	cleanUpConnectionInfo:				extern fn(c_ulong, c_short) -> c_uint,

	// client related
	requestClietnDBIDfromUID:			extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	requestClientNamefromUID:			extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	requestClientNamefromDBID:			extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestClientEditDescription:		extern fn(c_ulong, c_short, *const c_char, *const c_char) -> c_uint,
	requestClientSetIsTalker:			extern fn(c_ulong, c_int, *const c_char, *const c_char) -> c_uint,

	// plugin related
	requestSendClientQueryCommand:		extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,

	// Filetransfer
	getTransferFileName:				extern fn(c_short, **c_char) -> c_uint,
	getTransferFilePath:				extern fn(c_short, **c_char) -> c_uint,
	getTransferFileSize:				extern fn(c_short, *c_ulong) -> c_uint,
	getTransferFileSizeDone:			extern fn(c_short, *c_ulong) -> c_uint,
	isTransferSender:					extern fn(c_short, *c_int) -> c_uint,
	getTransferStatus:					extern fn(c_short, *c_int) -> c_uint,
	getCurrentTransferSpeed:			extern fn(c_short, *c_float) -> c_uint,
	getAverageTransferSpeed:			extern fn(c_short, *c_float) -> c_uint,
	getTransferRunTime:					extern fn(c_short, *c_ulong) -> c_uint,
	sendFile:							extern fn(c_ulong, c_ulong, *const c_char, *const c_char, c_int, c_int, *const c_char, *c_short, *const c_char) -> c_uint,
	requestFile:						extern fn(c_ulong, c_ulong, *const c_char, *const c_char, c_int, c_int, *const c_char, *c_short, *const c_char) -> c_uint,
	haltTransfer:						extern fn(c_ulong, c_short, c_int, *const c_char) -> c_uint,
	requestFileList:					extern fn(c_ulong, c_ulong, *const c_char, *const c_char, *const c_char) -> c_uint,
	requestFileInfo:					extern fn(c_ulong, c_ulong, *const c_char, *const c_char, *const c_char) -> c_uint,
	requestDeleteFile:					extern fn(c_ulong, c_ulong, *const c_char, **const c_char, *const c_char) -> c_uint,
	requestCreateDirectory:				extern fn(c_ulong, c_ulong, *const c_char, *const char, *const char) -> c_uint,
	requsetRenameFile:					extern fn(c_ulong, c_ulong, *const c_char, c_ulong, *const c_char, *const c_char, *const c_char, *const c_char) -> c_uint

	// Offline message management
	requestMessageAdd:					extern fn(c_ulong, *const c_char, *const c_char, *const c_char, *const c_char) -> c_uint,
	requestMessageDel:					extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestMessageGet:					extern fn(c_ulong, c_ulong, *const c_char) -> c_uint,
	requestMessageList:					extern fn(c_ulong, *const c_char) -> c_uint,
	requestMessageUpdateFlag:			extern fn(c_ulong, c_ulong, c_int, *const c_char) -> c_uint,

	// Interacting with the server - confirming passwords
	verifyServerPassword:				extern fn(c_ulong, *const c_char, *const c_char) -> c_uint,
	verifyChannelPassword:				extern fn(c_ulong, c_ulong, *const c_char, *const c_char) -> c_uint,
	
}