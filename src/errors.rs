/// Errors, as defined in public_erros.h and public_errors_rare.h of the ts3-plugin SDK.
use libc::c_uint;
use std;

#[derive(PartialEq)]
#[repr(u32)] 
pub enum Error { 
	// general
	ERROR_ok										= 0x00_00,
	ERROR_undefined									= 0x00_01,
	ERROR_not_implemented							= 0x00_02,
	ERROR_ok_no_update								= 0x00_03,
	ERROR_dont_notify								= 0x00_04,
	ERROR_lib_time_limit_reached					= 0x00_05,

	// dunno
	ERROR_command_not_found							= 0x01_00,
	ERROR_unable_to_bind_network_port				= 0x01_01,
	ERROR_no_network_port_available					= 0x01_02,

	// client
	ERROR_client_invalid_id							= 0x02_00,
	ERROR_client_nickname_inuse						= 0x02_01,
	ERROR_client_protocol_limit_reached				= 0x02_03,
	ERROR_client_invalid_type						= 0x02_04,
	ERROR_client_already_subscribed					= 0x02_05,
	ERROR_client_not_logged_in						= 0x02_06,
	ERROR_client_could_not_validate_identity		= 0x02_07,
	ERROR_client_invalid_password					= 0x02_08,
	ERROR_client_too_many_clones_connected			= 0x02_09,
	ERROR_client_version_outdated					= 0x02_0a,
	ERROR_client_is_online							= 0x02_0b,
	ERROR_client_is_flooding						= 0x02_0c,
	ERROR_client_hacked								= 0x02_0d,
	ERROR_client_cannot_verify_now					= 0x02_0e,
	ERROR_client_login_not_permitted				= 0x02_0f,
	ERROR_client_not_subscribed						= 0x02_10,

	// channel
	ERROR_channel_invalid_id						= 0x03_00,
	ERROR_channel_protocol_limit_reached			= 0x03_01,
	ERROR_channel_already_in						= 0x03_02,
	ERROR_channel_name_inuse						= 0x03_03,
	ERROR_channel_not_empty							= 0x03_04,
	ERROR_channel_cannot_delete_default				= 0x03_05,
	ERROR_channel_default_require_permanent 		= 0x03_06,
	ERROR_channel_invalid_flags						= 0x03_07,
	ERROR_channel_parent_not_permanent				= 0x03_08,
	ERROR_channel_maxclients_reached				= 0x03_09,
	ERROR_channel_maxfamily_reached					= 0x03_0a,
	ERROR_channel_invalid_order						= 0x03_0b,
	ERROR_channel_no_filetransfer_supported 		= 0x03_0c,
	ERROR_channel_invalid_password					= 0x03_0d,
	ERROR_channel_is_private_channel				= 0x03_0e,
	ERROR_channel_invalid_security_hash				= 0x03_0f,

	// server
	ERROR_server_invalid_id							= 0x04_00,
	ERROR_server_running							= 0x04_01,
	ERROR_server_is_shutting_down					= 0x04_02,
	ERROR_server_maxclients_reached					= 0x04_03,
	ERROR_server_invalid_password					= 0x04_04,
	ERROR_server_deployment_active					= 0x04_05,
	ERROR_server_unable_to_stop_own_server			= 0x04_06,
	ERROR_server_is_virtual							= 0x04_07,
	ERROR_server_wrong_machineid					= 0x04_08,
	ERROR_server_is_not_running						= 0x04_09,
	ERROR_server_is_booting							= 0x04_0a,
	ERROR_server_status_invalid						= 0x04_0b,
	ERROR_server_modal_quit							= 0x04_0c,
	ERROR_server_version_outdated					= 0x04_0d,
	ERROR_server_duplicated_running					= 0x04_0e,

	// database
	ERROR_database									= 0x05_00,
	ERROR_database_empty_result						= 0x05_01,
	ERROR_database_duplicate_entry					= 0x05_02,
	ERROR_database_no_modifications					= 0x05_03,
	ERROR_database_constraint						= 0x05_04,
	ERROR_database_reinvoke							= 0x05_05,

	// parameter
	ERROR_parameter_quote							= 0x06_00,
	ERROR_parameter_invalid_count					= 0x06_01,
	ERROR_parameter_invalid 						= 0x06_02,
	ERROR_parameter_not_found						= 0x06_03,
	ERROR_parameter_convert							= 0x06_04,
	ERROR_paramter_invalid_size						= 0x06_05,
	ERROR_parameter_missing							= 0x06_06,
	ERROR_parameter_checksum						= 0x06_07,

