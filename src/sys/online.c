#include <netfw.h>

int IsOnline() {
  char hostname[18] = "https://github.com";
  struct hostent *hostinfo;

  hostinfo = gethostbyname(hostname);

  return hostinfo != NULL;
}