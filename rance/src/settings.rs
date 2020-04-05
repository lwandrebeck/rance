//
// settings.rs
//
// Copyright 2020 Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
// MA 02110-1301, USA.
//

//! # settings
//!
//! Here we manage every possible values coming from config files
//! /etc/rance/rance.cfg
//! ~/.rance.cfg
//! ./rance.cfg
//! ${RANCE_CONFIG}

use std::env;
use std::path::PathBuf;
use config::{ConfigError, Config, File, Environment};
use serde;

#[derive(Debug, Deserialize)]
struct Defaults {
	inventory						: PathBuf,
	library							: PathBuf,
	module_utils					: PathBuf,
	remote_tmp						: PathBuf,
	local_tmp						: PathBuf,
	plugin_filters_cfg				: PathBuf,
	forks							: u32,
	poll_interval					: u32,
	sudo_user						: String,
	ask_sudo_pass					: bool,
	ask_pass						: bool,
	transport						: String,
	remote_port						: u16,
	module_lang						: String,
	module_set_locale				: bool,
	gathering						: String,
	gather_subject					: String,
	gather_timeout					: u32,
	inject_facts_as_vars			: bool,
	roles_path						: bool,
	host_key_checking				: bool,
	stdout_callback					: String,
	// FIXME callback_whitelist like elem1, elem2, …
	callback_whitelist 				: String,
	task_includes_static			: bool,
	handler_includes_static			: bool,
	error_on_missing_handler 		: bool,
	sudo_exe						: String,
	sudo_flags						: String,
	timeout							: u32,
	remote_user						: String,
	log_path						: Option<PathBuf>,
	module_name						: String,
	executable						: PathBuf,
	hash_behaviour					: String,
	private_role_vars				: bool,
	// FIXME jinja2_extensions are stored like jinja2.ext.do,jinja2.ext.i18n
	jinja2_extensions				: String,
	private_key_file				: PathBuf,
	vault_password_file				: PathBuf,
	// FIXME need to take care of jinja2 format. Use of askama crate ?
	ansible_managed					: String,
	display_skipped_hosts			: bool,
	display_args_to_stdout			: bool,
	error_on_undefined_vars			: bool,
	system_warnings					: bool,
	deprecation_warnings			: bool,
	command_warnings				: bool,
	action_plugins					: PathBuf,
	become_plugins					: PathBuf,
	cache_plugins					: PathBuf,
	callback_plugins				: PathBuf,
	connection_plugins				: PathBuf,
	lookup_plugins					: PathBuf,
	inventory_plugins				: PathBuf,
	vars_plugins					: PathBuf,
	filter_plugins					: PathBuf,
	test_plugins					: PathBuf,
	terminal_plugins				: PathBuf,
	strategy_plugins				: PathBuf,
	strategy						: String,
	bin_ansible_callbacks			: bool,
	nocows							: bool,
	cow_selection					: String,
	// FIXME formatted like bud-frogs,bunny,cheese
	cow_whitelist					: String,
	nocolor							: bool,
	fact_caching					: String,
	fact_caching_connection			: PathBuf,
	retry_files_enabled				: bool,
	retry_files_save_path			: PathBuf,
	// FIXME formatted like apk,apt,dnf,homebrew,pacman,pkgng,yum,zypper
	squash_actions					: String,
	no_log							: bool,
	no_target_syslog				: bool,
	allow_world_readable_tmpfiles	: bool,
	var_compression_level			: u8,
	module_compression				: String,
	max_diff_size					: u64,
	// FIXME said to be removed in ansible 2.8
	merge_multiple_cli_flags		: bool,
	show_custom_stats				: bool,
	// FIXME formatted like ~, .orig, .bak, .ini, .cfg, .retry, .pyc, .pyo
	inventory_ignore_extensions		: String,
	// FIXME formatted like eos, nxos, ios, iosxr, junos, vyos
	network_group_modules			: String,
	allow_unsafe_lookups			: bool,
	any_errors_fatal				: bool
}

#[derive(Debug, Deserialize)]
struct Inventory {
	// FIXME formatted like host_list, virtualbox, yaml, constructed
	enable_plugins 					: String,
	// FIXME formatted like .pyc, .pyo, .swp, .bak, ~, .rpm, .md, .txt, ~, .orig, .ini, .cfg, .retry
	ignore_extensions				: String,
	// FIXME formatted like bla, foo, bar
	ignore_patterns					: String,
	unparsed_is_failed				: bool
}

#[derive(Debug, Deserialize)]
struct PrivilegeEscalation {
	r#become						: bool,
	become_method					: String,
	become_user						: String,
	become_ask_pass					: bool,
}

// FIXME use sessh or ssh2 crate name for struct ?
#[derive(Debug, Deserialize)]
struct ParamikoConnection {
	record_host_keys				: bool,
	pty								: bool,
	look_for_keys					: bool,
	host_key_auto_add				: bool
}

#[derive(Debug, Deserialize)]
struct SshConnection {
	ssh_args						: String,
	control_path_dir				: PathBuf,
	// FIXME formatted like %(directory)s/%%h-%%r
	control_path					: String,
	pipelining						: bool,
	// FIXME can be smart, True or False
	scp_if_ssh						: String,
	// FIXME can be sftp, scp, piped, smart
	transfer_method					: String,
	sftp_batch_mode					: bool,
	usetty							: bool,
	retries							: u8
}

#[derive(Debug, Deserialize)]
struct PersistantConnection {
	connect_timeout					: u32,
	command_timeout					: u32
}

#[derive(Debug, Deserialize)]
struct Accelerate {
	accelerate_port					: u16,
	accelerate_timeout				: u32,
	accelerate_connect_timeout		: f32,
	accelerate_daemon_timeout		: u32,
	accelerate_multi_key			: bool
}

#[derive(Debug, Deserialize)]
struct Selinux {
	// FIXME formatted like nfs,vboxsf,fuse,ramfs,9p,vfat
	special_context_filesystems		: String,
	libvirt_lxc_noseclabel			: bool
}

// FIXME use some Color type from a display crate
#[derive(Debug, Deserialize)]
struct Colors {
	highlight						: String,
	verbose							: String,
	warn							: String,
	error							: String,
	debug							: String,
	deprecate						: String,
	skip							: String,
	unreachable						: String,
	ok								: String,
	changed							: String,
	diff_add						: String,
	diff_remove						: String,
	diff_lines						: String
}

#[derive(Debug, Deserialize)]
struct Diff {
	always							: bool,
	context							: u32
}

#[derive(Debug, Deserialize)]
pub struct Settings {
	defaults: Defaults,
	inventory: Inventory,
	privilege_escalation: PrivilegeEscalation,
	paramiko_connection: ParamikoConnection,
	ssh_connection: SshConnection,
	persistant_connection: PersistantConnection,
	accelerate: Accelerate,
	selinux: Selinux,
	colors: Colors,
	diff: Diff
}

impl Settings {
	pub fn new() -> Result<Self, ConfigError> {
		let mut s = Config::new();
		// small diff to ansible, default config defines default values so we load it first.
		s.merge(File::with_name("/etc/rance/rance.cfg"))?;
		s.try_into()
		//~ match env::var("RANCE_CONFIG") {
			//~ Some(val) 	=> s.merge(File::with_name(val)),
			//~ None		=> { match s
		//~ }

		
	}
}
