#ifndef GHOST_IDENTITY_MANAGER_H
#define GHOST_IDENTITY_MANAGER_H

#include <string>
#include <vector>
#include <map>

class Ghost_Identity_Manager {
public:
    Ghost_Identity_Manager();

    void loadIdentities(const std::string& directory);
    bool switchToIdentity(const std::string& identityName);
    std::string getCurrentIdentity() const;
    std::vector<std::string> listIdentities() const;

private:
    std::string activeIdentity;
    std::string identitiesRoot;
    std::map<std::string, std::string> identityPaths;

    bool applyMAC(const std::string& macFilePath);
    bool loadGPGProfile(const std::string& gpgKeyPath);
    bool loadBrowserProfile(const std::string& firefoxProfilePath);
};

#endif // GHOST_IDENTITY_MANAGER_H
