#ifndef HF_TOKENIZER_BRIDGE_H
#define HF_TOKENIZER_BRIDGE_H

#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

char* tokenize_text(const char* text, const char* model_name);
void free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif // HF_TOKENIZER_BRIDGE_H
