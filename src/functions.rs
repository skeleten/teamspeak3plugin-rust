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
	

}