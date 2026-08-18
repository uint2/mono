#include <stdio.h>

typedef struct {
  const char *value;
  const int len;
} str;

typedef struct {
  const str name;
  const str desc;
  const str aliases[8];
} Thing;

#define S(VALUE) {.value = VALUE, .len = sizeof(VALUE) - 1}

#define Definition(NAME, DESC)                                                 \
  {                                                                            \
      .name = Str(NAME),                                                       \
      .desc = Str(DESC),                                                       \
  }

const Thing THINGS[] = {
    {.name = S("Hello")},
    {.name = S("HEYYY"), .aliases = {S(""), NULL}},
};

const int N = sizeof(THINGS) / sizeof(Thing);

int main() {
  for (int i = 0; i < N; ++i) {

    // printf("[%d]size = %lu\n", i, sizeof(THINGS[i].name.value));
    // printf("[%d] %s %d\n", i, THINGS[i].name.value, THINGS[i].name.len);
    fwrite(THINGS[i].name.value, THINGS[i].name.len, 1, stdout);
    fwrite(THINGS[i].desc.value, THINGS[i].desc.len, 1, stdout);
    fwrite("\n", 1, 1, stdout);
  }
  // yes.name = "hey";
  // printf("GOT HERE %s\n", yes.desc);
  return 0;
}
