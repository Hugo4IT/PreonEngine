#pragma once

#include <stdio.h>

// For printing to the console with fancy
// colors, only gets included in debug build.
// Could possibly be upgraded in the future
// for actually logging to a file and stuff.

#define LOG_DEBUG_PREFIX "  \x1b[2m[DEBUG]: \x1b[0;36m"
#define LOG_DEBUG_SUFFIX   "\x1b[0m\n"
#define LOG_INFO_PREFIX "   \x1b[1;32m[INFO]: \x1b[0;36m"
#define LOG_INFO_SUFFIX    "\x1b[0m\n"
#define LOG_WARNING_PREFIX "\x1b[1;33m[WARNING]: \x1b[0;36m"
#define LOG_WARNING_SUFFIX "\x1b[0m\n"
#define LOG_ERROR_PREFIX "  \x1b[1;31m[ERROR]: \x1b[0;36m"
#define LOG_ERROR_SUFFIX   "\x1b[0m\n"

#ifdef DEBUG
#define debug(args...) printf(LOG_DEBUG_PREFIX); printf(args); printf(LOG_DEBUG_SUFFIX)
#define info(args...) printf(LOG_INFO_PREFIX); printf(args); printf(LOG_INFO_SUFFIX)
#define warning(args...) printf(LOG_WARNING_PREFIX); printf(args); printf(LOG_WARNING_SUFFIX)
#define error(args...) printf(LOG_ERROR_PREFIX); printf(args); printf(LOG_ERROR_SUFFIX)
#else
#define debug(args...)
#define info(args...)
#define warning(args...)
#define error(args...)
#endif