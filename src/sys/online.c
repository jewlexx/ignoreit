#ifdef _WIN32
#include <netfw.h>
#else
#include <netdb.h>
#endif

int IsOnline() {
  char hostname[18] = "https://github.com";
  struct hostent *hostinfo;

  hostinfo = gethostbyname(hostname);

  return hostinfo != NULL;
}