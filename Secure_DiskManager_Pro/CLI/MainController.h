#ifndef MAIN_CONTROLLER_H
#define MAIN_CONTROLLER_H

#include "command_context.h"

class MainController {
public:
    static CommandContext ParseArgs(int argc, char* argv[]);
};

#endif // MAIN_CONTROLLER_H