	// unsorted, needs further investigation
	ERROR_vs_critical								= 0x07_00,
	ERROR_connection_lost							= 0x07_01,
	ERROR_not_connected								= 0x07_02,
	ERROR_no_cached_connection_info					= 0x07_03,
	ERROR_currently_not_possible					= 0x07_04,
	ERROR_failed_connection_initialisation			= 0x07_05,
	ERROR_could_not_resolve_hostname				= 0x07_06,
	ERROR_invalid_server_connection_hanlder_id		= 0x07_07,
	ERROR_could_not_initialise_input_manager		= 0x07_08,
	ERROR_clientlibrary_not_initialised				= 0x07_09,
	ERROR_serverlibrary_not_initialised				= 0x07_0a,
	ERROR_whisper_too_many_targets					= 0x07_0b,
	ERROR_whisper_no_targets 						= 0x07_0c,

	// file transfer
	ERROR_file_invalid_name							= 0x08_00,
	ERROR_file_invalid_permissions					= 0x08_01,
	ERROR_file_already_exists						= 0x08_02,
	ERROR_file_not_found 							= 0x08_03,
	ERROR_file_io_error 							= 0x08_04,
	ERROR_file_invalid_transfer_id					= 0x08_05,
	ERROR_file_invalid_path 						= 0x08_06,
	ERROR_file_no_files_available					= 0x08_07,
	ERROR_file_overwrite_excludes_resume			= 0x08_08,
	ERROR_file_invalid_size							= 0x08_09,
	ERROR_file_already_in_use						= 0x08_0a,
	ERROR_file_could_not_open_connection			= 0x08_0b,
	ERROR_file_no_space_left_on_device				= 0x08_0c,
	ERROR_file_exceeds_file_system_maximum_size		= 0x08_0d,
	ERROR_file_transfer_connection_timeout			= 0x08_0e,
	ERROR_file_connection_lost 						= 0x08_0f,
	ERROR_file_exceeds_supplied_size				= 0x08_10,
	ERROR_file_transfer_complete					= 0x08_11,
	ERROR_file_transfer_canceled					= 0x08_12,
	ERROR_file_transfer_interrupted					= 0x08_13,
	ERROR_file_transfer_server_quota_exceeded		= 0x08_14,
	ERROR_file_transfer_client_quota_exceeded		= 0x08_15,
	ERROR_file_transfer_reset						= 0x08_16,
	ERROR_file_transfer_limit_reached				= 0x08_17,


	// sound
	ERROR_sound_preprocessor_disabled				= 0x09_00,
	ERROR_sound_internal_preprocessor				= 0x09_01,
	ERROR_sound_internal_encoder					= 0x09_02,
	ERROR_sound_internal_playback					= 0x09_03,
	ERROR_sound_no_capture_device_available			= 0x09_04,
	ERROR_sound_no_playback_device_available		= 0x09_05,
	ERROR_sound_could_not_open_capture_device		= 0x09_06,
	ERROR_sound_could_not_open_playback_device		= 0x09_07,
	ERROR_sound_handler_has_device					= 0x09_08,
	ERROR_sound_invalid_capture_device				= 0x09_09,
	ERROR_sound_invalid_playback_device				= 0x09_0a,
	ERROR_sound_invalid_wave						= 0x09_0b,
	ERROR_sound_unsupported_wave					= 0x09_0c,
	ERROR_sound_open_wave							= 0x09_0d,
	ERROR_sound_internal_capture					= 0x09_0e,
	ERROR_sound_device_in_use						= 0x09_0f,
	ERROR_sound_device_already_registerred			= 0x09_10,
	ERROR_sound_unknown_device						= 0x09_11,
	ERROR_sound_unsupported_frequency				= 0x09_12,
	ERROR_sound_invalid_channel_count				= 0x09_13,
	ERROR_sound_read_wave							= 0x09_14,
	ERROR_sound_need_more_data						= 0x09_15,
	ERROR_sound_device_busy							= 0x09_16,
	ERROR_sound_no_data 							= 0x09_17,
	ERROR_sound_channel_mask_mismatch				= 0x09_18,

