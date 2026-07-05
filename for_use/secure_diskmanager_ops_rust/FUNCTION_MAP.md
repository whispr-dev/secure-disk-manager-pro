# Function map from uploaded C++ archive to Rust crate

## Core

| C++ file/pair | C++ function/class | Rust equivalent |
|---|---|---|
| `Core/DiskManagement.h/.cpp` | `DiskManagement::getDiskUsage` | `disk_management::get_disk_usage` |
| `Core/FileCompressor.h/.cpp` | `FileCompressor::compress` | `compression::compress` |
| `Core/FileCompressor.h/.cpp` | `FileCompressor::decompress` | `compression::decompress` |
| `Core/FileCompressor_MT_SIMD.h/.cpp` | `FileCompressorMT::compress` | `compression::compress_mt` |
| `Core/FileCompressor_MT_SIMD.h/.cpp` | `FileCompressorMT::decompress` | `compression::decompress_mt` |
| `Core/FileCompressor_MT_SIMD_Enc.h/.cpp` | `FileCompressorMTEnc::compress` | `compression::compress_mt_enc` |
| `Core/FileCompressor_MT_SIMD_Enc.h/.cpp` | `FileCompressorMTEnc::decompress` | `compression::decompress_mt_enc` |
| `Core/FileEncryption.h/.cpp` | `initializeSBox` | `encryption::initialize_sbox` |
| `Core/FileEncryption.h/.cpp` | `rc4EncryptDecrypt` | `encryption::rc4_encrypt_decrypt` |
| `Core/FileEncryption.h/.cpp` | `encryptFile` | `encryption::encrypt_file` |
| `Core/FileEncryption.h/.cpp` | `decryptFile` | `encryption::decrypt_file` |
| `Core/FileEncryption.h/.cpp` | `encryptFileWithKeyfile` | `encryption::encrypt_file_with_keyfile` |
| `Core/FileEncryption.h/.cpp` | `decryptFileWithKeyfile` | `encryption::decrypt_file_with_keyfile` |
| `Core/FileTransferManager.h/.cpp` | `upload` | `file_transfer::upload` |
| `Core/FileTransferManager.h/.cpp` | `download` | `file_transfer::download` |
| `Core/FileUploader.h/.cpp` | `send_via_smtp_tor` | `file_transfer::send_via_smtp_tor` returns `Blocked` |
| `Core/FileUploader.h/.cpp` | `send_via_ftp_tor` | `file_transfer::send_via_ftp_tor` returns `Blocked` |
| `Core/Payload_Dispatcher.h/.cpp` | `PayloadJob` | `payload_dispatcher::PayloadJob` |
| `Core/Payload_Dispatcher.h/.cpp` | `queue_payload` | `payload_dispatcher::queue_payload` |
| `Core/Payload_Dispatcher.h/.cpp` | `dispatch_all` | `payload_dispatcher::dispatch_all` with covert dispatch blocked |
| `Core/SearchEngine.h/.cpp` | `SearchResult` | `search::SearchResult` |
| `Core/SearchEngine.h/.cpp` | `findMatchingFiles` | `search::find_matching_files` |
| `Core/SearchEngine.h/.cpp` | `printResults` | `search::format_search_results` |
| `Core/SecureDeletion.h/.cpp` | `SecureDeletion::shredFile` | `secure_deletion::shred_file` |
| `Core/SecureDeletion_SIMD.h/.cpp` | `SecureDeletion_SIMD::shredFile` | `secure_deletion::shred_file_simd_pattern` |
| `Core/StealthMailer.h/.cpp` | `send_payload_securely` | blocked via `SdmError::Blocked` in safe design |
| `Core/StealthMailer.h/.cpp` | `verify_tor_connection` | blocked/not ported |
| `Core/StealthMailer.h/.cpp` | `decrypt_credentials` | blocked/not ported |
| `Core/UltraFastSearch.h/.cpp` | `FileEntry` | `search::FileEntry` |
| `Core/UltraFastSearch.h/.cpp` | `search` | `search::ultra_fast_search` |
| `Core/UltraFastSearch.h/.cpp` | `printResults` | `search::format_file_entries` |
| `Core/UltraFastSearch.h/.cpp` | `exportToCSV` | `search::export_to_csv` |

## GhostMode Vanisher

