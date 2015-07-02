// public_definitions.h

const TS3_MAX_SIZE_CHANNEL_NAME: usize					= 40;
const TS3_MAX_SIZE_VIRTUALSERVER_NAME: usize			= 64;
const TS3_MAX_SIZE_CLIENT_NICKNAME: usize				= 64;
const TS3_MIN_SIZE_CLIENT_NICKNAME: usize				= 3;
const TS3_MAX_SIZE_REASON_MESSAGE: usize				= 80;

const TS3_MAX_SIZE_TEXTMESSAGE: usize					= 1024;


// TODO


enum LogLevel {
	LogLevel_CRITICAL = 0, //these messages stop the program
	LogLevel_ERROR,        //everything that is really bad, but not so bad we need to shut down
	LogLevel_WARNING,      //everything that *might* be bad
	LogLevel_DEBUG,        //output that might help find a problem
	LogLevel_INFO,         //informational output, like "starting database version x.y.z"
	LogLevel_DEVEL         //developer only output (will not be displayed in release mode)
}