	// permissions
	ERROR_permissions_invalid_group_id				= 0x0a_00,
	ERROR_permissions_duplicate_entry				= 0x0a_01,
	ERROR_permissions_invalid_perm_id				= 0x0a_02,
	ERROR_permissions_empty_result					= 0x0a_03,
	ERROR_permissions_default_group_forbidden		= 0x0a_04,
	ERROR_permissions_invalid_size					= 0x0a_05,
	ERROR_permissions_invalid_value					= 0x0a_06,
	ERROR_permissions_group_not_empty				= 0x0a_07,
	ERROR_permissions_client_insufficient			= 0x0a_08,
	ERROR_permissions_insufficient_group_power		= 0x0a_09,
	ERROR_permissions_insufficient_permission_power = 0x0a_0a,
	ERROR_permissions_template_group_is_used		= 0x0a_0b,
	ERROR_permissions 								= 0x0a_0c,

	// accounting
	ERROR_accounting_virtualserver_limit_reached	= 0x0b_00,
	ERROR_accounting_slot_limit_reached 			= 0x0b_01,
	ERROR_accounting_license_file_not_found			= 0x0b_02,
	ERROR_accounting_license_date_not_ok			= 0x0b_03,
	ERROR_accounting_unable_to_connect_to_server	= 0x0b_04,
	ERROR_accounting_unknown_error					= 0x0b_05,
	ERROR_acounting_server_error					= 0x0b_06,
	ERROR_accounting_instance_limit_reached			= 0x0b_07,
	ERROR_accounting_instance_check_error			= 0x0b_08,
	ERROR_accounting_license_file_invalid			= 0x0b_09,
	ERROR_accounting_running_elsewhere				= 0x0b_0a,
	ERROR_accounting_instance_duplicated			= 0x0b_0b,
	ERROR_accounting_already_started				= 0x0b_0c,
	ERROR_accounting_not_started					= 0x0b_0d,
	ERROR_accounting_to_many_starts					= 0x0b_0e,

	// messages
	ERROR_message_invalid_id						= 0x0c_00,

	// ban
	ERROR_ban_invalid_id							= 0x0d_00,
	ERROR_connect_failed_banned						= 0x0d_01,
	ERROR_rename_failed_banned						= 0x0d_02,
	ERROR_ban_flooding								= 0x0d_03,

	// tts
	ERROR_tts_unable_to_initialize					= 0x0e_00,

	// priviledge key
	ERROR_priviledge_key_invalid					= 0x0f_00,

	// voip
	ERROR_voip_pjsua								= 0x10_00,
	ERROR_voip_already_initialized					= 0x10_01,
	ERROR_voip_too_many_accounts					= 0x10_02,
	ERROR_voip_invalid_account						= 0x10_03,
	ERROR_voip_internal_error						= 0x10_04,
	ERROR_voip_invalid_connectionId					= 0x10_05,
	ERROR_voip_cannot_answer_initiated_call			= 0x10_06,
	ERROR_voip_not_initialized						= 0x10_07,


	// provisioning server
	ERROR_provisioning_invalid_password				= 0x11_00,
	ERROR_provisioning_invalid_request				= 0x11_01,
	ERROR_provisioning_no_slots_available			= 0x11_02,
	ERROR_provisioning_pool_missing					= 0x11_03,
	ERROR_provisioning_pool_unkown					= 0x11_04,
	ERROR_provisioning_unknown_ip_location			= 0x11_05,
	ERROR_provisioning_internal_tried_exceeded		= 0x11_06,
	ERROR_provisioning_too_many_slots_requested		= 0x11_07,
	ERROR_provisioning_too_many_reserved			= 0x11_08,
	ERROR_provisioning_could_not_connect			= 0x11_09,
	ERROR_provisioning_auth_server_not_connected	= 0x11_10,
	ERROR_provisioning_auth_data_too_large			= 0x11_11,
	ERROR_provisioning_already_initialized			= 0x11_12,
	ERROR_provisioning_not_initialized				= 0x11_13,
	ERROR_provisioning_connecting					= 0x11_14,
	ERROR_provisioning_already_connected			= 0x11_15,
	ERROR_provisioning_not_connected 				= 0x11_16,
	ERROR_provisioning_io_error						= 0x11_17,
	ERROR_provisioning_invalid_timeout				= 0x11_18,
	ERROR_provisioning_ts3sever_not_found			= 0x11_19,
	ERROR_provisioning_no_permission				= 0x11_1A,
}

impl Error {
	pub unsafe fn to_u32(self) -> u32 {
		std::mem::transmute(self)
	}

	pub unsafe fn from_u32(val: u32) -> Error {
		std::mem::transmute(val)
	}
}