| C++ file/pair | C++ function/class | Rust equivalent |
|---|---|---|
| `GhostMode_Vanisher/include/GhostMode_FS_Monitor.h` + `src/GhostMode_FS_Monitor.cpp` | `Ghost_FS_Monitor` | `fs_monitor::FsMonitor` |
| same | `watchDirectory` | `fs_monitor::FsMonitor::watch_directory` |
| same | `stopWatching` | `fs_monitor::FsMonitor::stop_watching` |
| `Ghost_AutoWipe.h/.cpp` | `Ghost_AutoWipe` | `ghost_controller::GhostAutoWipe` |
| same | `arm` | blocked via `GhostAutoWipe::arm` |
| same | `abort` | `GhostAutoWipe::abort` |
| `Ghost_NetUtils.h/.cpp` | `changeMacAddress` | `net_admin::change_mac_address` returns `Blocked` |
| same | `generateRandomMac` | `net_admin::generate_random_mac` |
| `GhostStealthNet.h/.cpp` | `enableDnsCloak` | `net_admin::GhostStealthNet::enable_dns_cloak` records intent only |
| same | `setupProxyTunnel` | `net_admin::GhostStealthNet::setup_proxy_tunnel` records explicit config only |
| same | `heartbeat` | `net_admin::GhostStealthNet::heartbeat` |
| `Ghost_Tunnel.h/.cpp` | `initialize` | `net_admin::GhostTunnel::initialize` |
| same | `startTunnel` | blocked via `GhostTunnel::start_tunnel` |
| same | `stopTunnel` | `GhostTunnel::stop_tunnel` |
| same | `isRunning` | `GhostTunnel::is_running` |
| `PrimeHilbertIndexer.h/.cpp` | `tokenize` | `quantum::tokenize` |
| same | `primeIndices` | `quantum::prime_indices` |
| same | `toHilbert3D` | `quantum::to_hilbert_3d` |
| same | `hashFileToHilbert` | `quantum::hash_file_to_hilbert` |
| `QubitTypes.h/.cpp` | `Qubit`, `normalize`, `asArray`, `toString` | `quantum::Qubit` methods |
| same | `getPauliX` | `quantum::get_pauli_x` |
| same | `applyGate` | `quantum::apply_gate` |
| `QuantumEngine.h/.cpp` | `encodeTextAsQubit` | `quantum::encode_text_as_qubit` |
| same | `similarity` | `quantum::similarity` |
| `SIMD_Entropy.h/.cpp` | `calculateEntropy` | `entropy::calculate_entropy` / `calculate_entropy_bytes` |
| same | `calculateFileEntropy` | `entropy::calculate_file_entropy` |

## Identity and utils

| C++ file/pair | C++ function/class | Rust equivalent |
|---|---|---|
| `identity/IdentitySwitcher.h/.cpp` | `IdentitySwitcher` | `identity::IdentitySwitcher` |
| same | `switchToIdentity` | `identity::IdentitySwitcher::switch_to_identity` |
| same | `listIdentities` | `identity::IdentitySwitcher::list_identities` |
| `identity/Persona_Switcher.h/.cpp` | `Persona_Switcher` | `identity::PersonaSwitcher` |
| same | `setPersonaRoot` | `PersonaSwitcher::set_persona_root` |
| same | `listPersonas` | `PersonaSwitcher::list_personas` |
| same | `activatePersona` | `PersonaSwitcher::activate_persona` |
| `identity/Ghost_IdentityManager.h` + `Ghost_IdentitySwitcher.cpp` | `Ghost_Identity_Manager` | `identity::GhostIdentityManager` |
| same | `loadIdentities` | `GhostIdentityManager::load_identities` |
| same | `switchToIdentity` | `GhostIdentityManager::switch_to_identity` returns an activation plan; no MAC/browser mutation |
| same | `getCurrentIdentity` | `GhostIdentityManager::get_current_identity` |
| same | `listIdentities` | `GhostIdentityManager::list_identities` |
| `utils/DPAPIEncryptor.h/.cpp` | `encrypt_to_xml` | `dpapi::encrypt_to_xml` on Windows |
| `utils/DPAPIDecryptor.h/.cpp` | `decrypt_from_xml` | `dpapi::decrypt_from_xml` on Windows |
| `utils/GPGWrapper.h/.cpp` | `encryptFile`, `decryptFile`, `importKey`, `keyExists` | `gpg_wrapper::*` |
| `utils/GhostScriptVault.h/.cpp` | `decrypt_script` | `dpapi::decrypt_script` |
| same | `execute_decrypted_script` | `dpapi::execute_decrypted_script` returns `Blocked` |
| `utils/GhostVPNManager.h/.cpp` | `start_vpn_connection` | `net_admin::start_vpn_connection` explicit profile only |
| same | `is_vpn_active` | `net_admin::is_vpn_active` |
| `utils/Ghost_Daemon.h/.cpp` | `start`, `stop`, `install`, `uninstall` | `process_services::daemon_*`, install/uninstall blocked |
| `utils/Ghost_KillSwitch.h/.cpp` | kill-switch/wipe/self-delete routines | `process_services::*` blocked |
| `utils/Ghost_Watchdog.h/.cpp` | `GhostWatchdog` methods | `process_services::Watchdog` |
| `utils/KeyManager.h/.cpp` | keyfile routines | `key_manager::*` |
| `utils/MacSpoofer.h/.cpp` | `generateRandomMac`, `getCurrentMac` | `net_admin::generate_random_mac`, `get_current_mac` |
| same | `spoofMac` | `net_admin::spoof_mac` returns `Blocked` |
| `utils/ProfileRotator.h/.cpp` | `ProfileRotator` / `Persona` | `identity::ProfileRotator` / `identity::Persona` |
| `utils/RNG.h/.cpp`, `RNG_SIMD.h/.cpp` | RNG/keyfile functions | `rng::*` |
| `utils/SelfCleanup.h/.cpp` | `SelfCleanup::run` | `process_services::self_cleanup_run` returns `Blocked` |
| `utils/ServiceManager.h/.cpp` | service list/start/stop | `process_services::{list_services,start_service,stop_service}` |
| `utils/SystemMonitor.h/.cpp` | CPU/memory stats | `system_monitor::*` |

## Empty/stub headers in archive

`AutoSelfDestruct`, `FileLockGuard`, `GeneralFunctions`, `Status`, and `UIHelper` were empty or comment-only in the uploaded archive, so no operational Rust equivalent was generated beyond the safe blocked/general modules above.
