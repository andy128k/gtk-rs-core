initSidebarItems({"fn":[["access","A wrapper for the POSIX `access()` function. This function is used to test a pathname for one or several of read, write or execute permissions, or just existence."],["application_name",""],["assert_warning",""],["assertion_message",""],["assertion_message_cmpstr",""],["base64_decode",""],["base64_encode",""],["bit_nth_lsf",""],["bit_nth_msf",""],["bit_storage",""],["build_filenamev",""],["build_pathv",""],["canonicalize_filename",""],["chdir",""],["check_version",""],["clear_error",""],["codeset",""],["compute_checksum_for_bytes",""],["compute_checksum_for_data",""],["compute_checksum_for_string",""],["compute_hmac_for_bytes",""],["compute_hmac_for_data",""],["compute_hmac_for_string",""],["console_charset",""],["dcgettext",""],["dgettext",""],["dngettext",""],["dpgettext",""],["dpgettext2",""],["environ",""],["file_get_contents",""],["file_open_tmp",""],["file_read_link",""],["file_set_contents",""],["file_set_contents_full",""],["file_test",""],["filename_display_basename",""],["filename_display_name",""],["format_size",""],["format_size_full",""],["host_name",""],["hostname_is_ascii_encoded","Tests if `hostname` contains segments with an ASCII-compatible encoding of an Internationalized Domain Name. If this returns [`true`], you should decode the hostname with [`hostname_to_unicode()`][crate::hostname_to_unicode()] before displaying it to the user."],["hostname_is_ip_address","Tests if `hostname` is the string form of an IPv4 or IPv6 address. (Eg, “192.168.0.1”.)"],["hostname_is_non_ascii","Tests if `hostname` contains Unicode characters. If this returns [`true`], you need to encode the hostname with [`hostname_to_ascii()`][crate::hostname_to_ascii()] before using it in non-IDN-aware contexts."],["hostname_to_ascii","Converts `hostname` to its canonical ASCII form; an ASCII-only string containing no uppercase letters and not ending with a trailing dot."],["hostname_to_unicode","Converts `hostname` to its canonical presentation form; a UTF-8 string in Unicode normalization form C, containing no uppercase letters, no forbidden characters, and no ASCII-encoded segments, and not ending with a trailing dot."],["language_names",""],["language_names_with_category",""],["listenv","Gets the names of all variables set in the environment."],["locale_variants",""],["main_current_source","Returns the currently firing source for this thread."],["main_depth","Returns the depth of the stack of calls to [`MainContext::dispatch()`][crate::MainContext::dispatch()] on any [`MainContext`][crate::MainContext] in the current thread. That is, when called from the toplevel, it gives 0. When called from within a callback from [`MainContext::iteration()`][crate::MainContext::iteration()] (or [`MainLoop::run()`][crate::MainLoop::run()], etc.) it returns 1. When called from within a callback to a recursive call to [`MainContext::iteration()`][crate::MainContext::iteration()], it returns 2. And so forth."],["markup_escape_text","Escapes text so that the markup parser will parse it verbatim. Less than, greater than, ampersand, etc. are replaced with the corresponding entities. This function would typically be used when writing out a file to be parsed with the markup parser."],["mkdir_with_parents","Create a directory if it doesn’t already exist. Create intermediate parent directories as needed, too."],["mkdtemp","Creates a temporary directory. See the `mkdtemp()` documentation on most UNIX-like systems."],["mkdtemp_full","Creates a temporary directory. See the `mkdtemp()` documentation on most UNIX-like systems."],["mkstemp_full","Opens a temporary file. See the `mkstemp()` documentation on most UNIX-like systems."],["monotonic_time",""],["num_processors",""],["on_error_query","Prompts the user with `[E]xit, [H]alt, show [S]tack trace or [P]roceed`. This function is intended to be used for debugging use only. The following example shows how it can be used together with the `g_log()` functions."],["on_error_stack_trace","Invokes gdb, which attaches to the current process and shows a stack trace. Called by [`on_error_query()`][crate::on_error_query()] when the “[S]tack trace” option is selected. You can get the current process’s program name with `g_get_prgname()`, assuming that you have called `gtk_init()` or `gdk_init()`."],["os_info",""],["path_get_basename","Gets the last component of the filename."],["path_get_dirname","Gets the directory components of a file name. For example, the directory component of `/usr/bin/test` is `/usr/bin`. The directory component of `/` is `/`."],["path_is_absolute","Returns [`true`] if the given `file_name` is an absolute file name. Note that this is a somewhat vague concept on Windows."],["path_skip_root","Returns a pointer into `file_name` after the root component, i.e. after the “/” in UNIX or “C:\" under Windows. If `file_name` is not an absolute path it returns [`None`]."],["pattern_match_simple","Matches a string against a pattern given as a string. If this function is to be called in a loop, it’s more efficient to compile the pattern once with `g_pattern_spec_new()` and call [`pattern_match_string()`][crate::pattern_match_string()] repeatedly."],["random_double","Returns a random `gdouble` equally distributed over the range [0..1)."],["random_double_range","Returns a random `gdouble` equally distributed over the range [`begin`..`end`)."],["random_int","Return a random `guint32` equally distributed over the range [0..2^32-1]."],["random_int_range","Returns a random `gint32` equally distributed over the range [`begin`..`end`-1]."],["random_set_seed","Sets the seed for the global random number generator, which is used by the g_random_* functions, to `seed`."],["real_time",""],["reload_user_special_dirs_cache","Resets the cache used for [`user_special_dir()`][crate::user_special_dir()], so that the latest on-disk version is used. Call this only if you just changed the data on disk yourself."],["return_if_fail_warning","Internal function used to print messages from the public `g_return_if_fail()` and `g_return_val_if_fail()` macros."],["rmdir","A wrapper for the POSIX `rmdir()` function. The `rmdir()` function deletes a directory from the filesystem."],["set_application_name","Sets a human-readable name for the application. This name should be localized if possible, and is intended for display to the user. Contrast with `g_set_prgname()`, which sets a non-localized name. `g_set_prgname()` will be called automatically by `gtk_init()`, but [`set_application_name()`][crate::set_application_name()] will not."],["shell_parse_argv","Parses a command line into an argument vector, in much the same way the shell would, but without many of the expansions the shell would perform (variable expansion, globs, operators, filename expansion, etc. are not supported). The results are defined to be the same as those you would get from a UNIX98 /bin/sh, as long as the input contains none of the unsupported shell expansions. If the input does contain such expansions, they are passed through literally. Possible errors are those from the `G_SHELL_ERROR` domain. Free the returned vector with `g_strfreev()`."],["shell_quote","Quotes a string so that the shell (/bin/sh) will interpret the quoted string to mean `unquoted_string`. If you pass a filename to the shell, for example, you should first quote it with this function. The return value must be freed with `g_free()`. The quoting style used is undefined (single or double quotes may be used)."],["shell_unquote","Unquotes a string as the shell (/bin/sh) would. Only handles quotes; if a string contains file globs, arithmetic operators, variables, backticks, redirections, or other special-to-the-shell features, the result will be different from the result a real shell would produce (the variables, backticks, etc. will be passed through literally instead of being expanded). This function is guaranteed to succeed if applied to the result of [`shell_quote()`][crate::shell_quote()]. If it fails, it returns [`None`] and sets the error. The `quoted_string` need not actually contain quoted or escaped text; [`shell_unquote()`][crate::shell_unquote()] simply goes through the string and unquotes/unescapes anything that the shell would. Both single and double quotes are handled, as are escapes including escaped newlines. The return value must be freed with `g_free()`. Possible errors are in the `G_SHELL_ERROR` domain."],["spaced_primes_closest","Gets the smallest prime number from a built-in array of primes which is larger than `num`. This is used within GLib to calculate the optimum size of a `GHashTable`."],["spawn_async","See `g_spawn_async_with_pipes()` for a full description; this function simply calls the `g_spawn_async_with_pipes()` without any pipes."],["spawn_check_exit_status","Set `error` if `exit_status` indicates the child exited abnormally (e.g. with a nonzero exit code, or via a fatal signal)."],["spawn_command_line_async","A simple version of [`spawn_async()`][crate::spawn_async()] that parses a command line with [`shell_parse_argv()`][crate::shell_parse_argv()] and passes it to [`spawn_async()`][crate::spawn_async()]. Runs a command line in the background. Unlike [`spawn_async()`][crate::spawn_async()], the [`SpawnFlags::SEARCH_PATH`][crate::SpawnFlags::SEARCH_PATH] flag is enabled, other flags are not. Note that [`SpawnFlags::SEARCH_PATH`][crate::SpawnFlags::SEARCH_PATH] can have security implications, so consider using [`spawn_async()`][crate::spawn_async()] directly if appropriate. Possible errors are those from [`shell_parse_argv()`][crate::shell_parse_argv()] and [`spawn_async()`][crate::spawn_async()]."],["stpcpy","Copies a nul-terminated string into the dest buffer, include the trailing nul, and return a pointer to the trailing nul byte. This is useful for concatenating multiple strings together without having to repeatedly scan for the end."],["system_config_dirs",""],["system_data_dirs",""],["unlink","A wrapper for the POSIX `unlink()` function. The `unlink()` function deletes a name from the filesystem. If this was the last link to the file and no processes have it opened, the diskspace occupied by the file is freed."],["user_cache_dir",""],["user_config_dir",""],["user_data_dir","Returns a base directory in which to access application data such as icons that is customized for a particular user."],["user_runtime_dir","Returns a directory that is unique to the current user on the local system."],["user_special_dir","Returns the full path of a special directory using its logical id."],["usleep","Pauses the current thread for the given number of microseconds."],["uuid_string_is_valid","Parses the string `str` and verify if it is a UUID."],["uuid_string_random","Generates a random UUID (RFC 4122 version 4) as a string. It has the same randomness guarantees as `GRand`, so must not be used for cryptographic purposes such as key generation, nonces, salts or one-time pads."],["warn_message","Internal function used to print messages from the public `g_warn_if_reached()` and `g_warn_if_fail()` macros."]]});