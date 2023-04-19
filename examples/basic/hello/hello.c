#include <string.h>
#include <unistd.h>

int main(void) {
  char *buf = "Hello!\n";
  write(1, buf, strlen(buf));
  return 0;
}
