/// Errors, as defined in public_erros.h and public_errors_rare.h of the ts3-plugin SDK.
use libc::c_uint;

#[derive(PartialEq)]
#[repr(u32)] 
pub enum Error { 
	// general
	ok										= 0x00_00,
	undefined									= 0x00_01,
	not_implemented							= 0x00_02,
	ok_no_update								= 0x00_03,
	dont_notify								= 0x00_04,
	lib_time_limit_reached					= 0x00_05,

	// dunno
	command_not_found							= 0x01_00,
	unable_to_bind_network_port				= 0x01_01,
	no_network_port_available					= 0x01_02,

	// client
	client_invalid_id							= 0x02_00,
	client_nickname_inuse						= 0x02_01,
	client_protocol_limit_reached				= 0x02_03,
	client_invalid_type						= 0x02_04,
	client_already_subscribed					= 0x02_05,
	client_not_logged_in						= 0x02_06,
	client_could_not_validate_identity		= 0x02_07,
	client_invalid_password					= 0x02_08,
	client_too_many_clones_connected			= 0x02_09,
	client_version_outdated					= 0x02_0a,
	client_is_online							= 0x02_0b,
	client_is_flooding						= 0x02_0c,
	client_hacked								= 0x02_0d,
	client_cannot_verify_now					= 0x02_0e,
	client_login_not_permitted				= 0x02_0f,
	client_not_subscribed						= 0x02_10,

	// channel
	channel_invalid_id						= 0x03_00,
	channel_protocol_limit_reached			= 0x03_01,
	channel_already_in						= 0x03_02,
	channel_name_inuse						= 0x03_03,
	channel_not_empty							= 0x03_04,
	channel_cannot_delete_default				= 0x03_05,
	channel_default_require_permanent 		= 0x03_06,
	channel_invalid_flags						= 0x03_07,
	channel_parent_not_permanent				= 0x03_08,
	channel_maxclients_reached				= 0x03_09,
	channel_maxfamily_reached					= 0x03_0a,
	channel_invalid_order						= 0x03_0b,
	channel_no_filetransfer_supported 		= 0x03_0c,
	channel_invalid_password					= 0x03_0d,
	channel_is_private_channel				= 0x03_0e,
	channel_invalid_security_hash				= 0x03_0f,

	// server
	server_invalid_id							= 0x04_00,
	server_running							= 0x04_01,
	server_is_shutting_down					= 0x04_02,
	server_maxclients_reached					= 0x04_03,
	server_invalid_password					= 0x04_04,
	server_deployment_active					= 0x04_05,
	server_unable_to_stop_own_server			= 0x04_06,
	server_is_virtual							= 0x04_07,
	server_wrong_machineid					= 0x04_08,
	server_is_not_running						= 0x04_09,
	server_is_booting							= 0x04_0a,
	server_status_invalid						= 0x04_0b,
	server_modal_quit							= 0x04_0c,
	server_version_outdated					= 0x04_0d,
	server_duplicated_running					= 0x04_0e,

	// database
	database									= 0x05_00,
	database_empty_result						= 0x05_01,
	database_duplicate_entry					= 0x05_02,
	database_no_modifications					= 0x05_03,
	database_constraint						= 0x05_04,
	database_reinvoke							= 0x05_05,

	// parameter
	parameter_quote							= 0x06_00,
	parameter_invalid_count					= 0x06_01,
	parameter_invalid 						= 0x06_02,
	parameter_not_found						= 0x06_03,
	parameter_convert							= 0x06_04,
	paramter_invalid_size						= 0x06_05,
	parameter_missing							= 0x06_06,
	parameter_checksum						= 0x06_07,

	// unsorted, needs further investigation
	vs_critical								= 0x07_00,
	connection_lost							= 0x07_01,
	not_connected								= 0x07_02,
	no_cached_connection_info					= 0x07_03,
	currently_not_possible					= 0x07_04,
	failed_connection_initialisation			= 0x07_05,
	could_not_resolve_hostname				= 0x07_06,
	invalid_server_connection_hanlder_id		= 0x07_07,
	could_not_initialise_input_manager		= 0x07_08,
	clientlibrary_not_initialised				= 0x07_09,
	serverlibrary_not_initialised				= 0x07_0a,
	whisper_too_many_targets					= 0x07_0b,
	whisper_no_targets 						= 0x07_0c,

	// file transfer
	file_invalid_name							= 0x08_00,
	file_invalid_permissions					= 0x08_01,
	file_already_exists						= 0x08_02,
	file_not_found 							= 0x08_03,
	file_io_error 							= 0x08_04,
	file_invalid_transfer_id					= 0x08_05,
	file_invalid_path 						= 0x08_06,
	file_no_files_available					= 0x08_07,
	file_overwrite_excludes_resume			= 0x08_08,
	file_invalid_size							= 0x08_09,
	file_already_in_use						= 0x08_0a,
	file_could_not_open_connection			= 0x08_0b,
	file_no_space_left_on_device				= 0x08_0c,
	file_exceeds_file_system_maximum_size		= 0x08_0d,
	file_transfer_connection_timeout			= 0x08_0e,
	file_connection_lost 						= 0x08_0f,
	file_exceeds_supplied_size				= 0x08_10,
	file_transfer_complete					= 0x08_11,
	file_transfer_canceled					= 0x08_12,
	file_transfer_interrupted					= 0x08_13,
	file_transfer_server_quota_exceeded		= 0x08_14,
	file_transfer_client_quota_exceeded		= 0x08_15,
	file_transfer_reset						= 0x08_16,
	file_transfer_limit_reached				= 0x08_17,


	// sound
	sound_preprocessor_disabled				= 0x09_00,
	sound_internal_preprocessor				= 0x09_01,
	sound_internal_encoder					= 0x09_02,
	sound_internal_playback					= 0x09_03,
	sound_no_capture_device_available			= 0x09_04,
	sound_no_playback_device_available		= 0x09_05,
	sound_could_not_open_capture_device		= 0x09_06,
	sound_could_not_open_playback_device		= 0x09_07,
	sound_handler_has_device					= 0x09_08,
	sound_invalid_capture_device				= 0x09_09,
	sound_invalid_playback_device				= 0x09_0a,
	sound_invalid_wave						= 0x09_0b,
	sound_unsupported_wave					= 0x09_0c,
	sound_open_wave							= 0x09_0d,
	sound_internal_capture					= 0x09_0e,
	sound_device_in_use						= 0x09_0f,
	sound_device_already_registerred			= 0x09_10,
	sound_unknown_device						= 0x09_11,
	sound_unsupported_frequency				= 0x09_12,
	sound_invalid_channel_count				= 0x09_13,
	sound_read_wave							= 0x09_14,
	sound_need_more_data						= 0x09_15,
	sound_device_busy							= 0x09_16,
	sound_no_data 							= 0x09_17,
	sound_channel_mask_mismatch				= 0x09_18,

	// permissions
	permissions_invalid_group_id				= 0x0a_00,
	permissions_duplicate_entry				= 0x0a_01,
	permissions_invalid_perm_id				= 0x0a_02,
	permissions_empty_result					= 0x0a_03,
	permissions_default_group_forbidden		= 0x0a_04,
	permissions_invalid_size					= 0x0a_05,
	permissions_invalid_value					= 0x0a_06,
	permissions_group_not_empty				= 0x0a_07,
	permissions_client_insufficient			= 0x0a_08,
	permissions_insufficient_group_power		= 0x0a_09,
	permissions_insufficient_permission_power = 0x0a_0a,
	permissions_template_group_is_used		= 0x0a_0b,
	permissions 								= 0x0a_0c,

	// accounting
	accounting_virtualserver_limit_reached	= 0x0b_00,
	accounting_slot_limit_reached 			= 0x0b_01,
	accounting_license_file_not_found			= 0x0b_02,
	accounting_license_date_not_ok			= 0x0b_03,
	accounting_unable_to_connect_to_server	= 0x0b_04,
	accounting_unknown_error					= 0x0b_05,
	acounting_server_error					= 0x0b_06,
	accounting_instance_limit_reached			= 0x0b_07,
	accounting_instance_check_error			= 0x0b_08,
	accounting_license_file_invalid			= 0x0b_09,
	accounting_running_elsewhere				= 0x0b_0a,
	accounting_instance_duplicated			= 0x0b_0b,
	accounting_already_started				= 0x0b_0c,
	accounting_not_started					= 0x0b_0d,
	accounting_to_many_starts					= 0x0b_0e,

	// messages
	message_invalid_id						= 0x0c_00,

	// ban
	ban_invalid_id							= 0x0d_00,
	connect_failed_banned						= 0x0d_01,
	rename_failed_banned						= 0x0d_02,
	ban_flooding								= 0x0d_03,

	// tts
	tts_unable_to_initialize					= 0x0e_00,

	// priviledge key
	priviledge_key_invalid					= 0x0f_00,

	// voip
	voip_pjsua								= 0x10_00,
	voip_already_initialized					= 0x10_01,
	voip_too_many_accounts					= 0x10_02,
	voip_invalid_account						= 0x10_03,
	voip_internal_error						= 0x10_04,
	voip_invalid_connectionId					= 0x10_05,
	voip_cannot_answer_initiated_call			= 0x10_06,
	voip_not_initialized						= 0x10_07,


	// provisioning server
	provisioning_invalid_password				= 0x11_00,
	provisioning_invalid_request				= 0x11_01,
	provisioning_no_slots_available			= 0x11_02,
	provisioning_pool_missing					= 0x11_03,
	provisioning_pool_unkown					= 0x11_04,
	provisioning_unknown_ip_location			= 0x11_05,
	provisioning_internal_tried_exceeded		= 0x11_06,
	provisioning_too_many_slots_requested		= 0x11_07,
	provisioning_too_many_reserved			= 0x11_08,
	provisioning_could_not_connect			= 0x11_09,
	provisioning_auth_server_not_connected	= 0x11_10,
	provisioning_auth_data_too_large			= 0x11_11,
	provisioning_already_initialized			= 0x11_12,
	provisioning_not_initialized				= 0x11_13,
	provisioning_connecting					= 0x11_14,
	provisioning_already_connected			= 0x11_15,
	provisioning_not_connected 				= 0x11_16,
	provisioning_io_error						= 0x11_17,
	provisioning_invalid_timeout				= 0x11_18,
	provisioning_ts3sever_not_found			= 0x11_19,
	provisioning_no_permission				= 0x11_1A,
}

impl Into<u32> for Error {
	fn into(self) -> u32 {
		unsafe {
			::std::mem::transmute(self)
		}
	}
}

impl From<u32> for Error {
	fn from(val: u32) -> Error {
		unsafe {
			::std::mem::transmute(val)
		}
	}
}
